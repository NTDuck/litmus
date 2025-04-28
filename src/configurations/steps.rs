use crate::utils::aliases::{Arc, Box, MaybeOwnedStr};

use super::World;

pub struct Step<Callback: ?Sized> {
    pub(crate) description: MaybeOwnedStr,
    pub(crate) label: StepLabel,
    pub(crate) callback: Arc<Callback>,
}

pub type GivenStep<WorldImpl> = Step<dyn GivenStepFn<WorldImpl>>;
pub type WhenStep<WorldImpl> = Step<dyn WhenStepFn<WorldImpl>>;
pub type ThenStep<WorldImpl> = Step<dyn ThenStepFn<WorldImpl>>;

pub struct StepOnce<Callback: ?Sized> {
    pub(crate) description: MaybeOwnedStr,
    pub(crate) label: StepLabel,    
    pub(crate) callback: Box<Callback>,
}

pub type GivenStepOnce<WorldImpl> = StepOnce<dyn GivenStepFnOnce<WorldImpl>>;
pub type WhenStepOnce<WorldImpl> = StepOnce<dyn WhenStepFnOnce<WorldImpl>>;
pub type ThenStepOnce<WorldImpl> = StepOnce<dyn ThenStepFnOnce<WorldImpl>>;

#[derive(strum::Display)]
pub enum StepLabel {
    Given,
    When,
    Then,

    And,
    But,
}

pub trait GivenStepFn<WorldImpl>:
    Fn(&mut WorldImpl) -> Result<(), libtest::Failed> + Send + Sync + 'static
where
    WorldImpl: World,
{
}

impl<T, WorldImpl> GivenStepFn<WorldImpl> for T
where
    T: Fn(&mut WorldImpl) -> Result<(), libtest::Failed> + Send + Sync + 'static,
    WorldImpl: World,
{
}

pub trait WhenStepFn<WorldImpl>:
    Fn(&mut WorldImpl) -> Result<(), libtest::Failed> + Send + Sync + 'static
where
    WorldImpl: World,
{
}

impl<T, WorldImpl> WhenStepFn<WorldImpl> for T
where
    T: Fn(&mut WorldImpl) -> Result<(), libtest::Failed> + Send + Sync + 'static,
    WorldImpl: World,
{
}

pub trait ThenStepFn<WorldImpl>:
    Fn(&WorldImpl) -> Result<(), libtest::Failed> + Send + Sync + 'static
where
    WorldImpl: World,
{
}

impl<T, WorldImpl> ThenStepFn<WorldImpl> for T
where
    T: Fn(&WorldImpl) -> Result<(), libtest::Failed> + Send + Sync + 'static,
    WorldImpl: World,
{
}

pub trait GivenStepFnOnce<WorldImpl>:
    FnOnce(&mut WorldImpl) -> Result<(), libtest::Failed> + Send + Sync + 'static
where
    WorldImpl: World,
{
}

impl<T, WorldImpl> GivenStepFnOnce<WorldImpl> for T
where
    T: FnOnce(&mut WorldImpl) -> Result<(), libtest::Failed> + Send + Sync + 'static,
    WorldImpl: World,
{
}

pub trait WhenStepFnOnce<WorldImpl>:
    FnOnce(&mut WorldImpl) -> Result<(), libtest::Failed> + Send + Sync + 'static
where
    WorldImpl: World,
{
}

impl<T, WorldImpl> WhenStepFnOnce<WorldImpl> for T
where
    T: FnOnce(&mut WorldImpl) -> Result<(), libtest::Failed> + Send + Sync + 'static,
    WorldImpl: World,
{
}

pub trait ThenStepFnOnce<WorldImpl>:
    FnOnce(&WorldImpl) -> Result<(), libtest::Failed> + Send + Sync + 'static
where
    WorldImpl: World,
{
}

impl<T, WorldImpl> ThenStepFnOnce<WorldImpl> for T
where
    T: FnOnce(&WorldImpl) -> Result<(), libtest::Failed> + Send + Sync + 'static,
    WorldImpl: World,
{
}
