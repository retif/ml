use std::fmt;
use std::rc::Rc;

use rustc_ast_pretty::pprust::ty_to_string;
use rustc_ast::ast;
use rustc_span::symbol;

use crate::module::path::ModulePath;

use crate::dot::escape_html;

/// The structure `Struct` is a structure abstract element.

#[derive(Debug, Clone)]
pub struct Struct<'a> {
    pub path: Rc<ModulePath>,
    /// Visibility
    pub vis: &'a ast::VisibilityKind,
    pub name: symbol::Symbol,
    pub fields: Vec<(&'a ast::VisibilityKind, symbol::Symbol, String)>,
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
            vis: &item.vis.kind,
            name: item.ident.name,
            fields: struct_field.iter()
                                .filter_map(|&ast::FieldDef { span: _, ident, ref vis, id: _, ref ty, .. }|
                                           ident.and_then(|symbol::Ident {name, ..}| Some((&vis.kind, name, ty_to_string(&ty)))))
                                .collect::<Vec<(&ast::VisibilityKind, symbol::Symbol, String)>>()
        }
    }
}

impl <'a>fmt::Display for Struct<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.fields.is_empty() {
            write!(f, "&lt;&lt;&lt;Structure&gt;&gt;&gt;\n{name}", name = self.name)
        } else {
            write!(f, "&lt;&lt;&lt;Structure&gt;&gt;&gt;\n{name}|{fields}",
                name = self.name,
                fields = escape_html(self.fields.iter()
                                                .map(|&(ref vis, ref name, ref ty): &(&ast::VisibilityKind, symbol::Symbol, String)|
                                                    match vis {
                                                        ast::VisibilityKind::Public => format!("+ {name}: {ty}", name = name, ty = ty),
                                                        _ => format!("- {name}: {ty}", name = name, ty = ty)
                                                    }
                                                )
                                                .collect::<Vec<String>>()
                                                .join("\n")
                                                .as_str()),
            )
        }
    }
}
