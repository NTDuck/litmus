pub mod aliases {
    pub mod hash {
        #[cfg(not(any(feature = "ahash", feature = "fxhash", feature = "metrohash", feature = "seahash")))]
        pub type BuildHasher = ::std::hash::RandomState;

        #[cfg(feature = "ahash")]
        pub type BuildHasher = ::ahash::RandomState;

        #[cfg(feature = "fxhash")]
        pub type BuildHasher = ::fxhash::FxBuildHasher;

        #[cfg(feature = "metrohash")]
        pub type BuildHasher = ::metrohash::MetroBuildHasher;

        #[cfg(feature = "seahash")]
        pub type BuildHasher = ::std::hash::BuildHasherDefault<::seahash::SeaHasher>;
    }

    pub mod marker {
        pub type PhantomCovariant<State> = ::core::marker::PhantomData<State>;
    }

    pub mod string {
        pub type String = ::std::borrow::Cow<'static, str>;
    }

    pub mod sync {
        #[cfg(not(any(feature = "triomphe")))]
        pub type Arc<T> = ::std::sync::Arc<T>;

        #[deprecated = "Deprecated due to missing required trait implementation:\n```\nimpl<T, U, A> \
                        ::std::ops::CoerceUnsized<::triomphe::Arc<U, A>> for ::triomphe::Arc<T, A>\nwhere\nT: \
                        ::core::marker::Unsize<U> + ?::core::marker::Sized,\nA: ::std::alloc::Allocator,\nU: \
                        ?::core::marker::Sized,\n```\n\\"]
        #[cfg(feature = "triomphe")]
        pub type Arc<T> = ::triomphe::Arc<T>;
    }
}
