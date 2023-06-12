//! A crate containing many types of linked lists.

#![no_std]
#![warn(missing_docs)]
#![deny(unsafe_code)]

#[cfg(feature = "stack")]
/// A singly linked list, with stack operations.
pub mod stack;