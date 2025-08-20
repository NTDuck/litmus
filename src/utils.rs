pub mod aliases {
    pub mod borrow {
        pub type MaybeOwnedString = ::std::borrow::Cow<'static, str>;
    }

    pub mod boxed {
        /// See also: [Custom allocators](https://nical.github.io/posts/rust-custom-allocators.html)
        pub type Box<T> = ::std::boxed::Box<T>;
    }

    pub mod collections {
        #[cfg(all(feature = "ahash", not(feature = "fxhash")))]
        pub type Set<T> = ::ahash::AHashSet<T>;

        #[cfg(all(feature = "ahash", not(feature = "fxhash")))]
        pub type Map<K, V> = ::ahash::AHashMap<K, V>;

        #[cfg(all(not(feature = "ahash"), feature = "fxhash"))]
        pub type Set<T> = ::fxhash::FxHashSet<T>;

        #[cfg(all(not(feature = "ahash"), feature = "fxhash"))]
        pub type Map<K, V> = ::fxhash::FxHashMap<K, V>;

        #[cfg(not(any(feature = "ahash", feature = "fxhash")))]
        pub type Set<T> = ::std::collections::HashSet<T>;

        #[cfg(not(any(feature = "ahash", feature = "fxhash")))]
        pub type Map<K, V> = ::std::collections::HashMap<K, V>;
    }

    pub mod rc {
        pub type Rc<T> = ::std::rc::Rc<T>;
    }

    pub mod vec {
        pub type Vec<T> = ::std::vec::Vec<T>;
    }
}
