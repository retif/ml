pub mod relation;
pub mod state;

pub use self::state::ItemState;

use std::rc::Rc;
use std::{iter, slice};

use rustc_ast::{ast, ptr};

use crate::module::path::ModulePath;

/// The structure Item is a iterable collection of abstract elements.

#[derive(Debug, Clone)]
pub struct Item<'a> {
    /// Iterator.
    it: iter::Peekable<slice::Iter<'a, (ptr::P<ast::Item>, Rc<ModulePath>)>>,
}

impl<'a> From<iter::Peekable<slice::Iter<'a, (ptr::P<ast::Item>, Rc<ModulePath>)>>> for Item<'a> {
    /// The constructor method `from` returns a typed and iterable collection of abstract element.
    fn from(iter: iter::Peekable<slice::Iter<'a, (ptr::P<ast::Item>, Rc<ModulePath>)>>) -> Item<'a> {
        Item { it: iter }
    }
}

impl<'a> Iterator for Item<'a> {
    type Item = ItemState<'a>;

    /// The method `next` will returns the first abstract elements defined like a structure,
    /// enumeration or trait.
    fn next(&mut self) -> Option<ItemState<'a>> {
        self.it.next().and_then(|item| {
            let mut list: Vec<&'a (ptr::P<ast::Item>, Rc<ModulePath>)> = vec![item];

            // Loop over all Items to find any Impl with a name that matches our name.
            // This way we can handle cases like:
            //      struct Foo{}
            //      struct Bar{}
            //      impl Foo {...}
            //      impl Bar {...}
            //      impl Foo {...}
            let item_name = &item.0.ident.name;
            list.extend(
                self.it
                    .clone()
                    .filter(
                        |&&(ref subitem, _): &&'a (ptr::P<ast::Item>, Rc<ModulePath>)| {
                            if let &ast::ItemKind::Impl(box ast::Impl {
                                self_ty: ref ty, ..
                            }) = &subitem.kind
                            {
                                if let &ast::Ty {
                                    kind:
                                        ast::TyKind::Path(
                                            _,
                                            ast::Path {
                                                segments: ref seg, ..
                                            },
                                        ),
                                    ..
                                } = &**ty
                                {
                                    if !seg.is_empty() && &seg[0].ident.name == item_name {
                                        return true;
                                    }
                                }
                            }
                            false
                        },
                    )
                    .collect::<Vec<&'a (ptr::P<ast::Item>, Rc<ModulePath>)>>(),
            );
            Some(ItemState::from(list))
        })
    }
}
