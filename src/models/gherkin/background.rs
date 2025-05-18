use crate::utils::aliases::MaybeOwnedStr;

use super::GivenSteps;

pub struct Background<WorldImpl> {
    pub(crate) description: Option<MaybeOwnedStr>,
    pub(crate) ignored: Option<bool>,

    pub(crate) steps: Option<GivenSteps<WorldImpl>>,
}
