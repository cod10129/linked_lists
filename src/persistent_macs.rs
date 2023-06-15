macro_rules! make_list_def {
    ($ptr: ident; $(#[$doc:meta])*) => {
        $(
            #[$doc]
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