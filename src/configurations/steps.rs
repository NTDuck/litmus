use crate::utils::aliases::{Box, MaybeOwnedStr};

pub struct Step {
    pub(crate) description: MaybeOwnedStr,
    pub(crate) label: StepLabel,    
    pub(crate) callback: Box<Callback>,
}

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
{
}

impl<T, WorldImpl> GivenStepFn<WorldImpl> for T
where
    T: Fn(&mut WorldImpl) -> Result<(), libtest::Failed> + Send + Sync + 'static,
{
}

pub trait WhenStepFn<WorldImpl>:
    Fn(&mut WorldImpl) -> Result<(), libtest::Failed> + Send + Sync + 'static
{
}

impl<T, WorldImpl> WhenStepFn<WorldImpl> for T
where
    T: Fn(&mut WorldImpl) -> Result<(), libtest::Failed> + Send + Sync + 'static,
{
}

pub trait ThenStepFn<WorldImpl>:
    Fn(&WorldImpl) -> Result<(), libtest::Failed> + Send + Sync + 'static
{
}

impl<T, WorldImpl> ThenStepFn<WorldImpl> for T
where
    T: Fn(&WorldImpl) -> Result<(), libtest::Failed> + Send + Sync + 'static,
{
}

pub trait GivenStepFnOnce<WorldImpl>:
    FnOnce(&mut WorldImpl) -> Result<(), libtest::Failed> + Send + Sync + 'static
{
}

impl<T, WorldImpl> GivenStepFnOnce<WorldImpl> for T
where
    T: FnOnce(&mut WorldImpl) -> Result<(), libtest::Failed> + Send + Sync + 'static,
{
}

pub trait WhenStepFnOnce<WorldImpl>:
    FnOnce(&mut WorldImpl) -> Result<(), libtest::Failed> + Send + Sync + 'static
{
}

impl<T, WorldImpl> WhenStepFnOnce<WorldImpl> for T
where
    T: FnOnce(&mut WorldImpl) -> Result<(), libtest::Failed> + Send + Sync + 'static,
{
}

pub trait ThenStepFnOnce<WorldImpl>:
    FnOnce(&WorldImpl) -> Result<(), libtest::Failed> + Send + Sync + 'static
{
}

impl<T, WorldImpl> ThenStepFnOnce<WorldImpl> for T
where
    T: FnOnce(&WorldImpl) -> Result<(), libtest::Failed> + Send + Sync + 'static,
{
}
