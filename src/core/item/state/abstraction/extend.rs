use std::fmt;
use std::rc::Rc;

use ::dot::escape_html;
use quote::ToTokens;
use syn::{FnArg, ItemTrait, PatType, Receiver, ReturnType, Signature, TraitItem, TraitItemMethod, TypeParam, Visibility};

use ::module::path::ModulePath;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Trait<'a> {
    pub path: Rc<ModulePath>,
    /// Visibility
    pub vis: &'a Visibility,
    pub name: String,
    pub params: Vec<String>,
    pub items: Vec<(String, Vec<String>, String)>,
}

impl<'a> From<((&'a ItemTrait, &'a Vec<TypeParam>, &'a Vec<TraitItem>), Rc<ModulePath>)> for Trait<'a> {
    fn from(((item, ty_params, trait_item), path): ((&'a ItemTrait, &'a Vec<TypeParam>, &'a Vec<TraitItem>), Rc<ModulePath>)) -> Trait<'a> {
        Trait {
            path,
            vis: &item.vis,
            name: item.ident.to_string(),
            params: ty_params.iter()
                .map(|TypeParam { ident, .. }| ident.to_string())
                .collect(),
            items: trait_item.iter()
                .filter_map(|item: &TraitItem|
                    if let TraitItem::Method(TraitItemMethod { sig: Signature { ident, inputs, output: ReturnType::Type(_, output), .. }, .. }) = item {
                        Some((ident.to_string(), inputs.iter().map(|input| {
                            dbg!(input);
                            match input {
                                FnArg::Typed(PatType { ty, .. }) => {
                                    ty.to_token_stream().to_string()
                                }
                                FnArg::Receiver(Receiver { reference, mutability, .. }) => { // FIXME
                                    let (r1, r2) = match reference {
                                        Some((and, lifetime)) => (Some(and.to_token_stream().to_string()), Some(lifetime.to_token_stream().to_string())),
                                        None => (None, None)
                                    };
                                    let mutability = Some(mutability.to_token_stream().to_string());
                                    let s = Some(" Self".into());
                                    [r1, r2, mutability, s].iter().flatten().map(ToString::to_string).collect()
                                }
                            }
                        }).collect(), output.to_token_stream().to_string()))
                    } else {
                        None
                    }
                )
                .collect::<Vec<(String, Vec<String>, String)>>(),
        }
    }
}

impl<'a> fmt::Display for Trait<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "&lt;&lt;&lt;Trait&gt;&gt;&gt;\n{name}|{items}",
               name = self.name,
               items = escape_html(self.items.iter()
                   .map(|&(ref name, ref ty, ref ret): &(String, Vec<String>, String)|
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
