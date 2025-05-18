use crate::{utils::aliases::{Arc, MaybeOwnedStr}, Result, World};

pub struct Hooks<WorldImpl> {
    pub(crate) before_scenario: Option<Vec<Hook<WorldImpl>>>,
    pub(crate) after_scenario: Option<Vec<Hook<WorldImpl>>>,
    pub(crate) before_step: Option<Vec<Hook<WorldImpl>>>,
    pub(crate) after_step: Option<Vec<Hook<WorldImpl>>>,
}

pub struct Hook<WorldImpl> {
    pub(crate) description: MaybeOwnedStr,
    pub(crate) callback: Arc<dyn HookFn<WorldImpl>>,
}

pub trait HookFn<WorldImpl>:
    Fn(&mut WorldImpl) -> Result + Send + Sync + 'static
where
    WorldImpl: World,
{
}

impl<T, WorldImpl> HookFn<WorldImpl> for T
where
    T: Fn(&mut WorldImpl) -> Result + Send + Sync + 'static,
    WorldImpl: World,
{
}
