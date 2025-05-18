use crate::utils::aliases::MaybeOwnedStr;

pub type Result = std::result::Result<(), Error>;

pub struct Error {
    message: Option<MaybeOwnedStr>,
}
