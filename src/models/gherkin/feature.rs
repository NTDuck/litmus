use crate::utils::aliases::MaybeOwnedStr;

use super::{Hook, Rule, Scenario, Tags};

pub struct Feature<WorldImpl> {
    pub(crate) description: Option<MaybeOwnedStr>,
    pub(crate) ignored: Option<bool>,
    pub(crate) tags: Option<Tags>,

    pub(crate) scenarios: Option<Vec<Scenario<WorldImpl>>>,
    pub(crate) rules: Option<Vec<Rule<WorldImpl>>>,
}
