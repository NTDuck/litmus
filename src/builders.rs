use ::sealed::sealed;

use crate::{utils::aliases::{MaybeOwnedString, Vec}, Background, StepLabel, Steps};

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

impl<Given, State: self::background::State> BackgroundBuilder<Given, State> {
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

    pub fn description(mut self, value: impl Into<MaybeOwnedString>) -> BackgroundBuilder<Given, self::background::SetDescription<State>>
    where
        State::Description: self::background::IsUnset,
    {
        self.description.replace(value.into());
        
        BackgroundBuilder {
            _phantom: ::core::default::Default::default(),

            description: self.description,
            ignored: self.ignored,
            given: self.given,
        }
    }

    pub fn ignored(mut self, value: impl Into<bool>) -> BackgroundBuilder<Given, self::background::SetIgnored<State>>
    where
        State::Ignored: self::background::IsUnset,
    {
        self.ignored.replace(value.into());
        
        BackgroundBuilder {
            _phantom: ::core::default::Default::default(),

            description: self.description,
            ignored: self.ignored,
            given: self.given,
        }
    }

    pub fn given<World>(mut self, description: impl Into<MaybeOwnedString>, callback: Given) -> BackgroundBuilder<Given, self::background::SetGiven<State>>
    where
        State::Given: self::background::IsUnset,
        Given: FnOnce() -> World + ::core::clone::Clone,
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

    pub fn and<World>(mut self, description: impl Into<MaybeOwnedString>, callback: impl FnOnce(&mut World) + ::core::clone::Clone) -> BackgroundBuilder<impl FnOnce() -> World + ::core::clone::Clone, self::background::SetGiven<State>>
    where
        State::Given: self::background::IsSet,
        Given: FnOnce() -> World + ::core::clone::Clone,
        World: ::core::marker::Send + ::core::marker::Sync,
    {
        let given = unsafe { self.given.take().unwrap_unchecked() };
        let given = Steps::builder()
            .labels(given.labels)
            .label(StepLabel::And)
            .descriptions(given.descriptions)
            .description(description)
            .callback(move || {
                let mut world = (given.callback)();
                callback(&mut world);
                world
            })
            .build();

        BackgroundBuilder {
            _phantom: ::core::default::Default::default(),

            description: self.description,
            ignored: self.ignored,
            given: ::core::option::Option::from(given),
        }
    }

    pub fn but<World>(mut self, description: impl Into<MaybeOwnedString>, callback: impl FnOnce(&mut World) + ::core::clone::Clone) -> BackgroundBuilder<impl FnOnce() -> World + ::core::clone::Clone, self::background::SetGiven<State>>
    where
        State::Given: self::background::IsSet,
        Given: FnOnce() -> World + ::core::clone::Clone,
        World: ::core::marker::Send + ::core::marker::Sync,
    {
        let given = unsafe { self.given.take().unwrap_unchecked() };
        let given = Steps::builder()
            .labels(given.labels)
            .label(StepLabel::But)
            .descriptions(given.descriptions)
            .description(description)
            .callback(move || {
                let mut world = (given.callback)();
                callback(&mut world);
                world
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
