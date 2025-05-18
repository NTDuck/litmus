use crate::utils::aliases::MaybeOwnedStr;

use super::{GivenStepsOnce, Tags, ThenStepsOnce, WhenStepsOnce};

pub struct Scenario<WorldImpl> {
    pub(crate) description: Option<MaybeOwnedStr>,
    pub(crate) ignored: Option<bool>,
    pub(crate) tags: Option<Tags>,

    pub(crate) given_steps: GivenStepsOnce<WorldImpl>,
    pub(crate) when_steps: WhenStepsOnce<WorldImpl>,
    pub(crate) then_steps: ThenStepsOnce<WorldImpl>,
}
