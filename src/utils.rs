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

    pub mod sync {
        #[cfg(not(any(feature = "triomphe")))]
        pub type Arc<T> = ::std::sync::Arc<T>;

        #[cfg(feature = "triomphe")]
        pub type Arc<T> = ::triomphe::Arc<T>;
    }
}
