//! A singly linked persistent list using `Arc`.
//! 
//! Because this list uses [`Arc`], it is thread-safe,
//! but incurs some additional overhead.
//! If you do not need your list to be thread-safe, 
//! enable the `persistent` feature, and use the [`persistent`] module instead.
//! That module has the exact same API as this one.
//! 
//! [`persistent`]: crate::persistent
use alloc::sync::Arc;

make_list_def!{Arc;
/// A singly linked shared persistent list that is thread safe.
/// See the [module-level documentation](self) for more.
}
