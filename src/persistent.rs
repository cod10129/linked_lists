//! A singly linked persistent list using reference-counting pointers.
//! 
//! This `List` uses [`Rc`], which makes it not thread-safe.
//! If you need this to be thread-safe, enable the `persistent_arc`
//! crate feature, and use the [`persistent_arc`] module instead.
//! That module has the exact same API as this one, and only the internals are changed.
//! 
//! [`persistent_arc`]: crate::persistent_arc
//! [`Rc`]: alloc::rc::Rc