use super::DEFAULT_FUNC;

use std::fmt;

use rustc_ast_pretty::pprust::ty_to_string;
use rustc_span::symbol;
use rustc_ast::ast;

use crate::dot::escape_html;

/// The structure `Implem` is a collection of methods and tyes for an abstract element.

#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub struct Implem {
    ty: Vec<(symbol::Symbol, Vec<String>)>,
    /// method's name, arguments, result.
    method: Vec<(symbol::Symbol, Vec<String>, Option<String>)>,
}

impl Implem {
    pub fn is_realization(&self, ty_name: &String) -> bool {
        if let Some(&(ref name, _)) = self.ty.first() {
            name.to_string().eq(ty_name)
        } else {
            false
        }
    }

    pub fn is_association(&self, ty_name: &String) -> bool {
        self.method.iter()
                   .any(|&(_, _, ref result): &(symbol::Symbol, Vec<String>, Option<String>)|
                       if let &Some(ref ret) = result {
                           ret.split(|at| "<[(;, )]>".contains(at))
                              .any(|ty| ty.eq(ty_name))
                       } else {
                           false
                       }
                   )
    }

    pub fn is_dependency(&self, _: &String) -> bool {
        false
        /*self.method.iter()
                   .any(|&( _, ref arg, _): &(symbol::Symbol, Vec<String>, Option<String>)|
                       arg.iter().any(|ty| ty.ends_with(name)))*/
    }
}

impl From<(Vec<(symbol::Symbol, Vec<String>)>, Vec<(symbol::Symbol, Vec<String>, Option<String>)>)> for Implem {
    fn from((ty, method): (Vec<(symbol::Symbol, Vec<String>)>, Vec<(symbol::Symbol, Vec<String>, Option<String>)>)) -> Implem {
        Implem {
            ty: ty,
            method: method,
        }
    }
}

impl <'a> From<(&'a Vec<ast::PathSegment>, &'a Vec<ast::Item>)> for Implem {
    fn from((segments, impl_item): (&'a Vec<ast::PathSegment>, &'a Vec<ast::Item>)) -> Implem {
        Implem::from((segments.iter()
                              .map(|&ast::PathSegment { ident: symbol::Ident {name, ..}, .. }| {
                                //   if let &Some(ref path) = parameters {
                                    //   if let &ast::AngleBracketedArg(
                                    //       ast::AngleBracketedParameterData { lifetimes: _, ref types, .. }
                                    //   ) = path.deref() {
                                    //       (name.as_str(), types.iter().map(|ty| ty_to_string(&ty)).collect::<Vec<String>>())
                                    //   } else {
                                    //       (name.as_str(), Vec::new())
                                    //   }
                                    // (name, Vec::new())
                                //   } else {
                                      (name, Vec::new())
                                //   }
                              })
                              .collect::<Vec<(symbol::Symbol, Vec<String>)>>(),
                      impl_item.iter()
                               .filter_map(|&ast::Item { id: _, ident: symbol::Ident {name, ..}, vis: _, attrs: _, ref kind, ..}|
                                         if let &ast::ItemKind::Fn(box ast::FnKind (_defaultness, ast::FnSig {ref decl, ..}, ..)) = kind {
                                             if let ast::FnRetTy::Ty(ref ty) = decl.output {
                                                 Some((name, decl.inputs.iter().map(|arg| ty_to_string(&arg.ty)).collect::<Vec<String>>(), Some(ty_to_string(&ty))))
                                             } else {
                                                 Some((name, decl.inputs.iter().map(|arg| ty_to_string(&arg.ty)).collect::<Vec<String>>(), None))
                                             }
                                         } else {
                                             None
                                         }
                               ).collect::<Vec<(symbol::Symbol, Vec<String>, Option<String>)>>()))
    }
}

impl fmt::Display for Implem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{item}",
           item = escape_html(self.method.iter()
                                         .map(|&(ref name, ref args, ref result): &(symbol::Symbol, Vec<String>, Option<String>)| {
                                             if let &Some(ref ret) = result {
                                                 format!("{}{}({}) -> {}", DEFAULT_FUNC, name, args.join(", "), ret)
                                             } else {
                                                 format!("{}{}({})", DEFAULT_FUNC, name, args.join(", "))
                                             }
                                         })
                                         .collect::<Vec<String>>()
                                         .join("\n")
                                         .as_str()))
        /*if let Some(&(ref name, ref template)) = self.ty.last() {
            if template.is_empty() {
                write!(f, "{name}", name = name.to_string())
            } else {
                write!(f, "{name}&lt;{item}&gt;",
                   name = name.to_string(),
                   item = dot::escape_html(template.join(", ")
                                                   .as_str()))
           }
        } else {
            Ok(())
        }*/
    }
}
