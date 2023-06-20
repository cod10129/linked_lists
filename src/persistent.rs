//! A singly linked persistent list using `Rc`.
//! 
//! Because this list uses [`Rc`], it is not thread-safe.
//! If you need this to be thread-safe, enable the `persistent_arc`
//! crate feature, and use the [`persistent_arc`] module instead.
//! That module has the exact same API as this one.
//! 
//! [`persistent_arc`]: crate::persistent_arc
use alloc::rc::Rc;

version!{1, 0, 0}

make_list!{Rc;
/// A singly linked shared persistent list that is not thread safe.
/// See the [module-level documentation](self) for more.
}

tests!{}
