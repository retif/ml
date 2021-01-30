use std::fmt;

use ::dot::escape_html;
use quote::ToTokens;
use syn::{AngleBracketedGenericArguments, FnArg, GenericArgument, ImplItem, ImplItemMethod, PathArguments, PathSegment, ReturnType, Signature};
use syn::punctuated::Punctuated;
use syn::token::Colon2;

use super::DEFAULT_FUNC;

/// The structure `Implem` is a collection of methods and tyes for an abstract element.
#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub struct Implem {
    ty: Vec<(String, Vec<String>)>,
    /// method's name, arguments, result.
    method: Vec<(String, Vec<String>, Option<String>)>,
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
            .any(|&(_, _, ref result): &(String, Vec<String>, Option<String>)|
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
                   .any(|&( _, ref arg, _): &(String, Vec<String>, Option<String>)|
                       arg.iter().any(|ty| ty.ends_with(name)))*/
    }
}

impl From<(Vec<(String, Vec<String>)>, Vec<(String, Vec<String>, Option<String>)>)> for Implem {
    fn from((ty, method): (Vec<(String, Vec<String>)>, Vec<(String, Vec<String>, Option<String>)>)) -> Implem {
        Implem {
            ty,
            method,
        }
    }
}

impl<'a> From<(&'a Punctuated<PathSegment, Colon2>, &'a Vec<ImplItem>)> for Implem {
    fn from((segments, impl_item): (&'a Punctuated<PathSegment, Colon2>, &'a Vec<ImplItem>)) -> Implem {
        Implem::from((segments.iter()
                          .map(|PathSegment { ident, arguments, .. }| {
                              if let PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) = arguments {
                                  (ident.to_string(), args.iter()
                                      .filter_map(|ty|
                                          if let GenericArgument::Type(ty) = ty {
                                              Some(ty.to_token_stream().to_string())
                                          } else {
                                              None
                                          })
                                      .collect::<Vec<String>>())
                              } else {
                                  (ident.to_string(), Vec::new())
                              }
                          })
                          .collect::<Vec<(String, Vec<String>)>>(),
                      impl_item.iter()
                          .filter_map(|item| {
                              if let ImplItem::Method(ImplItemMethod { sig: Signature { ident, inputs, output, .. }, .. }) = item {
                                  let inputs = inputs.iter()
                                      .filter_map(|arg|
                                          if let FnArg::Typed(ty) = arg {
                                              Some(ty.ty.to_token_stream().to_string())
                                          } else {
                                              None
                                          })
                                      .collect();
                                  if let ReturnType::Type(_, ref ty) = output {
                                      Some((ident.to_string(), inputs, Some(ty.to_token_stream().to_string())))
                                  } else {
                                      Some((ident.to_string(), inputs, None))
                                  }
                              } else {
                                  None
                              }
                          }).collect::<Vec<(String, Vec<String>, Option<String>)>>()))
    }
}

impl fmt::Display for Implem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{item}",
               item = escape_html(self.method.iter()
                   .map(|&(ref name, ref args, ref result): &(String, Vec<String>, Option<String>)| {
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
