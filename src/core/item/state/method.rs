use std::fmt;
use std::rc::Rc;

use dot::escape_html;
use quote::ToTokens;
use syn::{
    punctuated::Punctuated, token::Comma, FnArg, ImplItem, ImplItemMethod, ReturnType, Signature,
    Type, Visibility,
};

use module::path::ModulePath;

use super::DEFAULT_FUNC;

/// The structure `Method` is a collection of methods from a abstract element.
#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub struct Method<'a> {
    /// visibility, method's name, arguments, result.
    func: Vec<(
        &'a Visibility,
        String,
        &'a Punctuated<FnArg, Comma>,
        Option<&'a Box<Type>>,
    )>,
    path: Rc<ModulePath>,
}

impl<'a> Method<'a> {
    pub fn is_association(&self, ty_name: &str) -> bool {
        self.func.iter().any(|(_, _, _, result)| {
            if let Some(ret) = result {
                // TODO: replace by checking if ret (which is of type Type)
                // TODO: contains ty_name anywhere
                ret.to_token_stream()
                    .to_string()
                    .split(|at| "<[(;, )]>".contains(at))
                    .any(|ty| ty.eq(ty_name))
            } else {
                false
            }
        })
    }

    pub fn is_dependency(&self, name: &str) -> bool {
        self.func.iter().any(|(_, _, arg, _)| {
            arg.iter()
                .filter_map(|ty| {
                    if let FnArg::Typed(ty) = ty {
                        Some(ty)
                    } else {
                        None
                    }
                })
                .any(|ty| ty.ty.to_token_stream().to_string().ends_with(name))
        })
    }
}

impl<'a>
    From<(
        Vec<(
            &'a Visibility,
            String,
            &'a Punctuated<FnArg, Comma>,
            Option<&'a Box<Type>>,
        )>,
        Rc<ModulePath>,
    )> for Method<'a>
{
    fn from(
        (func, path): (
            Vec<(
                &'a Visibility,
                String,
                &'a Punctuated<FnArg, Comma>,
                Option<&'a Box<Type>>,
            )>,
            Rc<ModulePath>,
        ),
    ) -> Method<'a> {
        Method { func, path }
    }
}

impl<'a> From<(&'a Vec<ImplItem>, Rc<ModulePath>)> for Method<'a> {
    fn from((impl_item, path): (&'a Vec<ImplItem>, Rc<ModulePath>)) -> Method<'a> {
        Method::from((
            impl_item
                .iter()
                .filter_map(|item| {
                    if let ImplItem::Method(ImplItemMethod {
                        ref vis, ref sig, ..
                    }) = item
                    {
                        if let Signature {
                            ident,
                            inputs,
                            output: ReturnType::Default,
                            ..
                        } = sig
                        {
                            Some((vis, ident.to_string(), inputs, None))
                        } else if let Signature {
                            ident,
                            inputs,
                            output: ReturnType::Type(_, ref output),
                            ..
                        } = sig
                        {
                            Some((vis, ident.to_string(), inputs, Some(output)))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>(),
            path,
        ))
    }
}

impl<'a> fmt::Display for Method<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{item}",
            item = escape_html(
                self.func
                    .iter()
                    .map(|&(vis, ref name, ref inputs, ty)| {
                        match (vis, ty) {
                            (Visibility::Public(_), Some(ty)) => {
                                format!(
                                    "+{}{}({}) -> {}",
                                    DEFAULT_FUNC,
                                    name,
                                    inputs.to_token_stream().to_string(),
                                    ty.to_token_stream().to_string()
                                )
                            }
                            (Visibility::Public(_), None) => {
                                format!(
                                    "+{}{}({})",
                                    DEFAULT_FUNC,
                                    name,
                                    inputs.to_token_stream().to_string()
                                )
                            }
                            (_, Some(ty)) => {
                                format!(
                                    "-{}{}({}) -> {}",
                                    DEFAULT_FUNC,
                                    name,
                                    inputs.to_token_stream().to_string(),
                                    ty.to_token_stream().to_string()
                                )
                            }
                            (_, None) => {
                                format!(
                                    "-{}{}({})",
                                    DEFAULT_FUNC,
                                    name,
                                    inputs.to_token_stream().to_string()
                                )
                            }
                        }
                    })
                    .collect::<Vec<String>>()
                    .join("\n")
                    .as_str()
            )
        )
    }
}
