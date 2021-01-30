use std::fmt;
use std::rc::Rc;

use dot::escape_html;
use quote::ToTokens;
use syn::{Field, TypeParam, Variant};

use module::path::ModulePath;

/// The structure `Enum` is a enumerate abstract element.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Enum<'a> {
    pub path: Rc<ModulePath>,
    /// Visibility
    pub vis: &'a syn::Visibility,
    pub name: String,
    pub params: Vec<String>,
    pub variants: Vec<(String, Vec<String>)>,
}

impl<'a>
    From<(
        (&'a syn::ItemEnum, &'a Vec<TypeParam>, &'a Vec<Variant>),
        Rc<ModulePath>,
    )> for Enum<'a>
{
    fn from(
        ((item, ty_params, variants), path): (
            (&'a syn::ItemEnum, &'a Vec<TypeParam>, &'a Vec<Variant>),
            Rc<ModulePath>,
        ),
    ) -> Enum<'a> {
        Enum {
            path,
            vis: &item.vis,
            name: item.ident.to_string(),
            params: ty_params
                .iter()
                .map(|param| param.ident.to_string())
                .collect(),
            variants: variants
                .iter()
                .map(|Variant { ident, fields, .. }| {
                    (
                        ident.to_string(),
                        if fields.is_empty() {
                            vec![]
                        } else {
                            fields
                                .iter()
                                .map(|Field { ty, .. }| ty.to_token_stream().to_string())
                                .collect()
                        },
                    )
                })
                .collect(),
        }
    }
}

impl<'a> fmt::Display for Enum<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.variants.is_empty() {
            write!(
                f,
                "&lt;&lt;&lt;Enumeration&gt;&gt;&gt;\n{name}",
                name = self.name
            )
        } else {
            write!(
                f,
                "&lt;&lt;&lt;Enumeration&gt;&gt;&gt;\n{name}|{variants}",
                name = self.name,
                variants = escape_html(
                    self.variants
                        .iter()
                        .map(|(name, struct_field)| if struct_field.is_empty() {
                            name.to_owned()
                        } else {
                            format!("{}({})", name, struct_field.join(", "))
                        })
                        .collect::<Vec<String>>()
                        .join("\n")
                        .as_str()
                ),
            )
        }
    }
}
