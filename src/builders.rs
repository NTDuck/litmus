use ::sealed::sealed;

use crate::{utils::aliases::{MaybeOwnedString, Vec}, Background, Fallible, Scenario, StepLabel, Steps};

pub struct BackgroundBuilder<Given, State: self::background::State = self::background::Empty> {
    _phantom: ::core::marker::PhantomData<(
        PhantomCovariant<State>,
        PhantomCovariant<Steps<Given>>,
        PhantomCovariant<Given>,
    )>,

    description: ::core::option::Option<MaybeOwnedString>,
    ignored: ::core::option::Option<bool>,
    given: ::core::option::Option<Steps<Given>>,
}

impl<Given> Background<Given> {
    pub fn builder() -> BackgroundBuilder<Given> {
        BackgroundBuilder {
            _phantom: ::core::default::Default::default(),

            description: ::core::default::Default::default(),
            ignored: ::core::default::Default::default(),
            given: ::core::default::Default::default(),
        }
    }
}

impl<Given, State: self::background::State> BackgroundBuilder<Given, State>
where
    State::Given: self::background::IsUnset,
{
    pub fn description(self, value: impl Into<MaybeOwnedString>) -> BackgroundBuilder<Given, self::background::SetDescription<State>>
    where
        State::Description: self::background::IsUnset,
    {
        BackgroundBuilder {
            _phantom: ::core::default::Default::default(),

            description: ::core::option::Option::from(value.into()),
            ignored: self.ignored,
            given: self.given,
        }
    }

    pub fn ignored(self, value: impl Into<bool>) -> BackgroundBuilder<Given, self::background::SetIgnored<State>>
    where
        State::Ignored: self::background::IsUnset,
    {
        BackgroundBuilder {
            _phantom: ::core::default::Default::default(),

            description: self.description,
            ignored: ::core::option::Option::from(value.into()),
            given: self.given,
        }
    }

    pub fn given<World>(self, description: impl Into<MaybeOwnedString>, callback: Given) -> BackgroundBuilder<Given, self::background::SetGiven<State>>
    where
        State::Given: self::background::IsUnset,
        Given: FnOnce() -> Fallible<World> + ::core::clone::Clone,
        World: ::core::marker::Send + ::core::marker::Sync,
    {
        let given = Steps::builder()
            .label(StepLabel::Given)
            .description(description)
            .callback(callback)
            .build();

        BackgroundBuilder {
            _phantom: ::core::default::Default::default(),

            description: self.description,
            ignored: self.ignored,
            given: ::core::option::Option::from(given),
        }
    }
}

#[::bon::bon]
impl<Given, State: self::background::State> BackgroundBuilder<Given, self::background::SetGiven<State>>
where
    <self::background::SetGiven<State> as self::background::State>::Given: self::background::IsSet,
{
    pub fn and<World>(self, description: impl Into<MaybeOwnedString>, callback: impl FnOnce(&mut World) -> Fallible + ::core::clone::Clone) -> BackgroundBuilder<impl FnOnce() -> Fallible<World> + ::core::clone::Clone, self::background::SetGiven<State>>
    where
        Given: FnOnce() -> Fallible<World> + ::core::clone::Clone,
        World: ::core::marker::Send + ::core::marker::Sync,
    {
        self.conjoin(description, callback)
            .label(StepLabel::And)
            .call()
    }

    pub fn but<World>(self, description: impl Into<MaybeOwnedString>, callback: impl FnOnce(&mut World) -> Fallible + ::core::clone::Clone) -> BackgroundBuilder<impl FnOnce() -> Fallible<World> + ::core::clone::Clone, self::background::SetGiven<State>>
    where
        Given: FnOnce() -> Fallible<World> + ::core::clone::Clone,
        World: ::core::marker::Send + ::core::marker::Sync,
    {
        self.conjoin(description, callback)
            .label(StepLabel::But)
            .call()
    }

    #[builder]
    fn conjoin<World>(mut self, #[builder(start_fn)] description: impl Into<MaybeOwnedString>, #[builder(start_fn)] callback: impl FnOnce(&mut World) -> Fallible + ::core::clone::Clone, label: StepLabel) -> BackgroundBuilder<impl FnOnce() -> Fallible<World> + ::core::clone::Clone, self::background::SetGiven<State>>
    where
        Given: FnOnce() -> Fallible<World> + ::core::clone::Clone,
        World: ::core::marker::Send + ::core::marker::Sync,
    {
        let given = unsafe { self.given.take().unwrap_unchecked() };
        let given = Steps::builder()
            .labels(given.labels)
            .label(label)
            .descriptions(given.descriptions)
            .description(description)
            .callback(move || {
                let mut world = (given.callback)()?;
                (callback)(&mut world)?;
                Ok(world)
            })
            .build();

        BackgroundBuilder {
            _phantom: ::core::default::Default::default(),

            description: self.description,
            ignored: self.ignored,
            given: ::core::option::Option::from(given),
        }
    }
}

impl<Given, State: self::background::State> BackgroundBuilder<Given, State>
where
    State: self::background::IsComplete,
{
    pub fn build(self) -> Background<Given>
    where
        State: self::background::IsComplete,
    {
        Background {
            description: self.description,
            ignored: self.ignored,
            given: unsafe { self.given.unwrap_unchecked() },
        }
    }
}

#[sealed]
pub(crate) trait IntoBackground<Given>: ::core::marker::Sized {
    fn into_background(self) -> Background<Given>;
}

#[sealed]
impl<Given, World, State: self::background::State> IntoBackground<Given> for BackgroundBuilder<Given, State>
where
    State: self::background::IsComplete,
    Given: FnOnce() -> World + ::core::clone::Clone,
    World: ::core::marker::Send + ::core::marker::Sync,
{
    fn into_background(self) -> Background<Given> {
        self.build()
    }
}

mod background {
    pub(super) use super::*;

    #[sealed]
    pub trait State: ::core::marker::Sized {
        type Description;
        type Ignored;
        type Given;
    }

    #[sealed]
    pub trait IsComplete: self::State<Given: IsSet> {}

    #[sealed]
    impl<State: self::State> IsComplete for State
    where
        State::Given: IsSet,
    {
    }

    pub struct Empty;

    pub struct SetDescription<State: self::State = self::Empty>(PhantomCovariant<State>);
    pub struct SetIgnored<State: self::State = self::Empty>(PhantomCovariant<State>);
    pub struct SetGiven<State: self::State = self::Empty>(PhantomCovariant<State>);

    #[sealed]
    impl self::State for Empty {
        type Description = Unset<self::members::Description>;
        type Ignored = Unset<self::members::Ignored>;
        type Given = Unset<self::members::Given>;
    }

    #[sealed]
    impl<State: self::State> self::State for SetDescription<State> {
        type Description = Set<self::members::Description>;
        type Ignored = State::Ignored;
        type Given = State::Given;
    }

    #[sealed]
    impl<State: self::State> self::State for SetIgnored<State> {
        type Description = State::Description;
        type Ignored = Set<self::members::Ignored>;
        type Given = State::Given;
    }

    #[sealed]
    impl<State: self::State> self::State for SetGiven<State> {
        type Description = State::Description;
        type Ignored = State::Ignored;
        type Given = Set<self::members::Given>;
    }

    mod members {
        pub struct Description;
        pub struct Ignored;
        pub struct Given;
    }
}

pub struct ScenarioBuilder<Given, When, Then, State: self::scenario::State = self::scenario::Empty> {
    _phantom: ::core::marker::PhantomData<(
        PhantomCovariant<State>,
        PhantomCovariant<Steps<Given>>,
        PhantomCovariant<Given>,
        PhantomCovariant<Steps<When>>,
        PhantomCovariant<When>,
        PhantomCovariant<Steps<Then>>,
        PhantomCovariant<Then>,
    )>,

    description: ::core::option::Option<MaybeOwnedString>,
    ignored: ::core::option::Option<bool>,

    given: ::core::option::Option<Steps<Given>>,
    when: ::core::option::Option<Steps<When>>,
    then: ::core::option::Option<Steps<Then>>,
}

impl<Given, When, Then> Scenario<Given, When, Then> {
    pub fn builder() -> ScenarioBuilder<Given, When, Then> {
        ScenarioBuilder {
            _phantom: ::core::default::Default::default(),

            description: ::core::default::Default::default(),
            ignored: ::core::default::Default::default(),
            
            given: ::core::default::Default::default(),
            when: ::core::default::Default::default(),
            then: ::core::default::Default::default(),
        }
    }
}

impl<Given, When, Then, State: self::scenario::State> ScenarioBuilder<Given, When, Then, State>
where
    State::Given: self::scenario::IsUnset,
    State::When: self::scenario::IsUnset,
    State::Then: self::scenario::IsUnset,
{
    pub fn description(self, value: impl Into<MaybeOwnedString>) -> ScenarioBuilder<Given, When, Then, self::scenario::SetDescription<State>> {
        ScenarioBuilder {
            _phantom: ::core::default::Default::default(),

            description: ::core::option::Option::from(value.into()),
            ignored: self.ignored,
            
            given: self.given,
            when: self.when,
            then: self.then,
        }
    }

    pub fn ignored(self, value: impl Into<bool>) -> ScenarioBuilder<Given, When, Then, self::scenario::SetIgnored<State>> {
        ScenarioBuilder {
            _phantom: ::core::default::Default::default(),

            description: self.description,
            ignored: ::core::option::Option::from(value.into()),
            
            given: self.given,
            when: self.when,
            then: self.then,
        }
    }

    pub fn given<World>(self, description: impl Into<MaybeOwnedString>, callback: Given) -> ScenarioBuilder<Given, When, Then, self::scenario::SetGiven<State>>
    where
        Given: FnOnce() -> Fallible<World>,
        World: ::core::marker::Send + ::core::marker::Sync,
    {
        let given = Steps::builder()
            .label(StepLabel::Given)
            .description(description)
            .callback(callback)
            .build();

        ScenarioBuilder {
            _phantom: ::core::default::Default::default(),

            description: self.description,
            ignored: self.ignored,

            given: ::core::option::Option::from(given),
            when: self.when,
            then: self.then,
        }
    }
}

#[::bon::bon]
impl<Given, When, Then, State: self::scenario::State> ScenarioBuilder<Given, When, Then, self::scenario::SetGiven<State>>
where
    <self::scenario::SetGiven<State> as self::scenario::State>::Given: self::scenario::IsSet,
    <self::scenario::SetGiven<State> as self::scenario::State>::When: self::scenario::IsUnset,
    <self::scenario::SetGiven<State> as self::scenario::State>::Then: self::scenario::IsUnset,
{
    pub fn and<World>(self, description: impl Into<MaybeOwnedString>, callback: impl FnOnce(&mut World) -> Fallible) -> ScenarioBuilder<impl FnOnce() -> Fallible<World>, When, Then, self::scenario::SetGiven<State>>
    where
        Given: FnOnce() -> Fallible<World>,
        World: ::core::marker::Send + ::core::marker::Sync,
    {
        self.conjoin_given(description, callback)
            .label(StepLabel::And)
            .call()
    }

    pub fn but<World>(self, description: impl Into<MaybeOwnedString>, callback: impl FnOnce(&mut World) -> Fallible) -> ScenarioBuilder<impl FnOnce() -> Fallible<World>, When, Then, self::scenario::SetGiven<State>>
    where
        Given: FnOnce() -> Fallible<World>,
        World: ::core::marker::Send + ::core::marker::Sync,
    {
        self.conjoin_given(description, callback)
            .label(StepLabel::But)
            .call()
    }

    #[builder]
    fn conjoin_given<World>(mut self, #[builder(start_fn)] description: impl Into<MaybeOwnedString>, #[builder(start_fn)] callback: impl FnOnce(&mut World) -> Fallible, label: StepLabel) -> ScenarioBuilder<impl FnOnce() -> Fallible<World>, When, Then, self::scenario::SetGiven<State>>
    where
        Given: FnOnce() -> Fallible<World>,
        World: ::core::marker::Send + ::core::marker::Sync,
    {
        let given = unsafe { self.given.take().unwrap_unchecked() };
        let given = Steps::builder()
            .labels(given.labels)
            .label(label)
            .descriptions(given.descriptions)
            .description(description)
            .callback(move || {
                let mut world = (given.callback)()?;
                (callback)(&mut world)?;
                Ok(world)
            })
            .build();

        ScenarioBuilder {
            _phantom: ::core::default::Default::default(),

            description: self.description,
            ignored: self.ignored,

            given: ::core::option::Option::from(given),
            when: self.when,
            then: self.then,
        }
    }

    pub fn when<World>(self, description: impl Into<MaybeOwnedString>, callback: When) -> ScenarioBuilder<Given, When, Then, self::scenario::SetWhen<self::scenario::SetGiven<State>>>
    where
        Given: FnOnce() -> Fallible<World>, // Required for type deduction
        When: FnOnce(&mut World) -> Fallible,
        World: ::core::marker::Send + ::core::marker::Sync,
    {
        let when = Steps::builder()
            .label(StepLabel::When)
            .description(description)
            .callback(callback)
            .build();

        ScenarioBuilder {
            _phantom: ::core::default::Default::default(),

            description: self.description,
            ignored: self.ignored,

            given: self.given,
            when: ::core::option::Option::from(when),
            then: self.then,
        }
    }
}

#[::bon::bon]
impl<Given, When, Then, State: self::scenario::State> ScenarioBuilder<Given, When, Then, self::scenario::SetWhen<State>>
where
    <self::scenario::SetWhen<State> as self::scenario::State>::Given: self::scenario::IsSet,
    <self::scenario::SetWhen<State> as self::scenario::State>::When: self::scenario::IsSet,
    <self::scenario::SetWhen<State> as self::scenario::State>::Then: self::scenario::IsUnset,
{
    pub fn and<World>(self, description: impl Into<MaybeOwnedString>, callback: impl FnOnce(&mut World) -> Fallible) -> ScenarioBuilder<Given, impl FnOnce(&mut World) -> Fallible, Then, self::scenario::SetWhen<State>>
    where
        When: FnOnce(&mut World) -> Fallible,
        World: ::core::marker::Send + ::core::marker::Sync,
    {
        self.conjoin_when(description, callback)
            .label(StepLabel::And)
            .call()
    }

    pub fn but<World>(self, description: impl Into<MaybeOwnedString>, callback: impl FnOnce(&mut World) -> Fallible) -> ScenarioBuilder<Given, impl FnOnce(&mut World) -> Fallible, Then, self::scenario::SetWhen<State>>
    where
        When: FnOnce(&mut World) -> Fallible,
        World: ::core::marker::Send + ::core::marker::Sync,
    {
        self.conjoin_when(description, callback)
            .label(StepLabel::But)
            .call()
    }

    #[builder]
    fn conjoin_when<World>(mut self, #[builder(start_fn)] description: impl Into<MaybeOwnedString>, #[builder(start_fn)] callback: impl FnOnce(&mut World) -> Fallible, label: StepLabel) -> ScenarioBuilder<Given, impl FnOnce(&mut World) -> Fallible, Then, self::scenario::SetWhen<State>>
    where
        When: FnOnce(&mut World) -> Fallible,
        World: ::core::marker::Send + ::core::marker::Sync,
    {
        let when = unsafe { self.when.take().unwrap_unchecked() };
        let when = Steps::builder()
            .labels(when.labels)
            .label(label)
            .descriptions(when.descriptions)
            .description(description)
            .callback(move |world: &mut World| {
                (when.callback)(world)?;
                (callback)(world)?;
                Ok(())
            })
            .build();

        ScenarioBuilder {
            _phantom: ::core::default::Default::default(),

            description: self.description,
            ignored: self.ignored,

            given: self.given,
            when: ::core::option::Option::from(when),
            then: self.then,
        }
    }

    pub fn then<World>(self, description: impl Into<MaybeOwnedString>, callback: Then) -> ScenarioBuilder<Given, When, Then, self::scenario::SetThen<self::scenario::SetWhen<State>>>
    where
        When: FnOnce(&mut World) -> Fallible, // Required for type deduction
        Then: FnOnce(&World) -> Fallible,
        World: ::core::marker::Send + ::core::marker::Sync,
    {
        let then = Steps::builder()
            .label(StepLabel::Then)
            .description(description)
            .callback(callback)
            .build();

        ScenarioBuilder {
            _phantom: ::core::default::Default::default(),

            description: self.description,
            ignored: self.ignored,

            given: self.given,
            when: self.when,
            then: ::core::option::Option::from(then),
        }
    }
}

#[::bon::bon]
impl<Given, When, Then, State: self::scenario::State> ScenarioBuilder<Given, When, Then, self::scenario::SetThen<State>>
where
    <self::scenario::SetThen<State> as self::scenario::State>::Given: self::scenario::IsSet,
    <self::scenario::SetThen<State> as self::scenario::State>::When: self::scenario::IsSet,
    <self::scenario::SetThen<State> as self::scenario::State>::Then: self::scenario::IsSet,
{
    pub fn and<World>(self, description: impl Into<MaybeOwnedString>, callback: impl FnOnce(&World) -> Fallible) -> ScenarioBuilder<Given, When, impl FnOnce(&World) -> Fallible, self::scenario::SetThen<State>>
    where
        Then: FnOnce(&World) -> Fallible,
        World: ::core::marker::Send + ::core::marker::Sync,
    {
        self.conjoin_then(description, callback)
            .label(StepLabel::And)
            .call()
    }

    pub fn but<World>(self, description: impl Into<MaybeOwnedString>, callback: impl FnOnce(&World) -> Fallible) -> ScenarioBuilder<Given, When, impl FnOnce(&World) -> Fallible, self::scenario::SetThen<State>>
    where
        Then: FnOnce(&World) -> Fallible,
        World: ::core::marker::Send + ::core::marker::Sync,
    {
        self.conjoin_then(description, callback)
            .label(StepLabel::But)
            .call()
    }

    #[builder]
    fn conjoin_then<World>(mut self, #[builder(start_fn)] description: impl Into<MaybeOwnedString>, #[builder(start_fn)] callback: impl FnOnce(&World) -> Fallible, label: StepLabel) -> ScenarioBuilder<Given, When, impl FnOnce(&World) -> Fallible, self::scenario::SetThen<State>>
    where
        Then: FnOnce(&World) -> Fallible,
        World: ::core::marker::Send + ::core::marker::Sync,
    {
        let then = unsafe { self.then.take().unwrap_unchecked() };
        let then = Steps::builder()
            .labels(then.labels)
            .label(label)
            .descriptions(then.descriptions)
            .description(description)
            .callback(move |world: &World| {
                (then.callback)(world)?;
                (callback)(world)?;
                Ok(())
            })
            .build();

        ScenarioBuilder {
            _phantom: ::core::default::Default::default(),

            description: self.description,
            ignored: self.ignored,

            given: self.given,
            when: self.when,
            then: ::core::option::Option::from(then),
        }
    }
}

impl<Given, When, Then, State: self::scenario::State> ScenarioBuilder<Given, When, Then, State>
where
    State: self::scenario::IsComplete,
{
    pub fn build(self) -> Scenario<Given, When, Then>
    where
        State: self::scenario::IsComplete,
    {
        Scenario {
            description: self.description,
            ignored: self.ignored,

            given: unsafe { self.given.unwrap_unchecked() },
            when: unsafe { self.when.unwrap_unchecked() },
            then: unsafe { self.then.unwrap_unchecked() },
        }
    }
}

#[sealed]
trait IntoScenario<Given, When, Then>: ::core::marker::Sized {
    fn into_scenario(self) -> Scenario<Given, When, Then>;
}

#[sealed]
impl<Given, When, Then, State: self::scenario::State> IntoScenario<Given, When, Then> for ScenarioBuilder<Given, When, Then, State>
where
    State: self::scenario::IsComplete,
{
    fn into_scenario(self) -> Scenario<Given, When, Then> {
        self.build()
    }
}

mod scenario {
    pub(super) use super::*;

    #[sealed]
    pub trait State: ::core::marker::Sized {
        type Description;
        type Ignored;

        type Given;
        type When;
        type Then;
    }

    #[sealed]
    pub trait IsComplete: self::State<Given: IsSet, When: IsSet, Then: IsSet> {}

    #[sealed]
    impl<State: self::State> IsComplete for State
    where
        State::Given: IsSet,
        State::When: IsSet,
        State::Then: IsSet,
    {
    }

    pub struct Empty;

    pub struct SetDescription<State: self::State = self::Empty>(PhantomCovariant<State>);
    pub struct SetIgnored<State: self::State = self::Empty>(PhantomCovariant<State>);
    pub struct SetGiven<State: self::State = self::Empty>(PhantomCovariant<State>);
    pub struct SetWhen<State: self::State = self::Empty>(PhantomCovariant<State>);
    pub struct SetThen<State: self::State = self::Empty>(PhantomCovariant<State>);

    #[sealed]
    impl self::State for Empty {
        type Description = Unset<self::members::Description>;
        type Ignored = Unset<self::members::Ignored>;

        type Given = Unset<self::members::Given>;
        type When = Unset<self::members::When>;
        type Then = Unset<self::members::Then>;
    }

    #[sealed]
    impl<State: self::State> self::State for SetDescription<State> {
        type Description = Set<self::members::Description>;
        type Ignored = State::Ignored;

        type Given = State::Given;
        type When = State::When;
        type Then = State::Then;
    }

    #[sealed]
    impl<State: self::State> self::State for SetIgnored<State> {
        type Description = State::Description;
        type Ignored = Set<self::members::Ignored>;

        type Given = State::Given;
        type When = State::When;
        type Then = State::Then;
    }

    #[sealed]
    impl<State: self::State> self::State for SetGiven<State> {
        type Description = State::Description;
        type Ignored = State::Ignored;

        type Given = Set<self::members::Given>;
        type When = State::When;
        type Then = State::Then;
    }

    #[sealed]
    impl<State: self::State> self::State for SetWhen<State> {
        type Description = State::Description;
        type Ignored = State::Ignored;

        type Given = State::Given;
        type When = Set<self::members::When>;
        type Then = State::Then;
    }

    #[sealed]
    impl<State: self::State> self::State for SetThen<State> {
        type Description = State::Description;
        type Ignored = State::Ignored;

        type Given = State::Given;
        type When = State::When;
        type Then = Set<self::members::Then>;
    }

    mod members {
        pub struct Description;
        pub struct Ignored;

        pub struct Given;
        pub struct When;
        pub struct Then;
    }
}

pub(crate) struct StepsBuilder<Callback, State: self::steps::State = self::steps::Empty> {
    _phantom: ::core::marker::PhantomData<(
        PhantomCovariant<State>,
        PhantomCovariant<Steps<Callback>>,
        PhantomCovariant<Callback>,
    )>,

    labels: Vec<StepLabel>,
    descriptions: Vec<MaybeOwnedString>,
    callback: ::core::option::Option<Callback>,
}

impl<Callback> Steps<Callback> {
    fn builder() -> StepsBuilder<Callback> {
        StepsBuilder {
            _phantom: ::core::default::Default::default(),

            labels: ::core::default::Default::default(),
            descriptions: ::core::default::Default::default(),
            callback: ::core::default::Default::default(),
        }
    }
}

impl<Callback, State: self::steps::State> StepsBuilder<Callback, State> {
    fn build(self) -> Steps<Callback>
    where
        State: self::steps::IsComplete,
    {
        Steps {
            labels: self.labels,
            descriptions: self.descriptions,
            callback: unsafe { self.callback.unwrap_unchecked() },
        }
    }

    fn label(mut self, value: StepLabel) -> Self {
        self.labels.push(value);
        self
    }

    fn labels(mut self, value: impl IntoIterator<Item = StepLabel>) -> Self {
        self.labels.extend(value);
        self
    }

    fn description(mut self, value: impl Into<MaybeOwnedString>) -> Self {
        self.descriptions.push(value.into());
        self
    }

    fn descriptions(mut self, value: impl IntoIterator<Item = MaybeOwnedString>) -> Self {
        self.descriptions.extend(value);
        self
    }

    fn callback(mut self, value: Callback) -> StepsBuilder<Callback, self::steps::SetCallback<State>>
    where
        State::Callback: self::steps::IsUnset,
    {
        self.callback.replace(value);

        StepsBuilder {
            _phantom: ::core::marker::PhantomData,

            labels: self.labels,
            descriptions: self.descriptions,
            callback: self.callback,
        }
    }
}

mod steps {
    pub(super) use super::*;

    #[sealed]
    pub trait State: ::core::marker::Sized {
        type Callback;
    }

    #[sealed]
    pub trait IsComplete: self::State<Callback: IsSet> {}

    #[sealed]
    impl<State: self::State> IsComplete for State
    where
        State::Callback: IsSet,
    {
    }

    pub struct Empty;

    pub struct SetCallback<State: self::State = self::Empty>(PhantomCovariant<State>);

    #[sealed]
    impl self::State for Empty {
        type Callback = Unset<self::members::Callback>;
    }

    #[sealed]
    impl<State: self::State> self::State for SetCallback<State> {
        type Callback = Set<self::members::Callback>;
    }

    mod members {
        pub struct Callback;
    }
}

#[sealed]
pub trait IsSet {}

#[sealed]
pub trait IsUnset {}

pub struct Set<T>(::core::marker::PhantomData<T>);

pub struct Unset<T>(::core::marker::PhantomData<T>);

#[sealed]
impl<T> IsSet for Set<T> {}

#[sealed]
impl<T> IsUnset for Unset<T> {}

type PhantomCovariant<T> = ::core::marker::PhantomData<fn() -> T>;
