macro_rules! make_list {
    ($ptr: ident; $(#[$listdoc:meta])*) => {
        $(
            #[$listdoc]
        )*
        pub struct List<T> {
            head: Option<$ptr<Node<T>>>,
        }

        struct Node<T> {
            elem: T,
            next: Option<$ptr<Node<T>>>,
        }
    };
}