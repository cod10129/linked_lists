//! A singly linked list with stack operations.
use alloc::boxed::Box;
use super::ListVersion;

/// The `ListVersion` of this module. See [its documentation](ListVersion) for more information.
pub const VERSION: ListVersion = ListVersion {
    major: 1,
    minor: 0,
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

impl<'a, T> ExactSizeIterator for Iter<'a, T> {
    fn len(&self) -> usize {
        self.size_hint().0
    }

    // NOTE:
    // Once https://github.com/rust-lang/rust/issues/35428 is stabilized,
    // uncomment this fn
    /*
    fn is_empty(&self) -> bool {
        self.next.is_none()
    }
    */
}

/// An iterator that yields mutable references to the elements of a list.
pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<'a, T> IterMut<'a, T> {
    #[allow(clippy::needless_lifetimes)]
    fn as_iter<'b>(&'b self) -> Iter<'b, T> {
        Iter {
            next: self.next.as_deref()
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

impl<'a, T> ExactSizeIterator for IterMut<'a, T> {
    fn len(&self) -> usize {
        self.size_hint().0
    }

    // NOTE:
    // Once https://github.com/rust-lang/rust/issues/35428 is stabilized,
    // uncomment this fn
    /*
    fn is_empty(&self) -> bool {
        self.next.is_none()
    }
    */
}

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
}
