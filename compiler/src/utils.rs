use crate::extractor::hir_ty_to_t;
use crate::traits::analyze_trait;
use crate::types::{
    mir_ty_to_t, RuArray, RuEnum, RuGeneric, RuParam, RuStruct, RuTrait, RuTraitObj, RuTuple, RuTy,
    RuUnion,
};
use log::{debug, error, info, warn};
use rustc_data_structures::fx::FxIndexMap;
use rustc_hir::def::{DefKind, Res};
use rustc_hir::def_id::DefId;
use rustc_hir::definitions::{DefPathData, DisambiguatedDefPathData};
use rustc_hir::{
    AnonConst, ArrayLen, FnRetTy, GenericArg, GenericBound, GenericParam, GenericParamKind,
    Generics, HirId, Impl, Item, ItemKind, MutTy, Mutability, Node, ParamName, Path, PathSegment,
    PrimTy, QPath, Ty, TyKind, WherePredicate,
};
use rustc_middle::dep_graph::DepContext;
use rustc_middle::mir::interpret::Scalar;
use rustc_middle::mir::ConstValue;
use rustc_middle::ty::fast_reject::SimplifiedType;
use rustc_middle::ty::GenericArgKind;
use rustc_middle::ty::{AdtKind, TyCtxt, TypeckResults};
use rustc_span::def_id::{CrateNum, LocalDefId};
use rustc_span::{FileName, RealFileName, Span, DUMMY_SP};
use std::collections::HashMap;
use std::fmt::format;
use std::io;
use std::io::Write;
use std::option::Option::Some;
use std::path::PathBuf;
use std::process::{Command, Stdio};

pub fn rustc_get_crate_name(rustc_args: &[String]) -> String {
    // println!("{:#?}", rustc_args);
    let pos = rustc_args.iter().position(|a| a == "--crate-name").unwrap();
    rustc_args.get(pos + 1).map(|s| s.to_string()).unwrap()
}

pub fn cargo_path() -> io::Result<PathBuf> {
    match which::which("cargo") {
        Ok(p) => Ok(p),
        Err(e) => Err(io::Error::new(io::ErrorKind::Other, format!("{}", e))),
    }
}

pub fn fmt_path() -> io::Result<PathBuf> {
    match which::which("rustfmt") {
        Ok(p) => Ok(p),
        Err(e) => Err(io::Error::new(io::ErrorKind::Other, format!("{}", e))),
    }
}

pub fn ty_to_param(
    name: Option<&str>,
    ty: &Ty,
    self_ty: Option<&RuTy>,
    associated_types: Option<&HashMap<String, RuTy>>,
    parent_generics: &Vec<RuTy>,
    tcx: &TyCtxt<'_>,
) -> Option<RuParam> {
    let mutability = match &ty.kind {
        TyKind::Ref(_, mut_ty) => mut_ty.mutbl == Mutability::Mut,
        _ => false,
    };

    let real_ty = ty_to_t(ty, self_ty, associated_types, parent_generics, tcx)?;
    Some(RuParam::new(name, real_ty, mutability))
}

pub fn ty_to_t(
    ty: &Ty,
    self_ty: Option<&RuTy>,
    associated_types: Option<&HashMap<String, RuTy>>,
    defined_generics: &Vec<RuTy>,
    tcx: &TyCtxt<'_>,
) -> Option<RuTy> {
    match &ty.kind {
        TyKind::Ref(_, mut_ty) => {
            ty_to_t(mut_ty.ty, self_ty, associated_types, defined_generics, tcx).map(|t| {
                let mutable = match mut_ty.mutbl {
                    Mutability::Mut => true,
                    Mutability::Not => false,
                };
                RuTy::Ref(Box::new(t), mutable)
            })
        }
        TyKind::Path(q_path) => {
            match q_path {
                QPath::Resolved(_, path) => {
                    path_to_t(path, self_ty, associated_types, defined_generics, tcx)
                    // TODO parse generic args of the type
                }
                QPath::TypeRelative(ty, segment) => {
                    if let Some(associated_types) = associated_types {
                        if let Some(mapped_t) = associated_types.get(segment.ident.name.as_str()) {
                            return Some(mapped_t.clone());
                        }
                    }

                    info!("Did not find any mapping for {:?}", segment);
                    None
                    // let as_type = ty_to_t(ty, self_ty, associated_types, defined_generics, tcx);
                    // as_type.map(|as_type| RuTy::Relative(Box::new(as_type), segment.ident.to_string()))
                }
                _ => unimplemented!("{:?}", q_path),
            }
        }
        TyKind::Slice(ty) => ty_to_t(ty, self_ty, associated_types, defined_generics, tcx)
            .map(|t| RuTy::Slice(Box::new(t))),
        TyKind::OpaqueDef(item, generic_args, _) => {
            warn!(
                "HIR: Skipping opaquedef of {:?} with generic args {:?}",
                item, generic_args
            );
            None
        }
        TyKind::Tup(tys) => {
            let ts = tys
                .iter()
                .filter_map(|ty| ty_to_t(ty, self_ty, associated_types, defined_generics, tcx))
                .map(Box::new)
                .collect::<Vec<_>>();
            if ts.len() != tys.len() {
                return None;
            }
            Some(RuTy::Tuple(RuTuple::new(ts)))
        }
        TyKind::Array(ty, array_len) => {
            let array_len = eval_array_len(array_len, tcx);
            if let Some(array_len) = array_len {
                return ty_to_t(ty, self_ty, associated_types, defined_generics, tcx)
                    .map(|ty| RuTy::Array(Box::new(RuArray::new(ty, array_len))));
            }

            todo!("Unknown array length")
            // None
        }
        TyKind::TraitObject(trait_refs, _, _) => {
            // assert!(trait_refs.len() == 1);
            if trait_refs.len() > 1 {
                return None;
            }
            let poly_trait_ref = trait_refs.get(0).unwrap();
            let name = res_to_name(&poly_trait_ref.trait_ref.path.res, tcx);
            let trait_obj = RuTraitObj::new(&name, false);
            Some(RuTy::TraitObj(trait_obj))
        }
        TyKind::BareFn(fn_ty) => Some(RuTy::Fn),
        TyKind::Ptr(ptr) => {
            //let t = ty_to_t(ptr.ty, self_ty, associated_types, defined_generics, tcx);
            //t.map(|t| RuTy::RawPointer(Box::new(t), ptr.mutbl == Mutability::Mut))
            None
        }

        TyKind::Never => None,
        _ => todo!("Ty kind is: {:?}", &ty.kind),
    }
}

pub fn path_to_generics(
    path: &rustc_hir::Path<'_>,
    self_: Option<&RuTy>,
    associated_types: Option<&HashMap<String, RuTy>>,
    defined_generics: &Vec<RuTy>,
    tcx: &TyCtxt<'_>,
) -> Vec<RuTy> {
    let generics = path
        .segments
        .iter()
        .filter_map(|s| {
            if let Some(args) = s.args {
                Some(
                    args.args
                        .iter()
                        .filter_map(|a| {
                            generic_arg_to_t(a, self_, associated_types, defined_generics, tcx)
                        })
                        .collect::<Vec<_>>(),
                )
            } else {
                None
            }
        })
        .flatten()
        .collect::<Vec<_>>();

    generics
}

pub fn path_to_t(
    path: &Path,
    self_ty: Option<&RuTy>,
    associated_types: Option<&HashMap<String, RuTy>>,
    defined_generics: &Vec<RuTy>,
    tcx: &TyCtxt<'_>,
) -> Option<RuTy> {
    match &path.res {
        Res::Def(def_kind, def_id) => match def_kind {
            DefKind::Struct | DefKind::Enum => {
                let generics =
                    path_to_generics(path, self_ty, associated_types, defined_generics, tcx);
                def_kind_to_t(
                    def_kind,
                    *def_id,
                    self_ty,
                    path,
                    associated_types,
                    &generics,
                    tcx,
                )
            }
            DefKind::TyParam => {
                let name = path
                    .segments
                    .iter()
                    .map(|s| s.ident.name.to_ident_string())
                    .collect::<Vec<_>>()
                    .join("::");

                let bounds = defined_generics
                    .iter()
                    .find(|g| g.name() == name)
                    .map(|g| g.expect_generic().bounds())
                    .map_or(vec![], |bounds| bounds.clone());

                return Some(RuTy::Generic(RuGeneric::new(&name, bounds)));
            }
            _ => None,
        },
        Res::PrimTy(prim_ty) => Some(RuTy::from(*prim_ty)),
        Res::SelfTyAlias { alias_to, .. } => {
            self_ty.map(|self_ty| self_ty.clone())
            // if let Some(trait_) = trait_ {
            //     let trait_name = tcx.def_path_str(*trait_);
            //     let trait_instance = Trait::new(&trait_name, vec![], vec![]);
            //     self_ty.map(|self_ty| T::AsTrait(Box::new(self_ty.clone()), trait_instance))
            // } else {
            //
            // }
        }
        _ => {
            // unimplemented!("{:?}", &path.res)
            None
        }
    }
}

pub fn def_kind_to_t(
    def_kind: &DefKind,
    def_id: DefId,
    self_ty: Option<&RuTy>,
    path: &Path<'_>,
    associated_types: Option<&HashMap<String, RuTy>>,
    defined_generics: &Vec<RuTy>,
    tcx: &TyCtxt<'_>,
) -> Option<RuTy> {
    let name = tcx.def_path_str(def_id);
    let generics = path_to_generics(path, self_ty, associated_types, defined_generics, tcx);

    match def_kind {
        DefKind::Enum => Some(RuTy::Enum(RuEnum::new(
            &name,
            generics,
            vec![],
            is_local(def_id),
        ))),
        DefKind::Struct => Some(RuTy::Struct(RuStruct::new(
            &name,
            generics,
            is_local(def_id),
        ))),
        DefKind::Union => Some(RuTy::Union(RuUnion::new(&name, is_local(def_id)))),
        _ => None,
    }
}

pub fn generic_arg_to_t(
    generic_arg: &GenericArg,
    self_: Option<&RuTy>,
    associated_types: Option<&HashMap<String, RuTy>>,
    defined_generics: &Vec<RuTy>,
    tcx: &TyCtxt<'_>,
) -> Option<RuTy> {
    match generic_arg {
        GenericArg::Type(ty) => ty_to_t(ty, self_, associated_types, defined_generics, tcx),
        _ => None,
    }
}

pub fn def_id_to_t(def_id: DefId, tcx: &TyCtxt<'_>) -> Option<RuTy> {
    let ty = tcx.type_of(def_id).skip_binder();
    match ty.kind() {
        rustc_middle::ty::TyKind::Adt(adt_def, raw_lists) => {
            let generics = raw_lists
                .non_erasable_generics(tcx.clone(), adt_def.did())
                .filter_map(|kind| generic_to_t(kind, tcx))
                .collect::<Vec<_>>();

            let substs_len = raw_lists
                .non_erasable_generics(tcx.clone(), adt_def.did())
                .filter(|kind| match kind {
                    GenericArgKind::Type(_) => true,
                    _ => false,
                })
                .count();
            if generics.len() != substs_len {
                warn!("HIR: not all generics could be parsed: {:?}", raw_lists);
                return None;
            }
            let name = tcx.def_path_str(def_id);

            info!("Type of is {:?}", adt_def.adt_kind());

            match adt_def.adt_kind() {
                AdtKind::Struct => {
                    let t = RuTy::Struct(RuStruct::new(&name, generics, is_local(def_id)));

                    Some(t)
                }
                AdtKind::Union => {
                    warn!("HIR: Skipping tuple");
                    None
                }
                AdtKind::Enum => {
                    let t = RuTy::Enum(RuEnum::new(&name, generics, vec![], is_local(def_id)));
                    Some(t)
                }
            }
        }
        _ => todo!(),
    }
}

pub fn def_id_to_enum(def_id: DefId, tcx: &TyCtxt<'_>) -> Option<RuTy> {
    let ty = tcx.type_of(def_id).skip_binder();
    match ty.kind() {
        rustc_middle::ty::TyKind::Adt(adt_det, raw_lists) => {
            let generics = raw_lists
                .non_erasable_generics(tcx.clone(), adt_det.did())
                .filter_map(|kind| generic_to_t(kind, tcx))
                .collect::<Vec<_>>();
            if generics.len() != raw_lists.len() {
                warn!("HIR: not all generics could be parsed: {:?}", raw_lists);
                return None;
            }

            let name = tcx.def_path_str(def_id);
            let variants = vec![];
            let t = RuTy::Enum(RuEnum::new(&name, generics, variants, is_local(def_id)));

            Some(t)
        }
        _ => todo!(),
    }
}

// pub fn substs_len(substs: RawList) -> usize {
// substs
//     .non_erasable_generics()
//     .filter(|kind| match kind {
//         GenericArgKind::Type(_) => true,
//         _ => false,
//     })
//     .count()
// }

pub fn generic_to_t(generic_kind: GenericArgKind, tcx: &TyCtxt<'_>) -> Option<RuTy> {
    match generic_kind {
        GenericArgKind::Lifetime(_) => None,
        GenericArgKind::Type(ty) => tys_to_t(ty, tcx),
        GenericArgKind::Const(c) => {
            todo!("Const is {:?}", c)
        }
    }
}

pub fn tys_to_t(ty: rustc_middle::ty::Ty, tcx: &TyCtxt<'_>) -> Option<RuTy> {
    match ty.kind() {
        rustc_middle::ty::TyKind::Param(param) => {
            let name = param.name.to_string();
            let generic_param = RuTy::Generic(RuGeneric::new(&name, vec![]));
            Some(generic_param)
        }
        _ => todo!("Ty is {:?}", ty),
    }
}

pub fn generics_to_ts(
    generics: &Generics,
    self_ty: Option<&RuTy>,
    associated_types: Option<&HashMap<String, RuTy>>,
    tcx: &TyCtxt<'_>,
) -> Option<Vec<RuTy>> {
    let mut where_generics = generics
        .predicates
        .iter()
        .filter_map(|p| match p {
            WherePredicate::BoundPredicate(p) => {
                info!("Where predicate: {:?}", p.bounded_ty);
                let ty = ty_to_t(p.bounded_ty, self_ty, associated_types, &vec![], tcx);
                if let Some(ty) = &ty {
                    if !ty.is_generic() {
                        info!("Bound type is not generic, but: {:?}", ty);
                        return None;
                    }
                }

                let bounds = p
                    .bounds
                    .iter()
                    .filter_map(|b| generic_bound_to_trait(b, tcx))
                    .collect::<Vec<_>>();
                ty.map(|mut ty| {
                    ty.expect_generic_mut().set_bounds(bounds);
                    (ty.name(), ty)
                })
            }
            _ => None,
        })
        .collect::<HashMap<_, _>>();
    if where_generics.len() != generics.predicates.len() {
        return None;
    }

    generics
        .params
        .iter()
        .filter_map(|g| generic_param_to_t(generics, g, tcx))
        .for_each(|g| {
            let _ = where_generics
                .entry(g.name())
                .and_modify(|e| *e = merge_bounds(&g, e))
                .or_insert(g);
        });

    let overall_generics = where_generics.values().cloned().collect::<Vec<_>>();
    info!("Overall generics: {:?}", overall_generics);
    Some(overall_generics)
}

fn merge_bounds(a: &RuTy, b: &RuTy) -> RuTy {
    assert_eq!(a.name(), b.name());
    assert!(a.is_generic() && b.is_generic());

    let mut a_bounds = a.expect_generic().bounds().clone();
    let mut b_bounds = b.expect_generic().bounds().clone();

    a_bounds.append(&mut b_bounds);
    let generic = RuGeneric::new(&a.name(), a_bounds);
    RuTy::Generic(generic)
}

pub fn generic_param_to_t(
    generics: &Generics<'_>,
    param: &GenericParam<'_>,
    tcx: &TyCtxt<'_>,
) -> Option<RuTy> {
    if let GenericParamKind::Type { .. } = &param.kind {
        if let ParamName::Plain(ident) = &param.name {
            let name = ident.name.to_ident_string();

            let mut bounds: Vec<RuTrait> = Vec::new();
            let where_region_predicts = generics.bounds_for_param(param.def_id);
            for where_region_predict in where_region_predicts {
                bounds.extend(
                    where_region_predict
                        .bounds
                        .iter()
                        .filter_map(|b| generic_bound_to_trait(b, tcx))
                        .collect::<Vec<_>>(),
                );
            }
            return Some(RuTy::Generic(RuGeneric::new(&name, bounds)));
        }
    }

    None
}

pub fn generic_bound_to_trait(bound: &GenericBound<'_>, tcx: &TyCtxt<'_>) -> Option<RuTrait> {
    match bound {
        GenericBound::Trait(trait_ref, _) => {
            let def_id = trait_ref.trait_ref.trait_def_id()?;

            analyze_trait(def_id, tcx);

            let name = tcx.def_path_str(def_id);

            //let implemented_by = trait_implementations(def_id, tcx);
            //let std_impls = implemented_by.iter().filter(|im| im).collect::<Vec<_>>();
            //info!("Trait {}: implemented by {:?}", name, implemented_by);
            //info!(
            //    "Trait {} implemented by: {:?}",
            //    name, &implemented_by.non_blanket_impls
            //);
            Some(RuTrait::new(&name, vec![], vec![]))
        }
        // GenericBound::LangItemTrait(_, _, _, _) => todo!(),
        GenericBound::Outlives(_) => None,
        _ => todo!(),
    }
}

pub fn fn_ret_ty_to_t(
    ret_ty: &FnRetTy,
    self_ty: Option<&RuTy>,
    associated_types: Option<&HashMap<String, RuTy>>,
    defined_generics: &Vec<RuTy>,
    tcx: &TyCtxt<'_>,
) -> Option<RuTy> {
    match ret_ty {
        FnRetTy::DefaultReturn(_) => None,
        FnRetTy::Return(ty) => ty_to_t(ty, self_ty, associated_types, defined_generics, tcx),
    }
}

pub fn join_path_to_str(path: &rustc_hir::Path) -> String {
    path.segments
        .iter()
        .map(|s| s.ident.to_string())
        .collect::<Vec<String>>()
        .join("::")
}

pub fn node_to_name(node: &Node<'_>, tcx: &TyCtxt<'_>) -> Option<String> {
    match node {
        Node::Item(item) => Some(item_to_name(item, tcx)),
        Node::Crate(_) => Some("crate".to_string()),
        Node::ForeignItem(fi) => Some(fi.ident.name.to_ident_string()),
        Node::ImplItem(ii) => Some(ii.ident.name.to_ident_string()),
        Node::TraitItem(ti) => Some(ti.ident.name.to_ident_string()),
        Node::Variant(v) => Some(v.ident.name.to_ident_string()),
        Node::Field(f) => Some(f.ident.name.to_ident_string()),
        Node::Lifetime(lt) => Some(lt.ident.name.to_ident_string()),
        Node::GenericParam(param) => Some(param.name.ident().name.to_ident_string()),
        _ => None,
    }
}

pub fn item_to_name(item: &Item<'_>, tcx: &TyCtxt<'_>) -> String {
    match &item.kind {
        ItemKind::Impl(im) => ty_to_name(im.self_ty, tcx),
        ItemKind::Struct(_, _) => tcx.def_path_str(item.owner_id.def_id.to_def_id()),
        ItemKind::Enum(_, _) => tcx.def_path_str(item.owner_id.def_id.to_def_id()),
        ItemKind::Fn(_, _, _) => tcx.def_path_str(item.owner_id.def_id.to_def_id()),
        _ => item.ident.name.to_ident_string(),
    }
}

pub fn ty_to_name(ty: &Ty<'_>, tcx: &TyCtxt<'_>) -> String {
    match &ty.kind {
        TyKind::Path(path) => qpath_to_name(path, tcx),
        TyKind::Ref(_, mut_ty) => ty_to_name(mut_ty.ty, tcx),
        TyKind::Array(ty, len) => format!(
            "[{}; {}]",
            ty_to_name(ty, tcx),
            eval_array_len(len, tcx).unwrap()
        ),
        TyKind::Tup(types) => {
            let types = types
                .iter()
                .map(|ty| ty_to_name(ty, tcx))
                .collect::<Vec<_>>();

            format!("({})", types.join(", "))
        }
        TyKind::Slice(ty) => format!("[{}]", ty_to_name(ty, tcx)),
        _ => todo!("Trying to convert ty to name: {:?}", ty),
    }
}

pub fn qpath_to_name(qpath: &QPath<'_>, tcx: &TyCtxt<'_>) -> String {
    match qpath {
        QPath::Resolved(_, path) => res_to_name(&path.res, tcx),
        _ => todo!(),
    }
}

pub fn path_to_name(path: &Path, tcx: &TyCtxt<'_>) -> String {
    res_to_name(&path.res, tcx)
}

pub fn res_to_name(res: &Res, tcx: &TyCtxt<'_>) -> String {
    match res {
        Res::Def(_, def_id) => tcx.def_path_str(*def_id),
        Res::PrimTy(prim) => prim.name_str().to_string(),
        _ => todo!("{:?}", res),
    }
}

pub fn impl_to_def_id(im: &Impl, tcx: &TyCtxt<'_>) -> Option<DefId> {
    let self_ty = im.self_ty;
    ty_to_def_id(self_ty, tcx)
}

pub fn ty_to_def_id(ty: &Ty<'_>, tcx: &TyCtxt<'_>) -> Option<DefId> {
    match &ty.kind {
        TyKind::Path(qpath) => match qpath {
            QPath::Resolved(_, path) => path.res.opt_def_id(),
            _ => todo!(),
        },
        TyKind::Ref(lifetime, mut_ty) => {
            let ty = mut_ty.ty;
            ty_to_def_id(ty, tcx)
        }
        TyKind::Array(ty, _) => None,
        TyKind::Tup(_) => None,
        TyKind::Slice(_) => None,
        _ => todo!("Trying to convert to struct: {:?}", &ty.kind),
    }
}

fn eval_array_len(array_len: &ArrayLen, tcx: &TyCtxt<'_>) -> Option<usize> {
    // println!("{:#?}", array_len);
    let array_len_def_id = array_len.hir_id().owner.to_def_id();
    // println!("{:#?}", array_len.hir_id());
    // println!("{:#?}", array_len_def_id);
    // if format!("{:?}", array_len_def_id)
    //     == "DefId(0:2553 ~ hashbrown[d727]::control::group::sse2::{impl#0}::static_empty)"
    // {
    //     return None;
    // }
    let len = tcx.const_eval_poly(array_len_def_id);

    if let Some(len) = len.ok() {
        match len {
            ConstValue::Scalar(scalar) => match scalar {
                Scalar::Int(i) => Some(i.to_target_usize(tcx.clone()) as usize),
                Scalar::Ptr(_, _) => None,
            },
            ConstValue::Slice { .. } => None,
            ConstValue::Indirect { .. } => None,
            _ => None,
        }
    } else {
        None
    }
}

fn def_path_data(data: &DefPathData) -> Option<String> {
    match data {
        DefPathData::CrateRoot => Some("crate".to_string()),
        DefPathData::Impl => None,
        DefPathData::TypeNs(ty) => Some(ty.to_ident_string()),
        DefPathData::ValueNs(value) => Some(value.to_ident_string()),
        DefPathData::MacroNs(mac) => Some(mac.to_ident_string()),
        DefPathData::LifetimeNs(_) => None,
        DefPathData::Ctor => None,
        DefPathData::AnonConst => None,
        DefPathData::ForeignMod => None,
        _ => None,
    }
}

pub fn span_to_path(span: &Span, tcx: &TyCtxt<'_>) -> Option<PathBuf> {
    let file_name = tcx.sess().source_map().span_to_filename(span.clone());
    match file_name {
        FileName::Real(real_file_name) => match real_file_name {
            RealFileName::LocalPath(path) => Some(path),
            RealFileName::Remapped { .. } => None,
        },
        _ => todo!(),
    }
}

fn fmt_string(source: &str) -> io::Result<String> {
    let rustfmt = fmt_path()?;
    let mut cmd = Command::new(&*rustfmt);
    cmd.stdin(Stdio::piped()).stdout(Stdio::piped());

    let mut child = cmd.spawn()?;
    let mut child_stdin = child.stdin.take().unwrap();
    let mut child_stdout = child.stdout.take().unwrap();

    let source = source.to_owned();
    let stdin_handle = std::thread::spawn(move || {
        let _ = child_stdin.write_all(source.as_bytes());
        source
    });

    let mut output = vec![];
    io::copy(&mut child_stdout, &mut output)?;
    let status = child.wait()?;
    let source = stdin_handle.join().unwrap();

    match String::from_utf8(output) {
        Ok(source) => match status.code() {
            Some(0) => Ok(source),
            Some(2) => Err(io::Error::new(
                io::ErrorKind::Other,
                "Rustfmt parsing errors".to_string(),
            )),
            Some(3) => Ok(source),
            _ => Err(io::Error::new(
                io::ErrorKind::Other,
                "Internal rustfmt error".to_string(),
            )),
        },
        Err(_) => Ok(source),
    }
}

pub fn is_local(def_id: DefId) -> bool {
    def_id.krate == CrateNum::from_u32(0)
}

/// Fetch all implementations of a trait with given def_id
fn trait_implementations<'tcx>(def_id: DefId, tcx: &TyCtxt<'tcx>) -> &'tcx PublicTraitImpls {
    let trait_impls = tcx.trait_impls_of(def_id);
    // Make the private fields of trait impls instance public
    let public_trait_impls: &PublicTraitImpls = unsafe { std::mem::transmute(trait_impls) };

    public_trait_impls
}

pub struct PublicTraitImpls {
    pub blanket_impls: Vec<DefId>,
    pub non_blanket_impls: FxIndexMap<SimplifiedType, Vec<DefId>>,
}

/// Copied from Miri
/// Returns the "default sysroot" if no `--sysroot` flag is set.
/// Should be a compile-time constant.
pub fn compile_time_sysroot() -> Option<String> {
    if option_env!("RUSTC_STAGE").is_some() {
        // This is being built as part of rustc, and gets shipped with rustup.
        // We can rely on the sysroot computation in librustc.
        return None;
    }
    // For builds outside rustc, we need to ensure that we got a sysroot
    // that gets used as a default.  The sysroot computation in librustc would
    // end up somewhere in the build dir.
    // Taken from PR <https://github.com/Manishearth/rust-clippy/pull/911>.
    let home = option_env!("RUSTUP_HOME").or(option_env!("MULTIRUST_HOME"));
    let toolchain = option_env!("RUSTUP_TOOLCHAIN").or(option_env!("MULTIRUST_TOOLCHAIN"));
    Some(match (home, toolchain) {
        (Some(home), Some(toolchain)) => format!("{}/toolchains/{}", home, toolchain),
        _ => option_env!("RUST_SYSROOT")
            .expect("To build Miri without rustup, set the `RUST_SYSROOT` env var at build time")
            .to_owned(),
    })
}
