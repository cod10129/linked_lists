//! A singly linked list, with stack operations.
use alloc::boxed::Box;

/// A singly linked list that can do (most of) what a stack can.
pub struct List<T> {
    head: Option<Box<Node<T>>>
}

struct Node<T> {
    elem: T,
    next: Option<Box<Node<T>>>
}
