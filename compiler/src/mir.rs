#[cfg(feature = "analysis")]
use crate::data_structures::{cdg, log_graph_to, visualize_graph};
use crate::data_structures::{original_cfg, truncated_cfg};
use crate::mir::ValueDef::Var;
use crate::monitor::{BinaryOp, UnaryOp};
use crate::rustc_middle::ty::inherent::Ty as NTy;
use crate::types::{ty_name, RuConstVal, RuPrim, RuTy};
use crate::utils::{span_to_path, ty_to_name, ty_to_t, tys_to_t};
#[cfg(feature = "analysis")]
use crate::writer::{MirObject, MirObjectBuilder, MirWriter};
use crate::{RuConfig, DOT_DIR, INSTRUMENTED_MIR_LOG_NAME, LOG_DIR};
use log::{debug, error, info, warn};
use petgraph::dot::Dot;
use rustc_apfloat::ieee::Double;
use rustc_ast::Mutability;
use rustc_hir::def_id::DefId;
use rustc_hir::{HirId, ItemKind};
use rustc_index::IndexVec;
use rustc_middle::mir::interpret::{Allocation, Scalar};
use rustc_middle::mir::visit::{MutVisitor, PlaceContext, TyContext};
use rustc_middle::mir::CallSource;
use rustc_middle::mir::ConstOperand;
use rustc_middle::mir::ConstValue;
use rustc_middle::mir::StatementKind::{Assign, SetDiscriminant};
use rustc_middle::mir::UnwindAction;
use rustc_middle::mir::{
    AssertMessage, BasicBlock, BasicBlockData, BinOp, Body, CastKind, HasLocalDecls, Local,
    LocalDecl, LocalDecls, Location, Operand, Place, PlaceElem, RetagKind, Rvalue, SourceInfo,
    SourceScope, SourceScopeData, Statement, StatementKind, SwitchTargets, Terminator,
    TerminatorKind, UnOp, UserTypeProjection, VarDebugInfo,
};
use rustc_middle::query::queries::optimized_mir::LocalKey;
use rustc_middle::query::queries::optimized_mir::ProvidedValue;
use rustc_middle::ty;
use rustc_middle::ty::layout::{HasTyCtxt, LayoutOf, MaybeResult};
use rustc_middle::ty::FloatTy;
use rustc_middle::ty::TyKind;
use rustc_middle::ty::ValTree;
use rustc_middle::ty::{
    CanonicalUserTypeAnnotation, Const, ConstKind, List, Region, RegionKind, ScalarInt, Ty, TyCtxt,
    TypeAndMut, UintTy, Variance,
};
use rustc_span::def_id::LocalDefId;
use rustc_span::source_map::Spanned;
use rustc_span::Span;
use rustc_target::abi::Size;
use rustc_target::abi::{Align, VariantIdx};
use std::borrow::Borrow;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::ops::{Add, Index};
use std::path::Path;
use std::str::FromStr;

pub const CUSTOM_OPT_MIR: for<'tcx> fn(_: TyCtxt<'tcx>, _: LocalDefId) -> &'tcx Body<'tcx> =
    |tcx, def| {
        let local_def_id = def;
        let def_id = def.to_def_id();
        // println!("{:#?}", def_id);
        let opt_mir = rustc_interface::DEFAULT_QUERY_PROVIDERS
            .borrow()
            .optimized_mir;
        let body = opt_mir(tcx, local_def_id).clone();
        let crate_name = tcx.crate_name(def_id.krate);
        let hir_id = tcx.local_def_id_to_hir_id(local_def_id);
        let body_ref = tcx.arena.alloc(body);

        let file_path = span_to_path(&tcx.def_span(def_id), &tcx);
        if file_path.is_none() {
            return body_ref;
        }

        info!("MIR: Scanning file {:?}", file_path.as_ref());
        if let Some(path) = file_path.as_ref() {
            if path.ends_with("rusty_monitor.rs") {
                return body_ref;
            }

            if let Some(path) = path.to_str() {
                if path.contains(".cargo") {
                    return body_ref;
                }
            }
        }

        if crate_name.as_str() != RuConfig::env_crate_name()
            || is_rusty_monitor(hir_id, &tcx)
            || !allowed_item(def_id)
        {
            // Don't instrument extern crates
            return body_ref;
        }

        let item_name = tcx.hir().opt_name(hir_id);
        if let None = item_name {
            return body_ref;
        };

        let global_id = def_id_to_str(def_id, &tcx).replace("::", "__");

        #[cfg(feature = "analysis")]
        info!("MIR: Analyzing {:?}", def);

        #[cfg(feature = "analysis")]
        let (cfg, _) = truncated_cfg(body_ref);
        #[cfg(feature = "analysis")]
        let cdg = cdg(&cfg);

        #[cfg(feature = "analysis")]
        let mut orig_blocks: Vec<String> = Vec::new();
        #[cfg(feature = "analysis")]
        let orig_truncated_cfg;
        #[cfg(feature = "analysis")]
        let locals_str;
        #[cfg(feature = "analysis")]
        {
            let mut basic_blocks = body_ref.basic_blocks.reverse_postorder();
            for basic_block in basic_blocks.iter() {
                let data = body_ref.basic_blocks.get(*basic_block);
                orig_blocks.push(format!("{} -> {:?}", basic_block.as_usize(), data));
            }
            // orig_blocks = body
            //     .basic_blocks
            //     .iter_enumerated()
            //     .map(|(block, data)| format!("{} -> {:?}", block.as_usize(), data))
            //     .collect::<Vec<_>>();

            if cfg!(file_writer) {
                let path = Path::new(DOT_DIR).join(format!("{}.dot", &global_id));
                visualize_graph(&cdg, &global_id);
            }

            let locals_decls: &LocalDecls = &body_ref.local_decls;
            locals_str = locals_decls
                .iter_enumerated()
                .map(|(local, decl)| format!("{:?} -> {:?}", local, decl))
                .collect::<Vec<_>>();
            let (cfg, _) = original_cfg(body_ref);
            let (truncated_cfg, _) = truncated_cfg(body_ref);
            orig_truncated_cfg = truncated_cfg;
        }

        #[cfg(feature = "instrumentation")]
        {
            info!("MIR: Instrumenting {:?}", def);
        }

        // INSTRUMENT OR ANALYZE
        let mut mir_visitor = MirVisitor::new(&global_id, body_ref.clone(), tcx);
        let mut instrumented_body = mir_visitor.visit();

        // Write down things that happen before the instrumentation
        #[cfg(feature = "analysis")]
        {
            let mut mir_object = MirObjectBuilder::default()
                .global_id(global_id.to_owned())
                .basic_blocks(orig_blocks)
                .cdg(serde_json::to_string(&cdg).unwrap())
                .cfg(format!("{}", Dot::new(&cfg)))
                .branches(mir_visitor.branches())
                .truncated_cfg(format!("{}", Dot::new(&orig_truncated_cfg)))
                .cdg_dot(format!("{}", Dot::new(&cdg)))
                .constant_pool(mir_visitor.constant_pool)
                .assertions(mir_visitor.assertions)
                .locals(locals_str)
                .build()
                .unwrap();
            MirWriter::write(&mir_object);
        }

        #[cfg(feature = "analysis")]
        {
            let basic_blocks = instrumented_body.clone().basic_blocks_mut().clone();
            let local_decls = instrumented_body.local_decls.clone();
            // let (basic_blocks, local_decls) = instrumented_body.basic_blocks_and_local_decls_mut();

            let locals = local_decls
                .iter_enumerated()
                .map(|(local, decl)| format!("{:?} -> {:?}", local, decl))
                .collect::<Vec<_>>();

            let blocks = basic_blocks
                .iter_enumerated()
                .map(|(block, data)| format!("{} -> {:?}", block.as_usize(), data))
                .collect::<Vec<_>>();

            let instrumented_mir_object = MirObjectBuilder::default()
                .global_id(global_id)
                .locals(locals)
                .basic_blocks(blocks)
                .cdg(serde_json::to_string(&cdg).unwrap())
                .cdg_dot(format!("{}", Dot::new(&cdg)))
                .build()
                .unwrap();
            MirWriter::write_instrumented(&instrumented_mir_object);
        }

        return tcx.arena.alloc(instrumented_body);
    };

pub fn def_id_to_str(def_id: DefId, tcx: &TyCtxt<'_>) -> String {
    tcx.def_path_str(def_id)
}

pub fn allowed_item(id: DefId) -> bool {
    let name = format!("{:?}", id);
    !(name.contains("serialize") || name.contains("deserialize") || name.contains("tests"))
}

pub struct MirVisitor<'tcx> {
    tcx: TyCtxt<'tcx>,
    body: Body<'tcx>,
    // We need this to pretend this to be a global id since we cannot access anything outside
    // of the optimized_mir function
    global_id: String,
    locals_num: usize,
    branch_counter: u64,
    cut_points: Vec<(BasicBlock, usize, Vec<(BasicBlock, BasicBlockData<'tcx>)>)>,
    basic_blocks_num: usize,
    instrumentation: Vec<(BasicBlock, Vec<BasicBlockData<'tcx>>)>,
    constant_pool: Vec<RuConstVal>,
    // Stores the computed branch distance for a variable for (true, false) branches
    alias_map: HashMap<Local, (Local, Local)>,
    assertions: u64,
}

impl<'tcx> MirVisitor<'tcx> {
    pub fn new(global_id: &str, body: Body<'tcx>, tcx: TyCtxt<'tcx>) -> Self {
        MirVisitor {
            tcx,
            global_id: global_id.to_string(),
            locals_num: body.local_decls.len(),
            basic_blocks_num: body.basic_blocks.len(),
            body,
            assertions: 0,
            branch_counter: 0,
            cut_points: vec![],
            instrumentation: vec![],
            constant_pool: vec![],
            alias_map: HashMap::new(),
        }
    }

    fn branches(&self) -> u64 {
        self.branch_counter
    }

    pub fn visit(&mut self) -> Body<'tcx> {
        let mut body = self.body.clone();
        self.visit_body(&mut body);
        body
    }

    fn switch_value_to_const(&self, switch_ty: Ty<'tcx>, value: u128) -> Const<'tcx> {
        // let param_env = ty::ParamEnv::empty();
        // let switch_ty = self.tcx.lift(switch_ty).unwrap();
        // let size = self.tcx.layout_of(param_env.and(switch_ty)).unwrap().size;
        // ty::Const::from_scalar(self.tcx, Scalar::from_uint(value, size), switch_ty)
        // 1. 创建空的参数环境（无泛型约束）
        let param_env = ty::ParamEnv::empty();

        // 2. 将类型与参数环境封装为 ParamEnvAnd
        let param_env_and_ty = ty::ParamEnvAnd {
            param_env,
            value: switch_ty,
        };

        // 3. 直接通过整数位值构建常量（无需手动处理布局和 Scalar）
        ty::Const::from_bits(self.tcx, value, param_env_and_ty)
    }

    fn mk_place(&self, index: usize) -> Place<'tcx> {
        Place {
            local: Local::from_usize(index),
            projection: List::empty(),
        }
    }

    fn mk_local_decl(&self, ty: Ty<'tcx>) -> LocalDecl<'tcx> {
        LocalDecl::new(ty, Span::default())
    }

    fn mk_const_f64(&self, data: f64) -> Const<'tcx> {
        // let const_arg = Const::from_value(
        //     self.tcx(),
        //     ConstValue::Scalar(Scalar::from_f64(Double::from_str("1.0").unwrap())),
        //     self.tcx.types.f64,
        // );

        // const_arg
        // 1. 将 f64 转换为 u64 位模式（IEEE 754 标准）
        let bits = 1.0_f64.to_bits();

        // 2. 将位模式包装为 ScalarInt
        let scalar_int =
            ScalarInt::try_from_uint(bits, Size::from_bytes(8)).expect("f64 should fit in 8 bytes");

        // 3. 构建 ValTree 的 Leaf 节点
        let val_tree = ValTree::from_scalar_int(scalar_int);

        // 4. 创建常量（显式指定类型为 f64）
        Const::new_value(
            self.tcx,
            val_tree,
            self.tcx.types.f64, // 确保类型系统识别为 f64
        )
    }

    fn mk_const_int(&self, data: u64) -> Const<'tcx> {
        let const_arg = Const::from_target_usize(self.tcx, data);
        const_arg
    }

    fn mk_const_str(&self, str: &str) -> Const<'tcx> {
        let str_ty = self.mk_str_ty();
        let bytes = str.as_bytes();

        // 在tcx的arena中分配ValTree切片
        let elements = self.tcx.arena.alloc_from_iter(bytes.iter().map(|&byte| {
            let scalar = ScalarInt::try_from_uint(byte as u128, Size::from_bytes(1))
                .expect("Byte exceeds u8 range");
            ValTree::Leaf(scalar)
        }));

        let val_tree = ValTree::Branch(elements);
        let const_kind = ConstKind::Value(str_ty, val_tree);
        Const::new(self.tcx, const_kind)
    }

    fn mk_const_bool(&self, flag: bool) -> Const<'tcx> {
        Const::from_bool(self.tcx, flag)
    }

    fn mk_str_ty(&self) -> Ty<'tcx> {
        let region = Region::new_from_kind(self.tcx, RegionKind::ReErased);
        Ty::new_ref(self.tcx, region, self.tcx.types.str_, Mutability::Not)
    }

    fn mk_move_operand(&self, local: Local) -> Operand<'tcx> {
        Operand::Move(<Place as From<Local>>::from(local))
    }

    fn mk_add_1_stmt(&self, result: Local, operand: Local) -> Statement<'tcx> {
        let operands = Rvalue::BinaryOp(
            BinOp::Add,
            Box::new((
                self.mk_move_operand(operand),
                self.mk_const_f64_operand(1.0),
            )),
        );

        self.mk_assign_stmt(self.mk_place(result.index()), operands)
    }

    fn mk_add_stmt(&self, result: Local, left: Local, right: Local) -> Statement<'tcx> {
        let operands = Rvalue::BinaryOp(
            BinOp::Add,
            Box::new((self.mk_move_operand(left), self.mk_move_operand(right))),
        );
        self.mk_assign_stmt(self.mk_place(result.index()), operands)
    }

    fn mk_sub_stmt(&self, result: Local, left: Local, right: Local) -> Statement<'tcx> {
        let operands = Rvalue::BinaryOp(
            BinOp::Sub,
            Box::new((self.mk_move_operand(left), self.mk_move_operand(right))),
        );
        self.mk_assign_stmt(self.mk_place(result.index()), operands)
    }

    fn mk_cast_operand_as_u64_stmt(&self, operand: Operand<'tcx>, to: Local) -> Statement<'tcx> {
        match operand {
            Operand::Copy(from) | Operand::Move(from) => {
                self.mk_cast_local_as_u64_stmt(from.local, to)
            }
            Operand::Constant(_) => self.mk_cast_const_as_u64_stmt(operand, to),
        }
    }

    fn mk_cast_const_as_u64_stmt(&self, operand: Operand<'tcx>, to: Local) -> Statement<'tcx> {
        let to_place = self.mk_place(to.index());
        let u64_ty = self.tcx.types.u64;

        let rvalue = Rvalue::Cast(CastKind::Transmute, operand, u64_ty);
        self.mk_assign_stmt(to_place, rvalue)
    }

    fn mk_cast_local_as_u64_stmt(&self, from: Local, to: Local) -> Statement<'tcx> {
        let to_place = self.mk_place(to.index());
        let u64_ty = self.tcx.types.u64;
        let rvalue = Rvalue::Cast(CastKind::Transmute, self.mk_move_operand(from), u64_ty);
        self.mk_assign_stmt(to_place, rvalue)
    }

    fn mk_cast_operand_as_f64_stmt(&self, operand: Operand<'tcx>, to: Local) -> Statement<'tcx> {
        match operand {
            Operand::Copy(from) | Operand::Move(from) => {
                self.mk_cast_local_as_f64_stmt(from.local, to)
            }
            Operand::Constant(_) => self.mk_cast_const_as_f64_stmt(operand, to),
        }
    }

    fn mk_cast_local_as_f64_stmt(&self, from: Local, to: Local) -> Statement<'tcx> {
        let to_place = self.mk_place(to.index());
        let f64_ty = self.tcx.types.f64;

        let rvalue = Rvalue::Cast(CastKind::Transmute, self.mk_move_operand(from), f64_ty);
        self.mk_assign_stmt(to_place, rvalue)
    }

    fn mk_cast_const_as_f64_stmt(&self, operand: Operand<'tcx>, to: Local) -> Statement<'tcx> {
        let to_place = self.mk_place(to.index());
        let f64_ty = self.tcx.types.f64;

        let rvalue = Rvalue::Cast(CastKind::Transmute, operand, f64_ty);
        self.mk_assign_stmt(to_place, rvalue)
    }

    fn mk_move_stmt(&self) -> Statement<'tcx> {
        Statement {
            source_info: self.mk_dummy_source_info(),
            kind: StatementKind::Nop,
        }
    }

    fn mk_assign_1_stmt(&self, place: Place<'tcx>) -> Statement<'tcx> {
        let rvalue = Rvalue::Use(self.mk_const_f64_operand(1.0f64));
        self.mk_assign_stmt(place, rvalue)
    }

    fn mk_assign_stmt(&self, place: Place<'tcx>, rvalue: Rvalue<'tcx>) -> Statement<'tcx> {
        Statement {
            source_info: self.mk_dummy_source_info(),
            kind: Assign(Box::new((place, rvalue))),
        }
    }

    fn mk_const_int_operand(&self, data: u64) -> Operand<'tcx> {
        Operand::Constant(Box::new(ConstOperand {
            span: Default::default(),
            user_ty: None,
            const_: rustc_middle::mir::Const::from_usize(self.tcx, data),
        }))
    }

    fn mk_const_f64_operand(&self, value: f64) -> Operand<'tcx> {
        Operand::Constant(Box::new(ConstOperand {
            span: Default::default(),
            user_ty: None,
            const_: rustc_middle::mir::Const::Ty(
                Ty::new_float(self.tcx, FloatTy::F64),
                self.mk_const_f64(value),
            ),
        }))
    }

    fn mk_const_str_operand(&self, str: &str) -> Operand<'tcx> {
        Operand::Constant(Box::new(ConstOperand {
            span: Default::default(),
            user_ty: None,
            const_: rustc_middle::mir::Const::Ty(
                Ty::new(self.tcx, TyKind::Str),
                self.mk_const_str(str),
            ),
        }))
    }

    fn mk_const_bool_operand(&self, flag: bool) -> Operand<'tcx> {
        Operand::Constant(Box::new(ConstOperand {
            span: Default::default(),
            user_ty: None,
            const_: rustc_middle::mir::Const::Ty(
                Ty::new(self.tcx, TyKind::Bool),
                self.mk_const_bool(flag),
            ),
        }))
    }

    fn mk_const_operand(&self, ty: Ty<'tcx>, val: ConstKind<'tcx>) -> Operand<'tcx> {
        let const_s = rustc_middle::ty::Const::new(self.tcx, val);

        Operand::Constant(Box::new(ConstOperand {
            span: Default::default(),
            user_ty: None,
            const_: rustc_middle::mir::Const::Ty(ty, const_s),
        }))
    }

    fn mk_resume_terminator(&mut self) -> Terminator<'tcx> {
        let terminator = Terminator {
            source_info: self.mk_dummy_source_info(),
            kind: TerminatorKind::UnwindResume,
        };

        terminator
    }

    fn mk_call_terminator(
        &mut self,
        args: Vec<Operand<'tcx>>,
        point_to: BasicBlock,
        fn_def_id: DefId,
    ) -> Terminator<'tcx> {
        let terminator_local = self.store_local_decl(Ty::new_unit(self.tcx));
        let terminator_place = self.mk_place(terminator_local.index());

        let fn_ty = self.tcx.type_of(fn_def_id).skip_binder();
        let func_const = Const::zero_sized(self.tcx, fn_ty);

        let func_constant = ConstOperand {
            span: Span::default(),
            user_ty: None,
            const_: rustc_middle::mir::Const::Ty(fn_ty, func_const),
        };

        let func_call = Operand::Constant(Box::new(func_constant));

        let terminator_kind = TerminatorKind::Call {
            func: func_call,
            args: args
                .into_iter()
                .map(|arg| Spanned {
                    node: arg,
                    span: Span::default(),
                })
                .collect::<Vec<_>>()
                .into_boxed_slice(),
            destination: terminator_place,
            target: Some(point_to),
            unwind: UnwindAction::Continue,
            call_source: CallSource::Normal,
            fn_span: Default::default(),
        };

        let terminator = Terminator {
            source_info: self.mk_dummy_source_info(),
            kind: terminator_kind,
        };

        terminator
    }

    fn mk_resume_basic_block(&mut self, stmts: Vec<Statement<'tcx>>) -> BasicBlockData<'tcx> {
        let terminator = self.mk_resume_terminator();
        self.mk_basic_block(stmts, terminator)
    }

    fn mk_basic_block(
        &self,
        stmts: Vec<Statement<'tcx>>,
        terminator: Terminator<'tcx>,
    ) -> BasicBlockData<'tcx> {
        let mut block = BasicBlockData::new(Some(terminator));

        for stmt in stmts {
            block.statements.push(stmt);
        }
        block
    }

    fn mk_dummy_source_info(&self) -> SourceInfo {
        SourceInfo {
            span: Default::default(),
            scope: SourceScope::from_usize(0usize),
        }
    }

    fn mk_enum_var_stmt(&mut self, local: Local, variant_idx: u32) -> Statement<'tcx> {
        let place_idx = local.index();
        let stmt_kind = SetDiscriminant {
            place: Box::new(self.mk_place(place_idx)),
            variant_index: VariantIdx::from_u32(variant_idx),
        };
        Statement {
            source_info: self.mk_dummy_source_info(),
            kind: stmt_kind,
        }
    }

    fn update_terminator(
        &self,
        terminator: &mut Terminator<'tcx>,
        idx: usize,
        basic_block: BasicBlock,
    ) {
        match &mut terminator.kind {
            TerminatorKind::SwitchInt { targets, .. } => {
                let targets = targets.all_targets_mut();
                targets[idx] = basic_block;
            }
            _ => {}
        }
    }

    fn store_unit_local_decl(&mut self) -> Local {
        let unit_ty = Ty::new_unit(self.tcx);
        self.store_local_decl(unit_ty)
    }

    fn store_f64_local_decl(&mut self) -> Local {
        let f64_ty = self.tcx.types.f64;
        self.store_local_decl(f64_ty)
    }

    fn store_local_decl(&mut self, ty: Ty<'tcx>) -> Local {
        let local_decl = self.mk_local_decl(ty);
        let local_decls = &mut self.body.local_decls;
        local_decls.push(local_decl);
        let local = Local::from_usize(self.locals_num);
        self.locals_num += 1;
        local
    }

    fn get_local_ty(&self, local: Local) -> Ty<'tcx> {
        let local_decls = self.body.local_decls();
        let decl = local_decls.get(local).unwrap();
        decl.ty
    }

    fn mk_trace_branch_hit(&mut self, target_block: usize) -> Vec<Operand<'tcx>> {
        let trace_call_args = vec![
            self.mk_const_int_operand(RuConfig::env_run()),
            self.mk_const_str_operand(&self.global_id),
            self.mk_const_int_operand(target_block as u64),
        ];

        trace_call_args
    }

    fn mk_trace_statements_binary_op(
        &mut self,
        target_block: u64,
        op: &BinaryOp,
        left: &Box<ValueDef<'tcx>>,
        right: &Box<ValueDef<'tcx>>,
        is_true_branch: bool,
    ) -> (Vec<Statement<'tcx>>, Vec<Operand<'tcx>>) {
        let op_def_id = get_binary_op_def_id(&self.tcx);
        let op_ty = self.tcx.type_of(op_def_id).skip_binder();
        let op_enum_local = self.store_local_decl(op_ty);
        let op_def_stmt = self.mk_enum_var_stmt(op_enum_local, (*op).into());

        // If operand is a variable, then we have to create a new one and move the value
        // to use it later in the trace call. If it's a const, we can use it directly
        // as argument
        let (left_operand_stmt, left_local) = match left.as_ref() {
            ValueDef::Const(ty, val) => {
                let left_local = self.store_local_decl(*ty);
                let operand = self.mk_const_operand(*ty, *val);

                (
                    self.mk_cast_const_as_f64_stmt(operand, left_local),
                    left_local,
                )
            }
            ValueDef::Var(place, _) => {
                let left_ty = self.get_local_ty(place.local);
                let left_local = self.store_local_decl(left_ty);
                (
                    self.mk_cast_local_as_f64_stmt(place.local, left_local),
                    left_local,
                )
            }
            _ => todo!("Operand is {:?}", left),
        };

        let (right_operand_stmt, right_local) = match right.as_ref() {
            ValueDef::Const(ty, val) => {
                let right_local = self.store_local_decl(*ty);
                let operand = self.mk_const_operand(*ty, *val);

                (
                    self.mk_cast_const_as_f64_stmt(operand, right_local),
                    right_local,
                )
            }
            ValueDef::Var(place, _) => {
                let right_ty = self.get_local_ty(place.local);
                let right_local = self.store_local_decl(right_ty);
                (
                    self.mk_cast_local_as_f64_stmt(place.local, right_local),
                    right_local,
                )
            }
            _ => todo!("Operand is {:?}", right),
        };

        let stmts = vec![op_def_stmt, left_operand_stmt, right_operand_stmt];

        let trace_call_args = vec![
            // Run
            self.mk_const_int_operand(RuConfig::env_run()),
            // Global id
            self.mk_const_str_operand(&self.global_id),
            // Block id
            self.mk_const_int_operand(target_block),
            self.mk_move_operand(left_local),
            self.mk_move_operand(right_local),
            self.mk_move_operand(op_enum_local),
            self.mk_const_bool_operand(is_true_branch),
        ];
        (stmts, trace_call_args)
    }

    fn mk_trace_statements_entry(&mut self) -> Vec<Operand<'tcx>> {
        let args = vec![
            // Run
            self.mk_const_int_operand(RuConfig::env_run()),
            self.mk_const_str_operand(&self.global_id),
        ];

        args
    }

    fn mk_trace_statements_switch_int(
        &mut self,
        target_block: u64,
        switch_operand_def: &ValueDef<'tcx>,
        branch_value: Option<Const<'tcx>>,
        is_true_branch: bool,
        is_hit: bool,
    ) -> (Vec<Statement<'tcx>>, Vec<Operand<'tcx>>) {
        match switch_operand_def {
            ValueDef::BinaryOp(op, left, right) => {
                self.mk_trace_statements_binary_op(target_block, op, left, right, is_true_branch)
            }
            ValueDef::Const(ty, val) => (vec![], vec![]),

            ValueDef::Discriminant(_) => {
                let stmts = vec![];

                let trace_call_args = vec![
                    // Run
                    self.mk_const_int_operand(RuConfig::env_run()),
                    // Global id
                    self.mk_const_str_operand(&self.global_id),
                    // Block id
                    self.mk_const_int_operand(target_block as u64),
                    self.mk_const_bool_operand(is_hit),
                ];

                (stmts, trace_call_args)
            }
            ValueDef::UnaryOp(op, inner_value_def) => match op {
                UnaryOp::Not => self.mk_trace_statements_switch_int(
                    target_block,
                    inner_value_def,
                    branch_value,
                    is_true_branch,
                    !is_hit,
                ),
                UnaryOp::Neg => todo!("Neg unary op"),
            },
            ValueDef::Call => {
                let stmts = vec![];
                let trace_call_args = vec![
                    // Run
                    self.mk_const_int_operand(RuConfig::env_run()),
                    // Global id
                    self.mk_const_str_operand(&self.global_id),
                    // Block id
                    self.mk_const_int_operand(target_block as u64),
                    self.mk_const_bool_operand(is_hit),
                ];
                //panic!("Target block is {}", target_block as u64);
                (stmts, trace_call_args)
            }
            ValueDef::Field(place, deref) => {
                let stmts = vec![];
                let trace_call_args = vec![
                    // Run
                    self.mk_const_int_operand(RuConfig::env_run()),
                    // Global id
                    self.mk_const_str_operand(&self.global_id),
                    // Block id
                    self.mk_const_int_operand(target_block as u64),
                    self.mk_const_bool_operand(is_hit),
                ];

                (stmts, trace_call_args)
            }
            ValueDef::Var(place, _) => {
                // ValueDef::Var means that we are directly comparing a variable to some
                // constant value, i.e., switch_value, so what we need to construct is
                // a trace to a binary EQ operation

                let var_u64_local = self.store_local_decl(self.tcx.types.u64);
                let switch_value_operand = if let Some(v) = branch_value {
                    if let ConstKind::Value(ty, ..) = v.kind() {
                        self.mk_const_operand(ty, v.kind())
                    } else {
                        self.mk_const_int_operand(0)
                    }
                } else {
                    self.mk_const_int_operand(0)
                };

                let switch_value_u64_local = self.store_local_decl(self.tcx.types.u64);

                let stmts = vec![
                    self.mk_cast_local_as_u64_stmt(place.local, var_u64_local),
                    self.mk_cast_operand_as_u64_stmt(switch_value_operand, switch_value_u64_local),
                ];
                let trace_call_args = vec![
                    // Run
                    self.mk_const_int_operand(RuConfig::env_run()),
                    // Global id
                    self.mk_const_str_operand(&self.global_id),
                    // Block id
                    self.mk_const_int_operand(target_block as u64),
                    // Switch value
                    //switch_value_operand,
                    self.mk_move_operand(switch_value_u64_local),
                    // Var value
                    self.mk_move_operand(var_u64_local),
                    self.mk_const_bool_operand(is_hit),
                ];
                (stmts, trace_call_args)
            }
            _ => todo!("Value def is {:?}", switch_operand_def),
        }
    }

    fn operand_ty(&self, operand: &Operand<'tcx>) -> Ty<'tcx> {
        operand.ty(self.body.local_decls(), self.tcx.clone())
    }

    fn get_place<'a>(&self, operand: &'a Operand<'tcx>) -> Option<&'a Place<'tcx>> {
        match operand {
            Operand::Copy(place) => Some(place),
            Operand::Move(place) => Some(place),
            _ => None,
        }
    }

    fn get_place_definition_from_stmt(
        &self,
        var: &Place<'tcx>,
        stmt: &Statement<'tcx>,
    ) -> Option<ValueDef<'tcx>> {
        match &stmt.kind {
            Assign(assign) => {
                let (place, value) = assign.as_ref();
                if place != var {
                    return None;
                }

                match value {
                    Rvalue::BinaryOp(op, operands) => {
                        let (left, right) = operands.as_ref();
                        let left_ty = self.operand_ty(left);
                        let right_ty = self.operand_ty(right);
                        let left = ValueDef::from_operand(left, left_ty);
                        let right = ValueDef::from_operand(right, right_ty);
                        if left.is_none() || right.is_none() {
                            return None;
                        }
                        return Some(ValueDef::BinaryOp(
                            to_binary_op(op),
                            Box::new(left.unwrap()),
                            Box::new(right.unwrap()),
                        ));
                    }
                    Rvalue::UnaryOp(op, operand) => {
                        return match operand {
                            Operand::Copy(place) | Operand::Move(place) => {
                                let inner_value_def = self.get_place_definition(place);
                                if inner_value_def.is_none() {
                                    return inner_value_def;
                                }
                                let inner_value_def = inner_value_def.unwrap();
                                if let ValueDef::BinaryOp(op, left, right) = inner_value_def {
                                    return Some(ValueDef::BinaryOp(
                                        invert_binary_op(&op),
                                        left,
                                        right,
                                    ));
                                }
                                Some(ValueDef::UnaryOp(
                                    to_unary_op(op),
                                    Box::new(inner_value_def),
                                ))
                            }
                            Operand::Constant(_) => {
                                let value =
                                    ValueDef::from_operand(operand, self.operand_ty(operand));
                                if value.is_none() {
                                    return None;
                                }

                                Some(ValueDef::UnaryOp(to_unary_op(op), Box::new(value.unwrap())))
                            }
                        };
                    }
                    Rvalue::Use(operand) => match operand {
                        Operand::Constant(_) => {
                            return Some(ValueDef::Var(*var, self.operand_ty(operand)))
                        }
                        Operand::Move(place) | Operand::Copy(place) => {
                            return self.get_place_definition(place);
                        }
                    },
                    Rvalue::Discriminant(place) => {
                        return Some(ValueDef::Discriminant(*place));
                    }
                    Rvalue::Cast(_, operand, to_ty) => {
                        return match operand {
                            Operand::Copy(place) => self.get_place_definition(place),
                            Operand::Move(place) => self.get_place_definition(place),
                            Operand::Constant(const_operand) => match &const_operand.const_ {
                                rustc_middle::mir::Const::Ty(ty, c) => todo!("{:?}", c),
                                rustc_middle::mir::Const::Val(const_value, ty) => {
                                    Some(ValueDef::Const(
                                        *ty,
                                        ConstKind::Value(
                                            *ty,
                                            ValTree::from_scalar_int(
                                                const_value.try_to_scalar_int().unwrap(),
                                            ),
                                        ),
                                    ))
                                }
                                _ => todo!(),
                            },
                        };
                    }
                    Rvalue::Len(_) => return Some(ValueDef::Var(*place, self.tcx.types.usize)),
                    // _ => todo!("Value is {:?}", value),
                    _ => return None,
                }
            }
            SetDiscriminant { place, .. } => {
                let place = place.as_ref();
                if var == place {
                    return Some(ValueDef::Discriminant(*place));
                }
            }
            StatementKind::StorageLive(_) => return None,
            StatementKind::StorageDead(_) => return None,
            _ => todo!("{:?}", &stmt.kind),
        }
        None
    }

    fn get_place_definition_from_terminator(
        &self,
        var: &Place<'tcx>,
        terminator: &Terminator<'tcx>,
    ) -> Option<ValueDef<'tcx>> {
        if let TerminatorKind::Call { destination, .. } = &terminator.kind {
            if destination == var {
                return Some(ValueDef::Call);
            }
        }

        None
    }

    fn get_from_place_projection(&self, place: &Place<'tcx>) -> Option<ValueDef<'tcx>> {
        let projection = place.projection;
        let mut deref = projection.iter().any(|p| p == PlaceElem::Deref);
        let mut value_def = None;
        for p in projection {
            match p {
                PlaceElem::Field(_, _) => {
                    value_def = Some(ValueDef::Field(*place, deref));
                }
                PlaceElem::Index(v) => {
                    let var = self.mk_place(v.index());
                    value_def = Some(ValueDef::Var(var, self.tcx.types.usize))
                }
                _ => {}
            }
        }

        value_def
    }

    fn find_definitions(
        &self,
        block: BasicBlock,
        place: &Place<'tcx>,
    ) -> HashSet<(BasicBlock, ValueDef<'tcx>)> {
        let mut defs = HashSet::new();
        self.find_definitions_backwards(block, place, &mut defs);
        defs
    }

    fn find_definitions_backwards(
        &self,
        block: BasicBlock,
        place: &Place<'tcx>,
        output: &mut HashSet<(BasicBlock, ValueDef<'tcx>)>,
    ) {
        let predecessors = self.body.basic_blocks.predecessors();
        let p = predecessors.get(block);
        if let Some(p) = p {
            for bb in p {
                let predecessor = self.body.basic_blocks.get(*bb).unwrap();
                let value_def = predecessor
                    .statements
                    .iter()
                    .find_map(|stmt| self.get_place_definition_from_stmt(place, stmt));
                if let Some(value_def) = value_def {
                    output.insert((*bb, value_def));
                } else {
                    self.find_definitions_backwards(*bb, place, output);
                }
            }
        }
    }

    fn get_place_definition(&self, place: &Place<'tcx>) -> Option<ValueDef<'tcx>> {
        // TODO projection
        if !place.projection.is_empty() {
            let from_projection = self.get_from_place_projection(place);
            if from_projection.is_some() {
                return from_projection;
            }
        }

        let basic_blocks = self.body.basic_blocks.reverse_postorder();
        for basic_block in basic_blocks.iter() {
            let data = self.body.basic_blocks.get(*basic_block);
            if let Some(data) = data {
                let value_def = data
                    .statements
                    .iter()
                    .find_map(|stmt| self.get_place_definition_from_stmt(place, stmt));

                if value_def.is_some() {
                    return value_def;
                }

                if let Some(terminator) = &data.terminator {
                    let value_def = self.get_place_definition_from_terminator(place, terminator);
                    return value_def;
                }
            }
        }

        for arg in self.body.args_iter() {
            if place.local == arg {
                let ty = self.body.local_decls().get(arg).unwrap().ty;
                return Some(ValueDef::Var(
                    <Place as From<rustc_middle::mir::Local>>::from(arg),
                    ty,
                ));
            }
        }

        None
    }

    fn shift_block_pointers(&self, body: &mut Body<'tcx>) {
        let basic_blocks = body.basic_blocks_mut();
        for basic_block in basic_blocks {
            if let Some(terminator) = &mut basic_block.terminator {
                match &mut terminator.kind {
                    TerminatorKind::Goto { target } => {
                        *target = *target + 1;
                    }
                    TerminatorKind::SwitchInt { targets, .. } => {
                        for target in targets.all_targets_mut() {
                            *target = *target + 1;
                        }
                    }
                    TerminatorKind::Return => {}
                    TerminatorKind::Unreachable => {}
                    TerminatorKind::Drop { target, unwind, .. } => {
                        *target = *target + 1;
                        if let UnwindAction::Cleanup(cleanup) = unwind {
                            *cleanup = *cleanup + 1;
                        }
                    }
                    TerminatorKind::Call {
                        destination,
                        target,
                        unwind,
                        ..
                    } => {
                        // *destination = destination.map(|(place, bb)| (place, bb + 1));
                        *target = target.map(|b| b + 1);
                        if let UnwindAction::Cleanup(cleanup) = unwind {
                            *cleanup = *cleanup + 1;
                        }
                    }
                    TerminatorKind::Assert { target, unwind, .. } => {
                        *target = *target + 1;
                        if let UnwindAction::Cleanup(cleanup) = unwind {
                            *cleanup = *cleanup + 1;
                        }
                    }
                    TerminatorKind::Yield { resume, drop, .. } => {
                        *resume = *resume + 1;
                        *drop = drop.map(|d| d + 1);
                    }
                    TerminatorKind::FalseEdge {
                        real_target,
                        imaginary_target,
                    } => {
                        *real_target = *real_target + 1;
                        *imaginary_target = *imaginary_target + 1;
                    }
                    TerminatorKind::FalseUnwind {
                        real_target,
                        unwind,
                    } => {
                        *real_target = *real_target + 1;
                        if let UnwindAction::Cleanup(cleanup) = unwind {
                            *cleanup = *cleanup + 1;
                        }
                    }
                    TerminatorKind::InlineAsm {
                        targets, unwind, ..
                    } => {
                        // *targets = targets.into_iter().map(|d| *d + 1).collect::<Vec<_>>();
                        for target in targets.iter_mut() {
                            *target = *target + 1;
                        }
                        if let UnwindAction::Cleanup(cleanup) = unwind {
                            *cleanup = *cleanup + 1;
                        }
                    }
                    _ => todo!(),
                }
            }
        }
    }
}

impl<'tcx> MirVisitor<'tcx> {
    fn instrument_first_block(&mut self, body: &mut Body<'tcx>) {
        self.basic_blocks_num += 1;

        // We have to shift all pointers by 1, like switch_int and so on
        self.shift_block_pointers(body);

        let args = self.mk_trace_statements_entry();
        let trace_fn = find_trace_entry_fn(&self.tcx);
        let terminator = self.mk_call_terminator(args, BasicBlock::from_usize(1usize), trace_fn);
        let trace_block = self.mk_basic_block(vec![], terminator);

        let basic_blocks = body.basic_blocks_mut();
        basic_blocks.raw.insert(0, trace_block);
    }

    fn instrument_block(&mut self, block: &mut BasicBlock) {
        let new_target_block = BasicBlock::from_usize(self.basic_blocks_num);
        self.basic_blocks_num += 1;

        let args = self.mk_trace_branch_hit(block.as_usize());
        let trace_fn = find_trace_branch_hit_fn(&self.tcx);
        let terminator = self.mk_call_terminator(args, *block, trace_fn);
        let trace_block = self.mk_basic_block(Vec::new(), terminator);

        let trace_chain = vec![trace_block];

        *block = new_target_block;
        let mut instrumentation = vec![(new_target_block, trace_chain)];
        self.instrumentation.append(&mut instrumentation);
    }

    fn instrument_assert(
        &mut self,
        cond: &Operand<'tcx>,
        expected: bool,
        target: &mut BasicBlock,
        cleanup: &mut Option<BasicBlock>,
    ) {
        let cond_place = self
            .get_place(cond)
            .expect("Place has been defined in a previous block");
        let cond_def = self.get_place_definition(cond_place);
        if let Some(cond_def) = cond_def {
            let mut branch_ids = vec![target.as_u32() as u64];
            if let Some(cleanup_target) = cleanup {
                branch_ids.push(cleanup_target.as_u32() as u64);
            } else {
                // Create a resume block
                let cleanup_index = (self.basic_blocks_num + 3) as u64;
                branch_ids.push(cleanup_index);
            };

            let mut values = HashMap::new();
            // true
            values.insert(
                target.as_u32() as u64,
                self.switch_value_to_const(self.tcx.types.bool, 1),
            );
            // false
            values.insert(
                target.as_u32() as u64,
                self.switch_value_to_const(self.tcx.types.bool, 0),
            );

            // let mut false_tracing_chain = self.mk_tracing_chain(
            //     &cond_def,
            //     &branch_ids,
            //     false,
            //     &values,
            //     cleanup_target
            // );

            // if let Some(cleanup) = cleanup {
            //     *cleanup = false_tracing_chain.0;
            // } else {
            //     *cleanup = Some(false_tracing_chain.0);
            //     let resume = self.mk_resume_basic_block(vec![]);
            //     false_tracing_chain.1.push(resume);
            //     self.basic_blocks_num += 1;
            // }

            let true_tracing_chain =
                self.mk_tracing_chain(&cond_def, &branch_ids, true, &values, *target);
            *target = true_tracing_chain.0;
            let mut instrumentation = vec![true_tracing_chain];
            self.instrumentation.append(&mut instrumentation);
            //let resume = self.mk_resume_basic_block(vec![]);

            //info!("Tracing chain is: {:?}", true_tracing_chain);
        }
    }

    fn instrument_switch_int(
        &mut self,
        terminator: &mut Terminator<'tcx>,
        source_block: BasicBlock,
    ) {
        info!("MIR: Instrument switch int");
        let mut instrumentation = match &mut terminator.kind {
            TerminatorKind::SwitchInt { discr, targets } => {
                // let switch_operand_place = self
                //     .get_place(discr)
                //     .expect("Place has been defined in a previous block");
                let switch_operand_place = self.get_place(discr);
                if switch_operand_place.is_none() {
                    return;
                }
                let switch_operand_place = switch_operand_place.unwrap();
                let switch_operand_def = self.get_place_definition(switch_operand_place);
                if switch_operand_def.is_none() {
                    return;
                }
                let switch_operand_def = switch_operand_def.unwrap();

                let branch_ids = targets
                    .all_targets()
                    .iter()
                    // Shift ids back because they are off by one due to root instrumentation
                    .map(|t| t.as_u32() as u64)
                    .collect::<Vec<u64>>();

                let mut instrumentation = Vec::with_capacity(targets.all_targets().len());
                let mut all_targets = targets
                    .iter()
                    .map(|(switch_value, target_block)| (Some(switch_value), target_block))
                    .collect::<Vec<_>>();
                all_targets.push((None, *targets.all_targets().last().unwrap()));

                let switch_ty = discr.ty(self.body.local_decls(), self.tcx);
                let values_const = all_targets
                    .iter()
                    .map(|(value, target)| (target, value.unwrap_or_else(|| 0)))
                    .map(|(target, value)| {
                        (
                            target.as_u32() as u64,
                            self.switch_value_to_const(switch_ty, value),
                        )
                    })
                    .collect::<HashMap<_, _>>();
                // Switch value is like false (0), or some numeric value, e.g., when comparing x == 2
                for (idx, (switch_value, target_block)) in all_targets.iter().enumerate() {
                    let switch_value_const =
                        switch_value.map(|sv| self.switch_value_to_const(switch_ty, sv));
                    let (first_tracing_block, tracing_chain) = self.mk_tracing_chain(
                        &switch_operand_def,
                        &branch_ids,
                        switch_value_const.is_none(),
                        &values_const,
                        *target_block,
                    );
                    instrumentation.push((first_tracing_block, tracing_chain));
                }

                instrumentation
            }
            _ => panic!("Not a switch int"),
        };

        for (idx, (first_block, _)) in instrumentation.iter().enumerate() {
            self.update_terminator(terminator, idx, *first_block);
        }

        self.instrumentation.append(&mut instrumentation);
    }

    fn mk_tracing_chain(
        &mut self,
        switch_operand_def: &ValueDef<'tcx>,
        branch_to_trace_ids: &Vec<u64>,
        is_true_branch: bool,
        values: &HashMap<u64, Const<'tcx>>,
        target_block: BasicBlock,
    ) -> (BasicBlock, Vec<BasicBlockData<'tcx>>) {
        let mut tracing_chain = Vec::with_capacity(branch_to_trace_ids.len());
        let trace_fn = find_trace_fn_for(&self.tcx, &switch_operand_def);
        let first_block = BasicBlock::from_usize(self.basic_blocks_num);

        let mut branches = branch_to_trace_ids.iter().peekable();
        while let Some(&branch_to_trace_id) = branches.next() {
            let is_branch_hit = branch_to_trace_id == target_block.as_u32() as u64;
            self.basic_blocks_num += 1;

            let branch_value = values.get(&branch_to_trace_id).map(|v| v.clone());
            let next_block = if branches.peek().is_some() {
                BasicBlock::from_usize(self.basic_blocks_num)
            } else {
                // If this is the last element in the tracing chain, then point to the
                // original basic block
                target_block
            };

            if is_branch_hit {
                let args = self.mk_trace_branch_hit(target_block.as_usize());
                let trace_fn = find_trace_branch_hit_fn(&self.tcx);
                let terminator = self.mk_call_terminator(args, next_block, trace_fn);
                let trace_block = self.mk_basic_block(Vec::new(), terminator);
                tracing_chain.push(trace_block);
            } else {
                let (stmts, args) = self.mk_trace_statements_switch_int(
                    branch_to_trace_id,
                    switch_operand_def,
                    branch_value,
                    is_true_branch,
                    is_branch_hit,
                );
                let terminator = self.mk_call_terminator(args, next_block, trace_fn);
                let trace_block = self.mk_basic_block(stmts, terminator);
                tracing_chain.push(trace_block);
            }
        }

        (first_block, tracing_chain)
    }
}

impl<'tcx> MirVisitor<'tcx> {
    fn compute_distance_in_place(&mut self, block_data: &mut BasicBlockData<'tcx>) {
        let mut new_statements = Vec::new();
        for stmt in block_data.statements.iter().rev() {
            new_statements.push(stmt.clone());

            match &stmt.kind {
                Assign(assign) => {
                    let (var, rvalue) = assign.as_ref();
                    let alias = self.alias_map.get(&var.local).cloned();
                    match rvalue {
                        Rvalue::BinaryOp(op, operands) => {
                            let (left, right) = operands.as_ref();
                            let distances = self.mk_stmt_distance_for_binary_op(
                                &mut new_statements,
                                op,
                                left,
                                right,
                                alias,
                            );

                            if let Some(distances) = distances {
                                self.alias_map.insert(var.local, distances);
                            }
                        }
                        Rvalue::Use(operand) => {
                            match operand {
                                Operand::Constant(constant) => {
                                    self.mk_stmt_distance_for_const(
                                        &mut new_statements,
                                        constant,
                                        alias,
                                    );
                                }
                                Operand::Move(moved) => {
                                    if let Some(alias) = self.alias_map.get(&moved.local) {
                                        self.alias_map.insert(var.local, *alias);
                                    }
                                }
                                _ => {
                                    // do nothing
                                }
                            }
                        }
                        _ => {
                            // do nothing
                        }
                    }
                }
                _ => {
                    // do nothing
                }
            }
        }

        block_data.statements = new_statements;
    }

    fn mk_stmt_distance_for_const(
        &mut self,
        stmts: &mut Vec<Statement<'tcx>>,
        constant: &Box<ConstOperand>,
        alias: Option<(Local, Local)>,
    ) {
        let ty = constant.const_.ty();

        let (true_branch, false_branch) = alias.unwrap_or_else(|| {
            (
                self.store_local_decl(self.tcx.types.f64),
                self.store_local_decl(self.tcx.types.f64),
            )
        });

        if ty.is_bool() {
            stmts.push(self.mk_assign_1_stmt(self.mk_place(true_branch.index())));
            stmts.push(self.mk_assign_1_stmt(self.mk_place(false_branch.index())));
        }
    }

    fn mk_stmt_distance_for_binary_op(
        &mut self,
        stmts: &mut Vec<Statement<'tcx>>,
        op: &BinOp,
        left: &Operand<'tcx>,
        right: &Operand<'tcx>,
        alias: Option<(Local, Local)>,
    ) -> Option<(Local, Local)> {
        // Move left as f64
        let left_f64_local = self.store_f64_local_decl();
        stmts.push(self.mk_cast_operand_as_f64_stmt(left.clone(), left_f64_local));
        // Move right f64
        let right_f64_local = self.store_f64_local_decl();
        stmts.push(self.mk_cast_operand_as_f64_stmt(right.clone(), right_f64_local));

        let (true_branch, false_branch) = alias.unwrap_or_else(|| {
            (
                self.store_local_decl(self.tcx.types.f64),
                self.store_local_decl(self.tcx.types.f64),
            )
        });
        match op {
            // left <= right
            BinOp::Le => {
                // True branch
                stmts.push(self.mk_sub_stmt(true_branch, right_f64_local, left_f64_local));
                // Add 1.0
                stmts.push(self.mk_add_1_stmt(true_branch, true_branch));

                // False branch
                stmts.push(self.mk_sub_stmt(false_branch, left_f64_local, right_f64_local));

                Some((true_branch, false_branch))
            }
            _ => None, //_ => todo!("{:?}", op),
        }
    }
}

impl<'tcx> MutVisitor<'tcx> for MirVisitor<'tcx> {
    fn visit_body(&mut self, body: &mut Body<'tcx>) {
        self.super_body(body);

        #[cfg(feature = "instrumentation")]
        {
            // Now push the tracing chains after they have created
            for (_, tracing_chain) in &self.instrumentation {
                let basic_blocks = body.basic_blocks_mut();
                tracing_chain.iter().for_each(|tb| {
                    let _ = basic_blocks.push(tb.clone());
                });
            }

            self.instrument_first_block(body);

            // Also apply local definitions
            body.local_decls = self.body.local_decls.clone();
        }
    }

    #[cfg(feature = "instrumentation")]
    fn visit_basic_block_data(&mut self, block: BasicBlock, data: &mut BasicBlockData<'tcx>) {
        self.super_basic_block_data(block, data);

        if let Some(terminator) = &mut data.terminator {
            match &mut terminator.kind {
                // Instrument branching
                TerminatorKind::SwitchInt { targets, .. } => {
                    self.branch_counter += targets.all_targets().len() as u64;
                    self.instrument_switch_int(terminator, block);
                }
                TerminatorKind::Call {
                    destination,
                    unwind,
                    target,
                    ..
                } => {
                    if let UnwindAction::Cleanup(_) = unwind {
                        self.branch_counter += 2
                    }
                    if let Some(block) = target {
                        self.instrument_block(block);
                    }
                }
                TerminatorKind::Goto { target } => {
                    self.instrument_block(target);
                }
                TerminatorKind::Drop { target, unwind, .. } => {
                    if let UnwindAction::Cleanup(_) = unwind {
                        self.branch_counter += 2;
                    }
                    self.instrument_block(target);
                }
                TerminatorKind::Assert {
                    cond,
                    expected,
                    msg,
                    target,
                    unwind,
                } => {
                    //self.instrument_assert(cond, *expected, target, cleanup);
                    self.assertions += 1;
                    if let UnwindAction::Cleanup(cleanup) = unwind {
                        self.branch_counter += 2;
                        self.instrument_block(cleanup);
                    }
                    self.instrument_block(target);
                }
                TerminatorKind::Yield { resume, drop, .. } => {
                    if drop.is_some() {
                        self.branch_counter += 2;
                    }
                    self.instrument_block(resume);
                }
                TerminatorKind::FalseEdge { real_target, .. } => {
                    self.instrument_block(real_target);
                }
                TerminatorKind::FalseUnwind {
                    real_target,
                    unwind,
                } => {
                    if let UnwindAction::Cleanup(_) = unwind {
                        self.branch_counter += 1;
                    }
                    self.instrument_block(real_target);
                }
                _ => {
                    // ignore the rest
                }
            }
        }
    }

    #[cfg(feature = "analysis")]
    fn visit_const_operand(&mut self, constant: &mut ConstOperand<'tcx>, location: Location) {
        let literal = &constant.const_;
        match literal {
            rustc_middle::mir::Const::Val(value, ty) => match value {
                ConstValue::Scalar(scalar) => {
                    if ty.is_numeric() {
                        let value = scalar.to_string();
                        let t = <RuTy as From<String>>::from(format!("{}", ty));
                        debug!("Constant number {:?} of type {}", scalar.to_string(), t);
                        self.constant_pool.push(RuConstVal::new(value, t));
                    }
                }
                ConstValue::Slice { data, meta } => {
                    let bytes = data
                        .inner()
                        .inspect_with_uninit_and_ptr_outside_interpreter(0..*meta as usize);
                    let string = String::from_utf8_lossy(bytes);
                    debug!(
                        "Constant string: {}, is string: {}",
                        String::from_utf8_lossy(bytes),
                        ty.is_str()
                    );
                    self.constant_pool
                        .push(RuConstVal::new(string.to_string(), RuTy::Prim(RuPrim::Str)));
                }
                _ => {}
            },
            _ => {}
        }
    }

    #[cfg(feature = "analysis")]
    fn visit_terminator(&mut self, terminator: &mut Terminator<'tcx>, location: Location) {
        match &terminator.kind {
            TerminatorKind::SwitchInt { discr, targets } => {
                if let Some(const_operand) = discr.constant() {
                    let switch_ty = const_operand.ty();
                    if switch_ty.is_integral() {
                        let ty_name = ty_name(switch_ty);
                        let t = <RuTy as From<String>>::from(format!("{}", ty_name));
                        for (val, _) in targets.iter() {
                            let value = val.to_string();
                            self.constant_pool
                                .push(RuConstVal::new(value.clone(), t.clone()));
                            debug!("Constant number {:?} of type {}", value, t);
                        }
                    }
                }
            }
            _ => {}
        }
    }

    fn tcx<'a>(&'a self) -> TyCtxt<'tcx> {
        self.tcx.tcx()
    }
}

fn find_monitor_fn_by_name(tcx: &TyCtxt<'_>, name: &str) -> DefId {
    tcx.hir()
        .items()
        .find_map(|i| {
            let item = tcx.hir().item(i);
            if let ItemKind::Fn(_, _, _) = &item.kind {
                if item.ident.name.to_string().contains(name) {
                    // println!("{:#?}", item.ident.name.to_string());
                    return Some(item.owner_id.to_def_id());
                }
            }
            None
        })
        .expect(&format!(
            "Could not find rusty_monitor::{} in the crate",
            name
        ))
}

fn find_trace_bool_fn(tcx: &TyCtxt<'_>) -> DefId {
    find_monitor_fn_by_name(tcx, "trace_branch_bool")
}

fn find_trace_entry_fn(tcx: &TyCtxt<'_>) -> DefId {
    find_monitor_fn_by_name(tcx, "trace_entry")
}

fn find_trace_0_or_1_fn(tcx: &TyCtxt<'_>) -> DefId {
    find_monitor_fn_by_name(tcx, "trace_zero_or_one")
}

fn find_trace_const(tcx: &TyCtxt<'_>) -> DefId {
    find_monitor_fn_by_name(tcx, "trace_const")
}

fn find_trace_block(tcx: &TyCtxt<'_>) -> DefId {
    find_monitor_fn_by_name(tcx, "trace_branch_hit")
}

fn find_trace_fn_for(tcx: &TyCtxt<'_>, value_def: &ValueDef<'_>) -> DefId {
    match value_def {
        ValueDef::BinaryOp(_, _, _) => find_trace_bool_fn(tcx),
        ValueDef::Discriminant(_) => find_trace_0_or_1_fn(tcx),
        ValueDef::UnaryOp(_, inner_value_def) => find_trace_fn_for(tcx, inner_value_def.as_ref()),
        ValueDef::Field(_, _) => find_trace_0_or_1_fn(tcx),
        ValueDef::Call => find_trace_0_or_1_fn(tcx),
        ValueDef::Var(_, ty) => {
            if ty.is_bool() {
                find_trace_switch_value_with_var_bool(tcx)
            } else if ty.is_integral() {
                find_trace_switch_value_with_var_int(tcx)
            } else if ty.is_char() {
                find_trace_switch_value_with_var_char(tcx)
            } else {
                todo!("{:?}", ty)
            }
        }
        ValueDef::Const(_, _) => find_trace_const(tcx),
        _ => {
            todo!("Value def is {:?}", value_def)
        }
    }
}

fn find_trace_switch_value_with_var_bool(tcx: &TyCtxt<'_>) -> DefId {
    find_monitor_fn_by_name(tcx, "trace_switch_value_with_var_bool")
}

fn find_trace_switch_value_with_var_int(tcx: &TyCtxt<'_>) -> DefId {
    find_monitor_fn_by_name(tcx, "trace_switch_value_with_var_int")
}

fn find_trace_switch_value_with_var_char(tcx: &TyCtxt<'_>) -> DefId {
    find_monitor_fn_by_name(tcx, "trace_switch_value_with_var_char")
}

fn find_trace_branch_hit_fn(tcx: &TyCtxt<'_>) -> DefId {
    find_monitor_fn_by_name(tcx, "trace_branch_hit")
}

pub fn is_rusty_monitor(hir_id: HirId, tcx: &TyCtxt<'_>) -> bool {
    let name = format!("{:?}", hir_id);
    name.contains("rusty_monitor")
}

fn get_binary_op_def_id(tcx: &TyCtxt<'_>) -> DefId {
    tcx.hir()
        .items()
        .find_map(|i| {
            let item = tcx.hir().item(i);
            if let ItemKind::Enum(_, _) = &item.kind {
                if item.ident.name.to_string() == "BinaryOp" {
                    return Some(item.owner_id.to_def_id());
                }
            }
            Option::None
        })
        .unwrap()
}

fn to_binary_op(op: &BinOp) -> BinaryOp {
    match op {
        BinOp::Add | BinOp::AddUnchecked | BinOp::AddWithOverflow => BinaryOp::Add,
        BinOp::Sub | BinOp::SubUnchecked | BinOp::SubWithOverflow => BinaryOp::Sub,
        BinOp::Mul | BinOp::MulUnchecked | BinOp::MulWithOverflow => BinaryOp::Mul,
        BinOp::Div => BinaryOp::Div,
        BinOp::Rem => BinaryOp::Rem,
        BinOp::BitXor => BinaryOp::BitXor,
        BinOp::BitAnd => BinaryOp::BitAnd,
        BinOp::BitOr => BinaryOp::BitOr,
        BinOp::Shl | BinOp::ShlUnchecked => BinaryOp::Shl,
        BinOp::Shr | BinOp::ShrUnchecked => BinaryOp::Shr,
        BinOp::Eq => BinaryOp::Eq,
        BinOp::Lt => BinaryOp::Lt,
        BinOp::Le => BinaryOp::Le,
        BinOp::Ne => BinaryOp::Ne,
        BinOp::Ge => BinaryOp::Ge,
        BinOp::Gt => BinaryOp::Gt,
        BinOp::Cmp => todo!(),
        BinOp::Offset => BinaryOp::Offset,
    }
}

fn to_unary_op(op: &UnOp) -> UnaryOp {
    match op {
        UnOp::Not => UnaryOp::Not,
        UnOp::Neg => UnaryOp::Neg,
        UnOp::PtrMetadata => todo!(),
    }
}

fn invert_binary_op(op: &BinaryOp) -> BinaryOp {
    match op {
        BinaryOp::Eq => BinaryOp::Ne,
        BinaryOp::Lt => BinaryOp::Ge,
        BinaryOp::Le => BinaryOp::Gt,
        BinaryOp::Ne => BinaryOp::Eq,
        BinaryOp::Ge => BinaryOp::Lt,
        BinaryOp::Gt => BinaryOp::Le,
        _ => todo!("Should never happpen"),
    }
}

enum SwitchPath {
    Value(u128),
    Otherwise,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum ValueDef<'a> {
    BinaryOp(BinaryOp, Box<ValueDef<'a>>, Box<ValueDef<'a>>),
    UnaryOp(UnaryOp, Box<ValueDef<'a>>),
    Const(Ty<'a>, ConstKind<'a>),
    Var(Place<'a>, Ty<'a>),
    Discriminant(Place<'a>),
    Call,
    Index(Place<'a>),
    // Deref?
    Field(Place<'a>, bool),
}

impl<'a> ValueDef<'a> {
    fn expect_const(&self) -> (&Ty<'a>, &ConstKind<'a>) {
        if let ValueDef::Const(ty, val) = self {
            return (ty, val);
        }
        panic!("Is not const");
    }

    fn is_const(&self) -> bool {
        if let ValueDef::Const(_, _) = self {
            return true;
        }

        false
    }

    fn is_var(&self) -> bool {
        if let ValueDef::Var(_, _) = self {
            return true;
        }

        false
    }

    fn expect_var(&self) -> Place<'a> {
        if let ValueDef::Var(place, _) = self {
            return *place;
        }

        panic!("Is not var");
    }

    fn from_operand(operand: &Operand<'a>, ty: Ty<'a>) -> Option<ValueDef<'a>> {
        match operand {
            Operand::Copy(place) | Operand::Move(place) => Some(ValueDef::Var(*place, ty)),
            Operand::Constant(const_operand) => match &const_operand.const_ {
                rustc_middle::mir::Const::Ty(ty, constant_ty) => {
                    let val = constant_ty.kind();
                    Some(ValueDef::Const(*ty, val))
                }
                rustc_middle::mir::Const::Val(const_value, ty) => Some(ValueDef::Const(
                    *ty,
                    ConstKind::Value(
                        *ty,
                        ValTree::from_scalar_int(const_value.try_to_scalar_int().unwrap()),
                    ),
                )),
                _ => None,
            },
        }
    }
}
