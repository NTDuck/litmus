use ::sealed::sealed;

use crate::{utils::aliases::{MaybeOwnedString, Vec}, StepLabel, Steps};

pub(crate) struct StepsBuilder<Callback, State: self::steps::State = self::steps::Empty> {
    _phantom: ::core::marker::PhantomData<(
        PhantomCovariant<State>,
        PhantomCovariant<Steps<Callback>>,
        PhantomCovariant<Callback>,
    )>,

    labels: Vec<StepLabel>,
    descriptions: Vec<MaybeOwnedString>,
    callback: ::std::mem::MaybeUninit<Callback>,
}

impl<Callback> Steps<Callback> {
    pub fn builder() -> StepsBuilder<Callback> {
        StepsBuilder {
            _phantom: ::core::marker::PhantomData,

            labels: ::core::default::Default::default(),
            descriptions: ::core::default::Default::default(),
            callback: ::std::mem::MaybeUninit::uninit(),
        }
    }
}

impl<Callback, State: self::steps::State> StepsBuilder<Callback, State> {
    pub fn build(self) -> Steps<Callback>
    where
        State: self::steps::IsComplete,
    {
        Steps {
            labels: self.labels,
            descriptions: self.descriptions,
            callback: unsafe { self.callback.assume_init() },
        }
    }

    pub fn label(mut self, value: StepLabel) -> Self {
        self.labels.push(value);
        self
    }

    pub fn description(mut self, value: impl Into<MaybeOwnedString>) -> Self {
        self.descriptions.push(value.into());
        self
    }

    pub fn callback(mut self, value: Callback) -> StepsBuilder<Callback, self::steps::SetCallback<State>>
    where
        State::Callback: self::steps::IsUnset,
    {
        self.callback.write(value);

        StepsBuilder {
            _phantom: ::core::marker::PhantomData,

            labels: self.labels,
            descriptions: self.descriptions,
            callback: self.callback,
        }
    }
}

impl<Callback, State: self::steps::State> From<StepsBuilder<Callback, State>> for Steps<Callback>
where
    State: self::steps::IsComplete,
{
    fn from(value: StepsBuilder<Callback, State>) -> Self {
        value.build()
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
