#[macro_export]
macro_rules! assert {
    ($expr:expr, $message:expr) => {{
        if $expr {
            ::core::result::Result::Ok(())
        } else {
            ::core::result::Result::Err(::litmus::builders::models::IntoFailed::into_failed($message))
        }
    }};

    ($expr:expr) => {
        $crate::assertions::assert!($expr, ::std::stringify!($expr))
    };
}

#[macro_export]
macro_rules! panic {
    ($message:expr) => {
        ::core::result::Result::Err(::litmus::builders::models::IntoFailed::into_failed($message))
    };
}

/// Gives nice backticks
#[macro_export]
macro_rules! format {
    ($fmt:literal $(, $arg:expr)+ $(,)?) => {
        ::std::format!($fmt $(, $crate::assertions::__Backtick($arg))*)
    };
}


pub struct __Backtick<T>(pub T);

impl<T> ::core::fmt::Display for __Backtick<T>
where
    T: ::core::fmt::Display,
{
    fn fmt(&self, formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        ::core::write!(formatter, "`{}`", self.0)
    }
}

pub use assert;
pub use panic;
pub use format;
