//! Core Syntax and definitions.

use std::borrow::Cow;
use std::ops::BitOr;
use std::rc::Rc;
use std::{iter, slice};

use dot::{Arrow, Edges, GraphWalk, Id, LabelText, Labeller, Nodes, Style};
use itertools::Itertools;

use module::path::ModulePath;

use self::item::relation::Relation;
use self::item::{Item, ItemState};
use self::segment::Segment;

pub mod item;
pub mod segment;

#[derive(Debug, Clone)]
pub struct ListItem<'a> {
    parse: Item<'a>,
}

impl<'a> From<Item<'a>> for ListItem<'a> {
    fn from(parse: Item<'a>) -> ListItem<'a> {
        ListItem { parse }
    }
}

impl<'a> From<iter::Peekable<slice::Iter<'a, (syn::Item, Rc<ModulePath>)>>> for ListItem<'a> {
    fn from(list: iter::Peekable<slice::Iter<'a, (syn::Item, Rc<ModulePath>)>>) -> ListItem<'a> {
        ListItem::from(Item::from(list))
    }
}

impl<'a> Iterator for ListItem<'a> {
    type Item = ItemState<'a>;

    fn next(&mut self) -> Option<ItemState<'a>> {
        self.parse
            .by_ref()
            .skip_while(|state| state.is_none())
            .next()
    }
}

impl<'a> Labeller<'a, ItemState<'a>, Segment<'a>> for ListItem<'a> {
    fn graph_id(&'a self) -> Id<'a> {
        Id::new("ml").unwrap()
    }

    fn node_id(&'a self, state: &ItemState<'a>) -> Id<'a> {
        match state.as_name() {
            Some(name) => Id::new(format!("nd{}", name)).unwrap(),
            _ => unreachable!(),
        }
    }

    fn node_shape(&'a self, _node: &ItemState<'a>) -> Option<LabelText<'a>> {
        Some(LabelText::LabelStr(Cow::from("record")))
    }

    fn node_label(&'a self, state: &ItemState<'a>) -> LabelText<'a> {
        LabelText::LabelStr(state.to_string().into())
    }

    fn edge_end_arrow(&'a self, seg: &Segment<'a>) -> Arrow {
        match (seg.left.as_arrow(&seg.right), seg.right.as_arrow(&seg.left)) {
            (Relation::Association, Relation::Association) => Arrow::none(),
            (edge_left, _) => Arrow::from_arrow(edge_left.as_style()),
        }
    }

    fn edge_style(&'a self, seg: &Segment<'a>) -> Style {
        if seg
            .left
            .is_realization(&seg.right)
            .bitor(seg.left.is_dependency(&seg.right))
        {
            Style::Dashed
        } else {
            Style::None
        }
    }
}

impl<'a> GraphWalk<'a, ItemState<'a>, Segment<'a>> for ListItem<'a> {
    fn nodes(&'a self) -> Nodes<'a, ItemState<'a>> {
        Cow::Owned(self.clone().collect::<Vec<ItemState<'a>>>())
    }

    fn edges(&'a self) -> Edges<'a, Segment<'a>> {
        let items = self.clone().collect::<Vec<ItemState<'a>>>();

        let dbg = items.iter().map(|s| format!("{:?}", s)).collect::<Vec<_>>();
        println!("{:#?}", dbg);

        let items = items
            .iter()
            .map(|item| {
                items
                    .iter()
                    .filter(|rhs| item.ne(rhs))
                    .filter(|rhs| item.is_relation(rhs))
                    .map(|rhs| Segment::from((item.clone(), rhs.clone())))
                    .collect::<Vec<Segment<'a>>>()
            })
            .collect::<Vec<Vec<Segment<'a>>>>()
            .concat()
            .into_iter()
            .unique()
            .collect::<Vec<Segment<'a>>>();

        Cow::Owned(items)
    }

    fn source(&self, seg: &Segment<'a>) -> ItemState<'a> {
        seg.left.clone()
    }

    fn target(&self, seg: &Segment<'a>) -> ItemState<'a> {
        seg.right.clone()
    }
}
