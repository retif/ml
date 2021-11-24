use std::fmt;
use std::rc::Rc;

use rustc_ast_pretty::pprust::ty_to_string;
use rustc_ast::ast;
use rustc_span::symbol;

use crate::module::path::ModulePath;

use crate::dot::escape_html;
use crate::Config;

/// The structure `Struct` is a structure abstract element.

#[derive(Debug, Clone)]
pub struct Struct<'a> {
    pub path: Rc<ModulePath>,
    pub span: rustc_span::Span,

    /// Visibility
    pub vis: &'a ast::VisibilityKind,
    pub name: symbol::Symbol,
    pub fields: Vec<(&'a ast::VisibilityKind, Option<symbol::Symbol>, String)>,
}

impl <'a>PartialEq for Struct<'a> {
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

        // ignore fields for now, because it contains Visibility

        a.path == b.path &&
        a.name == b.name &&
        bvis
    }
}

impl <'a>Eq for Struct<'a> {}


impl <'a>From<((&'a ast::Item, &'a Vec<ast::FieldDef>), Rc<ModulePath>)> for Struct<'a> {
    fn from(((item, struct_field), path): ((&'a ast::Item, &'a Vec<ast::FieldDef>), Rc<ModulePath>)) -> Struct<'a> {
        Struct {
            path: path,
            span: item.span,
            vis: &item.vis.kind,
            name: item.ident.name,
            fields: struct_field.iter()
                                .filter_map(|&ast::FieldDef { span: _, ident, ref vis, id: _, ref ty, .. }|
                                            match ident {
                                                Some(i) => Some((&vis.kind, Some(i.name), ty_to_string(&ty))),
                                                None => Some((&vis.kind, None, ty_to_string(&ty))),
                                            })
                                .collect::<Vec<(&ast::VisibilityKind, Option<symbol::Symbol>, String)>>()
        }
    }
}

impl <'a>fmt::Display for Struct<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let include_fields = !self.fields.is_empty() && Config::global().include_fields;

        if !include_fields {
            write!(f,
                    "<tr><td bgcolor=\"{bgcolor}\"><b>{name}</b></td></tr>",
                    bgcolor = Config::global().struct_header_bgcolor,
                    name = self.name)
        } else {
            write!(f, "<tr><td bgcolor=\"{header_bgcolor}\"><b>{name}</b></td></tr><tr><td align=\"left\" bgcolor=\"{fields_bgcolor}\">{fields}<br align=\"left\"/></td></tr>",
                header_bgcolor = Config::global().struct_header_bgcolor,
                fields_bgcolor = Config::global().struct_fields_bgcolor,
                name = self.name,
                fields = self.fields.iter()
                                                .map(|&(ref vis, ref name, ref ty): &(&ast::VisibilityKind, Option<symbol::Symbol>, String)|{
                                                    let name_part = match name {
                                                        Some(n) => format!("{}: ", n),
                                                        None => "".to_string(),
                                                    };                                                    
                                                    escape_html(
                                                    match vis {
                                                        ast::VisibilityKind::Public => format!("+ {name}{ty}", name = name_part, ty = ty),
                                                        _ => format!("- {name}{ty}", name = name_part, ty = ty)
                                                    }.as_str())
                                                })
                                                .collect::<Vec<String>>()
                                                .join("<br align=\"left\"/>\n")
                                                .as_str(),
            )
        }
    }
}
