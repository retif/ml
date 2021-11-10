use std::fmt;
use std::rc::Rc;

use rustc_ast_pretty::pprust::ty_to_string;
use rustc_ast::ast;
use rustc_span::symbol;

use crate::module::path::ModulePath;

use crate::dot::escape_html;
use crate::Config;

/// The structure `Enum` is a enumerate abstract element.
#[derive(Debug, Clone)]
pub struct Enum<'a> {
    pub path: Rc<ModulePath>,
    /// Visibility
    pub vis: &'a ast::VisibilityKind,
    pub name: symbol::Symbol,
    pub params: Vec<symbol::Symbol>,
    pub variants: Vec<(symbol::Symbol, Vec<String>)>,
}

impl <'a>PartialEq for Enum<'a> {
    fn eq(&self, b: &Self) -> bool {

        let a = self;

        use ast::VisibilityKind::*;

        let bvis = match (a.vis, b.vis) {
            (Public, Public) => true,
            (Crate(_), Crate(_)) => true,
            (Restricted{..}, Restricted{..}) => true,
            (Inherited, Inherited) => true,
            _ => false,
        };

        a.path == b.path &&
        a.name == b.name &&
        a.params == b.params &&
        a.variants == b.variants &&
        bvis
    }
}

impl <'a>Eq for Enum<'a> {}


impl <'a>From<((&'a ast::Item, &'a Vec<ast::GenericParam>, &'a Vec<ast::Variant>), Rc<ModulePath>)> for Enum<'a> {
    fn from(((item, params, variants), path): ((&'a ast::Item, &'a Vec<ast::GenericParam>, &'a Vec<ast::Variant>), Rc<ModulePath>)) -> Enum<'a> {
        Enum {
            path: path,
            vis: &item.vis.kind,
            name: item.ident.name,
            params: params.iter()
                             .map(|&ast::GenericParam {attrs: _, ident: symbol::Ident {name, ..}, ..}| name)
                             .collect::<Vec<symbol::Symbol>>(),
            variants: variants.iter()
                              .map(|&ast::Variant {ident: symbol::Ident {name, ..}, attrs: _, ref data, ..}| {
                                   if let &ast::VariantData::Tuple(ref struct_field, _) = data {
                                       (name,
                                        struct_field.iter()
                                                    .filter_map(|&ast::FieldDef { span: _, ident: _, vis: _, id: _, ref ty, .. }| Some(ty_to_string(&ty)))
                                                    .collect::<Vec<String>>())
                                   } else {
                                       (name, Vec::new())
                                   }
                              })
                              .collect::<Vec<(symbol::Symbol, Vec<String>)>>(),
        }
    }
}

impl <'a>fmt::Display for Enum<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let include_variants = !self.variants.is_empty() && Config::global().include_fields;

        if !include_variants {
            write!(f, 
                    "<tr><td bgcolor=\"{bgcolor}\"><b>{name}</b></td></tr>",
                    bgcolor = Config::global().enum_header_bgcolor,
                    name = self.name)
        } else {
            write!(f, "<tr><td bgcolor=\"{header_bgcolor}\"><b>{name}</b></td></tr><tr><td align=\"left\" bgcolor=\"{fields_bgcolor}\">{variants}<br align=\"left\"/></td></tr>",
                header_bgcolor = Config::global().enum_header_bgcolor,
                fields_bgcolor = Config::global().enum_fields_bgcolor,
                name = self.name,
                variants = self.variants.iter()
                                           .map(|&(ref name, ref struct_field): &(symbol::Symbol, Vec<String>)|
                                                if struct_field.is_empty() {
                                                    escape_html(&format!("{}", name))
                                                } else {
                                                    escape_html(&format!("{}({})", name, struct_field.join(", ")))
                                                }
                                           )
                                           .collect::<Vec<String>>()
                                           .join("<br align=\"left\"/>\n")
                                           .as_str(),
            )
        }
    }
}