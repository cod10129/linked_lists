//! A crate containing many types of linked lists.

#![no_std]
#![warn(missing_docs)]
#![deny(unsafe_code)]

extern crate alloc;
use core::fmt;

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
    pub patch: u16,
}

impl ListVersion {
    /// Gets the effective major version.
    /// By SemVer spec, in `0.x.y` version, minor should be considered the "major" version.
    #[inline]
    fn get_maj(self) -> u16 {
        if self.experimental() { self.minor }
        else { self.major }
    }

    /// Gets the effective minor version.
    /// By SemVer spec, in `0.x.y` versions, patch should be considered the "minor" version.
    #[inline]
    fn get_min(self) -> u16 {
        if self.experimental() { self.patch }
        else { self.minor }
    }

    /// Returns whether this list is considered "experimental" or not.
    ///
    /// An experimental list can have breaking changes in non-breaking crate releases.
    #[inline]
    pub fn experimental(self) -> bool {
        self.major == 0
    }

    /// Returns whether `self` could be used when `other` was the specified version.
    /// 
    /// # Examples
    /// 
    /// ```
    /// use linked_lists::ListVersion;
    /// 
    /// let version = ListVersion {
    ///     major: 1,
    ///     minor: 2,
    ///     patch: 3,
    /// };
    /// 
    /// let expected = linked_lists::ListVersion {
    ///     major: 1,
    ///     minor: 0,
    ///     patch: 0
    /// };
    /// 
    /// assert!(version.compatible(expected));
    /// ```
    pub fn compatible(self, other: ListVersion) -> bool {
        if self.get_maj() == other.get_maj() && self.get_min() > other.get_min() {
            true
        } else {
            self.get_min() == other.get_min() && self.patch >= other.patch
        }
    }
}

macro_rules! version {
    ($major: literal, $minor: literal, $patch: literal) => {
        use crate::ListVersion;
        /// The `ListVersion` of this module. See [its documentation](ListVersion) for more information.
        pub const VERSION: ListVersion = ListVersion {
            major: $major,
            minor: $minor,
            patch: $patch
        };
    };
}

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

#[macro_use]
#[cfg(any(feature = "persistent", feature = "persistent_arc"))]
mod persistent_common;

#[cfg(feature = "persistent")]
pub mod persistent;

#[cfg(feature = "persistent_arc")]
pub mod persistent_arc;

#[cfg(test)]
mod tests {
    use super::ListVersion;

    #[test]
    fn compatibility() {
        let experimental = ListVersion {
            major: 0,
            minor: 1,
            patch: 4,
        };

        let less_experimental = ListVersion {
            major: 0,
            minor: 1,
            patch: 77,
        };

        assert!(less_experimental.compatible(experimental));

        let one = ListVersion {
            major: 1,
            minor: 3,
            patch: 4,
        };

        assert!(!one.compatible(experimental));

        let two = ListVersion {
            major: 2,
            minor: 0,
            patch: 0,
        };

        assert!(!one.compatible(two));
        assert!(!two.compatible(one));
    }
}
