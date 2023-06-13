//! A singly linked list with stack operations.
use super::ListVersion;
use alloc::boxed::Box;
use core::iter::FusedIterator;

/// The `ListVersion` of this module. See [its documentation](ListVersion) for more information.
pub const VERSION: ListVersion = ListVersion {
    major: 1,
    minor: 2,
    patch: 0,
};

/// A singly linked list that can do (most of) what a stack can.
pub struct List<T> {
    head: Option<Box<Node<T>>>,
}

struct Node<T> {
    elem: T,
    next: Option<Box<Node<T>>>,
}

impl<T> List<T> {
    /// Creates a new list.
    pub fn new() -> Self {
        List { head: None }
    }

    /// Pushes an element onto the back of the list.
    pub fn push(&mut self, elem: T) {
        let new = Box::new(Node {
            elem,
            next: self.head.take(),
        });

        self.head = Some(new);
    }

    /// Removes the first element from the list.
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    /// Returns whether the list is empty.
    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    /// Removes all elements from the list.
    pub fn clear(&mut self) {
        let mut current = self.head.take();

        while let Some(mut node) = current {
            current = node.next.take();
        }
    }

    /// Returns a shared reference to the first element in the list.
    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    /// Returns a mutable reference to the first element in the list.
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
    }

    /// Creates an iterator over shared references to each element in the list.
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_deref(),
        }
    }

    /// Creates an iterator over mutable references to each element in the list.
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            next: self.head.as_deref_mut(),
        }
    }
}

macro_rules! into_iter_impl {
    ($type: ty, $item: ty, $intoiter: ty, $conv_fn: path) => {
        impl<'a, T> IntoIterator for $type {
            type Item = $item;
            type IntoIter = $intoiter;

            fn into_iter(self) -> $intoiter {
                $conv_fn(self)
            }
        }
    };
}

// This exists due to how into_iter_impl! takes a converter
fn list_into_iter<T>(list: List<T>) -> IntoIter<T> {
    IntoIter { list }
}

into_iter_impl!{List<T>, T, IntoIter<T>, list_into_iter}
into_iter_impl!{&'a List<T>, &'a T, Iter<'a, T>, List::iter}
into_iter_impl!{&'a mut List<T>, &'a mut T, IterMut<'a, T>, List::iter_mut}

impl<T> Default for List<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        self.clear()
    }
}

macro_rules! exact_size_iter_impl {
    ($type: ty) => {
        impl<'a, T> ExactSizeIterator for $type {
            fn len(&self) -> usize {
                self.size_hint().0
            }

            // NOTE:
            // Once https://github.com/rust-lang/rust/issues/35428 is stabilized,
            // get ExactSizeIterator::is_empty from List::is_empty or similar
        }
    }
}

/// An iterator that yields shared references to the elements of a list.
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let mut current = self.next;
        let mut len = 0;
        while let Some(node) = current {
            current = node.next.as_deref();
            len += 1;
        }
        (len, Some(len))
    }
}

impl<'a, T> FusedIterator for Iter<'a, T> {}

exact_size_iter_impl!{Iter<'a, T>}

/// An iterator that yields mutable references to the elements of a list.
pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<'a, T> IterMut<'a, T> {
    // Allow this because it makes it clearer that the Iter lives for 'b and not 'a.
    #[allow(clippy::needless_lifetimes)]
    fn as_iter<'b>(&'b self) -> Iter<'b, T> {
        Iter {
            next: self.next.as_deref(),
        }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.as_iter().size_hint()
    }
}

impl<'a, T> FusedIterator for IterMut<'a, T> {}

exact_size_iter_impl!{IterMut<'a, T>}

/// An iterator that consumes a list and yields its elements.
pub struct IntoIter<T> {
    list: List<T>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.list.pop()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.list.iter().size_hint()
    }
}

impl<T> FusedIterator for IntoIter<T> {}

exact_size_iter_impl!{IntoIter<T>}

#[cfg(test)]
mod tests {
    use super::List;

    #[test]
    fn push_pop() {
        let mut list = List::new();
        assert_eq!(list.pop(), None);

        list.push(1);
        list.push(2);

        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn peek() {
        let mut list = List::new();
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);

        list.push(1);
        list.push(2);

        assert_eq!(list.peek(), Some(&2));
        assert_eq!(list.peek_mut(), Some(&mut 2));

        list.peek_mut().map(|val| *val = 42);
        assert_eq!(list.peek(), Some(&42));
        assert_eq!(list.pop(), Some(42));
    }

    #[test]
    fn is_empty() {
        let mut list = List::new();
        assert!(list.is_empty());

        list.push(1);
        assert!(!list.is_empty());

        list.pop();
        assert!(list.is_empty());
    }

    #[test]
    fn clear() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.clear();
        assert!(list.is_empty());
        assert!(list.pop().is_none());
    }

    #[test]
    fn iter() {
        let mut list = List::new();

        list.push(1);
        list.push(2);

        let mut iter = list.iter();

        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter_mut() {
        let mut list = List::new();

        list.push(1);
        list.push(2);

        let iter = list.iter_mut();

        for elem in iter {
            *elem += 10;
        }

        let mut iter = list.iter_mut();

        assert_eq!(iter.next(), Some(&mut 12));
        assert_eq!(iter.next(), Some(&mut 11));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();

        list.push(1);
        list.push(2);

        let mut iter = list.into_iter();

        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn for_loop() {
        let mut list = List::new();

        list.push(1);
        list.push(2);
        list.push(3);

        let mut i = 3;

        // Remember which way the iteration goes
        for elem in list {
            assert_eq!(elem, i);
            i -= 1;
        }
    }
}
