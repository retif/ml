use std::fmt;
use std::ops::Deref;
use std::rc::Rc;

use rustc_ast_pretty::pprust::ty_to_string;
use rustc_ast::{ast, ptr};
use rustc_span::symbol;

use crate::module::path::ModulePath;

use crate::dot::escape_html;

#[derive(Debug, Clone)]
pub struct Trait<'a> {
    pub path: Rc<ModulePath>,
    /// Visibility
    pub vis: &'a ast::VisibilityKind,
    pub name: symbol::Symbol,
    pub params: Vec<symbol::Symbol>,
    pub items: Vec<(symbol::Symbol, Vec<String>, String)>,
}

impl <'a>PartialEq for Trait<'a> {
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
        a.items == b.items &&
        bvis
    }
}

impl <'a>Eq for Trait<'a> {}

impl <'a>From<((&'a ast::Item, &'a Vec<ast::GenericParam>, &'a Vec<ptr::P<ast::Item<ast::AssocItemKind>>>), Rc<ModulePath>)> for Trait<'a> {
    fn from(((item, params, trait_item), path): ((&'a ast::Item, &'a Vec<ast::GenericParam>, &'a Vec<ptr::P<ast::Item<ast::AssocItemKind>>>), Rc<ModulePath>)) -> Trait<'a> {
        Trait {
            path: path,
            vis: &item.vis.kind,
            name: item.ident.name,
            params: params.iter()
                             .map(|&ast::GenericParam {attrs: _, ident: symbol::Ident {name, ..}, ..}| name)
                             .collect::<Vec<symbol::Symbol>>(),
            items: trait_item.iter()
                            .filter_map(|p| {
                                let name = p.ident.name;
                                let kind = &p.kind;
                                if let &ast::AssocItemKind::Fn(box ast::FnKind (_defaultness, ast::FnSig {ref decl, ..}, ..)) = &kind {
                                    if let &ast::FnDecl {ref inputs, output: ast::FnRetTy::Ty(ref ty), ..} = decl.deref() {
                                        Some((name, inputs.iter().map(|input| ty_to_string(&input.ty)).collect::<Vec<String>>(), ty_to_string(&ty)))
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            })
                            .collect::<Vec<(symbol::Symbol, Vec<String>, String)>>()
        }
    }
}

impl <'a>fmt::Display for Trait<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "&lt;&lt;&lt;Trait&gt;&gt;&gt;\n{name}|{items}",
           name = self.name,
           items = escape_html(self.items.iter()
                                   .map(|&(ref name, ref ty, ref ret): &(symbol::Symbol, Vec<String>, String)|
                                        format!("{name}({ty}) -> {ret}",
                                            name = name,
                                            ty = ty.join(", "),
                                            ret = ret
                                        ))
                                   .collect::<Vec<String>>()
                                   .join("\n")
                                   .as_str())
        )
    }
}
