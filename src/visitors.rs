use crate::{utils::aliases::MaybeOwnedString, Background, Fallible, Scenario, Tags};

trait Executor<Unit> {
    type Output;

    fn execute(&self, unit: Unit) -> Self::Output;
}

struct LibtestMimicExecutor;

impl<Given, When, Then, World> Executor<Scenario<Given, When, Then>> for LibtestMimicExecutor
where
    Given: FnOnce() -> Fallible<World>,
    When: FnOnce(&mut World) -> Fallible,
    Then: FnOnce(&World) -> Fallible,
    World: ::core::marker::Send + ::core::marker::Sync,
{
    type Output = ::libtest_mimic::Trial;
    
    fn execute(&self, scenario: Scenario<Given, When, Then>) -> Self::Output {
        todo!()
    }
}

#[::bon::bon]
impl LibtestMimicExecutor {
    #[builder]
    fn execute<Callback>(&self, #[builder(start_fn)] callback: Callback, description: impl Into<MaybeOwnedString>, ignored: ::core::option::Option<impl Into<bool>>, tags: ::core::option::Option<impl Into<Tags>>) -> ::libtest_mimic::Trial {
        todo!()
    }
}
