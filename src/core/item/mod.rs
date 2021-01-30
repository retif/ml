use std::rc::Rc;
use std::{iter, slice};

use itertools::Itertools;

use module::path::ModulePath;

pub use self::state::ItemState;

pub mod relation;
pub mod state;

/// The structure Item is a iterable collection of abstract elements.
#[derive(Debug, Clone)]
pub struct Item<'a> {
    /// Iterator.
    it: iter::Peekable<slice::Iter<'a, (syn::Item, Rc<ModulePath>)>>,
}

impl<'a> From<iter::Peekable<slice::Iter<'a, (syn::Item, Rc<ModulePath>)>>> for Item<'a> {
    /// The constructor method `from` returns a typed and iterable collection of abstract element.
    fn from(iter: iter::Peekable<slice::Iter<'a, (syn::Item, Rc<ModulePath>)>>) -> Item {
        Item { it: iter }
    }
}

impl<'a> Iterator for Item<'a> {
    type Item = ItemState<'a>;

    /// The method `next` will returns the first abstract elements defined like a structure,
    /// enumeration or trait.
    fn next(&mut self) -> Option<ItemState<'a>> {
        self.it.next().map(|item| {
            let mut list: Vec<&'a (syn::Item, Rc<ModulePath>)> = vec![item];

            list.extend(
                self.it
                    .peeking_take_while(|(item, _)| matches!(item, syn::Item::Impl(_))),
            );
            ItemState::from(list)
        })
    }
}
