pub mod enumerate;
pub mod extend;
pub mod structure;

use std::fmt;
use std::rc::Rc;
use std::vec;

use rustc_ast::{ast, ptr};
use rustc_span::symbol;

use crate::module::path::ModulePath;

use self::enumerate::Enum;
use self::extend::Trait;
use self::structure::Struct;

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
    pub fn as_name(&self) -> Option<&symbol::Symbol> {
        match self {
            &Abstract::Trait(Trait {
                vis: _, ref name, ..
            }) => Some(name),
            &Abstract::Struct(Struct {
                vis: _, ref name, ..
            }) => Some(name),
            &Abstract::Enum(Enum {
                vis: _, ref name, ..
            }) => Some(name),
            &Abstract::None => None,
        }
    }

    pub fn as_type(&self) -> Option<&str> {
        match self {
            &Abstract::Trait(_) => Some("trait"),
            &Abstract::Struct(_) => Some("struct"),
            &Abstract::Enum(_) => Some("enum"),
            &Abstract::None => None,
        }
    }

    pub fn span(&self) -> Option<&rustc_span::Span> {
        match self {
            &Abstract::Trait(ref t) => Some(&t.span),
            &Abstract::Struct(ref s) => Some(&s.span),
            &Abstract::Enum(ref e) => Some(&e.span),
            &Abstract::None => None,
        }
    }

    pub fn path(&self) -> Option<&Rc<ModulePath>> {
        match self {
            &Abstract::Trait(ref t) => Some(&t.path),
            &Abstract::Struct(ref s) => Some(&s.path),
            &Abstract::Enum(ref e) => Some(&e.path),
            &Abstract::None => None,
        }
    }
}

impl<'a> IntoIterator for &'a Abstract<'a> {
    type Item = &'a String;
    type IntoIter = vec::IntoIter<&'a String>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            &Abstract::Struct(ref s) => s
                .fields
                .iter()
                .map(
                    |&(_, _, ref ty): &'a (
                        &'a ast::VisibilityKind,
                        Option<symbol::Symbol>,
                        String,
                    )| ty,
                )
                .collect::<Vec<&'a String>>()
                .into_iter(),
            &Abstract::Enum(ref e) => e
                .variants
                .iter()
                .map(|&(_, ref ty_field): &'a (symbol::Symbol, Vec<String>)| {
                    ty_field
                        .iter()
                        .map(|ty: &'a String| ty)
                        .collect::<Vec<&'a String>>()
                })
                .collect::<Vec<Vec<&'a String>>>()
                .concat()
                .into_iter(),
            _ => Vec::default().into_iter(),
        }
    }
}

impl<'a> Default for Abstract<'a> {
    fn default() -> Abstract<'a> {
        Abstract::None
    }
}

impl<'a>
    From<(
        (
            &'a ast::Item,
            &'a Vec<ast::GenericParam>,
            &'a Vec<ptr::P<ast::Item<ast::AssocItemKind>>>,
        ),
        Rc<ModulePath>,
    )> for Abstract<'a>
{
    fn from(
        arguments: (
            (
                &'a ast::Item,
                &'a Vec<ast::GenericParam>,
                &'a Vec<ptr::P<ast::Item<ast::AssocItemKind>>>,
            ),
            Rc<ModulePath>,
        ),
    ) -> Abstract<'a> {
        Abstract::Trait(Trait::from(arguments))
    }
}

impl<'a> From<((&'a ast::Item, &'a Vec<ast::FieldDef>), Rc<ModulePath>)> for Abstract<'a> {
    fn from(arguments: ((&'a ast::Item, &'a Vec<ast::FieldDef>), Rc<ModulePath>)) -> Abstract<'a> {
        Abstract::Struct(Struct::from(arguments))
    }
}

impl<'a>
    From<(
        (
            &'a ast::Item,
            &'a Vec<ast::GenericParam>,
            &'a Vec<ast::Variant>,
        ),
        Rc<ModulePath>,
    )> for Abstract<'a>
{
    fn from(
        arguments: (
            (
                &'a ast::Item,
                &'a Vec<ast::GenericParam>,
                &'a Vec<ast::Variant>,
            ),
            Rc<ModulePath>,
        ),
    ) -> Abstract<'a> {
        Abstract::Enum(Enum::from(arguments))
    }
}

impl<'a> fmt::Display for Abstract<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Abstract::Struct(ref item) => write!(f, "{}", item),
            &Abstract::Enum(ref item) => write!(f, "{}", item),
            &Abstract::Trait(ref item) => write!(f, "{}", item),
            &Abstract::None => Err(fmt::Error),
        }
    }
}
