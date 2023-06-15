#![allow(unused)]
// NOTE:
// This is a private module, but it is still somewhat documented for convienence.
use core::ops::Deref;
use core::marker::PhantomData;

#[cfg(feature = "persistent")]
use alloc::rc::Rc;

#[cfg(feature = "persistent_arc")]
use alloc::sync::Arc;

/// A persistent singly linked list.
pub struct PersistentList<T, Ptr: RefCountingPtr<Node<T, Ptr>>> {
    // Ptr is either Rc<Node<T>> or Arc<Node<T>>
    head: Option<Ptr>,
    _ghost: PhantomData<T>
}

impl<T, Ptr: RefCountingPtr<Node<T, Ptr>>> PersistentList<T, Ptr> {
    pub fn new() -> Self {
        Self {
            head: None,
            _ghost: PhantomData
        }
    }

    pub fn prepend(&self, elem: T) -> Self {
        Self {
            head: Some(Ptr::new(Node {
                elem,
                next: self.head.clone()
            })),
            _ghost: PhantomData
        }
    }

    pub fn tail(&self) -> Self {
        Self {
            head: self.head.as_ref().and_then(|node| node.next.clone()),
            _ghost: PhantomData
        }
    }

    pub fn head(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }
}

pub struct Node<T, Ptr: RefCountingPtr<Node<T, Ptr>>> {
    elem: T,
    next: Option<Ptr>
}

/// This is a trait that is implemented on `rc::Rc<T>` and `sync::Arc<T>`.
/// It is used as a generic bound (`PersistentList<T, P: RefCountingPtr<T>>`)
/// so PersistentList can use both types without knowing which.
pub trait RefCountingPtr<T>: Clone + Deref<Target = T> {
    fn new(val: T) -> Self;
    fn into_inner(self) -> Option<T>;
}

#[cfg(feature = "persistent")]
impl<T> RefCountingPtr<T> for Rc<T> {
    fn new(val: T) -> Self {
        Rc::new(val)
    }

    fn into_inner(self) -> Option<T> {
        Rc::into_inner(self)
    }
}

#[cfg(feature = "persistent_arc")]
impl<T> RefCountingPtr<T> for Arc<T> {
    fn new(val: T) -> Self {
        Arc::new(val)
    }

    fn into_inner(self) -> Option<T> {
        Arc::into_inner(self)
    }
}
