use std::rc::Rc;
use std::path::PathBuf;
use std::ffi::OsString;
use std::vec;

extern crate syn;
pub mod path;

use self::path::ModulePath;
use syn::Item;

#[derive(Default, Debug, Clone)]
pub struct Module {
    pub list: Vec<Item>,
    pub path: ModulePath,
}

impl From<(Vec<Item>, PathBuf)> for Module {
    fn from((list, mut path): (Vec<Item>, PathBuf)) -> Module {
        path.set_extension("");
        Module {
            list,
            path: ModulePath {
                path: path.components()
                          .skip(1)
                          .map(|comp| comp.as_os_str().to_os_string())
                          .collect::<Vec<OsString>>(),
            },
        }
    }
}

impl IntoIterator for Module {
    type Item = (Item, Rc<ModulePath>);
    type IntoIter = vec::IntoIter<(Item, Rc<ModulePath>)>;

    fn into_iter(self) -> Self::IntoIter {
        let rc = &Rc::new(self.path);
        self.list.into_iter()
                 .map(|item| (item, Rc::clone(rc)))
                 .collect::<Vec<(Item, Rc<ModulePath>)>>()
                 .into_iter()
    }
}
