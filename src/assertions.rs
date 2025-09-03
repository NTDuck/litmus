#[macro_export]
macro_rules! assert {
    ($expr:expr, $message:expr) => {{
        if $expr {
            ::core::result::Result::Ok(())
        } else {
            ::core::result::Result::Err($message.into())
        }
    }};

    ($expr:expr) => {
        $crate::assertions::assert!($expr, ::std::stringify!($expr))
    };
}

pub use assert;
