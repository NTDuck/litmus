use crate::utils::aliases::MaybeOwnedStr;

pub type Result = std::result::Result<(), Error>;

pub struct Error {
    pub message: Option<MaybeOwnedStr>,
}

impl<U> From<U> for Error
where
    U: Into<MaybeOwnedStr>,
{
    fn from(message: U) -> Self {
        Self {
            message: Some(message.into()),
        }
    }
}
