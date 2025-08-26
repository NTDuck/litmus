pub mod assertions;
pub mod builders;
pub mod models;
pub mod visitors;

mod utils;

pub use self::assertions::*;
pub use self::builders::*;
pub use self::models::*;
pub use self::visitors::*;

#[cfg(any(
    all(feature = "ahash", any(feature = "fxhash", feature = "metrohash", feature = "seahash")),
    all(feature = "fxhash", any(feature = "ahash", feature = "metrohash", feature = "seahash")),
    all(feature = "metrohash", any(feature = "ahash", feature = "fxhash", feature = "seahash")),
    all(feature = "seahash", any(feature = "ahash", feature = "fxhash", feature = "metrohash")),
))]
compile_error!("The following feature flags are mutually exclusive: `ahash`, `fxhash`, `metrohash`, `seahash`");
