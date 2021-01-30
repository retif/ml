#![crate_name = "mml"]
#![crate_type = "lib"]

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

extern crate dot;
extern crate itertools;
extern crate syn;
extern crate walkdir;
extern crate quote;

use std::ffi::OsStr;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::Path;
use std::process::{Command, Stdio};
use std::rc::Rc;
use std::result::Result::Err;

use syn::parse_file;
use walkdir::WalkDir;

use core::ListItem;
use module::Module;
use module::path::ModulePath;

pub mod prelude;
pub mod module;
pub mod core;

/// The default name of *graph/dot* file.
pub const DEFAULT_NAME_DOT: &str = "ml.dot";
/// The default name of *image/svg* file.
pub const DEFAULT_NAME_PNG: &str = "ml.svg";

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    SynError(syn::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        let mut debug_struct = f.debug_struct("MlError");

        match self {
            Error::IoError(err) => { debug_struct.field("err", &err.to_string()); }
            Error::SynError(err) => { debug_struct.field("err", &err.to_string()); }
        }

        debug_struct.finish()
    }
}

impl std::error::Error for Error {}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IoError(err)
    }
}

impl From<syn::Error> for Error {
    fn from(err: syn::Error) -> Self {
        Error::SynError(err)
    }
}

/// The function `file2crate` returns a syntex module.
fn file2crate<P: AsRef<Path>>(path: P) -> Result<syn::File, Error> {
    let mut content = String::new();
    let _ = File::open(path)?.read_to_string(&mut content)?;
    match parse_file(&content) {
        Ok(file) => Ok(file),
        Err(err) => Err(err.into()),
    }
}

/// The function `items2chars` returns a graph formated for *Graphiz/Dot*.
fn items2chars(modules: Vec<Module>) -> Result<Vec<u8>, Error> {
    let mut f: Vec<u8> = Vec::new();
    let iter: Vec<(syn::Item, Rc<ModulePath>)> =
        modules.into_iter()
            .flat_map(|s: Module| s.into_iter())
            .collect::<Vec<(syn::Item, Rc<ModulePath>)>>();
    let iter: ListItem = ListItem::from(iter.iter().peekable());

    dot::render(&iter, &mut f).map(|()| f).map_err(Into::into)
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
pub fn rs2dot<P: AsRef<Path>>(path: P) -> Result<Vec<u8>, Error> {
    file2crate(path.as_ref()).and_then(|parse: syn::File| items2chars(vec![Module::from((parse.items, path.as_ref().to_path_buf()))]))
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
pub fn src2dot<P: AsRef<Path>>(path: P) -> Result<Vec<u8>, Error> {
    items2chars(WalkDir::new(path).into_iter()
        .filter_map(|entry: Result<walkdir::DirEntry, _>| entry.ok())
        .filter(|entry| entry.file_type().is_file())
        .filter_map(|entry: walkdir::DirEntry| {
            let path: &Path = entry.path();

            if path.extension().eq(&Some(OsStr::new("rs"))) {
                file2crate(path).ok().map(|file| Module::from((file.items, path.to_path_buf())))
            } else {
                None
            }
        })
        .collect::<Vec<_>>()).map_err(Into::into)
}

/// The function `content2svg` returns structured vector graphics content of modules.
pub fn content2svg(buf: Vec<u8>) -> Result<Vec<u8>, Error> {
        Command::new("dot").arg("-Tsvg")
                           .stdin(Stdio::piped()).stdout(Stdio::piped())
                           .spawn()
                           .map(|child| {
                                let mut ret = vec![];

            child.stdin.unwrap().write_all(buf.as_slice()).unwrap();
            child.stdout.unwrap().read_to_end(&mut ret).unwrap();
            ret
        }).map_err(Into::into)
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
pub fn rs2svg<P: AsRef<Path>>(path: P) -> Result<Vec<u8>, Error> {
    rs2dot(path).and_then(|buf| content2svg(buf).map_err(Into::into))
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
pub fn src2svg<P: AsRef<Path>>(path: P) -> Result<Vec<u8>, Error> {
    src2dot(path).and_then(|buf| content2svg(buf).map_err(Into::into))
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
pub fn src2both<P: AsRef<Path>>(src: P, dest: P) -> Result<(), Error> {
    let _ = fs::create_dir_all(dest.as_ref())?;
    let mut file_dot = File::create(dest.as_ref().join(DEFAULT_NAME_DOT))?;
    let mut file_svg = File::create(dest.as_ref().join(DEFAULT_NAME_PNG))?;

    let content_dot: Vec<u8> = src2dot(src)?;
    let _ = file_dot.write_all(content_dot.as_slice())?;

    let content_svg: Vec<u8> = content2svg(content_dot)?;
    let _ = file_svg.write_all(content_svg.as_slice())?;

    Ok(())
}
