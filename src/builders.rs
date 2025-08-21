use ::sealed::sealed;

use crate::models::*;

pub struct BackgroundBuilder<World, State: self::background::BuilderState = self::background::Empty> {
    description: ::core::option::Option<::std::borrow::Cow<'static, str>>,
    ignored: ::core::option::Option<bool>,

    given: (
        ::core::option::Option<Step<::std::rc::Rc<dyn Fn() -> Fallible<World>>>>,
        ::core::option::Option<Steps<::std::rc::Rc<dyn Fn(&mut World) -> Fallible>>>,
    ),

    __phantom: self::marker::PhantomCovariant<State>,
}

impl<World> Background<World> {
    pub fn builder() -> BackgroundBuilder<World> {
        BackgroundBuilder {
            description: ::core::default::Default::default(),
            ignored: ::core::default::Default::default(),
            given: ::core::default::Default::default(),

            __phantom: ::core::default::Default::default(),
        }
    }
}

impl<World, State: self::background::BuilderState> BackgroundBuilder<World, State>
where
    State::Given: self::marker::IsUnset,
{
    pub fn description(mut self, value: impl Into<::std::borrow::Cow<'static, str>>) -> BackgroundBuilder<World, self::background::SetDescription<State>>
    where
        State::Description: self::marker::IsUnset,
    {
        self.description = ::core::option::Option::from(value.into());

        BackgroundBuilder {
            description: self.description,
            ignored: self.ignored,
            given: self.given,

            __phantom: ::core::default::Default::default(),
        }
    }

    pub fn ignored(mut self, value: impl Into<bool>) -> BackgroundBuilder<World, self::background::SetIgnored<State>>
    where
        State::Ignored: self::marker::IsUnset,
    {
        self.ignored = ::core::option::Option::from(value.into());

        BackgroundBuilder {
            description: self.description,
            ignored: self.ignored,
            given: self.given,

            __phantom: ::core::default::Default::default(),
        }
    }

    pub fn given(mut self, description: impl Into<::std::borrow::Cow<'static, str>>, callback: impl Fn() -> Fallible<World> + 'static) -> BackgroundBuilder<World, self::background::SetGiven<State>> {
        let step = Step::builder()
            .label(StepLabel::Given)
            .description(description)
            .callback(::std::rc::Rc::new(callback) as ::std::rc::Rc<dyn Fn() -> Fallible<World>>)
            .build();

        self.given.0 = ::core::option::Option::from(step);

        BackgroundBuilder {
            description: self.description,
            ignored: self.ignored,
            given: self.given,

            __phantom: ::core::default::Default::default(),
        }
    }
}

#[::bon::bon]
impl<World, InnerState: self::background::BuilderState> BackgroundBuilder<World, self::background::SetGiven<InnerState>>
where
    <self::background::SetGiven<InnerState> as self::background::BuilderState>::Given: self::marker::IsSet,
{
    fn and(self, description: impl Into<::std::borrow::Cow<'static, str>>, callback: impl Fn(&mut World) -> Fallible + 'static) -> BackgroundBuilder<World, self::background::SetGiven<self::background::SetGiven<InnerState>>> {
        self.conjoin(description, callback)
            .label(StepLabel::And)
            .call()
    }

    fn but(self, description: impl Into<::std::borrow::Cow<'static, str>>, callback: impl Fn(&mut World) -> Fallible + 'static) -> BackgroundBuilder<World, self::background::SetGiven<self::background::SetGiven<InnerState>>> {
        self.conjoin(description, callback)
            .label(StepLabel::But)
            .call()
    }

    #[builder]
    fn conjoin(mut self, #[builder(start_fn)] description: impl Into<::std::borrow::Cow<'static, str>>, #[builder(start_fn)] callback: impl Fn(&mut World) -> Fallible + 'static, label: StepLabel) -> BackgroundBuilder<World, self::background::SetGiven<self::background::SetGiven<InnerState>>> {
        let step = Step::builder()
            .label(label)
            .description(description)
            .callback(::std::rc::Rc::new(callback) as ::std::rc::Rc<dyn Fn(&mut World) -> Fallible>)
            .build();

        self.given.1.get_or_insert_with(|| Steps::builder().build()).0.push(step);

        BackgroundBuilder {
            description: self.description,
            ignored: self.ignored,
            given: self.given,

            __phantom: ::core::default::Default::default(),
        }
    }
}

impl<World, State: self::background::BuilderState> BackgroundBuilder<World, State>
where
    State: self::background::IsComplete,
{
    pub fn build(self) -> Background<World> {
        Background {
            description: self.description,
            ignored: self.ignored,
            given: (
                unsafe { self.given.0.unwrap_unchecked() },
                self.given.1,
            ),
        }
    }
}

#[sealed]
impl<World, State: self::background::BuilderState> IntoBackground<World> for BackgroundBuilder<World, State>
where
    State: self::background::IsComplete,
{
    fn into_background(self) -> Background<World> {
        self.build()
    }
}

mod background {
    pub(super) use super::*;

    #[sealed]
    pub trait BuilderState: ::core::marker::Sized {
        type Description;
        type Ignored;
        type Given;
    }

    #[sealed]
    pub trait IsComplete: BuilderState<Given: self::marker::IsSet> {}

    #[sealed]
    impl<State: BuilderState> IsComplete for State
    where
        State::Given: self::marker::IsSet,
    {
    }

    pub struct Empty;

    pub struct SetDescription<State: BuilderState = Empty>(self::marker::PhantomCovariant<State>);
    pub struct SetIgnored<State: BuilderState = Empty>(self::marker::PhantomCovariant<State>);
    pub struct SetGiven<State: BuilderState = Empty>(self::marker::PhantomCovariant<State>);

    #[sealed]
    impl BuilderState for Empty {
        type Description = self::marker::Unset<self::members::Description>;
        type Ignored = self::marker::Unset<self::members::Ignored>;
        type Given = self::marker::Unset<self::members::Given>;
    }

    #[sealed]
    impl<State: BuilderState> BuilderState for SetDescription<State> {
        type Description = self::marker::Set<self::members::Description>;
        type Ignored = State::Ignored;
        type Given = State::Given;
    }

    #[sealed]
    impl<State: BuilderState> BuilderState for SetIgnored<State> {
        type Description = State::Description;
        type Ignored = self::marker::Set<self::members::Ignored>;
        type Given = State::Given;
    }

    #[sealed]
    impl<State: BuilderState> BuilderState for SetGiven<State> {
        type Description = State::Description;
        type Ignored = State::Ignored;
        type Given = self::marker::Set<self::members::Given>;
    }

    mod members {
        pub struct Description;
        pub struct Ignored;
        pub struct Given;
    }
}

// pub struct ScenarioBuilder<Given, When, Then, State: self::scenario::State = self::scenario::Empty> {
//     _phantom: ::core::marker::PhantomData<(
//         PhantomCovariant<State>,
//         PhantomCovariant<Steps<Given>>,
//         PhantomCovariant<Given>,
//         PhantomCovariant<Steps<When>>,
//         PhantomCovariant<When>,
//         PhantomCovariant<Steps<Then>>,
//         PhantomCovariant<Then>,
//     )>,

//     description: ::core::option::Option<MaybeOwnedString>,
//     ignored: ::core::option::Option<bool>,
//     tags: ::core::option::Option<Tags>,

//     given: ::core::option::Option<Steps<Given>>,
//     when: ::core::option::Option<Steps<When>>,
//     then: ::core::option::Option<Steps<Then>>,
// }

// impl<Given, When, Then> Scenario<Given, When, Then> {
//     pub fn builder() -> ScenarioBuilder<Given, When, Then> {
//         ScenarioBuilder {
//             _phantom: ::core::default::Default::default(),

//             description: ::core::default::Default::default(),
//             ignored: ::core::default::Default::default(),
//             tags: ::core::default::Default::default(),
            
//             given: ::core::default::Default::default(),
//             when: ::core::default::Default::default(),
//             then: ::core::default::Default::default(),
//         }
//     }
// }

// impl<Given, When, Then, State: self::scenario::State> ScenarioBuilder<Given, When, Then, State>
// where
//     State::Given: self::scenario::IsUnset,
//     State::When: self::scenario::IsUnset,
//     State::Then: self::scenario::IsUnset,
// {
//     pub fn description(self, value: impl Into<MaybeOwnedString>) -> ScenarioBuilder<Given, When, Then, self::scenario::SetDescription<State>>
//     where
//         State::Description: self::scenario::IsUnset,
//     {
//         ScenarioBuilder {
//             _phantom: ::core::default::Default::default(),

//             description: ::core::option::Option::from(value.into()),
//             ignored: self.ignored,
//             tags: self.tags,
            
//             given: self.given,
//             when: self.when,
//             then: self.then,
//         }
//     }

//     pub fn ignored(self, value: impl Into<bool>) -> ScenarioBuilder<Given, When, Then, self::scenario::SetIgnored<State>>
//     where
//         State::Ignored: self::scenario::IsUnset,
//     {
//         ScenarioBuilder {
//             _phantom: ::core::default::Default::default(),

//             description: self.description,
//             ignored: ::core::option::Option::from(value.into()),
//             tags: self.tags,
            
//             given: self.given,
//             when: self.when,
//             then: self.then,
//         }
//     }

//     pub fn tags(self, value: impl IntoTags) -> ScenarioBuilder<Given, When, Then, self::scenario::SetTags<State>>
//     where
//         State::Tags: self::scenario::IsUnset,
//     {
//         ScenarioBuilder {
//             _phantom: ::core::default::Default::default(),

//             description: self.description,
//             ignored: self.ignored,
//             tags: ::core::option::Option::from(value.into_tags()),
            
//             given: self.given,
//             when: self.when,
//             then: self.then,
//         }
//     }

//     pub fn given<World>(self, description: impl Into<MaybeOwnedString>, callback: Given) -> ScenarioBuilder<Given, When, Then, self::scenario::SetGiven<State>>
//     where
//         Given: FnOnce() -> Fallible<World>,
//         World: ::core::marker::Send + ::core::marker::Sync,
//     {
//         let given = Steps::builder()
//             .label(StepLabel::Given)
//             .description(description)
//             .callback(callback)
//             .build();

//         ScenarioBuilder {
//             _phantom: ::core::default::Default::default(),

//             description: self.description,
//             ignored: self.ignored,
//             tags: self.tags,

//             given: ::core::option::Option::from(given),
//             when: self.when,
//             then: self.then,
//         }
//     }
// }

// #[::bon::bon]
// impl<Given, When, Then, State: self::scenario::State> ScenarioBuilder<Given, When, Then, self::scenario::SetGiven<State>>
// where
//     <self::scenario::SetGiven<State> as self::scenario::State>::Given: self::scenario::IsSet,
//     <self::scenario::SetGiven<State> as self::scenario::State>::When: self::scenario::IsUnset,
//     <self::scenario::SetGiven<State> as self::scenario::State>::Then: self::scenario::IsUnset,
// {
//     pub fn and<World>(self, description: impl Into<MaybeOwnedString>, callback: impl FnOnce(&mut World) -> Fallible) -> ScenarioBuilder<impl FnOnce() -> Fallible<World>, When, Then, self::scenario::SetGiven<State>>
//     where
//         Given: FnOnce() -> Fallible<World>,
//         World: ::core::marker::Send + ::core::marker::Sync,
//     {
//         self.conjoin_given(description, callback)
//             .label(StepLabel::And)
//             .call()
//     }

//     pub fn but<World>(self, description: impl Into<MaybeOwnedString>, callback: impl FnOnce(&mut World) -> Fallible) -> ScenarioBuilder<impl FnOnce() -> Fallible<World>, When, Then, self::scenario::SetGiven<State>>
//     where
//         Given: FnOnce() -> Fallible<World>,
//         World: ::core::marker::Send + ::core::marker::Sync,
//     {
//         self.conjoin_given(description, callback)
//             .label(StepLabel::But)
//             .call()
//     }

//     #[builder]
//     fn conjoin_given<World>(mut self, #[builder(start_fn)] description: impl Into<MaybeOwnedString>, #[builder(start_fn)] callback: impl FnOnce(&mut World) -> Fallible, label: StepLabel) -> ScenarioBuilder<impl FnOnce() -> Fallible<World>, When, Then, self::scenario::SetGiven<State>>
//     where
//         Given: FnOnce() -> Fallible<World>,
//         World: ::core::marker::Send + ::core::marker::Sync,
//     {
//         let given = unsafe { self.given.take().unwrap_unchecked() };
//         let given = Steps::builder()
//             .labels(given.labels)
//             .label(label)
//             .descriptions(given.descriptions)
//             .description(description)
//             .callback(move || {
//                 let mut world = (given.callback)()?;
//                 (callback)(&mut world)?;
//                 Ok(world)
//             })
//             .build();

//         ScenarioBuilder {
//             _phantom: ::core::default::Default::default(),

//             description: self.description,
//             ignored: self.ignored,
//             tags: self.tags,

//             given: ::core::option::Option::from(given),
//             when: self.when,
//             then: self.then,
//         }
//     }

//     pub fn when<World>(self, description: impl Into<MaybeOwnedString>, callback: When) -> ScenarioBuilder<Given, When, Then, self::scenario::SetWhen<self::scenario::SetGiven<State>>>
//     where
//         Given: FnOnce() -> Fallible<World>, // Required for type deduction
//         When: FnOnce(&mut World) -> Fallible,
//         World: ::core::marker::Send + ::core::marker::Sync,
//     {
//         let when = Steps::builder()
//             .label(StepLabel::When)
//             .description(description)
//             .callback(callback)
//             .build();

//         ScenarioBuilder {
//             _phantom: ::core::default::Default::default(),

//             description: self.description,
//             ignored: self.ignored,
//             tags: self.tags,

//             given: self.given,
//             when: ::core::option::Option::from(when),
//             then: self.then,
//         }
//     }
// }

// #[::bon::bon]
// impl<Given, When, Then, State: self::scenario::State> ScenarioBuilder<Given, When, Then, self::scenario::SetWhen<State>>
// where
//     <self::scenario::SetWhen<State> as self::scenario::State>::Given: self::scenario::IsSet,
//     <self::scenario::SetWhen<State> as self::scenario::State>::When: self::scenario::IsSet,
//     <self::scenario::SetWhen<State> as self::scenario::State>::Then: self::scenario::IsUnset,
// {
//     pub fn and<World>(self, description: impl Into<MaybeOwnedString>, callback: impl FnOnce(&mut World) -> Fallible) -> ScenarioBuilder<Given, impl FnOnce(&mut World) -> Fallible, Then, self::scenario::SetWhen<State>>
//     where
//         When: FnOnce(&mut World) -> Fallible,
//         World: ::core::marker::Send + ::core::marker::Sync,
//     {
//         self.conjoin_when(description, callback)
//             .label(StepLabel::And)
//             .call()
//     }

//     pub fn but<World>(self, description: impl Into<MaybeOwnedString>, callback: impl FnOnce(&mut World) -> Fallible) -> ScenarioBuilder<Given, impl FnOnce(&mut World) -> Fallible, Then, self::scenario::SetWhen<State>>
//     where
//         When: FnOnce(&mut World) -> Fallible,
//         World: ::core::marker::Send + ::core::marker::Sync,
//     {
//         self.conjoin_when(description, callback)
//             .label(StepLabel::But)
//             .call()
//     }

//     #[builder]
//     fn conjoin_when<World>(mut self, #[builder(start_fn)] description: impl Into<MaybeOwnedString>, #[builder(start_fn)] callback: impl FnOnce(&mut World) -> Fallible, label: StepLabel) -> ScenarioBuilder<Given, impl FnOnce(&mut World) -> Fallible, Then, self::scenario::SetWhen<State>>
//     where
//         When: FnOnce(&mut World) -> Fallible,
//         World: ::core::marker::Send + ::core::marker::Sync,
//     {
//         let when = unsafe { self.when.take().unwrap_unchecked() };
//         let when = Steps::builder()
//             .labels(when.labels)
//             .label(label)
//             .descriptions(when.descriptions)
//             .description(description)
//             .callback(move |world: &mut World| {
//                 (when.callback)(world)?;
//                 (callback)(world)?;
//                 Ok(())
//             })
//             .build();

//         ScenarioBuilder {
//             _phantom: ::core::default::Default::default(),

//             description: self.description,
//             ignored: self.ignored,
//             tags: self.tags,

//             given: self.given,
//             when: ::core::option::Option::from(when),
//             then: self.then,
//         }
//     }

//     pub fn then<World>(self, description: impl Into<MaybeOwnedString>, callback: Then) -> ScenarioBuilder<Given, When, Then, self::scenario::SetThen<self::scenario::SetWhen<State>>>
//     where
//         When: FnOnce(&mut World) -> Fallible, // Required for type deduction
//         Then: FnOnce(&World) -> Fallible,
//         World: ::core::marker::Send + ::core::marker::Sync,
//     {
//         let then = Steps::builder()
//             .label(StepLabel::Then)
//             .description(description)
//             .callback(callback)
//             .build();

//         ScenarioBuilder {
//             _phantom: ::core::default::Default::default(),

//             description: self.description,
//             ignored: self.ignored,
//             tags: self.tags,

//             given: self.given,
//             when: self.when,
//             then: ::core::option::Option::from(then),
//         }
//     }
// }

// #[::bon::bon]
// impl<Given, When, Then, State: self::scenario::State> ScenarioBuilder<Given, When, Then, self::scenario::SetThen<State>>
// where
//     <self::scenario::SetThen<State> as self::scenario::State>::Given: self::scenario::IsSet,
//     <self::scenario::SetThen<State> as self::scenario::State>::When: self::scenario::IsSet,
//     <self::scenario::SetThen<State> as self::scenario::State>::Then: self::scenario::IsSet,
// {
//     pub fn and<World>(self, description: impl Into<MaybeOwnedString>, callback: impl FnOnce(&World) -> Fallible) -> ScenarioBuilder<Given, When, impl FnOnce(&World) -> Fallible, self::scenario::SetThen<State>>
//     where
//         Then: FnOnce(&World) -> Fallible,
//         World: ::core::marker::Send + ::core::marker::Sync,
//     {
//         self.conjoin_then(description, callback)
//             .label(StepLabel::And)
//             .call()
//     }

//     pub fn but<World>(self, description: impl Into<MaybeOwnedString>, callback: impl FnOnce(&World) -> Fallible) -> ScenarioBuilder<Given, When, impl FnOnce(&World) -> Fallible, self::scenario::SetThen<State>>
//     where
//         Then: FnOnce(&World) -> Fallible,
//         World: ::core::marker::Send + ::core::marker::Sync,
//     {
//         self.conjoin_then(description, callback)
//             .label(StepLabel::But)
//             .call()
//     }

//     #[builder]
//     fn conjoin_then<World>(mut self, #[builder(start_fn)] description: impl Into<MaybeOwnedString>, #[builder(start_fn)] callback: impl FnOnce(&World) -> Fallible, label: StepLabel) -> ScenarioBuilder<Given, When, impl FnOnce(&World) -> Fallible, self::scenario::SetThen<State>>
//     where
//         Then: FnOnce(&World) -> Fallible,
//         World: ::core::marker::Send + ::core::marker::Sync,
//     {
//         let then = unsafe { self.then.take().unwrap_unchecked() };
//         let then = Steps::builder()
//             .labels(then.labels)
//             .label(label)
//             .descriptions(then.descriptions)
//             .description(description)
//             .callback(move |world: &World| {
//                 (then.callback)(world)?;
//                 (callback)(world)?;
//                 Ok(())
//             })
//             .build();

//         ScenarioBuilder {
//             _phantom: ::core::default::Default::default(),

//             description: self.description,
//             ignored: self.ignored,
//             tags: self.tags,

//             given: self.given,
//             when: self.when,
//             then: ::core::option::Option::from(then),
//         }
//     }
// }

// impl<Given, When, Then, State: self::scenario::State> ScenarioBuilder<Given, When, Then, State>
// where
//     State: self::scenario::IsComplete,
// {
//     pub fn build(self) -> Scenario<Given, When, Then>
//     where
//         State: self::scenario::IsComplete,
//     {
//         Scenario {
//             description: self.description,
//             ignored: self.ignored,
//             tags: self.tags,

//             given: unsafe { self.given.unwrap_unchecked() },
//             when: unsafe { self.when.unwrap_unchecked() },
//             then: unsafe { self.then.unwrap_unchecked() },
//         }
//     }
// }

// #[sealed(pub(crate))]
// impl<Given, When, Then, State: self::scenario::State> IntoScenario<Given, When, Then> for ScenarioBuilder<Given, When, Then, State>
// where
//     State: self::scenario::IsComplete,
// {
//     fn into_scenario(self) -> Scenario<Given, When, Then> {
//         self.build()
//     }
// }

// mod scenario {
//     pub(super) use super::*;

//     #[sealed]
//     pub trait State: ::core::marker::Sized {
//         type Description;
//         type Ignored;
//         type Tags;

//         type Given;
//         type When;
//         type Then;
//     }

//     #[sealed]
//     pub trait IsComplete: self::State<Given: IsSet, When: IsSet, Then: IsSet> {}

//     #[sealed]
//     impl<State: self::State> IsComplete for State
//     where
//         State::Given: IsSet,
//         State::When: IsSet,
//         State::Then: IsSet,
//     {
//     }

//     pub struct Empty;

//     pub struct SetDescription<State: self::State = self::Empty>(PhantomCovariant<State>);
//     pub struct SetIgnored<State: self::State = self::Empty>(PhantomCovariant<State>);
//     pub struct SetTags<State: self::State = self::Empty>(PhantomCovariant<State>);

//     pub struct SetGiven<State: self::State = self::Empty>(PhantomCovariant<State>);
//     pub struct SetWhen<State: self::State = self::Empty>(PhantomCovariant<State>);
//     pub struct SetThen<State: self::State = self::Empty>(PhantomCovariant<State>);

//     #[sealed]
//     impl self::State for Empty {
//         type Description = Unset<self::members::Description>;
//         type Ignored = Unset<self::members::Ignored>;
//         type Tags = Unset<self::members::Tags>;

//         type Given = Unset<self::members::Given>;
//         type When = Unset<self::members::When>;
//         type Then = Unset<self::members::Then>;
//     }

//     #[sealed]
//     impl<State: self::State> self::State for SetDescription<State> {
//         type Description = Set<self::members::Description>;
//         type Ignored = State::Ignored;
//         type Tags = State::Tags;

//         type Given = State::Given;
//         type When = State::When;
//         type Then = State::Then;
//     }

//     #[sealed]
//     impl<State: self::State> self::State for SetIgnored<State> {
//         type Description = State::Description;
//         type Ignored = Set<self::members::Ignored>;
//         type Tags = State::Tags;

//         type Given = State::Given;
//         type When = State::When;
//         type Then = State::Then;
//     }

//     #[sealed]
//     impl<State: self::State> self::State for SetTags<State> {
//         type Description = State::Description;
//         type Ignored = State::Ignored;
//         type Tags = Set<self::members::Tags>;

//         type Given = State::Given;
//         type When = State::When;
//         type Then = State::Then;
//     }

//     #[sealed]
//     impl<State: self::State> self::State for SetGiven<State> {
//         type Description = State::Description;
//         type Ignored = State::Ignored;
//         type Tags = State::Tags;

//         type Given = Set<self::members::Given>;
//         type When = State::When;
//         type Then = State::Then;
//     }

//     #[sealed]
//     impl<State: self::State> self::State for SetWhen<State> {
//         type Description = State::Description;
//         type Ignored = State::Ignored;
//         type Tags = State::Tags;

//         type Given = State::Given;
//         type When = Set<self::members::When>;
//         type Then = State::Then;
//     }

//     #[sealed]
//     impl<State: self::State> self::State for SetThen<State> {
//         type Description = State::Description;
//         type Ignored = State::Ignored;
//         type Tags = State::Tags;

//         type Given = State::Given;
//         type When = State::When;
//         type Then = Set<self::members::Then>;
//     }

//     mod members {
//         pub struct Description;
//         pub struct Ignored;
//         pub struct Tags;

//         pub struct Given;
//         pub struct When;
//         pub struct Then;
//     }
// }

#[sealed]
impl<I, T> IntoTags for I
where
    I: IntoIterator<Item = T>,
    T: Into<::std::borrow::Cow<'static, str>>,
{
    fn into_tags(self) -> Tags {
        let inner = self.into_iter()
            .map(Into::into)
            .collect();

        Tags(inner)
    }
}

pub(crate) struct StepsBuilder<Callback>(::std::vec::Vec<Step<Callback>>);

impl<Callback> Steps<Callback> {
    fn builder() -> StepsBuilder<Callback> {
        StepsBuilder(::core::default::Default::default())
    }
}

impl<Callback> StepsBuilder<Callback> {
    fn step(mut self, value: impl IntoStep<Callback>) -> Self {
        self.0.push(value.into_step());
        self
    }

    fn steps<T>(mut self, value: impl IntoIterator<Item = T>) -> Self
    where
        T: IntoStep<Callback>,
    {
        self.0.extend(value
            .into_iter()
            .map(IntoStep::into_step));
        self
    }

    fn build(self) -> Steps<Callback> {
        Steps(self.0)
    }
}

pub(crate) struct StepBuilder<Callback, State: self::step::BuilderState = self::step::Empty> {
    label: ::core::option::Option<StepLabel>,
    description: ::core::option::Option<::std::borrow::Cow<'static, str>>,
    callback: ::core::option::Option<Callback>,

    __phantom: self::marker::PhantomCovariant<State>,
}

impl<Callback> Step<Callback> {
    fn builder() -> StepBuilder<Callback> {
        StepBuilder {
            label: ::core::default::Default::default(),
            description: ::core::default::Default::default(),
            callback: ::core::default::Default::default(),

            __phantom: ::core::default::Default::default(),
        }
    }
}

impl<Callback, State: self::step::BuilderState> StepBuilder<Callback, State> {
    fn label(mut self, value: StepLabel) -> StepBuilder<Callback, self::step::SetLabel<State>>
    where
        State::Label: self::marker::IsUnset,
    {
        self.label = ::core::option::Option::from(value);

        StepBuilder {
            label: self.label,
            description: self.description,
            callback: self.callback,
            
            __phantom: ::core::default::Default::default(),
        }
    }

    fn description(mut self, value: impl Into<::std::borrow::Cow<'static, str>>) -> StepBuilder<Callback, self::step::SetDescription<State>>
    where
        State::Description: self::marker::IsUnset,
    {
        self.description = ::core::option::Option::from(value.into());
        
        StepBuilder {
            label: self.label,
            description: self.description,
            callback: self.callback,
            
            __phantom: ::core::default::Default::default(),
        }
    }

    fn callback(mut self, value: Callback) -> StepBuilder<Callback, self::step::SetCallback<State>>
    where
        State::Callback: self::marker::IsUnset,
    {
        self.callback = ::core::option::Option::from(value);

        StepBuilder {
            label: self.label,
            description: self.description,
            callback: self.callback,
            
            __phantom: ::core::default::Default::default(),
        }
    }

    fn build(self) -> Step<Callback>
    where
        State: self::step::IsComplete,
    {
        Step {
            label: unsafe { self.label.unwrap_unchecked() },
            description: unsafe { self.description.unwrap_unchecked() },
            callback: unsafe { self.callback.unwrap_unchecked() },
        }
    }
}

#[sealed]
impl<Callback, State: self::step::BuilderState> IntoStep<Callback> for StepBuilder<Callback, State>
where
    State: self::step::IsComplete,
{
    fn into_step(self) -> Step<Callback>  {
        self.build()
    }
}

mod step {
    pub(super) use super::*;

    #[sealed]
    pub trait BuilderState: ::core::marker::Sized {
        type Label;
        type Description;
        type Callback;
    }

    #[sealed]
    pub trait IsComplete: BuilderState<Callback: self::marker::IsSet> {}

    #[sealed]
    impl<State: BuilderState> IsComplete for State
    where
        State::Callback: self::marker::IsSet,
    {
    }

    pub struct Empty;

    pub struct SetLabel<State: BuilderState = Empty>(self::marker::PhantomCovariant<State>);
    pub struct SetDescription<State: BuilderState = Empty>(self::marker::PhantomCovariant<State>);
    pub struct SetCallback<State: BuilderState = Empty>(self::marker::PhantomCovariant<State>);

    #[sealed]
    impl BuilderState for Empty {
        type Label = self::marker::Unset<self::members::Label>;
        type Description = self::marker::Unset<self::members::Description>;
        type Callback = self::marker::Unset<self::members::Callback>;
    }

    #[sealed]
    impl<State: BuilderState> BuilderState for SetLabel<State> {
        type Label = self::marker::Set<self::members::Label>;
        type Description = State::Description;
        type Callback = State::Callback;
    }

    #[sealed]
    impl<State: BuilderState> BuilderState for SetDescription<State> {
        type Label = State::Label;
        type Description = self::marker::Set<self::members::Description>;
        type Callback = State::Callback;
    }

    #[sealed]
    impl<State: BuilderState> BuilderState for SetCallback<State> {
        type Label = State::Label;
        type Description = State::Description;
        type Callback = self::marker::Set<self::members::Callback>;
    }

    mod members {
        pub struct Label;
        pub struct Description;
        pub struct Callback;
    }
}

mod marker {
    pub(super) use super::*;

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

    pub type PhantomCovariant<State> = ::core::marker::PhantomData<State>;
}
