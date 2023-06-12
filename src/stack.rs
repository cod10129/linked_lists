//! A singly linked list, with stack operations.
use alloc::boxed::Box;

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

    /// Pushes an element onto the list.
    pub fn push(&mut self, elem: T) {
        let new = Box::new(Node {
            elem,
            next: self.head.take(),
        });

        self.head = Some(new);
    }

    /// Removes an element from the list.
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }
}

impl<T> Default for List<T> {
    fn default() -> Self {
        Self::new()
    }
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
}
