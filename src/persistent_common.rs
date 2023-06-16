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
        }
    }
}
