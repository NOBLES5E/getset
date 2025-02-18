#[macro_use]
extern crate getset_scoped;

use crate::submodule::other::{Generic, Plain, Where};

// For testing `pub(super)`
mod submodule {
    // For testing `pub(super::other)`
    pub mod other {
        #[derive(Getters)]
        #[get]
        pub struct Plain {
            /// A doc comment.
            /// Multiple lines, even.
            private_accessible: usize,

            /// A doc comment.
            #[get = "pub"]
            public_accessible: usize,
            // /// A doc comment.
            // #[get = "pub(crate)"]
            // crate_accessible: usize,

            // /// A doc comment.
            // #[get = "pub(super)"]
            // super_accessible: usize,

            // /// A doc comment.
            // #[get = "pub(super::other)"]
            // scope_accessible: usize,
        }

        impl Default for Plain {
            fn default() -> Plain {
                Plain {
                    private_accessible: 17,
                    public_accessible: 18,
                }
            }
        }

        #[derive(Getters, Default)]
        #[get]
        pub struct Generic<T: Copy + Clone + Default> {
            /// A doc comment.
            /// Multiple lines, even.
            private_accessible: T,

            /// A doc comment.
            #[get = "pub(crate)"]
            public_accessible: T,
            // /// A doc comment.
            // #[get = "pub(crate)"]
            // crate_accessible: usize,

            /// A doc comment.
            #[get = "pub(super)"]
            super_accessible: usize,

            /// A doc comment.
            #[get = "pub(in crate)"]
            path_accessible: usize,

            // /// A doc comment.
            // #[get = "pub(super::other)"]
            // scope_accessible: usize,
        }

        #[derive(Getters, Default)]
        #[get]
        pub struct Where<T>
        where
            T: Copy + Clone + Default,
        {
            /// A doc comment.
            /// Multiple lines, even.
            private_accessible: T,

            /// A doc comment.
            #[get = "pub"]
            public_accessible: T,
            // /// A doc comment.
            // #[get = "pub(crate)"]
            // crate_accessible: usize,

            // /// A doc comment.
            // #[get = "pub(super)"]
            // super_accessible: usize,

            // /// A doc comment.
            // #[get = "pub(super::other)"]
            // scope_accessible: usize,
        }

        #[test]
        fn test_plain() {
            let val = Plain::default();
            val.private_accessible();
        }

        #[test]
        fn test_generic() {
            let val = Generic::<usize>::default();
            val.private_accessible();
        }

        #[test]
        fn test_where() {
            let val = Where::<usize>::default();
            val.private_accessible();
        }
    }
}

#[test]
fn test_plain() {
    let val = Plain::default();
    assert_eq!(18, *val.public_accessible());
}

#[test]
fn test_generic() {
    let val = Generic::<usize>::default();
    assert_eq!(usize::default(), *val.public_accessible());
}

#[test]
fn test_where() {
    let val = Where::<usize>::default();
    assert_eq!(usize::default(), *val.public_accessible());
}
