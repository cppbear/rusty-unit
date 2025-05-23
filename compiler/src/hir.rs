use crate::analysis::Analysis;
use crate::mir::def_id_to_str;
use crate::rustc_hir::intravisit::Map;
use crate::types::{
    def_id_name, FieldAccessItem, RuCallable, RuEnum, RuEnumInit, RuEnumVariant, RuFunction,
    RuMethod, RuParam, RuStaticMethod, RuStruct, RuStructInit, RuTrait, RuTy,
};
use crate::utils::{
    def_id_to_enum, def_id_to_t, fn_ret_ty_to_t, generics_to_ts, impl_to_def_id, is_local,
    item_to_name, node_to_name, path_to_name, res_to_name, span_to_path, ty_to_name, ty_to_param,
    ty_to_t,
};
#[cfg(feature = "analysis")]
use crate::writer::{HirObject, HirObjectBuilder, HirWriter};
use crate::{RuConfig, HIR_LOG_PATH, LOG_DIR};
use log::{debug, info, warn};
use petgraph::visit::Walker;
use rustc_ast::IsAuto;
use rustc_data_structures::undo_log::UndoLogs;
use rustc_hir::Safety;
use rustc_hir::{
    AssocItemKind, BodyId, EnumDef, FnSig, Generics, HirId, Impl, ImplItemKind, Item, ItemKind,
    Node, Variant, VariantData,
};
use rustc_middle::ty::Visibility;
use rustc_middle::ty::{Ty, TyCtxt};
use rustc_span::def_id::{DefId, LocalDefId, LOCAL_CRATE};
use rustc_span::Span;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

#[cfg(feature = "analysis")]
pub fn hir_analysis(tcx: TyCtxt<'_>) {
    let current_crate_name = tcx.crate_name(LOCAL_CRATE);
    // if current_crate_name.as_str() != RuConfig::env_crate_name() {
    //   return;
    // }

    println!("Hir analysis for crate: {}", current_crate_name);

    let mut callables = vec![];
    for i in tcx.hir().items() {
        let item = tcx.hir().item(i);
        let def_id = item.owner_id.to_def_id();
        let def_id_to_str = def_id_to_str(def_id, &tcx);
        println!("def_id_to_str == {:?}", def_id_to_str);
        if def_id_to_str.contains("serde")
            || def_id_to_str ==
                "map::slice::<impl std::cmp::PartialEq<map::slice::Slice<K2, V2>> for [(K, V); N]>"
            || def_id_to_str == "<map::slice::Slice<K, V> as std::cmp::Eq>"
            || def_id_to_str == "<map::slice::Slice<K, V> as std::cmp::PartialOrd>"
            || def_id_to_str == "<map::IndexMap<K, V, S> as std::cmp::Eq>"
            || def_id_to_str
                == "set::slice::<impl std::cmp::PartialEq<set::slice::Slice<U>> for [T; N]>"
            || def_id_to_str == "<set::slice::Slice<T> as std::cmp::Eq>"||def_id_to_str=="<set::slice::Slice<T> as std::cmp::PartialOrd>"||def_id_to_str=="<set::IndexSet<T, S> as std::cmp::Eq>"
            || def_id_to_str=="<TryReserveError as std::cmp::Eq>" || def_id_to_str=="<TryReserveErrorKind as std::cmp::Eq>"||def_id_to_str=="<GetDisjointMutError as std::cmp::Eq>"
            // || def_id_to_str == "read::decoder::{use#5}"
            // || def_id_to_str == "read::decoder::DecoderReader"
            // || def_id_to_str == "<read::decoder::DecoderReader<'e, E, R> as std::io::Read>"
            // || def_id_to_str == "write::encoder::EncoderWriter"
            // || def_id_to_str == "write::encoder_string_writer::StrConsumer"
            // || def_id_to_str == "engine::general_purpose::decode::decode_helper"
            // || def_id_to_str == "engine::general_purpose::decode::complete_quads_len"
            // || def_id_to_str.contains("engine::general_purpose::decode")
            // || def_id_to_str == "engine::general_purpose::decode_suffix::{use#6}"
            // || def_id_to_str == "engine::general_purpose::GeneralPurpose"
            // || def_id_to_str == "engine::general_purpose::encode_table"
            // || def_id_to_str == "alphabet::Alphabet"
            // || def_id_to_str == "<[T; N] as __private::ext::RepAsIteratorExt<'q>>"
            // || def_id_to_str == "<[T; N] as rng::Fill>"
            // || def_id_to_str == "<rngs::small::SmallRng as rand_core::SeedableRng>"
            // || def_id_to_str == "<rngs::mock::StepRng as std::cmp::PartialEq>"
            // || def_id_to_str == "rngs::xoshiro256plusplus::Xoshiro256PlusPlus"
            // || def_id_to_str == "rngs::reseeding::ReseedingRng"
            // || def_id_to_str == "<rngs::xoshiro256plusplus::Xoshiro256PlusPlus as rand_core::SeedableRng>"
            // || def_id_to_str == "<rngs::std::StdRng as rand_core::SeedableRng>"
            // || def_id_to_str == "seq::index::sample_array"
            // || def_id_to_str == "<&'a [u8; N] as regex::bytes::Replacer>"
            // || def_id_to_str == "<[u8; N] as regex::bytes::Replacer>"
            // || def_id_to_str == "buffer::Buffer"
            || def_id_to_str == "<de::impls::ArrayVisitor<[T; 0]> as de::Visitor<'de>>"
            || def_id_to_str == "de::impls::<impl de::Deserialize<'de> for [T; 0]>"
            || def_id_to_str == "<de::impls::ArrayVisitor<[T; 1]> as de::Visitor<'de>>"
            || def_id_to_str == "<de::impls::ArrayInPlaceVisitor<'a, [T; 1]> as de::Visitor<'de>>"
            || def_id_to_str == "de::impls::<impl de::Deserialize<'de> for [T; 1]>"
            || def_id_to_str.contains(" as de::Visitor<'de>>")
            || def_id_to_str.contains("de::impls::<impl de::Deserialize<'de> for ")
            || def_id_to_str == "<dyn de::Expected as std::fmt::Display>"
            || def_id_to_str == "ser::impls::<impl ser::Serialize for [T; 0]>"
            || def_id_to_str.contains("ser::impls::<impl ser::Serialize for [T;")
            || def_id_to_str == "read::build_hex_table"
            || def_id_to_str == "HybridGrowingHashmapChar"
            || def_id_to_str == "<(dyn std::error::Error + 'a) as aserror::AsDynError<'a>>"
            || def_id_to_str.contains(" + 'a) as aserror::AsDynError<'a>>")
            || def_id_to_str == "<dyn std::error::Error as aserror::Sealed>"
            || def_id_to_str == "<dyn std::error::Error + std::marker::Send as aserror::Sealed>"
            || (def_id_to_str.contains(" as aserror::Sealed>") && def_id_to_str != "<T as aserror::Sealed>")
        {
            continue;
        }

        let span: &Span = &item.span;
        let file_path = span_to_path(span, &tcx);
        if file_path.is_none() {
            continue;
        }

        //info!("HIR: Scanning file {:?}", file_path.as_ref());
        if let Some(path) = file_path.as_ref() {
            if path.ends_with("rusty_monitor.rs") {
                continue;
            }

            if let Some(path) = path.to_str() {
                if path.contains(".cargo") {
                    continue;
                }
            }
        }

        println!("Hir analysis for span: {:?}", span);

        match &item.kind {
            ItemKind::Fn(sig, generics, body_id) => {
                if &item.ident.name.to_string() != "main" && allowed_item(item, &tcx) {
                    info!("HIR: Analyzing function {}", item_to_name(item, &tcx));
                    analyze_fn(
                        sig,
                        i.owner_id.def_id,
                        body_id,
                        tcx.visibility(i.owner_id.to_def_id()),
                        generics,
                        file_path.unwrap(),
                        &mut callables,
                        &tcx,
                    )
                }
            }
            ItemKind::Impl(im) => {
                if allowed_item(item, &tcx) {
                    info!("HIR: Analyzing impl {}", item_to_name(item, &tcx));
                    analyze_impl(im, file_path.unwrap(), &mut callables, &tcx)
                }
            }
            ItemKind::Struct(s, g) => {
                if allowed_item(item, &tcx) {
                    info!("HIR: Analyzing struct {}", item_to_name(item, &tcx));
                    analyze_struct(
                        item.owner_id.def_id,
                        s,
                        g,
                        tcx.visibility(i.owner_id.to_def_id()),
                        file_path.unwrap(),
                        &mut callables,
                        &tcx,
                    );
                }
            }
            ItemKind::Enum(enum_def, g) => {
                if allowed_item(item, &tcx) {
                    info!("HIR: Analyzing enum {}", item_to_name(item, &tcx));
                    analyze_enum(
                        item.owner_id.def_id,
                        enum_def,
                        g,
                        tcx.visibility(i.owner_id.to_def_id()),
                        file_path.unwrap(),
                        &mut callables,
                        &tcx,
                    );
                }
            }
            ItemKind::Trait(is_auto, _, _, _, _) => {
                let auto = match is_auto {
                    IsAuto::Yes => true,
                    IsAuto::No => false,
                };
                info!("HIR: trait is auto: {}", auto);
            }
            //ItemKind::Mod(e)
            _ => {}
        }
    }

    // let hir_output_path = Path::new(LOG_DIR).join(HIR_LOG_PATH);
    // let content = serde_json::to_string(&callables).unwrap();
    //
    // #[cfg(file_writer)]
    // FileWriter::new(hir_output_path).write(&content).unwrap();
    //
    // #[cfg(redis_writer)]
    // todo!()

    if callables.is_empty() {
        return;
    }

    let hir_object: HirObject = HirObjectBuilder::default()
        .name(current_crate_name.to_ident_string())
        .callables(callables)
        .impls(HashMap::new())
        .build()
        .unwrap();
    HirWriter::write(&hir_object);
}

fn allowed_item(item: &Item<'_>, tcx: &TyCtxt<'_>) -> bool {
    //let item_name = item_to_name(item, &tcx).to_lowercase();
    //item_name.contains("serde") && !item_name.contains("test") && !item_name.contains("snafu")
    true
}

fn analyze_fn(
    sig: &FnSig,
    local_def_id: LocalDefId,
    body_id: &BodyId,
    vis: Visibility<DefId>,
    generics: &Generics,
    file_path: PathBuf,
    callables: &mut Vec<RuCallable>,
    tcx: &TyCtxt<'_>,
) {
    if let Safety::Unsafe = sig.header.safety {
        // Skip unsafe functions
        return;
    }

    let parent_node = tcx
        .hir()
        .get_parent_item(tcx.local_def_id_to_hir_id(local_def_id));
    let parent_hir_id = tcx.local_def_id_to_hir_id(parent_node.def_id);
    let parent_node = tcx.hir().hir_node(parent_hir_id);
    if let Node::Item(item) = &parent_node {
        if let ItemKind::Fn(_, _, _) = &item.kind {
            // Ignore functions that are defined within another functions
            return;
        }
    }

    let hir_id = tcx.local_def_id_to_hir_id(local_def_id);
    let global_id = def_id_to_str(local_def_id.to_def_id(), &tcx).replace("::", "__");
    let is_public = is_public(vis);
    let fn_decl = &sig.decl;

    let fn_name = node_to_name(&tcx.hir().hir_node(hir_id), tcx).unwrap();
    //let fn_name = tcx.hir().get(hir_id).ident().unwrap().to_string();

    let fn_generics = generics_to_ts(generics, None, None, tcx);
    if let Some(fn_generics) = fn_generics {
        // self_hir_id must never be used, so just pass a dummy value
        let mut params = Vec::with_capacity(fn_decl.inputs.len());
        for input in fn_decl.inputs.iter() {
            if let Some(param) = ty_to_param(None, input, None, None, &fn_generics, tcx) {
                params.push(param);
            } else {
                return;
            }
        }

        let return_type = fn_ret_ty_to_t(&fn_decl.output, None, None, &fn_generics, tcx);

        if let Some(return_type) = &return_type {
            debug!("HIR: Output type is {:?}", return_type);
        }

        let function_item = RuFunction::new(
            is_public,
            &fn_name,
            vec![],
            params,
            return_type,
            file_path.to_str().unwrap(),
            global_id,
        );
        let fn_callable = RuCallable::Function(function_item);
        callables.push(fn_callable);
    }
}

fn analyze_enum(
    enum_local_def_id: LocalDefId,
    enum_def: &EnumDef,
    g: &Generics,
    visibility: Visibility<DefId>,
    file_path: PathBuf,
    callables: &mut Vec<RuCallable>,
    tcx: &TyCtxt<'_>,
) {
    let is_public = is_public(visibility);
    let enum_hir_id = tcx.local_def_id_to_hir_id(enum_local_def_id);
    let enum_def_id = enum_local_def_id.to_def_id();
    let self_name = node_to_name(&tcx.hir().hir_node(enum_hir_id), tcx).unwrap();

    let self_ty = RuTy::Struct(RuStruct::new(&self_name, vec![], is_local(enum_def_id)));

    let generics = generics_to_ts(g, Some(&self_ty), None, tcx);
    if let Some(generics) = generics {
        //let self_ty = def_id_to_enum(enum_def_id, tcx).unwrap();
        let self_ty = RuTy::Enum(RuEnum::new(
            &self_name,
            generics.clone(),
            vec![],
            is_local(enum_def_id),
        ));
        if self_name.contains("serde") {
            // Skip too hard stuff
            return;
        }

        for variant in enum_def.variants {
            let variant_name = variant.ident.name.to_ident_string();

            let variant = extract_enum_variant(variant, enum_hir_id, &generics, tcx);
            if let Some(variant) = variant {
                debug!(
                    "HIR: Extracted enum variant {}::{}",
                    &self_name, &variant_name
                );
                let enum_init = RuCallable::EnumInit(RuEnumInit::new(
                    file_path.to_str().unwrap(),
                    variant,
                    self_ty.clone(),
                    is_public,
                ));
                callables.push(enum_init);
            } else {
                warn!(
                    "HIR: Could not extract enum variant {}::{}",
                    &self_name, &variant_name
                );
            }
        }
    }
}

fn extract_enum_variant(
    variant: &Variant,
    hir_id: HirId,
    generics: &Vec<RuTy>,
    tcx: &TyCtxt<'_>,
) -> Option<RuEnumVariant> {
    match &variant.data {
        VariantData::Struct { fields, .. } => {
            let def_id = variant.def_id.to_def_id();
            let struct_name = node_to_name(&tcx.hir().hir_node(variant.hir_id), tcx).unwrap();
            let params = fields
                .iter()
                .filter_map(|f| ty_to_param(Some(f.ident.as_str()), f.ty, None, None, &vec![], tcx))
                .collect::<Vec<_>>();

            if params.len() != fields.len() {
                warn!("Could not extract enum variant: {}", struct_name);
                return None;
            }

            let v = RuEnumVariant::Struct(variant.ident.name.to_ident_string(), params);
            Some(v)
        }
        VariantData::Tuple(fields, variant_hir_id, ..) => {
            debug!("--> ENUM variant extracting {:?}", variant_hir_id);
            let params = fields
                .iter()
                .filter_map(|f| ty_to_t(&f.ty, None, None, generics, tcx))
                .map(|ty| RuParam::new(None, ty, false))
                .collect::<Vec<_>>();
            if params.len() < fields.len() {
                return None;
            }

            let v = RuEnumVariant::Tuple(variant.ident.name.to_ident_string(), params);
            Some(v)
        }
        VariantData::Unit(_, _) => Some(RuEnumVariant::Unit(variant.ident.name.to_ident_string())),
    }
}

fn analyze_struct(
    struct_local_def_id: LocalDefId,
    vd: &VariantData,
    g: &Generics,
    vis: Visibility<DefId>,
    file_path: PathBuf,
    callables: &mut Vec<RuCallable>,
    tcx: &TyCtxt<'_>,
) {
    let mut struct_is_public = is_public(vis);

    let struct_generics = generics_to_ts(g, None, None, tcx);
    if let Some(struct_generics) = struct_generics {
        let struct_hir_id = tcx.local_def_id_to_hir_id(struct_local_def_id);
        match vd {
            VariantData::Struct { fields, .. } => {
                let def_id = struct_local_def_id.to_def_id();
                //let self_ty = def_id_to_t(def_id, tcx).unwrap();
                let self_name = node_to_name(&tcx.hir().hir_node(struct_hir_id), tcx).unwrap();
                info!("HIR: {} is public: {}", self_name, struct_is_public);
                let generics = generics_to_ts(g, None, None, tcx);
                if let Some(generics) = generics {
                    let self_ty =
                        RuTy::Struct(RuStruct::new(&self_name, generics, is_local(def_id)));
                    // if self_name.contains("serde") || self_name.contains("Buffer") {
                    //     // Skip too hard stuff
                    //     return;
                    // }
                    if self_name.contains("serde") {
                        // Skip too hard stuff
                        return;
                    }

                    for field in fields.iter() {
                        let ty = ty_to_t(field.ty, Some(&self_ty), None, &struct_generics, tcx);
                        if let Some(ty) = ty {
                            let def_id = struct_local_def_id.to_def_id();

                            let field_name = tcx
                                .hir()
                                .hir_node(field.hir_id)
                                .ident()
                                .unwrap()
                                .to_string();
                            debug!("HIR: Extracted field {}::{}", &self_name, &field_name);

                            /*let parent = def_id_to_t(def_id, tcx).unwrap();
                            let field_item = FieldAccessItem::new(
                              &field_name,
                              file_path.to_str().unwrap(),
                              ty,
                              parent,
                              is_public,
                            );*/
                        }

                        let field_is_public = is_public(tcx.visibility(field.def_id.to_def_id()));
                        if !field_is_public {
                            struct_is_public = false;
                        }
                    }

                    let mut params = Vec::with_capacity(fields.len());
                    for field in fields.iter() {
                        let name = field.ident.name.to_ident_string();
                        let param = ty_to_param(
                            Some(&name),
                            field.ty,
                            Some(&self_ty),
                            None,
                            &struct_generics,
                            tcx,
                        );
                        if let Some(param) = param {
                            params.push(param);
                        } else {
                            // An unknown type, ignore function
                            return;
                        }
                    }

                    debug!("HIR: Extracted struct init {}: {:?}", self_ty, params);
                    callables.push(RuCallable::StructInit(RuStructInit::new(
                        struct_is_public,
                        file_path.to_str().unwrap(),
                        params,
                        self_ty,
                    )));
                }
            }
            _ => {}
        }
    }
}

fn analyze_impl_items(
    im: &Impl,
    associated_types: &mut HashMap<String, RuTy>,
    self_ty: &RuTy,
    impl_generics: &Vec<RuTy>,
    tcx: &TyCtxt<'_>,
) {
    let items = im.items;
    for item in items {
        let impl_item = tcx.hir().impl_item(item.id);
        match &impl_item.kind {
            ImplItemKind::Type(ty) => {
                let t = ty_to_t(ty, Some(&self_ty), None, &impl_generics, tcx);
                if let Some(t) = t {
                    let associated_type_name = tcx
                        .hir()
                        .name(tcx.local_def_id_to_hir_id(ty.hir_id.owner))
                        .to_string();
                    associated_types.insert(associated_type_name, t);
                }
            }
            _ => {}
        }
    }
}

// Get associated types defined in the super traits
fn analyze_super_traits(
    trait_: DefId,
    self_ty: &RuTy,
    associated_types: &mut HashMap<String, RuTy>,
    impl_generics: &Vec<RuTy>,
    tcx: &TyCtxt<'_>,
) {
    for t in tcx.explicit_super_predicates_of(trait_).predicates {
        let pred = t.0.as_predicate().as_trait_clause();
        if let Some(pred) = pred {
            info!("Super trait: {:?}", pred.def_id());

            let super_trait: DefId = pred.def_id();
            let impls = tcx.hir().trait_impls(super_trait);
            for i in impls {
                let item = tcx.hir().expect_item(*i);
                match &item.kind {
                    ItemKind::Impl(im) => {
                        let self_ty_opt = ty_to_t(im.self_ty, None, None, &impl_generics, tcx);
                        if self_ty_opt.is_none() {
                            return;
                        }
                        let ty = self_ty_opt.unwrap();
                        if &ty == self_ty {
                            analyze_impl_items(im, associated_types, self_ty, impl_generics, tcx);
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}

fn analyze_impl(im: &Impl, file_path: PathBuf, callables: &mut Vec<RuCallable>, tcx: &TyCtxt<'_>) {
    // println!("{:#?}", im);
    let parent_def_id_opt = impl_to_def_id(im, tcx);
    if let Safety::Unsafe = im.safety {
        // Skip unsafe functions
        return;
    }
    if let Some(parent_def_id) = parent_def_id_opt {
        if parent_def_id.as_local().is_none() {
            return;
        }
    } else {
        return;
    }

    let trait_name = im
        .of_trait
        .as_ref()
        .map(|trait_ref| path_to_name(&trait_ref.path, tcx));

    let parent_def_id = parent_def_id_opt.unwrap();

    let parent_hir_id = tcx.local_def_id_to_hir_id(parent_def_id.expect_local());

    let impl_generics = generics_to_ts(&im.generics, None, None, tcx);
    if let Some(impl_generics) = impl_generics {
        let self_ty_opt = ty_to_t(im.self_ty, None, None, &impl_generics, tcx);
        if self_ty_opt.is_none() {
            info!("---->>>");
            return;
        }
        let self_ty = self_ty_opt.unwrap();

        let items = im.items;
        let mut associated_types = HashMap::new();
        analyze_impl_items(im, &mut associated_types, &self_ty, &impl_generics, tcx);

        if let Some(trait_) = im.of_trait.as_ref() {
            analyze_super_traits(
                trait_.trait_def_id().unwrap(),
                &self_ty,
                &mut associated_types,
                &impl_generics,
                tcx,
            );
        }

        info!("Created mapping: {:?}", associated_types);

        for item in items {
            // println!("item");
            let item_def_id = item.id.owner_id.def_id;
            let global_id = def_id_to_str(item_def_id.to_def_id(), tcx).replace("::", "__");

            let hir_id = tcx.local_def_id_to_hir_id(item_def_id);
            let impl_item = tcx.hir().impl_item(item.id);
            // println!("{:#?}", impl_item);
            match &impl_item.kind {
                ImplItemKind::Fn(sig, body_id) => {
                    // println!("fn");
                    if let Safety::Unsafe = sig.header.safety {
                        continue;
                    }
                    info!(
                        "HIR: Found method {}, parent: {}",
                        &impl_item.ident, parent_hir_id
                    );
                    let fn_name = tcx
                        .hir()
                        .hir_node(item.id.hir_id())
                        .ident()
                        .unwrap()
                        .to_string();
                    println!("fn_name == {:?}", fn_name);
                    // if fn_name == "static_empty"
                    //     || fn_name == "get_many_mut"
                    //     || fn_name == "get_many_key_value_mut"
                    if fn_name == "get_disjoint_opt_mut"
                        || fn_name == "get_disjoint_mut"
                        || fn_name == "get_disjoint_indices_mut"
                        || fn_name == "from"
                    {
                        continue;
                    }
                    let parent_name =
                        node_to_name(&tcx.hir().hir_node(parent_hir_id), tcx).unwrap();
                    println!("parent_name == {:?}", parent_name);
                    if parent_name == "map::HashMap" || parent_name == "set::HashSet" 
                    || parent_name == "write::encoder::EncoderWriter"
                    || parent_name == "rngs::small::SmallRng"
                    || parent_name == "rngs::reseeding::ReseedingCore"
                    || parent_name == "regex::bytes::Captures"
                    || parent_name == "regex::string::Captures"
                    || parent_name == "value::Value"
                    {
                        // Skip too hard stuff
                        warn!("HIR: Skipping too hard method");
                        continue;
                    }
                    let fn_generics = generics_to_ts(
                        &impl_item.generics,
                        Some(&self_ty),
                        Some(&associated_types),
                        tcx,
                    );
                    if let Some(fn_generics) = fn_generics {
                        let mut overall_generics = impl_generics.clone();
                        overall_generics.append(&mut fn_generics.clone());

                        info!("Returns: {:?}", &sig.decl.output);
                        let return_type = fn_ret_ty_to_t(
                            &sig.decl.output,
                            Some(&self_ty),
                            Some(&associated_types),
                            &overall_generics,
                            tcx,
                        );
                        if let Some(return_type) = return_type.as_ref() {
                            debug!("HIR: Return type is {:?}", &return_type);
                        }

                        let is_public = is_public(tcx.visibility(impl_item.owner_id.to_def_id()));
                        let parent_name =
                            node_to_name(&tcx.hir().hir_node(parent_hir_id), tcx).unwrap();
                        println!("parent name: {:?}", parent_name);
                        if parent_name.contains("serde")
                            || parent_name == "map::slice::Slice"
                            || parent_name == "map::IndexMap"
                            || parent_name == "set::slice::Slice"
                            || parent_name == "set::IndexSet"
                            || parent_name == "value::Value"
                        {
                            // Skip too hard stuff
                            warn!("HIR: Skipping serde method");
                            continue;
                        }

                        let mut params = Vec::with_capacity(sig.decl.inputs.len());
                        for input in sig.decl.inputs.iter() {
                            // info!("input: {:#?}", input);
                            let param = ty_to_param(
                                None,
                                input,
                                Some(&self_ty),
                                Some(&associated_types),
                                &overall_generics,
                                tcx,
                            );
                            // info!("param: {:#?}", param);
                            if let Some(param) = param {
                                debug!("HIR: Extracting parameter {:?}", param);
                                params.push(param);
                            } else {
                                // An unknown type, ignore function
                                warn!("HIR: Unknown parameter, skipping method.");
                                return;
                            }
                        }

                        if !sig.decl.implicit_self.has_implicit_self() {
                            // Static method
                            info!("HIR: Method is static");
                            let static_method_item = RuStaticMethod::new(
                                &fn_name,
                                file_path.to_str().unwrap(),
                                params,
                                return_type,
                                self_ty.clone(),
                                fn_generics,
                                is_public,
                                trait_name.clone(),
                                global_id,
                            );
                            let static_method_callable =
                                RuCallable::StaticFunction(static_method_item);
                            callables.push(static_method_callable);
                        } else {
                            // Dynamic method
                            info!("HIR: Method is associative");
                            let method_item = RuMethod::new(
                                &fn_name,
                                file_path.to_str().unwrap(),
                                params,
                                return_type,
                                self_ty.clone(),
                                fn_generics,
                                is_public,
                                trait_name.clone(),
                                global_id,
                            );
                            let method_callable = RuCallable::Method(method_item);
                            info!("HIR: Added method {}", fn_name);
                            callables.push(method_callable);
                        }
                    }
                }
                ImplItemKind::Type(ty) => {
                    info!("Impl ty alias: {:?}", ty);
                }
                _ => {}
            }
        }
    }
}

fn is_public(vis: Visibility<DefId>) -> bool {
    match &vis {
        Visibility::Public => true,
        _ => false,
    }
}
