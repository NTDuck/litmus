#[macro_export]
macro_rules! assert {
    ($expr:expr, $message:expr) => {{
        if $expr {
            ::core::result::Result::Ok(())
        } else {
            // qualify?
            ::core::result::Result::Err($message.into_failed())
        }
    }};

    ($expr:expr) => {
        $crate::assertions::assert!($expr, ::std::stringify!($expr))
    };
}

pub use assert;
