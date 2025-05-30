#![feature(rustc_private)]
extern crate rustc_apfloat;
extern crate rustc_ast;
extern crate rustc_data_structures;
extern crate rustc_driver;
extern crate rustc_hir;
extern crate rustc_index;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_session;
extern crate rustc_span;
extern crate rustc_target;

#[macro_use]
extern crate derive_builder;

extern crate clap;

mod analysis;
mod data_structures;
mod extractor;
mod hir;
mod mir;
mod monitor;
mod options;
mod traits;
mod types;
pub mod utils;
mod writer;

#[cfg(feature = "analysis")]
use crate::hir::hir_analysis;
use crate::mir::CUSTOM_OPT_MIR;
use crate::options::RuConfig;
use crate::utils::rustc_get_crate_name;
use clap::Parser;
use log::{debug, info, warn};
use rustc_driver::Compilation;
use rustc_hir::intravisit::nested_filter::None;
use rustc_interface::interface::Compiler;
use rustc_interface::{Config, Queries};
use rustc_middle::ty::TyCtxt;
use std::path::Path;
use std::process::exit;
use std::str::FromStr;
use std::{fs, process};

pub const LOG_DIR: &'static str = "ru_log";
pub const LOG_EXT: &'static str = "json";
pub const MIR_LOG_NAME: &'static str = "mir";
pub const HIR_LOG_PATH: &'static str = "hir";

// Mainly for debugging
pub const INSTRUMENTED_MIR_LOG_NAME: &'static str = "instrumented-mir";
pub const DOT_DIR: &'static str = "ru_dot";

pub struct EmptyCallbacks {}

impl rustc_driver::Callbacks for EmptyCallbacks {}

pub struct CompilerCallbacks {}

impl CompilerCallbacks {
    pub fn new() -> Self {
        CompilerCallbacks {}
    }
}

fn enter_with_fn<'tcx, TyCtxtFn>(queries: &'tcx rustc_interface::Queries<'tcx>, enter_fn: TyCtxtFn)
where
    TyCtxtFn: Fn(TyCtxt),
{
    queries.global_ctxt().unwrap().enter(enter_fn);
}

impl rustc_driver::Callbacks for CompilerCallbacks {
    fn config(&mut self, _config: &mut Config) {
        _config.override_queries = Some(|session, providers| {
            providers.queries.optimized_mir = CUSTOM_OPT_MIR;
        });
    }

    #[cfg(feature = "analysis")]
    fn after_analysis<'tcx>(
        &mut self,
        _compiler: &Compiler,
        _queries: &'tcx Queries<'tcx>,
    ) -> Compilation {
        enter_with_fn(_queries, hir_analysis);
        Compilation::Continue
    }
}

// impl rustc_driver::Callbacks for RfocxtCallbacks {
//     fn config(&mut self, config: &mut interface::Config) {
//         self.source_name = format!("{:?}", config.input.source_name());
//         config.crate_cfg.push("rfocxt".to_string());
//         info!("Source file: {}", self.source_name);
//     }

//     fn after_expansion<'tcx>(
//         &mut self,
//         _compiler: &interface::Compiler,
//         _queries: &'tcx Queries<'tcx>,
//     ) -> Compilation {
//         _queries
//             .global_ctxt()
//             .unwrap()
//             .enter(|tcx| self.run_analysis(tcx));
//         Compilation::Continue
//     }
// }

pub fn arg_value<'a>(
    args: impl IntoIterator<Item = &'a String>,
    find_arg: &str,
    pred: impl Fn(&str) -> bool,
) -> Option<&'a str> {
    let mut args = args.into_iter().map(String::as_str);

    while let Some(arg) = args.next() {
        let arg: Vec<_> = arg.splitn(2, '=').collect();
        if arg.get(0) != Some(&find_arg) {
            continue;
        }

        let value = arg.get(1).cloned().or_else(|| args.next());
        if value.as_ref().map_or(false, |p| pred(p)) {
            return value;
        }
    }
    None
}

pub fn sysroot() -> String {
    let out = process::Command::new("rustc")
        .arg("--print=sysroot")
        .current_dir("..")
        .output()
        .unwrap();

    let sysroot = String::from_utf8(out.stdout).unwrap();
    let sysroot = sysroot.trim();
    sysroot.to_string()
}

pub fn get_compiler_args(args: &[String]) -> Vec<String> {
    let have_sys_root = arg_value(args, "--sysroot", |_| true).is_some();
    // Setting RUSTC_WRAPPER causes Cargo to pass 'rustc' as the first argument.
    // We're invoking the compiler programmatically, so we ignore this/
    let wrapper_mode = Path::new(&args[1]).file_stem() == Some("rustc".as_ref());

    let mut rustc_args: Vec<_>;

    if wrapper_mode {
        // we still want to be able to invoke it normally though
        rustc_args = args.iter().skip(1).map(|s| s.to_string()).collect();
    } else {
        rustc_args = args
            .iter()
            .skip(1)
            .take_while(|s| *s != "--")
            .map(|s| s.to_string())
            .collect();
        rustc_args.insert(0, "".to_owned());
    }

    rustc_args.push("--emit".to_string());
    rustc_args.push("mir".to_string());

    // this conditional check for the --sysroot flag is there so users can call
    // `clippy_driver` directly
    // without having to pass --sysroot or anything
    if !have_sys_root {
        rustc_args.push("--sysroot".to_owned());
        rustc_args.push(sysroot());
    }
    rustc_args.push("--allow".to_owned());
    rustc_args.push("dead_code".to_owned());
    rustc_args.push("--allow".to_owned());
    rustc_args.push("deprecated".to_owned());
    rustc_args.push("--allow".to_owned());
    rustc_args.push("unused".to_owned());

    rustc_args
}

fn run_rustc() -> Result<(), i32> {
    #[cfg(feature = "analysis")]
    {
        if let Ok(_) = fs::remove_dir_all(LOG_DIR) {}
        fs::create_dir_all(LOG_DIR).expect("Could not create the log directory");
    }

    let mut std_env_args: Vec<String> = std::env::args().collect();
    // println!("{:#?}", std_env_args);

    let rustc_args = get_compiler_args(&std_env_args);
    // println!("{:#?}", rustc_args);

    let do_instrument = rustc_get_crate_name(&rustc_args) == RuConfig::env_crate_name();

    pass_to_rustc(&rustc_args, do_instrument);
    return Ok(());
}

pub fn pass_to_rustc(rustc_args: &[String], instrumentation: bool) {
    let err = if instrumentation {
        // The crate we want to analyze, so throw up the instrumentation
        info!(
            "MAIN: Instrumenting crate {}",
            rustc_get_crate_name(&rustc_args)
        );
        let mut callbacks = CompilerCallbacks::new();
        rustc_driver::RunCompiler::new(&rustc_args, &mut callbacks).run()
    } else {
        // A dependency, don't do anything, otherwise we might break incremental compilation
        let mut callbacks = EmptyCallbacks {};
        rustc_driver::RunCompiler::new(&rustc_args, &mut callbacks).run()
    };
    //let mut callbacks = CompilerCallbacks::new();
    //let err = rustc_driver::RunCompiler::new(&rustc_args, &mut callbacks).run();

    if err.is_err() {
        eprintln!("Error while compiling dependency");
        exit(-1);
    }
}

// fn main() {
//     // Initialize the logger
//     env_logger::init();

//     exit(run_rustc().err().unwrap_or(0))
// }
