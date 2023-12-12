use mml;
use rust2uml::Config;

use argi::{cli, data};

fn main() {
     cli!(
        help: "Parses rust source code and generates UML diagram",
        run: (run),
        --include_fields [bool]: { help: "include fields/variants in diagram" },
        --include_implems [bool]: { help: "include trait implementation methods in diagram" },
        --include_methods [bool]: { help: "include methods in diagram" },
        --struct_header_bgcolor [str]: { help: "header background color for structs" },
        --struct_fields_bgcolor [str]: { help: "fields background color for structs" },
        --struct_method_bgcolor [str]: { help: "methods background color for structs" },
        --struct_implem_bgcolor [str]: { help: "implems background color for structs" },
        --enum_header_bgcolor [str]: { help: "header background color for enums" },
        --enum_fields_bgcolor [str]: { help: "fields background color for enums" },
        --enum_method_bgcolor [str]: { help: "methods background color for enums" },
        --enum_implem_bgcolor [str]: { help: "implems background color for enums" },
        --trait_header_bgcolor [str]: { help: "header background color for traits" },
        --trait_method_bgcolor [str]: { help: "methods background color for traits" },
        --trait_implem_bgcolor [str]: { help: "implems background color for traits" },
        --src_url_mask [str]: { help: "url mask for src links, eg http://host/crate/{file}, or 'none'" },
        --font [str]: { help: "Font name" },
    )
    .launch();   
}

fn run(ctx: &argi::Command, _: Option<String>) {
    let dest: String = concat!("target/doc/", env!("CARGO_PKG_NAME")).to_string();  

    let config = command_to_config(ctx);
    rust2uml::Config::set_global(config);

    let _ = rust2uml::src2both("src", dest.replace("-", "_").as_str());
}

fn command_to_config(ctx: &argi::Command) -> Config {

    let mut config = rust2uml::Config::default();

    match data!(bool, ctx => --include_fields) {
        Some(v) => config.include_fields = v,
        None => {},
    }

    match data!(bool, ctx => --include_implems) {
        Some(v) => config.include_implems = v,
        None => {},
    }

    match data!(bool, ctx => --include_methods) {
        Some(v) => config.include_methods = v,
        None => {},
    }

    match data!(ctx => --struct_header_bgcolor) {
        Some(v) => config.struct_header_bgcolor = v,
        None => {},
    }

    match data!(ctx => --struct_fields_bgcolor) {
        Some(v) => config.struct_fields_bgcolor = v,
        None => {},
    }

    match data!(ctx => --struct_method_bgcolor) {
        Some(v) => config.struct_method_bgcolor = v,
        None => {},
    }

    match data!(ctx => --struct_implem_bgcolor) {
        Some(v) => config.struct_implem_bgcolor = v,
        None => {},
    }

    match data!(ctx => --trait_header_bgcolor) {
        Some(v) => config.trait_header_bgcolor = v,
        None => {},
    }

    match data!(ctx => --trait_method_bgcolor) {
        Some(v) => config.trait_method_bgcolor = v,
        None => {},
    }

    match data!(ctx => --trait_implem_bgcolor) {
        Some(v) => config.trait_implem_bgcolor = v,
        None => {},
    }

    match data!(ctx => --enum_header_bgcolor) {
        Some(v) => config.enum_header_bgcolor = v,
        None => {},
    }

    match data!(ctx => --enum_fields_bgcolor) {
        Some(v) => config.enum_fields_bgcolor = v,
        None => {},
    }

    match data!(ctx => --enum_method_bgcolor) {
        Some(v) => config.enum_method_bgcolor = v,
        None => {},
    }

    match data!(ctx => --enum_implem_bgcolor) {
        Some(v) => config.enum_implem_bgcolor = v,
        None => {},
    }

    match data!(ctx => --font) {
        Some(v) => config.font_name = v,
        None => {},
    }

    match data!(ctx => --src_url_mask) {
        Some(v) if v == "none" => config.src_url_mask = "".to_string(),
        Some(v) => config.src_url_mask = v,
        None => {},
    }

    config
}
