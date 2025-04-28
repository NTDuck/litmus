use std::ops::Deref;

use crate::utils::aliases::{Arc, Box, MaybeOwnedStr};

use super::World;

pub struct Steps<StepFnImpl: ?Sized>(Vec<Step<StepFnImpl>>);

impl<StepFnImpl> Steps<StepFnImpl> {
    pub(crate) fn with(self, step: Step<StepFnImpl>) -> Self {
        let mut steps = self.0;
        steps.push(step);

        Self(steps)
    }
}

impl<StepFnImpl> From<Step<StepFnImpl>> for Steps<StepFnImpl> {
    fn from(step: Step<StepFnImpl>) -> Self {
        let mut steps = Vec::new();
        steps.push(step);

        Self(steps)
    }
}

impl<StepFnImpl> Deref for Steps<StepFnImpl> {
    type Target = Vec<Step<StepFnImpl>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub type GivenSteps<WorldImpl> = Steps<dyn GivenStepFn<WorldImpl>>;
pub type WhenSteps<WorldImpl> = Steps<dyn WhenStepFn<WorldImpl>>;
pub type ThenSteps<WorldImpl> = Steps<dyn ThenStepFn<WorldImpl>>;

pub struct StepsOnce<StepFnOnceImpl: ?Sized>(Vec<StepOnce<StepFnOnceImpl>>);

impl<StepFnOnceImpl> StepsOnce<StepFnOnceImpl> {
    pub(crate) fn with(self, step: StepOnce<StepFnOnceImpl>) -> Self {
        let mut steps = self.0;
        steps.push(step);

        Self(steps)
    }
}

impl<StepFnOnceImpl> From<StepOnce<StepFnOnceImpl>> for StepsOnce<StepFnOnceImpl> {
    fn from(step: StepOnce<StepFnOnceImpl>) -> Self {
        let mut steps = Vec::new();
        steps.push(step);

        Self(steps)
    }
}

impl<StepFnOnceImpl> Deref for StepsOnce<StepFnOnceImpl> {
    type Target = Vec<StepOnce<StepFnOnceImpl>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub type GivenStepsOnce<WorldImpl> = StepsOnce<dyn GivenStepFnOnce<WorldImpl>>;
pub type WhenStepsOnce<WorldImpl> = StepsOnce<dyn WhenStepFnOnce<WorldImpl>>;
pub type ThenStepsOnce<WorldImpl> = StepsOnce<dyn ThenStepFnOnce<WorldImpl>>;

pub struct Step<StepFnImpl: ?Sized> {
    pub(crate) description: MaybeOwnedStr,
    pub(crate) label: StepLabel,
    pub(crate) callback: Arc<StepFnImpl>,
}

pub type GivenStep<WorldImpl> = Step<dyn GivenStepFn<WorldImpl>>;
pub type WhenStep<WorldImpl> = Step<dyn WhenStepFn<WorldImpl>>;
pub type ThenStep<WorldImpl> = Step<dyn ThenStepFn<WorldImpl>>;

pub struct StepOnce<StepFnOnceImpl: ?Sized> {
    pub(crate) description: MaybeOwnedStr,
    pub(crate) label: StepLabel,    
    pub(crate) callback: Box<StepFnOnceImpl>,
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
