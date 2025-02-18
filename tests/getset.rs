#[macro_use]
extern crate getset_scoped;

use crate::submodule::other::{Generic, Plain, Where};

// For testing `pub(super)`
mod submodule {
    // For testing `pub(in super::other)`
    pub mod other {
        #[derive(MutGetters, Getters, Default)]
        #[getset(get_mut)]
        pub struct Plain {
            /// A doc comment.
            /// Multiple lines, even.
            private_accessible: usize,

            /// A doc comment.
            #[getset(get = "pub", get_mut = "pub")]
            public_accessible: usize,
            // /// A doc comment.
            // #[get_mut = "pub(crate)"]
            // crate_accessible: usize,

            // /// A doc comment.
            // #[get_mut = "pub(super)"]
            // super_accessible: usize,

            // /// A doc comment.
            // #[get_mut = "pub(in super::other)"]
            // scope_accessible: usize,
        }

        #[derive(MutGetters, Getters, Default)]
        #[getset(get_mut)]
        pub struct Generic<T: Copy + Clone + Default> {
            /// A doc comment.
            /// Multiple lines, even.
            private_accessible: T,

            /// A doc comment.
            #[getset(get = "pub", get_mut = "pub")]
            public_accessible: T,
            // /// A doc comment.
            // #[get_mut = "pub(crate)"]
            // crate_accessible: usize,

            // /// A doc comment.
            // #[get_mut = "pub(super)"]
            // super_accessible: usize,

            // /// A doc comment.
            // #[get_mut = "pub(in super::other)"]
            // scope_accessible: usize,
        }

        #[derive(MutGetters, Getters, Default)]
        #[getset(get_mut)]
        pub struct Where<T>
        where
            T: Copy + Clone + Default,
        {
            /// A doc comment.
            /// Multiple lines, even.
            private_accessible: T,

            /// A doc comment.
            #[getset(get = "pub", get_mut = "pub")]
            public_accessible: T,
            // /// A doc comment.
            // #[get_mut = "pub(crate)"]
            // crate_accessible: usize,

            // /// A doc comment.
            // #[get_mut = "pub(super)"]
            // super_accessible: usize,

            // /// A doc comment.
            // #[get_mut = "pub(in super::other)"]
            // scope_accessible: usize,
        }

        #[test]
        fn test_plain() {
            let mut val = Plain::default();
            (*val.private_accessible_mut()) += 1;
        }

        #[test]
        fn test_generic() {
            let mut val = Generic::<usize>::default();
            (*val.private_accessible_mut()) += 1;
        }

        #[test]
        fn test_where() {
            let mut val = Where::<usize>::default();
            (*val.private_accessible_mut()) += 1;
        }
    }
}

#[test]
fn test_plain() {
    let mut val = Plain::default();
    let _ = val.public_accessible();
    (*val.public_accessible_mut()) += 1;
}

#[test]
fn test_generic() {
    let mut val = Generic::<usize>::default();
    let _ = val.public_accessible();
    (*val.public_accessible_mut()) += 1;
}

#[test]
fn test_where() {
    let mut val = Where::<usize>::default();
    let _ = val.public_accessible();
    (*val.public_accessible_mut()) += 1;
}
