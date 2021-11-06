//! The purpose of this module is to provide reexports of core traits so that they can be then
//! glob-imported all at once.

pub use crate::DEFAULT_NAME_DOT;
pub use crate::DEFAULT_NAME_PNG;
pub use crate::core::segment::Segment;
pub use crate::core::item::Item;
pub use crate::core::item::relation::Relation;
pub use crate::core::item::state::ItemState;
pub use crate::core::item::state::method::Method;
pub use crate::core::item::state::implem::Implem;
pub use crate::core::item::state::abstraction::Abstract;
pub use crate::core::item::state::abstraction::extend::Trait;
pub use crate::core::item::state::abstraction::structure::Struct;
pub use crate::core::item::state::abstraction::enumerate::Enum;
