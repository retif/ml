#![crate_name="mml"]
#![crate_type= "lib"]

#![feature(rustc_private)]
#![feature(box_patterns)]


#![doc(html_root_url = "https://docs.rs/mml/0.1.41")]

#![cfg_attr(feature = "nightly", feature(plugin))]

#![cfg_attr(feature = "lints", plugin(clippy))]
#![cfg_attr(feature = "lints", deny(warnings))]
#![cfg_attr(not(any(feature = "lints", feature = "nightly")), deny())]
#![deny(
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unused_import_braces,
    unused_qualifications
)]

//! ![uml](ml.svg)

extern crate itertools;
extern crate walkdir;
extern crate dot;

extern crate rustc_ast;
extern crate rustc_ast_pretty;
extern crate rustc_errors;
extern crate rustc_error_codes;
extern crate rustc_session;
extern crate rustc_span;
extern crate rustc_serialize;
extern crate rustc_hir;
extern crate rustc_parse;
extern crate rustc_hash;
extern crate rustc_interface;

pub mod prelude;
pub mod module;
pub mod core;

use std::process::{Command, Stdio};
use std::io::{self, Write, Read};
use std::path::Path;
use std::fs::{self, File};
use std::ffi::OsStr;
use std::rc::Rc;

use rustc_errors::emitter::ColorConfig;
use rustc_errors::Handler;

use rustc_session::parse::ParseSess;
use rustc_ast::{ast, ptr};
use rustc_span::source_map::{SourceMap, FilePathMapping};

use walkdir::WalkDir;
use crate::core::ListItem;
use module::Module;
use module::path::ModulePath;
use once_cell::sync::OnceCell;

/// The default name of *graph/dot* file.
pub const DEFAULT_NAME_DOT: &'static str = "ml.dot";
/// The default name of *image/svg* file.
pub const DEFAULT_NAME_PNG: &'static str = "ml.svg";

#[derive(Debug)]
pub struct Config {
    pub include_methods: bool,
    pub include_fields: bool,
    pub include_implems: bool,
    pub struct_header_bgcolor: String,
    pub struct_fields_bgcolor: String,
    pub struct_method_bgcolor: String,
    pub struct_implem_bgcolor: String,
    pub enum_header_bgcolor: String,
    pub enum_fields_bgcolor: String,
    pub enum_method_bgcolor: String,
    pub enum_implem_bgcolor: String,
    pub trait_header_bgcolor: String,
    pub trait_method_bgcolor: String,
    pub trait_implem_bgcolor: String,
    pub font_name: String,
}
static INSTANCE: OnceCell<Config> = OnceCell::new();

impl Config {
    pub fn set_global(config: Self) {
        INSTANCE.set(config).unwrap();
    }

    pub(crate) fn global() -> &'static Self {
        INSTANCE.get().expect("config is not initialized")
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            include_methods: true,
            include_fields: true,
            include_implems: false,  // has dups with methods. 
            struct_header_bgcolor: "lightblue".to_string(),
            struct_fields_bgcolor: "white".to_string(),
            struct_method_bgcolor: "white".to_string(),
            struct_implem_bgcolor: "white".to_string(),
            enum_header_bgcolor: "yellow".to_string(),
            enum_fields_bgcolor: "white".to_string(),
            enum_method_bgcolor: "white".to_string(),
            enum_implem_bgcolor: "white".to_string(),
            trait_header_bgcolor: "lightgreen".to_string(),
            trait_method_bgcolor: "white".to_string(),
            trait_implem_bgcolor: "white".to_string(),
            font_name: "Arial".to_string(),
        }
    }
}

/// The function `file2crate` returns a syntex module.
fn file2crate(path: &Path) -> io::Result<ast::Crate> {
    let sourcemap = Rc::new(SourceMap::new(FilePathMapping::empty()));
    let tty_handler =
        Handler::with_tty_emitter(ColorConfig::Auto, true, None, Some(sourcemap.clone()));
    let parse_session: ParseSess = ParseSess::with_span_handler(tty_handler, sourcemap.clone());
    let parse = rustc_parse::parse_crate_from_file(path.as_ref(), &parse_session);

    let ast: ast::Crate = parse.unwrap();
    Ok(ast)

    // An alternate way of doing this.
    // let session = rustc_session::build_session(
    //     rustc_session::config::Options::default(), 
    //     None,  // local_crate_source_file
    //     rustc_errors::registry::Registry::new(&[]),
    //     rustc_session::DiagnosticOutput::Default,
    //     FxHashMap::default(), // driver_lint_caps
    //     None,  // file_loader
    //     None,  // target_override
    // );
    // let parse = rustc_parse::parse_crate_from_file(path.as_ref(), &session.parse_sess);
    // let ast: ast::Crate = parse.unwrap();
    // Ok(ast)
}

/// The function `items2chars` returns a graph formated for *Graphiz/Dot*.
fn items2chars<'a>(modules: Vec<Module>) -> io::Result<Vec<u8>> {
    let mut f: Vec<u8> = Vec::new();
    let itt: Vec<(ptr::P<ast::Item>, Rc<ModulePath>)> =
        modules.into_iter()
               .flat_map(|s: Module| s.into_iter())
               .collect::<Vec<(ptr::P<ast::Item>, Rc<ModulePath>)>>();
    let it: ListItem = ListItem::from(itt.as_slice().into_iter().peekable());

    dot::render(&it, &mut f).and_then(|()| Ok(f))
}

/// The function `rs2dot` returns graphed file module.
///
/// # Examples
/// ```
/// extern crate mml;
///
/// fn main() {
///     let _ = mml::rs2dot("src/lib.rs");
/// }
/// ```
pub fn rs2dot<'a, P: AsRef<Path>>(path: P) -> io::Result<Vec<u8>> {
    rustc_span::create_session_if_not_set_then( rustc_span::edition::LATEST_STABLE_EDITION, |_sg| {
        file2crate(path.as_ref()).and_then(|parse: ast::Crate| items2chars(vec![Module::from((parse.items, path.as_ref().to_path_buf()))]))
    })
}

/// The function `src2dot` returns graphed repository of modules.
///
/// # Examples
/// ```
/// extern crate mml;
///
/// fn main() {
///     let _ = mml::src2dot("src");
/// }
/// ```
pub fn src2dot<'a, P: AsRef<Path>>(path: P) -> io::Result<Vec<u8>> {
    rustc_span::create_session_if_not_set_then( rustc_span::edition::LATEST_STABLE_EDITION, |_sg| {
        items2chars(WalkDir::new(path).into_iter()
                                    .filter_map(|entry: Result<walkdir::DirEntry, _>| entry.ok())
                                    .filter(|entry| entry.file_type().is_file())
                                    .filter_map(|entry: walkdir::DirEntry| {
                                        let path: &Path = entry.path();

                                        if path.extension() == Some(OsStr::new("rs")) {
                                            file2crate(path).ok().and_then(|parse| Some(Module::from((parse.items, path.to_path_buf()))))
                                        } else {
                                            None
                                        }
                                    })
                                    .collect::<Vec<Module>>())
    })
}

/// The function `content2svg` returns structured vector graphics content of modules.
fn content2svg(buf: Vec<u8>) -> io::Result<Vec<u8>> {
        Command::new("dot").arg("-Tsvg")
                           .stdin(Stdio::piped()).stdout(Stdio::piped())
                           .spawn()
                           .and_then(|child| {
                                let mut ret = vec![];

                                child.stdin.unwrap().write_all(buf.as_slice()).unwrap();
                                child.stdout.unwrap().read_to_end(&mut ret).unwrap();
                                Ok(ret)
                           })
}

/// The function `rs2svg` returns structured vector graphics file modules.
///
/// # Examples
/// ```
/// extern crate mml;
///
/// fn main() {
///     let _ = mml::rs2svg("src/lib.rs");
/// }
/// ```
pub fn rs2svg<P: AsRef<Path>>(path: P) -> io::Result<Vec<u8>> {
    rustc_span::create_session_if_not_set_then( rustc_span::edition::LATEST_STABLE_EDITION, |_sg| {
        rs2dot(path).and_then(|buf| content2svg(buf))
    })
}

/// The function `src2svg` returns structured vector graphics repository of modules.
///
/// # Examples
/// ```
/// extern crate mml;
///
/// fn main() {
///     let _ = mml::src2svg("src");
/// }
/// ```
pub fn src2svg<P: AsRef<Path>>(path: P) -> io::Result<Vec<u8>> {
    rustc_span::create_session_if_not_set_then( rustc_span::edition::LATEST_STABLE_EDITION, |_sg| {
        src2dot(path).and_then(|buf| content2svg(buf))
    })
}

/// The function `src2both` creates two files formated like a graph/dot and a structured vector graphics.
///
/// # Examples
/// ```
/// extern crate mml;
///
/// fn main() {
///    let dest: String = concat!("target/doc/", env!("CARGO_PKG_NAME")).to_string();
///
///    let _ = mml::src2both("src", dest.replace("-", "_").as_str());
/// }
/// ```
pub fn src2both<P: AsRef<Path>>(src: P, dest: P) -> io::Result<()> {

    fn worker<P: AsRef<Path>>(src: P, dest: P) -> io::Result<()> {
        let _ = fs::create_dir_all(dest.as_ref())?;
        let mut file_dot = File::create(dest.as_ref().join(DEFAULT_NAME_DOT))?;
        let mut file_svg = File::create(dest.as_ref().join(DEFAULT_NAME_PNG))?;

        let content_dot: Vec<u8> = src2dot(src)?;
        let _ = file_dot.write_all(content_dot.as_slice())?;

        let content_svg: Vec<u8> = content2svg(content_dot)?;
        let _ = file_svg.write_all(content_svg.as_slice())?;
        Ok(())
    }

    rustc_span::create_session_if_not_set_then( rustc_span::edition::LATEST_STABLE_EDITION, |_sg| {
        worker(src, dest)
    })?;

    Ok(())
}
