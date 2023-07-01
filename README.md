# linked_lists
A crate containing many different types of linked lists.

## no_std:
`linked_lists` is no_std. However, it still requires [`alloc`](https://doc.rust-lang.org/alloc/).

## Currently existing lists:
- `stack` (*[1.3.0][stackversion]*): A singly linked list with stack operations.
- `persistent` (*[1.0.0][persistentversion]*): A persistent immutable non-thread safe list.
- `persistent_arc` (*[1.0.0][persistentarcversion]*): A persistent immutable thread safe list.

[stackversion]: https://docs.rs/linked_lists/0.1.6/linked_lists/stack/constant.VERSION.html
[persistentversion]: https://docs.rs/linked_lists/0.1.6/linked_lists/persistent/constant.VERSION.html
[persistentarcversion]: https://docs.rs/linked_lists/0.1.6/linked_lists/persistent_arc/constant.VERSION.html
