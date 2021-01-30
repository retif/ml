use std::fmt;
use std::ops::BitOr;
use std::rc::Rc;

use syn::{Field, Item, ItemEnum, ItemImpl, ItemTrait, Path, TraitItem, TypeParam, Variant};

use ::module::path::ModulePath;

use super::relation::Relation;

use self::abstraction::Abstract;
use self::implem::Implem;
use self::method::Method;

pub mod abstraction;
pub mod implem;
pub mod method;

#[cfg(not(feature = "fn-emilgardis"))]
const DEFAULT_FUNC: &'static str = " ";
#[cfg(feature = "fn-emilgardis")]
const DEFAULT_FUNC: &'static str = " fn ";

/// The structure `ItemState` describes an abstract element with a collections of methodes
/// and implementations.
#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub struct ItemState<'a> {
    /// Data Type.
    node: Abstract<'a>,
    /// Implementation of Method.
    method: Vec<Method<'a>>,
    /// Implementation of Trait.
    implem: Vec<Implem>,
}

impl<'a> ItemState<'a> {
    pub fn is_none(&self) -> bool {
        self.node.eq(&Abstract::None)
    }

    pub fn is_association(&self, rhs: &ItemState<'a>) -> bool {
        if let Some(ref name) = self.as_name() {
            let ref ty_name: String = name.to_string();

            rhs.method.iter()
                .any(|func| func.is_association(ty_name))
                .bitor(rhs.implem.iter()
                    .any(|implem| implem.is_association(&ty_name)))
        } else {
            false
        }
    }

    pub fn is_dependency(&self, rhs: &ItemState<'a>) -> bool {
        if let Some(ref name) = self.as_name() {
            let ref ty_name: String = name.to_string();

            rhs.method.iter()
                .any(|method| method.is_dependency(&ty_name))
                .bitor(self.implem.iter()
                    .any(|implem| implem.is_dependency(&ty_name)))
        } else {
            false
        }
    }

    pub fn is_aggregation(&self, rhs: &ItemState<'a>) -> bool {
        if let Some(name) = self.as_name() {
            let mut ty_name_mut: String = String::from("* mut ");
            let mut ty_name_const: String = String::from("* const ");

            ty_name_mut.push_str(&name);
            ty_name_const.push_str(&name);
            rhs.node.into_iter()
                .any(|attribut: &String|
                    attribut.split(|at| "<[(;,)]>".contains(at))
                        .any(|ty| ty_name_mut.eq(ty).bitor(ty_name_const.eq(ty))))
        } else {
            false
        }
    }

    pub fn is_composition(&self, rhs: &ItemState<'a>) -> bool {
        if let Some(ref name) = self.as_name() {
            let ty_name: String = name.to_string();

            rhs.node.into_iter()
                .any(|attribut: &String|
                    attribut.split(|at| "<[(;,)]>".contains(at))
                        .any(|ty| ty.eq(&ty_name)))
        } else {
            false
        }
    }

    pub fn is_realization(&self, rhs: &ItemState<'a>) -> bool {
        if let Some(ref name) = self.as_name() {
            let ty_name: String = name.to_string();

            rhs.implem.iter()
                .any(|implem| implem.is_realization(&ty_name))
        } else {
            false
        }
    }

    pub fn is_relation(&self, rhs: &ItemState<'a>) -> bool {
        self.is_association(rhs)
            .bitor(self.is_dependency(rhs))
            .bitor(self.is_aggregation(rhs))
            .bitor(self.is_composition(rhs))
            .bitor(self.is_realization(rhs))
    }

    pub fn as_name(&self) -> Option<&String> {
        self.node.as_name()
    }

    pub fn as_arrow(&self, rhs: &ItemState<'a>) -> Relation {
        Relation::from((self, rhs))
    }
}

impl<'a> From<(Abstract<'a>, Vec<&'a (Item, Rc<ModulePath>)>)> for ItemState<'a> {
    fn from((node, properties): (Abstract<'a>, Vec<&'a (Item, Rc<ModulePath>)>)) -> ItemState<'a> {
        ItemState {
            node,
            method: properties.iter()
                .filter_map(|(item, path)|
                    match item {
                        Item::Impl(ItemImpl { trait_, items, .. })
                        if trait_.is_none() => Some(Method::from((items, Rc::clone(path)))),
                        _ => None,
                    }
                )
                .collect(),
            implem: properties.iter()
                .filter_map(|(item, _)|
                    if let Item::Impl(ItemImpl { trait_: Some((_, Path { segments, .. }, _)), items, .. }) = item {
                        Some(Implem::from((segments, items)))
                    } else {
                        None
                    }
                )
                .collect(),
        }
    }
}

impl<'a> From<Vec<&'a (Item, Rc<ModulePath>)>> for ItemState<'a> {
    fn from(state: Vec<&'a (Item, Rc<ModulePath>)>) -> ItemState<'a> {
        state.split_first().and_then(|(&&(ref item, ref path), properties): (&&'a (Item, Rc<ModulePath>), &[&'a (Item, Rc<ModulePath>)])| {
            match &item {
                // Trait.
                &Item::Trait(item) => {
                    let ItemTrait { generics, items, .. } = item;
                    let ty_params = Box::leak(Box::new(generics.type_params().cloned().collect())); // FIXME
                    let kind: (_, &'a Vec<TypeParam>, &'a Vec<TraitItem>) = (item, ty_params, items);
                    let kind: (Abstract, Vec<&'a (Item, Rc<ModulePath>)>) = (Abstract::Trait((kind, Rc::clone(path)).into()), properties.to_vec());
                    Some(ItemState::from(kind))
                }
                // Structure with variables.
                &Item::Struct(item) => {
                    let fields = Box::leak(Box::new(item.fields.iter().cloned().collect())); // FIXME
                    let kind: (_, &Vec<Field>) = (item, fields);
                    let kind = (Abstract::Struct((kind, Rc::clone(path)).into()), properties.to_vec());
                    Some(ItemState::from(kind))
                }
                // Enumeration with variables.
                &Item::Enum(item) => {
                    let ItemEnum { generics, variants, .. } = item;
                    let ty_params = Box::leak(Box::new(generics.type_params().cloned().collect())); // FIXME
                    let variants = Box::leak(Box::new(variants.iter().cloned().collect())); // FIXME
                    let kind: (&'a ItemEnum, &'a Vec<TypeParam>, &'a Vec<Variant>) = (item, ty_params, variants);
                    let kind: (Abstract, Vec<&'a (Item, Rc<ModulePath>)>) = (Abstract::Enum((kind, Rc::clone(path)).into()), properties.to_vec());
                    Some(ItemState::from(kind))
                }
                _ => None,
            }
        }).unwrap_or_default()
    }
}

impl<'a> fmt::Display for ItemState<'a> {
    #[cfg(feature = "implem")]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{{node}|{method}|{implem}}}",
               node = self.node,
               method = self.method.iter()
                   .map(|ref methods| format!("{}", methods))
                   .collect::<Vec<String>>().join("\n").as_str(),
               implem = self.implem.iter()
                   .map(|ref implem| format!("{}", implem))
                   .collect::<Vec<String>>().join("\n").as_str())
    }

    #[cfg(not(feature = "implem"))]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.method.is_empty() {
            write!(f, "{{{node}}}", node = self.node)
        } else {
            write!(f, "{{{node}|{method}}}",
                   node = self.node,
                   method = self.method.iter()
                       .map(|ref methods| format!("{}", methods))
                       .collect::<Vec<String>>().join("\n").as_str())
        }
    }
}
