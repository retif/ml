use super::DEFAULT_FUNC;

use std::ops::Deref;
use std::fmt;
use std::rc::Rc;

use rustc_ast_pretty::pprust::{ty_to_string, state, state::PrintState};
use rustc_span::symbol;
use rustc_ast::ast;

use crate::module::path::ModulePath;

use crate::dot::escape_html;

/// The structure `Method` is a collection of methods from a abstract element.

#[derive(Default, Debug, Clone)]
pub struct Method {
    /// visibility, method's name, arguments, result.
    func: Vec<(ast::VisibilityKind, symbol::Symbol, Vec<String>, Option<String>)>,
    path: Rc<ModulePath>,
}

impl PartialEq for Method {
    fn eq(&self, b: &Self) -> bool {

        let a = self;

        // ignore func for now, as it uses Visibility.

        // use ast::VisibilityKind::*;
        // let bvis = match (a.vis, b.vis) {
        //     (Public, Public) => true,
        //     (Crate(_), Crate(_)) => true,
        //     (Restricted{..}, Restricted{..}) => true,
        //     (Inherited, Inherited) => true,
        //     _ => false,
        // };

        a.path == b.path
    }
}

impl Eq for Method {}


impl Method {
    pub fn is_association(&self, ty_name: &String) -> bool {
        self.func.iter()
                 .any(|&(_, _, _, ref result): &(ast::VisibilityKind, symbol::Symbol, Vec<String>, Option<String>)|
                     if let &Some(ref ret) = result {
                         ret.split(|at| "<[(;, )]>".contains(at))
                            .any(|ty| ty.eq(ty_name))
                     } else {
                         false
                     }
                 )
    }

    pub fn is_dependency(&self, name: &String) -> bool {
        self.func.iter()
                 .any(|&(_, _, ref arg, _): &(ast::VisibilityKind, symbol::Symbol, Vec<String>, Option<String>)|
                     arg.iter().any(|ty| ty.ends_with(name)))
    }
}

impl From<(Vec<(ast::VisibilityKind, symbol::Symbol, Vec<String>, Option<String>)>, Rc<ModulePath>)> for Method {
    fn from((func, path): (Vec<(ast::VisibilityKind, symbol::Symbol, Vec<String>, Option<String>)>, Rc<ModulePath>)) -> Method {
        Method {
            func: func,
            path: path,
        }
    }
}

impl From<(Vec<ast::Item>, Rc<ModulePath>)> for Method {
    fn from((impl_item, path): (Vec<ast::Item>, Rc<ModulePath>)) -> Method {
        Method::from((impl_item.iter()
                .flat_map(|&ast::Item {id: _, ident: _, ref vis, attrs: _, ref kind, .. }| {
                    if let ast::ItemKind::Impl(box ast::ImplKind {unsafety: _, polarity: _, defaultness: _, constness: _, generics: _, of_trait: _, self_ty: _, items}) = kind {
                        items.iter().filter_map(|item| {
                            if let ast::AssocItemKind::Fn(box ast::FnKind (_, ast::FnSig{ref decl, ..}, ..)) = (*item).kind {
                                let name = (*item).ident.name;
                                if let &ast::FnDecl {ref inputs, output: ast::FnRetTy::Ty(ref ty), ..} = decl.deref() {
                                    Some((vis.kind.clone(), name, inputs.iter().map(|ref arg| {
                                        state::State::new().param_to_string(&arg)
                                    }).collect::<Vec<String>>(), Some(ty_to_string(&ty))))
                                } else if let &ast::FnDecl {ref inputs, output: ast::FnRetTy::Default(_), ..} = decl.deref() {
                                    Some((vis.kind.clone(), name, inputs.iter().map(|ref arg| {
                                        state::State::new().param_to_string(&arg)
                                    }).collect::<Vec<String>>(), None))
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<(ast::VisibilityKind, symbol::Symbol, Vec<String>, Option<String>)>>()
                    } else {
                        vec![]
                    }
                })
                .collect::<Vec<(ast::VisibilityKind, symbol::Symbol, Vec<String>, Option<String>)>>(),
        path))
    }
}


impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{item}",
            item = escape_html(self.func.iter()
                                        .map(|&(ref vis, ref name, ref inputs, ref ty)|
                                               match (vis, ty) {
                                                   (&ast::VisibilityKind::Public, &Some(ref ty)) => {
                                                       format!("+{}{}({}) -> {}", DEFAULT_FUNC, name, inputs.iter().map(|arg| arg.to_string()).collect::<Vec<String>>().join(", "), ty)
                                                   },
                                                   (&ast::VisibilityKind::Public, &None) => {
                                                       format!("+{}{}({})", DEFAULT_FUNC, name, inputs.iter().map(|arg| arg.to_string()).collect::<Vec<String>>().join(", "))
                                                   },
                                                   (_, &Some(ref ty)) => {
                                                       format!("-{}{}({}) -> {}", DEFAULT_FUNC, name, inputs.iter().map(|arg| arg.to_string()).collect::<Vec<String>>().join(", "), ty)
                                                   },
                                                   (_, &None) => {
                                                       format!("-{}{}({})", DEFAULT_FUNC, name, inputs.iter().map(|arg| arg.to_string()).collect::<Vec<String>>().join(", "))
                                                   },
                                               }
                                           )
                                           .collect::<Vec<String>>()
                                           .join("\n")
                                           .as_str())
        )
    }
}
