//! A crate containing many types of linked lists.

#![no_std]
#![warn(missing_docs)]
#![deny(unsafe_code)]

extern crate alloc;

#[cfg(feature = "stack")]
pub mod stack;
