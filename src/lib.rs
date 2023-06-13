//! A crate containing many types of linked lists.

#![no_std]
#![warn(missing_docs)]
#![deny(unsafe_code)]

extern crate alloc;

/// A struct describing the version of a module in this crate.
/// 
/// List-containing modules in this crate have their own version, separate from the full crate version.
/// A module on a `0.x.y` version is subject to ANY change during a crate *minor* release,
/// including removal of it entirely.
/// 
/// This system also exists so that you can use a breaking *crate-level* update,
/// while knowing that you will not have to change any of your code, 
/// because the update only broke a specific list that you are not using.
/// 
/// Essentially, when using this crate, you should always use an exact version specifier `=x.y.z`,
/// and only update when you have checked the new ListVersion of the list(s) you are using.
/// The ListVersion of all of the lists is next to the module name in the README, and in `module::VERSION`.
/// 
/// See the [SemVer specification](https://semver.org) for details on the meaning of 
/// "major", "minor", and "patch".
#[derive(Clone, Copy)]
pub struct ListVersion {
    /// The major version of a module.
    /// 
    /// When this is 0, all items in the module are considered "experimental",
    /// and may be removed at any crate minor release.
    /// Otherwise, the biggest change to *any* module will cause that level of crate update.
    pub major: u16,
    /// The minor version of a module.
    pub minor: u16,
    /// The patch version of a module.
    pub patch: u16
}

impl ListVersion {
    /// Returns whether this list is considered "experimental" or not.
    /// 
    /// An experimental list can have breaking changes in non-breaking crate releases.
    pub fn experimental(self) -> bool {
        self.major == 0
    }
}

use core::fmt;

impl fmt::Display for ListVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl fmt::Debug for ListVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <Self as fmt::Display>::fmt(self, f)
    }
}

#[cfg(feature = "stack")]
pub mod stack;
