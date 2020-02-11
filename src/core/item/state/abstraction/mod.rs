use std::fmt;
use std::vec;

use syn::Visibility;

use self::enumerate::Enum;
use self::extend::Trait;
use self::structure::Struct;

pub mod extend;
pub mod structure;
pub mod enumerate;

/// The structure `Abstract` is a enumerate for abstract element types or none.
#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Abstract<'a> {
    Trait(Trait<'a>),
    Struct(Struct<'a>),
    Enum(Enum<'a>),
    None,
}

impl<'a> Abstract<'a> {
    /// The method `as_name` returns the name of the abstract element
    /// or else declare a panic.
    pub fn as_name(&self) -> Option<&String> {
        match self {
            Abstract::Trait(Trait { name, .. }) => Some(name),
            Abstract::Struct(Struct { name, .. }) => Some(name),
            Abstract::Enum(Enum { name, .. }) => Some(name),
            Abstract::None => None,
        }
    }
}

impl<'a> IntoIterator for &'a Abstract<'a> {
    type Item = &'a String;
    type IntoIter = vec::IntoIter<&'a String>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            Abstract::Struct(Struct { fields, .. }) => {
                fields.iter()
                    .map(|&(_, _, ref ty): &'a (&'a Visibility, String, String)| ty)
                    .collect::<Vec<&'a String>>()
                    .into_iter()
            }
            Abstract::Enum(Enum { variants, .. }) => {
                variants.iter()
                    .map(|&(_, ref ty_field): &'a (String, Vec<String>)|
                        ty_field.iter()
                            .map(|ty: &'a String| ty)
                            .collect::<Vec<&'a String>>())
                    .collect::<Vec<Vec<&'a String>>>()
                    .concat()
                    .into_iter()
            }
            _ => Vec::default().into_iter(),
        }
    }
}

impl<'a> Default for Abstract<'a> {
    fn default() -> Abstract<'a> {
        Abstract::None
    }
}

impl<'a> fmt::Display for Abstract<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Abstract::Struct(item) => write!(f, "{}", item),
            Abstract::Enum(item) => write!(f, "{}", item),
            Abstract::Trait(item) => write!(f, "{}", item),
            Abstract::None => Err(fmt::Error),
        }
    }
}
