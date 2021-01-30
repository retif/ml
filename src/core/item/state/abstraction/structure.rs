use std::fmt;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;

use dot::escape_html;
use quote::ToTokens;
use syn::{Field, ItemStruct, Visibility};

use module::path::ModulePath;

/// The structure `Struct` is a structure abstract element.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Struct<'a> {
    pub path: Rc<ModulePath>,
    /// Visibility
    pub vis: &'a Visibility,
    pub name: String,
    pub fields: Vec<(&'a Visibility, String, String)>,
}

impl<'a> From<((&'a ItemStruct, &'a Vec<Field>), Rc<ModulePath>)> for Struct<'a> {
    fn from(
        ((item, struct_field), path): ((&'a ItemStruct, &'a Vec<Field>), Rc<ModulePath>),
    ) -> Struct<'a> {
        Struct {
            path,
            vis: &item.vis,
            name: item.ident.to_string(),
            fields: struct_field
                .iter()
                .enumerate()
                .map(|(index, Field { vis, ident, ty, .. })| {
                    let ident = if let Some(ident) = ident {
                        ident.to_string()
                    } else {
                        index.to_string()
                    };
                    (vis, ident, ty.to_token_stream().to_string())
                })
                .collect(),
        }
    }
}

impl<'a> fmt::Display for Struct<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if self.fields.is_empty() {
            write!(
                f,
                "&lt;&lt;&lt;Structure&gt;&gt;&gt;\n{name}",
                name = self.name
            )
        } else {
            write!(
                f,
                "&lt;&lt;&lt;Structure&gt;&gt;&gt;\n{name}|{fields}",
                name = self.name,
                fields = escape_html(
                    self.fields
                        .iter()
                        .map(
                            |&(ref vis, ref name, ref ty): &(&Visibility, String, String)| {
                                if let Visibility::Public(_) = vis {
                                    format!("+ {name}: {ty}", name = name, ty = ty)
                                } else {
                                    format!("- {name}: {ty}", name = name, ty = ty)
                                }
                            }
                        )
                        .collect::<Vec<String>>()
                        .join("\n")
                        .as_str()
                ),
            )
        }
    }
}
