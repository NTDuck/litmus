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
        $crate::macros::assert!($expr, ::std::stringify!($expr))
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
        ::std::format!($fmt $(, $crate::macros::__Backtick($arg))*)
    };
}

#[macro_export]
macro_rules! r#async {
    () => {
        $crate::macros::r#async!({ ::core::result::Result::Ok(()) })
    };
    
    ($expr:expr) => {
        ::futures::future::FutureExt::boxed(async move { $expr })
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
pub use format;
pub use panic;
pub use r#async;