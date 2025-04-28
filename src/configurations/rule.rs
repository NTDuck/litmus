use crate::utils::aliases::MaybeOwnedStr;

use super::{Scenario, Tags};

pub struct Rule<WorldImpl> {
    pub(crate) description: Option<MaybeOwnedStr>,
    pub(crate) ignored: Option<bool>,
    pub(crate) tags: Option<Tags>,

    pub(crate) scenarios: Option<Vec<Scenario<WorldImpl>>>,
}
