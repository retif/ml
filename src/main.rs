#![feature(rustc_private)]
#![feature(box_patterns)]
extern crate clap;

use clap::Parser;
use rust2uml::Config;

#[derive(Parser, Debug)]
#[clap(author, version, about = "Parses rust source code and generates UML diagram", long_about = None)]
struct Cli {
    /// Exclude fields/variants in diagram
    #[arg(long, default_value = "false")]
    exclude_fields: bool,

    /// Exclude methods in diagram
    #[clap(long, default_value = "false")]
    exclude_methods: bool,

    /// Include trait method impls in diagram
    #[clap(long, default_value = "false")]
    include_impls: bool,

    /// Background color for struct header
    #[clap(long, value_name = "COLOR")]
    struct_header_bgcolor: Option<String>,

    /// Background color for struct fields
    #[clap(long, value_name = "COLOR")]
    struct_fields_bgcolor: Option<String>,

    /// Background color for struct methods
    #[clap(long, value_name = "COLOR")]
    struct_method_bgcolor: Option<String>,

    /// Background color for struct impls
    #[clap(long, value_name = "COLOR")]
    struct_impl_bgcolor: Option<String>,

    /// Background color for enum header
    #[clap(long, value_name = "COLOR")]
    enum_header_bgcolor: Option<String>,

    /// Background color for enum fields
    #[clap(long, value_name = "COLOR")]
    enum_fields_bgcolor: Option<String>,

    /// Background color for enum methods
    #[clap(long, value_name = "COLOR")]
    enum_method_bgcolor: Option<String>,

    /// Background color for enum impls
    #[clap(long, value_name = "COLOR")]
    enum_impl_bgcolor: Option<String>,

    /// Background color for trait header
    #[clap(long, value_name = "COLOR")]
    trait_header_bgcolor: Option<String>,

    /// Background color for trait methods
    #[clap(long, value_name = "COLOR")]
    trait_method_bgcolor: Option<String>,

    /// Background color for trait impls
    #[clap(long, value_name = "COLOR")]
    trait_impl_bgcolor: Option<String>,

    /// Mask for source code urls, eg "http://host/crate/{file}", or "none"
    #[clap(long, value_name = "URL")]
    src_url_mask: Option<String>,

    /// Font name
    #[clap(long)]
    font: Option<String>,
}

fn main() {
    let args = Cli::parse();
    let dest = format!("target/doc/{}", env!("CARGO_PKG_NAME"));
    let config = command_to_config(&args);
    Config::set_global(config);

    let dest_mod = dest.replace("-", "_");
    let _ = rust2uml::src2both("src", dest_mod.as_str());
}

fn command_to_config(args: &Cli) -> Config {
    let mut config = Config::default();
    
    config.include_fields = !args.exclude_fields;
    config.include_methods = !args.exclude_methods;
    config.include_impls = args.include_impls;
    
    if let Some(ref v) = args.struct_header_bgcolor {
        config.struct_header_bgcolor = v.clone();
    }
    if let Some(ref v) = args.struct_fields_bgcolor {
        config.struct_fields_bgcolor = v.clone();
    }
    if let Some(ref v) = args.struct_method_bgcolor {
        config.struct_method_bgcolor = v.clone();
    }
    if let Some(ref v) = args.struct_impl_bgcolor {
        config.struct_impl_bgcolor = v.clone();
    }
    if let Some(ref v) = args.enum_header_bgcolor {
        config.enum_header_bgcolor = v.clone();
    }
    if let Some(ref v) = args.enum_fields_bgcolor {
        config.enum_fields_bgcolor = v.clone();
    }
    if let Some(ref v) = args.enum_method_bgcolor {
        config.enum_method_bgcolor = v.clone();
    }
    if let Some(ref v) = args.enum_impl_bgcolor {
        config.enum_impl_bgcolor = v.clone();
    }
    if let Some(ref v) = args.trait_header_bgcolor {
        config.trait_header_bgcolor = v.clone();
    }
    if let Some(ref v) = args.trait_method_bgcolor {
        config.trait_method_bgcolor = v.clone();
    }
    if let Some(ref v) = args.trait_impl_bgcolor {
        config.trait_impl_bgcolor = v.clone();
    }
    if let Some(ref v) = args.src_url_mask {
        config.src_url_mask = if v == "none" { "".to_string() } else { v.clone() };
    }
    if let Some(ref v) = args.font {
        config.font_name = v.clone();
    }
    config
}
