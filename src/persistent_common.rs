macro_rules! make_list {
    ($ptr: ident; $(#[$listdoc:meta])*) => {
        $(#[$listdoc])*
        pub struct List<T> {
            head: Option<$ptr<Node<T>>>,
        }

        struct Node<T> {
            elem: T,
            next: Option<$ptr<Node<T>>>,
        }

        list_impl!{$ptr}
        make_iter!{}
    };
}

macro_rules! list_impl {
    ($ptr: ident) => {
        impl<T> List<T> {
            /// Creates a new list.
            pub fn new() -> Self {
                Self { head: None }
            }

            /// Returns whether the list is empty.
            pub fn is_empty(&self) -> bool {
                self.head.is_none()
            }

            /// Returns the length of the list.
            pub fn len(&self) -> usize {
                self.iter().len()
            }

            /// Prepends an element to the front of the list, returning the new list.
            pub fn prepend(&self, elem: T) -> Self {
                Self { head: Some($ptr::new(Node {
                    elem,
                    next: self.head.clone()
                }))}
            }

            /// Returns the list with the first node removed.
            pub fn tail(&self) -> Self {
                Self {
                    head: self.head.as_ref().and_then(|node| node.next.clone())
                }
            }

            /// Returns a reference to the first element in the list, if it exists.
            pub fn head(&self) -> Option<&T> {
                self.head.as_ref().map(|node| &node.elem)
            }

            /// Creates an iterator that yields shared references to each element in the list.
            pub fn iter(&self) -> Iter<'_, T> {
                Iter { next: self.head.as_deref() }
            }
        }

        impl<T> Clone for List<T> {
            fn clone(&self) -> Self {
                Self { head: self.head.clone() }
            }
        }

        impl<T> Default for List<T> {
            fn default() -> Self {
                Self::new()
            }
        }

        impl<T> Drop for List<T> {
            fn drop(&mut self) {
                let mut cur = self.head.take();
                while let Some(node) = cur {
                    if let Some(mut node) = $ptr::into_inner(node) {
                        cur = node.next.take();
                    } else {
                        break;
                    }
                }
            }
        }
    };
}

macro_rules! make_iter {
    () => {
        /// An iterator that yields shared references to the elements of a list.
        pub struct Iter<'a, T> {
            next: Option<&'a Node<T>>
        }

        impl<'a, T> Iterator for Iter<'a, T> {
            type Item = &'a T;

            fn next(&mut self) -> Option<Self::Item> {
                self.next.map(|node| {
                    self.next = node.next.as_deref();
                    &node.elem
                })
            }

            fn size_hint(&self) -> (usize, Option<usize>) {
                let mut len = 0;
                let mut current = self.next;
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
        }
    };
}

macro_rules! tests {
    () => {
        #[cfg(test)]
        mod tests {
            use super::List;
            #[test]
            fn prepend_tail_head() {
                let list = List::new().prepend(1).prepend(2);

                assert_eq!(list.head(), Some(&2));
                let list = list.tail();

                assert_eq!(list.head(), Some(&1));
                let list = list.tail();
                
                assert_eq!(list.head(), None);
                let list = list.tail();

                assert_eq!(list.head(), None);
            }

            #[test]
            fn is_empty() {
                let list = List::new();
                assert!(list.is_empty());

                let list = list.prepend(1);
                assert!(!list.is_empty());

                let list = list.tail();
                assert!(list.is_empty());
            }

            #[test]
            fn len() {
                let list = List::new();
                assert_eq!(list.len(), 0);

                let list = list.prepend(1).prepend(2);
                assert_eq!(list.len(), 2);

                let list = list.tail();
                assert_eq!(list.len(), 1);
            }

            #[test]
            fn iter() {
                let list = List::new().prepend(1).prepend(2);

                let mut iter = list.iter();

                assert_eq!(iter.next(), Some(&2));
                assert_eq!(iter.next(), Some(&1));
                assert_eq!(iter.next(), None);
            }
        }
    }
}
