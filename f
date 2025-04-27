
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
