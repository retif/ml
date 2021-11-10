pub mod abstraction;
pub mod implem;
pub mod method;

#[cfg(not(feature = "fn-emilgardis"))]
const DEFAULT_FUNC: &'static str = " ";
#[cfg(feature = "fn-emilgardis")]
const DEFAULT_FUNC: &'static str = " fn ";

use self::abstraction::Abstract;
use self::implem::Implem;
use self::method::Method;

use super::relation::Relation;

use std::ops::BitOr;
use std::fmt;
use std::rc::Rc;

use rustc_span::symbol::Symbol;
use rustc_ast::{ptr, ast};

use crate::module::path::ModulePath;
use crate::Config;

/// The structure `ItemState` describes an abstract element with a collections of methodes
/// and implementations.
#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub struct ItemState<'a> {
    /// Data Type.
    pub(crate) node: Abstract<'a>,
    /// Implementation of Method.
    method: Vec<Method>,
    /// Implementation of Trait.
    implem: Vec<Implem>,
}

impl <'a> ItemState <'a> {

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
        if let Some(ref name) = self.as_name() {
            let mut ty_name_mut: String = String::from("*mut ");
            let mut ty_name_const: String = String::from("*const ");
            
            ty_name_mut.push_str(&name.as_str());
            ty_name_const.push_str(&name.as_str());
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

    pub fn as_name(&self) -> Option<&Symbol> {
        self.node.as_name()
    }

    pub fn as_arrow(&self, rhs: &ItemState<'a>) -> Relation {
        Relation::from((self, rhs))
    }
}

impl <'a>From<(Abstract<'a>, Vec<&'a (ptr::P<ast::Item>, Rc<ModulePath>)>)> for ItemState<'a> {
    fn from((node, properties): (Abstract<'a>, Vec<&'a (ptr::P<ast::Item>, Rc<ModulePath>)>)) -> ItemState<'a> {
        ItemState {
            node: node,
            method: properties.iter()
                .filter_map(|&&(ref item, ref path): &&'a (ptr::P<ast::Item>, Rc<ModulePath>)|
                    if let ast::ItemKind::Impl(_b) = item.kind.clone() {
                        Some(Method::from((vec![(**item).clone()], Rc::clone(path))))
                    } else {
                        None
                    }
                )
                .collect::<Vec<Method>>(),
            implem: properties.iter()
                .filter_map(|&&(ref item, _): &&'a (ptr::P<ast::Item>, Rc<ModulePath>)| {
                    if let ast::ItemKind::Impl(b) = &item.kind {
                        let ast::ImplKind { of_trait, .. } = &**b;
                        if let Some(ast::TraitRef {path: ast::Path {span: _, ref segments, ..}, ..}) = of_trait {
                            Some(Implem::from((segments, &vec![(**item).clone()])))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .collect::<Vec<Implem>>()
        }
    }
}

impl <'a>From<Vec<&'a (ptr::P<ast::Item>, Rc<ModulePath>)>> for ItemState<'a> {
    fn from(state: Vec<&'a (ptr::P<ast::Item>, Rc<ModulePath>)>) -> ItemState<'a> {
        state.split_first().and_then(|(&&(ref item, ref path), properties): (&&'a (ptr::P<ast::Item>, Rc<ModulePath>), &[&'a (ptr::P<ast::Item>, Rc<ModulePath>)])| {
            match &item.kind {
                // Trait.
                &ast::ItemKind::Trait(box ast::TraitKind(_, _, ast::Generics {ref params, ..}, _, ref trait_item)) => {
                    let kind: (&'a ast::Item, &'a Vec<ast::GenericParam>, &'a Vec<ptr::P<ast::AssocItem>>) = (item, params, trait_item);
                    let kind: (Abstract, Vec<&'a (ptr::P<ast::Item>, Rc<ModulePath>)>) = (Abstract::from((kind, Rc::clone(path))), properties.to_vec());
                    Some(ItemState::from(kind))
                },
                // Structure with variables.
                &ast::ItemKind::Struct(ast::VariantData::Struct(ref struct_field, _), ..) => {
                    let kind: (&'a ast::Item, &'a Vec<ast::FieldDef>) = (item, struct_field);
                    let kind: (Abstract, Vec<&'a (ptr::P<ast::Item>, Rc<ModulePath>)>) = (Abstract::from((kind, Rc::clone(path))), properties.to_vec());
                    Some(ItemState::from(kind))
                },
                // Enumeration with variables.
                &ast::ItemKind::Enum(ast::EnumDef {ref variants}, ast::Generics {ref params, ..}) => {
                    let kind: (&'a ast::Item, &'a Vec<ast::GenericParam>, &'a Vec<ast::Variant>) = (item, params, variants);
                    let kind: (Abstract, Vec<&'a (ptr::P<ast::Item>, Rc<ModulePath>)>) = (Abstract::from((kind, Rc::clone(path))), properties.to_vec());
                    Some(ItemState::from(kind))
                },
                _ => None,
            }
        }).unwrap_or_default()
    }
}


impl <'a>fmt::Display for ItemState<'a> {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        write!(f, 
                "<font face=\"{font}\"><table border=\"1\" cellspacing=\"0\" cellpadding=\"10\">{node}",
                font = Config::global().font_name,
                node = self.node,
        )?;

        let include_method = !self.method.is_empty() &&Config::global().include_methods;

        if include_method {

            let bgcolor = match self.node {
                Abstract::Struct{..} => Config::global().struct_method_bgcolor.clone(),
                Abstract::Trait{..} => Config::global().trait_method_bgcolor.clone(),
                Abstract::Enum{..} => Config::global().enum_method_bgcolor.clone(),
                Abstract::None => "".to_string(),
            };

            write!(f, "<tr><td align=\"left\" bgcolor=\"{bgcolor}\">{method}<br align=\"left\"/></td></tr>",
                bgcolor = bgcolor,
                method = self.method.iter()
                                    .map(|ref methods| format!("{}", methods))
                                    .collect::<Vec<String>>().join("<br align=\"left\"/>\n").as_str())?;
        }

        let include_implem = !self.implem.is_empty() && Config::global().include_implems;

        if include_implem { // Config::global().include_implem {

            let bgcolor = match self.node {
                Abstract::Struct{..} => Config::global().struct_implem_bgcolor.clone(),
                Abstract::Trait{..} => "".to_string(),
                Abstract::Enum{..} => Config::global().enum_implem_bgcolor.clone(),
                Abstract::None => "".to_string(),
            };

            write!(f, "<tr><td align=\"left\" bgcolor=\"{bgcolor}\">{implem}<br align=\"left\"/></td></tr>",
                bgcolor = bgcolor,
                implem = self.implem.iter()
                                    .map(|ref implem| format!("{}", implem))
                                    .collect::<Vec<String>>().join("<br align=\"left\"/>\n").as_str())?;
        }

        write!(f, "</table></font>")
    }
}
