use ::sealed::sealed;

use crate::models::*;
use crate::utils::aliases;

pub struct FeatureBuilder<World, State: self::feature::BuilderState = self::feature::Empty> {
    description: ::core::option::Option<::std::borrow::Cow<'static, str>>,
    ignored: ::core::option::Option<bool>,
    tags: ::core::option::Option<Tags>,

    background: ::core::option::Option<Background<World>>,
    scenarios: ::std::vec::Vec<Scenario<World>>,
    rules: ::std::vec::Vec<Rule<World>>,

    __phantom: self::marker::PhantomCovariant<State>,
}

impl<World> Feature<World> {
    pub fn builder() -> FeatureBuilder<World> {
        FeatureBuilder {
            description: ::core::default::Default::default(),
            ignored: ::core::default::Default::default(),
            tags: ::core::default::Default::default(),

            background: ::core::default::Default::default(),
            scenarios: ::core::default::Default::default(),
            rules: ::core::default::Default::default(),

            __phantom: ::core::default::Default::default(),
        }
    }
}

impl<World, State: self::feature::BuilderState> FeatureBuilder<World, State> {
    pub fn description(mut self, value: impl Into<::std::borrow::Cow<'static, str>>) -> FeatureBuilder<World, self::feature::SetDescription<State>>
    where
        State::Description: self::marker::IsUnset,
    {
        self.description = ::core::option::Option::from(value.into());

        FeatureBuilder {
            description: self.description,
            ignored: self.ignored,
            tags: self.tags,

            background: self.background,
            scenarios: self.scenarios,
            rules: self.rules,

            __phantom: ::core::default::Default::default(),
        }
    }

    pub fn ignored(mut self, value: impl Into<bool>) -> FeatureBuilder<World, self::feature::SetIgnored<State>>
    where
        State::Ignored: self::marker::IsUnset,
    {
        self.ignored = ::core::option::Option::from(value.into());

        FeatureBuilder {
            description: self.description,
            ignored: self.ignored,
            tags: self.tags,

            background: self.background,
            scenarios: self.scenarios,
            rules: self.rules,

            __phantom: ::core::default::Default::default(),
        }
    }

    pub fn tags(mut self, value: impl Into<Tags>) -> FeatureBuilder<World, self::feature::SetTags<State>>
    where
        State::Tags: self::marker::IsUnset,
    {
        self.tags = ::core::option::Option::from(value.into());

        FeatureBuilder {
            description: self.description,
            ignored: self.ignored,
            tags: self.tags,

            background: self.background,
            scenarios: self.scenarios,
            rules: self.rules,

            __phantom: ::core::default::Default::default(),
        }
    }

    pub fn background(mut self, value: impl Into<Background<World>>) -> FeatureBuilder<World, self::feature::SetBackground<State>>
    where
        State::Background: self::marker::IsUnset,
    {
        self.background = ::core::option::Option::from(value.into());

        FeatureBuilder {
            description: self.description,
            ignored: self.ignored,
            tags: self.tags,

            background: self.background,
            scenarios: self.scenarios,
            rules: self.rules,

            __phantom: ::core::default::Default::default(),
        }
    }

    pub fn scenario(mut self, value: impl Into<Scenario<World>>) -> Self {
        self.scenarios.push(value.into());
        self
    }

    pub fn scenarios<T>(mut self, values: impl IntoIterator<Item = T>) -> Self
    where
        T: Into<Scenario<World>>,
    {
        self.scenarios.extend(values.into_iter().map(Into::into));
        self
    }

    pub fn rule(mut self, value: impl Into<Rule<World>>) -> Self {
        self.rules.push(value.into());
        self
    }

    pub fn rules<T>(mut self, values: impl IntoIterator<Item = T>) -> Self
    where
        T: Into<Rule<World>>,
    {
        self.rules.extend(values.into_iter().map(Into::into));
        self
    }

    pub fn build(mut self) -> Feature<World> {
        self.propagate_ignored();
        self.propagate_tags();

        Feature {
            description: self.description,
            ignored: self.ignored,
            tags: self.tags,

            background: self.background,
            scenarios: self.scenarios,
            rules: self.rules,
        }
    }

    fn propagate_ignored(&mut self) {
        if let Some(ignored) = self.ignored.as_ref() {
            self.scenarios.iter_mut()
                .for_each(|scenario| scenario.ignored = ::core::option::Option::from(*ignored));

            self.rules.iter_mut()
                .for_each(|rule| rule.ignored = ::core::option::Option::from(*ignored));
        }
    }

    /// See also: [Tag inheritance](https://cucumber.io/docs/cucumber/api/#tag-inheritance)
    fn propagate_tags(&mut self) {
        if let Some(tags) = self.tags.as_ref() {
            self.scenarios.iter_mut()
                .for_each(|scenario| scenario.tags
                    .get_or_insert_with(|| Tags::from(::std::iter::empty::<::std::borrow::Cow<'static, str>>()))
                    .extend(tags.clone()));

            self.rules.iter_mut()
                .for_each(|rule| rule.tags
                    .get_or_insert_with(|| Tags::from(::std::iter::empty::<::std::borrow::Cow<'static, str>>()))
                    .extend(tags.clone()));
        }
    }
}

impl<World, State: self::feature::BuilderState> From<FeatureBuilder<World, State>> for Feature<World> {
    fn from(builder: FeatureBuilder<World, State>) -> Self {
        builder.build()
    }
}

mod feature {
    pub(super) use super::*;

    #[sealed]
    pub trait BuilderState: ::core::marker::Sized {
        type Description;
        type Ignored;
        type Tags;

        type Background;
    }

    pub struct Empty;

    pub struct SetDescription<State: BuilderState = Empty>(self::marker::PhantomCovariant<State>);
    pub struct SetIgnored<State: BuilderState = Empty>(self::marker::PhantomCovariant<State>);
    pub struct SetTags<State: BuilderState = Empty>(self::marker::PhantomCovariant<State>);

    pub struct SetBackground<State: BuilderState = Empty>(self::marker::PhantomCovariant<State>);

    #[sealed]
    impl BuilderState for Empty {
        type Description = self::marker::Unset<self::members::Description>;
        type Ignored = self::marker::Unset<self::members::Ignored>;
        type Tags = self::marker::Unset<self::members::Tags>;

        type Background = self::marker::Unset<self::members::Background>;
    }

    #[sealed]
    impl<State: BuilderState> BuilderState for SetDescription<State> {
        type Description = self::marker::Set<self::members::Description>;
        type Ignored = State::Ignored;
        type Tags = State::Tags;

        type Background = State::Background;
    }

    #[sealed]
    impl<State: BuilderState> BuilderState for SetIgnored<State> {
        type Description = State::Description;
        type Ignored = self::marker::Set<self::members::Ignored>;
        type Tags = State::Tags;

        type Background = State::Background;
    }

    #[sealed]
    impl<State: BuilderState> BuilderState for SetTags<State> {
        type Description = State::Description;
        type Ignored = State::Ignored;
        type Tags = self::marker::Set<self::members::Tags>;

        type Background = State::Background;
    }

    #[sealed]
    impl<State: BuilderState> BuilderState for SetBackground<State> {
        type Description = State::Description;
        type Ignored = State::Ignored;
        type Tags = State::Tags;

        type Background = self::marker::Set<self::members::Background>;
    }

    mod members {
        pub struct Description;
        pub struct Ignored;
        pub struct Tags;

        pub struct Background;
    }
}

pub struct RuleBuilder<World, State: self::rule::BuilderState = self::rule::Empty> {
    description: ::core::option::Option<::std::borrow::Cow<'static, str>>,
    ignored: ::core::option::Option<bool>,
    tags: ::core::option::Option<Tags>,

    background: ::core::option::Option<Background<World>>,
    scenarios: ::std::vec::Vec<Scenario<World>>,

    __phantom: self::marker::PhantomCovariant<State>,
}

impl<World> Rule<World> {
    pub fn builder() -> RuleBuilder<World> {
        RuleBuilder {
            description: ::core::default::Default::default(),
            ignored: ::core::default::Default::default(),
            tags: ::core::default::Default::default(),

            background: ::core::default::Default::default(),
            scenarios: ::core::default::Default::default(),

            __phantom: ::core::default::Default::default(),
        }
    }
}

impl<World, State: self::rule::BuilderState> RuleBuilder<World, State> {
    pub fn description(mut self, value: impl Into<::std::borrow::Cow<'static, str>>) -> RuleBuilder<World, self::rule::SetDescription<State>>
    where
        State::Description: self::marker::IsUnset,
    {
        self.description = ::core::option::Option::from(value.into());

        RuleBuilder {
            description: self.description,
            ignored: self.ignored,
            tags: self.tags,

            background: self.background,
            scenarios: self.scenarios,

            __phantom: ::core::default::Default::default(),
        }
    }

    pub fn ignored(mut self, value: impl Into<bool>) -> RuleBuilder<World, self::rule::SetIgnored<State>>
    where
        State::Ignored: self::marker::IsUnset,
    {
        self.ignored = ::core::option::Option::from(value.into());

        RuleBuilder {
            description: self.description,
            ignored: self.ignored,
            tags: self.tags,

            background: self.background,
            scenarios: self.scenarios,

            __phantom: ::core::default::Default::default(),
        }
    }

    pub fn tags(mut self, value: impl Into<Tags>) -> RuleBuilder<World, self::rule::SetTags<State>>
    where
        State::Tags: self::marker::IsUnset,
    {
        self.tags = ::core::option::Option::from(value.into());

        RuleBuilder {
            description: self.description,
            ignored: self.ignored,
            tags: self.tags,

            background: self.background,
            scenarios: self.scenarios,

            __phantom: ::core::default::Default::default(),
        }
    }

    pub fn background(mut self, value: impl Into<Background<World>>) -> RuleBuilder<World, self::rule::SetBackground<State>>
    where
        State::Background: self::marker::IsUnset,
    {
        self.background = ::core::option::Option::from(value.into());

        RuleBuilder {
            description: self.description,
            ignored: self.ignored,
            tags: self.tags,

            background: self.background,
            scenarios: self.scenarios,

            __phantom: ::core::default::Default::default(),
        }
    }

    pub fn scenario(mut self, value: impl Into<Scenario<World>>) -> Self {
        self.scenarios.push(value.into());
        self
    }

    pub fn scenarios<T>(mut self, values: impl IntoIterator<Item = T>) -> Self
    where
        T: Into<Scenario<World>>,
    {
        self.scenarios.extend(values.into_iter().map(Into::into));
        self
    }

    pub fn build(mut self) -> Rule<World> {
        self.propagate_ignored();
        self.propagate_tags();

        Rule {
            description: self.description,
            ignored: self.ignored,
            tags: self.tags,

            background: self.background,
            scenarios: self.scenarios,
        }
    }

    fn propagate_ignored(&mut self) {
        if let Some(ignored) = self.ignored.as_ref() {
            self.scenarios.iter_mut()
                .for_each(|scenario| scenario.ignored = ::core::option::Option::from(*ignored));
        }
    }

    /// See also: [Tag inheritance](https://cucumber.io/docs/cucumber/api/#tag-inheritance)
    fn propagate_tags(&mut self) {
        if let Some(tags) = self.tags.as_ref() {
            self.scenarios.iter_mut()
                .for_each(|scenario| scenario.tags
                    .get_or_insert_with(|| Tags::from(::std::iter::empty::<::std::borrow::Cow<'static, str>>()))
                    .extend(tags.clone()));
        }
    }
}

impl<World, State: self::rule::BuilderState> From<RuleBuilder<World, State>> for Rule<World> {
    fn from(builder: RuleBuilder<World, State>) -> Self {
        builder.build()
    }
}

mod rule {
    pub(super) use super::*;

    #[sealed]
    pub trait BuilderState: ::core::marker::Sized {
        type Description;
        type Ignored;
        type Tags;

        type Background;
    }

    pub struct Empty;

    pub struct SetDescription<State: BuilderState = Empty>(self::marker::PhantomCovariant<State>);
    pub struct SetIgnored<State: BuilderState = Empty>(self::marker::PhantomCovariant<State>);
    pub struct SetTags<State: BuilderState = Empty>(self::marker::PhantomCovariant<State>);

    pub struct SetBackground<State: BuilderState = Empty>(self::marker::PhantomCovariant<State>);

    #[sealed]
    impl BuilderState for Empty {
        type Description = self::marker::Unset<self::members::Description>;
        type Ignored = self::marker::Unset<self::members::Ignored>;
        type Tags = self::marker::Unset<self::members::Tags>;

        type Background = self::marker::Unset<self::members::Background>;
    }

    #[sealed]
    impl<State: BuilderState> BuilderState for SetDescription<State> {
        type Description = self::marker::Set<self::members::Description>;
        type Ignored = State::Ignored;
        type Tags = State::Tags;

        type Background = State::Background;
    }

    #[sealed]
    impl<State: BuilderState> BuilderState for SetIgnored<State> {
        type Description = State::Description;
        type Ignored = self::marker::Set<self::members::Ignored>;
        type Tags = State::Tags;

        type Background = State::Background;
    }

    #[sealed]
    impl<State: BuilderState> BuilderState for SetTags<State> {
        type Description = State::Description;
        type Ignored = State::Ignored;
        type Tags = self::marker::Set<self::members::Tags>;

        type Background = State::Background;
    }

    #[sealed]
    impl<State: BuilderState> BuilderState for SetBackground<State> {
        type Description = State::Description;
        type Ignored = State::Ignored;
        type Tags = State::Tags;

        type Background = self::marker::Set<self::members::Background>;
    }

    mod members {
        pub struct Description;
        pub struct Ignored;
        pub struct Tags;

        pub struct Background;
    }
}

pub struct ScenarioBuilder<World, State: self::scenario::BuilderState = self::scenario::Empty> {
    description: ::core::option::Option<::std::borrow::Cow<'static, str>>,
    ignored: ::core::option::Option<bool>,
    tags: ::core::option::Option<Tags>,

    given: ::std::vec::Vec<Step<::std::boxed::Box<dyn FnOnce(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync>>>,
    when: ::std::vec::Vec<Step<Box<dyn FnOnce(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync>>>,
    then: ::std::vec::Vec<Step<Box<dyn FnOnce(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync>>>,

    __phantom: self::marker::PhantomCovariant<State>,
}

impl<World> Scenario<World> {
    pub fn builder() -> ScenarioBuilder<World> {
        ScenarioBuilder {
            description: ::core::default::Default::default(),
            ignored: ::core::default::Default::default(),
            tags: ::core::default::Default::default(),
            
            given: ::core::default::Default::default(),
            when: ::core::default::Default::default(),
            then: ::core::default::Default::default(),

            __phantom: ::core::default::Default::default(),
        }
    }
}

impl<World, State: self::scenario::BuilderState> ScenarioBuilder<World, State>
where
    State::Given: self::marker::IsUnset,
    State::When: self::marker::IsUnset,
    State::Then: self::marker::IsUnset,
{
    pub fn description(mut self, value: impl Into<::std::borrow::Cow<'static, str>>) -> ScenarioBuilder<World, self::scenario::SetDescription<State>>
    where
        State::Description: self::marker::IsUnset,
    {
        self.description = ::core::option::Option::from(value.into());
        
        ScenarioBuilder {
            description: self.description,
            ignored: self.ignored,
            tags: self.tags,
            
            given: self.given,
            when: self.when,
            then: self.then,

            __phantom: ::core::default::Default::default(),
        }
    }

    pub fn ignored(mut self, value: impl Into<bool>) -> ScenarioBuilder<World, self::scenario::SetIgnored<State>>
    where
        State::Ignored: self::marker::IsUnset,
    {
        self.ignored = ::core::option::Option::from(value.into());
        
        ScenarioBuilder {
            description: self.description,
            ignored: self.ignored,
            tags: self.tags,
            
            given: self.given,
            when: self.when,
            then: self.then,

            __phantom: ::core::default::Default::default(),
        }
    }

    pub fn tags(mut self, value: impl Into<Tags>) -> ScenarioBuilder<World, self::scenario::SetTags<State>>
    where
        State::Tags: self::marker::IsUnset,
    {
        self.tags = ::core::option::Option::from(value.into());
        
        ScenarioBuilder {
            description: self.description,
            ignored: self.ignored,
            tags: self.tags,
            
            given: self.given,
            when: self.when,
            then: self.then,

            __phantom: ::core::default::Default::default(),
        }
    }

    pub fn given<Callback, Output>(mut self, description: impl Into<::std::borrow::Cow<'static, str>>, callback: Callback) -> ScenarioBuilder<World, self::scenario::SetGiven<State>>
    where
        Callback: FnOnce(&mut World) -> Output + ::core::marker::Send + ::core::marker::Sync + 'static,
        Output: IntoFallible,
    {
        let callback = ::std::boxed::Box::new(move |world: &mut World| (callback)(world).into_fallible())
            as ::std::boxed::Box<dyn FnOnce(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync>;

        let step = Step::builder()
            .label(StepLabel::Given)
            .description(description)
            .callback(callback)
            .build();

        self.given.push(step);

        ScenarioBuilder {
            description: self.description,
            ignored: self.ignored,
            tags: self.tags,
            
            given: self.given,
            when: self.when,
            then: self.then,

            __phantom: ::core::default::Default::default(),
        }
    }
}

#[::bon::bon]
impl<World, InnerState: self::scenario::BuilderState> ScenarioBuilder<World, self::scenario::SetGiven<InnerState>>
where
    <self::scenario::SetGiven<InnerState> as self::scenario::BuilderState>::Given: self::marker::IsSet,
    <self::scenario::SetGiven<InnerState> as self::scenario::BuilderState>::When: self::marker::IsUnset,
    <self::scenario::SetGiven<InnerState> as self::scenario::BuilderState>::Then: self::marker::IsUnset,
{
    pub fn and<Callback, Output>(self, description: impl Into<::std::borrow::Cow<'static, str>>, callback: Callback) -> ScenarioBuilder<World, self::scenario::SetGiven<self::scenario::SetGiven<InnerState>>>
    where
        Callback: FnOnce(&mut World) -> Output + ::core::marker::Send + ::core::marker::Sync + 'static,
        Output: IntoFallible,
    {
        self.conjoin_given(description, callback)
            .label(StepLabel::And)
            .call()
    }

    pub fn but<Callback, Output>(self, description: impl Into<::std::borrow::Cow<'static, str>>, callback: Callback) -> ScenarioBuilder<World, self::scenario::SetGiven<self::scenario::SetGiven<InnerState>>>
    where
        Callback: FnOnce(&mut World) -> Output + ::core::marker::Send + ::core::marker::Sync + 'static,
        Output: IntoFallible,
    {
        self.conjoin_given(description, callback)
            .label(StepLabel::But)
            .call()
    }

    #[builder]
    fn conjoin_given<Callback, Output>(mut self, #[builder(start_fn)] description: impl Into<::std::borrow::Cow<'static, str>>, #[builder(start_fn)] callback: Callback, label: StepLabel) -> ScenarioBuilder<World, self::scenario::SetGiven<self::scenario::SetGiven<InnerState>>>
    where
        Callback: FnOnce(&mut World) -> Output + ::core::marker::Send + ::core::marker::Sync + 'static,
        Output: IntoFallible,
    {
        let callback = ::std::boxed::Box::new(move |world: &mut World| (callback)(world).into_fallible())
            as ::std::boxed::Box<dyn FnOnce(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync>;

        let step = Step::builder()
            .label(label)
            .description(description)
            .callback(callback)
            .build();

        self.given.push(step);

        ScenarioBuilder {
            description: self.description,
            ignored: self.ignored,
            tags: self.tags,
            
            given: self.given,
            when: self.when,
            then: self.then,

            __phantom: ::core::default::Default::default(),
        }
    }

    pub fn when<Callback, Output>(mut self, description: impl Into<::std::borrow::Cow<'static, str>>, callback: Callback) -> ScenarioBuilder<World, self::scenario::SetWhen<self::scenario::SetGiven<InnerState>>>
    where
        Callback: FnOnce(&mut World) -> Output + ::core::marker::Send + ::core::marker::Sync + 'static,
        Output: IntoFallible,
    {
        let callback = ::std::boxed::Box::new(move |world: &mut World| (callback)(world).into_fallible())
            as ::std::boxed::Box<dyn FnOnce(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync>;

        let step = Step::builder()
            .label(StepLabel::When)
            .description(description)
            .callback(callback)
            .build();

        self.when.push(step);

        ScenarioBuilder {
            description: self.description,
            ignored: self.ignored,
            tags: self.tags,
            
            given: self.given,
            when: self.when,
            then: self.then,

            __phantom: ::core::default::Default::default(),
        }
    }
}

#[::bon::bon]
impl <World, InnerState: self::scenario::BuilderState> ScenarioBuilder<World, self::scenario::SetWhen<InnerState>>
where
    <self::scenario::SetWhen<InnerState> as self::scenario::BuilderState>::Given: self::marker::IsSet,
    <self::scenario::SetWhen<InnerState> as self::scenario::BuilderState>::When: self::marker::IsSet,
    <self::scenario::SetWhen<InnerState> as self::scenario::BuilderState>::Then: self::marker::IsUnset,
{
    pub fn and<Callback, Output>(self, description: impl Into<::std::borrow::Cow<'static, str>>, callback: Callback) -> ScenarioBuilder<World, self::scenario::SetWhen<self::scenario::SetWhen<InnerState>>>
    where
        Callback: FnOnce(&mut World) -> Output + ::core::marker::Send + ::core::marker::Sync + 'static,
        Output: IntoFallible,
    {
        self.conjoin_when(description, callback)
            .label(StepLabel::And)
            .call()
    }

    pub fn but<Callback, Output>(self, description: impl Into<::std::borrow::Cow<'static, str>>, callback: Callback) -> ScenarioBuilder<World, self::scenario::SetWhen<self::scenario::SetWhen<InnerState>>>
    where
        Callback: FnOnce(&mut World) -> Output + ::core::marker::Send + ::core::marker::Sync + 'static,
        Output: IntoFallible,
    {
        self.conjoin_when(description, callback)
            .label(StepLabel::But)
            .call()
    }

    #[builder]
    fn conjoin_when<Callback, Output>(mut self, #[builder(start_fn)] description: impl Into<::std::borrow::Cow<'static, str>>, #[builder(start_fn)] callback: Callback, label: StepLabel) -> ScenarioBuilder<World, self::scenario::SetWhen<self::scenario::SetWhen<InnerState>>>
    where
        Callback: FnOnce(&mut World) -> Output + ::core::marker::Send + ::core::marker::Sync + 'static,
        Output: IntoFallible,
    {
        let callback = ::std::boxed::Box::new(move |world: &mut World| (callback)(world).into_fallible())
            as ::std::boxed::Box<dyn FnOnce(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync>;

        let step = Step::builder()
            .label(label)
            .description(description)
            .callback(callback)
            .build();

        self.when.push(step);

        ScenarioBuilder {
            description: self.description,
            ignored: self.ignored,
            tags: self.tags,
            
            given: self.given,
            when: self.when,
            then: self.then,

            __phantom: ::core::default::Default::default(),
        }
    }

    pub fn then<Callback, Output>(mut self, description: impl Into<::std::borrow::Cow<'static, str>>, callback: Callback) -> ScenarioBuilder<World, self::scenario::SetThen<self::scenario::SetWhen<InnerState>>>
    where
        Callback: FnOnce(&World) -> Output + ::core::marker::Send + ::core::marker::Sync + 'static,
        Output: IntoFallible,
    {
        let callback = ::std::boxed::Box::new(move |world: &mut World| (callback)(world).into_fallible())
            as ::std::boxed::Box<dyn FnOnce(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync>;

        let step = Step::builder()
            .label(StepLabel::Then)
            .description(description)
            .callback(callback)
            .build();

        self.then.push(step);

        ScenarioBuilder {
            description: self.description,
            ignored: self.ignored,
            tags: self.tags,
            
            given: self.given,
            when: self.when,
            then: self.then,

            __phantom: ::core::default::Default::default(),
        }
    }
}

#[::bon::bon]
impl<World, InnerState: self::scenario::BuilderState> ScenarioBuilder<World, self::scenario::SetThen<InnerState>>
where
    <self::scenario::SetThen<InnerState> as self::scenario::BuilderState>::Given: self::marker::IsSet,
    <self::scenario::SetThen<InnerState> as self::scenario::BuilderState>::When: self::marker::IsSet,
    <self::scenario::SetThen<InnerState> as self::scenario::BuilderState>::Then: self::marker::IsSet,
{
    pub fn and<Callback, Output>(self, description: impl Into<::std::borrow::Cow<'static, str>>, callback: Callback) -> ScenarioBuilder<World, self::scenario::SetThen<self::scenario::SetThen<InnerState>>>
    where
        Callback: FnOnce(&World) -> Output + ::core::marker::Send + ::core::marker::Sync + 'static,
        Output: IntoFallible,
    {
        self.conjoin_then(description, callback)
            .label(StepLabel::And)
            .call()
    }

    pub fn but<Callback, Output>(self, description: impl Into<::std::borrow::Cow<'static, str>>, callback: Callback) -> ScenarioBuilder<World, self::scenario::SetThen<self::scenario::SetThen<InnerState>>>
    where
        Callback: FnOnce(&World) -> Output + ::core::marker::Send + ::core::marker::Sync + 'static,
        Output: IntoFallible,
    {
        self.conjoin_then(description, callback)
            .label(StepLabel::But)
            .call()
    }

    #[builder]
    fn conjoin_then<Callback, Output>(mut self, #[builder(start_fn)] description: impl Into<::std::borrow::Cow<'static, str>>, #[builder(start_fn)] callback: Callback, label: StepLabel) -> ScenarioBuilder<World, self::scenario::SetThen<self::scenario::SetThen<InnerState>>>
    where
        Callback: FnOnce(&World) -> Output + ::core::marker::Send + ::core::marker::Sync + 'static,
        Output: IntoFallible,
    {
        let callback = ::std::boxed::Box::new(move |world: &mut World| (callback)(world).into_fallible())
            as ::std::boxed::Box<dyn FnOnce(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync>;

        let step = Step::builder()
            .label(label)
            .description(description)
            .callback(callback)
            .build();

        self.then.push(step);

        ScenarioBuilder {
            description: self.description,
            ignored: self.ignored,
            tags: self.tags,
            
            given: self.given,
            when: self.when,
            then: self.then,

            __phantom: ::core::default::Default::default(),
        }
    }
}

impl<World, State: self::scenario::BuilderState> ScenarioBuilder<World, State>
where
    State: self::scenario::IsComplete,
{
    pub fn build(self) -> Scenario<World> {
        Scenario {
            description: self.description,
            ignored: self.ignored,
            tags: self.tags,

            given: self.given,
            when: self.when,
            then: self.then,
        }
    }
}

impl<World, State: self::scenario::BuilderState> From<ScenarioBuilder<World, State>> for Scenario<World>
where
    State: self::scenario::IsComplete,
{
    fn from(builder: ScenarioBuilder<World, State>) -> Self {
        builder.build()
    }
}

mod scenario {
    pub(super) use super::*;

    #[sealed]
    pub trait BuilderState: ::core::marker::Sized {
        type Description;
        type Ignored;
        type Tags;

        type Given;
        type When;
        type Then;
    }

    #[sealed]
    pub trait IsComplete: BuilderState<Given: self::marker::IsSet, When: self::marker::IsSet, Then: self::marker::IsSet> {}

    #[sealed]
    impl<State: BuilderState> IsComplete for State
    where
        State::Given: self::marker::IsSet,
        State::When: self::marker::IsSet,
        State::Then: self::marker::IsSet,
    {
    }

    pub struct Empty;

    pub struct SetDescription<State: BuilderState = Empty>(self::marker::PhantomCovariant<State>);
    pub struct SetIgnored<State: BuilderState = Empty>(self::marker::PhantomCovariant<State>);
    pub struct SetTags<State: BuilderState = Empty>(self::marker::PhantomCovariant<State>);

    pub struct SetGiven<State: BuilderState = Empty>(self::marker::PhantomCovariant<State>);
    pub struct SetWhen<State: BuilderState = Empty>(self::marker::PhantomCovariant<State>);
    pub struct SetThen<State: BuilderState = Empty>(self::marker::PhantomCovariant<State>);

    #[sealed]
    impl BuilderState for Empty {
        type Description = self::marker::Unset<self::members::Description>;
        type Ignored = self::marker::Unset<self::members::Ignored>;
        type Tags = self::marker::Unset<self::members::Tags>;

        type Given = self::marker::Unset<self::members::Given>;
        type When = self::marker::Unset<self::members::When>;
        type Then = self::marker::Unset<self::members::Then>;
    }

    #[sealed]
    impl<State: BuilderState> BuilderState for SetDescription<State> {
        type Description = self::marker::Set<self::members::Description>;
        type Ignored = State::Ignored;
        type Tags = State::Tags;

        type Given = State::Given;
        type When = State::When;
        type Then = State::Then;
    }

    #[sealed]
    impl<State: BuilderState> BuilderState for SetIgnored<State> {
        type Description = State::Description;
        type Ignored = self::marker::Set<self::members::Ignored>;
        type Tags = State::Tags;

        type Given = State::Given;
        type When = State::When;
        type Then = State::Then;
    }

    #[sealed]
    impl<State: BuilderState> BuilderState for SetTags<State> {
        type Description = State::Description;
        type Ignored = State::Ignored;
        type Tags = self::marker::Set<self::members::Tags>;

        type Given = State::Given;
        type When = State::When;
        type Then = State::Then;
    }

    #[sealed]
    impl<State: BuilderState> BuilderState for SetGiven<State> {
        type Description = State::Description;
        type Ignored = State::Ignored;
        type Tags = State::Tags;

        type Given = self::marker::Set<self::members::Given>;
        type When = State::When;
        type Then = State::Then;
    }

    #[sealed]
    impl<State: BuilderState> BuilderState for SetWhen<State> {
        type Description = State::Description;
        type Ignored = State::Ignored;
        type Tags = State::Tags;

        type Given = State::Given;
        type When = self::marker::Set<self::members::When>;
        type Then = State::Then;
    }

    #[sealed]
    impl<State: BuilderState> BuilderState for SetThen<State> {
        type Description = State::Description;
        type Ignored = State::Ignored;
        type Tags = State::Tags;

        type Given = State::Given;
        type When = State::When;
        type Then = self::marker::Set<self::members::Then>;
    }

    mod members {
        pub struct Description;
        pub struct Ignored;
        pub struct Tags;

        pub struct Given;
        pub struct When;
        pub struct Then;
    }
}

pub struct BackgroundBuilder<World, State: self::background::BuilderState = self::background::Empty> {
    description: ::core::option::Option<::std::borrow::Cow<'static, str>>,
    ignored: ::core::option::Option<bool>,

    given: ::std::vec::Vec<Step<aliases::sync::Arc<dyn Fn(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync>>>,

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

    pub fn given<Callback, Output>(mut self, description: impl Into<::std::borrow::Cow<'static, str>>, callback: Callback) -> BackgroundBuilder<World, self::background::SetGiven<State>>
    where
        Callback: Fn(&mut World) -> Output + ::core::marker::Send + ::core::marker::Sync + 'static,
        Output: IntoFallible,
    {
        let callback  = aliases::sync::Arc::new(move |world: &mut World| (callback)(world).into_fallible())
            as aliases::sync::Arc<dyn Fn(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync>;

        let step = Step::builder()
            .label(StepLabel::Given)
            .description(description)
            .callback(callback)
            .build();

        self.given.push(step);

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
    pub fn and<Callback, Output>(self, description: impl Into<::std::borrow::Cow<'static, str>>, callback: Callback) -> BackgroundBuilder<World, self::background::SetGiven<self::background::SetGiven<InnerState>>>
    where
        Callback: Fn(&mut World) -> Output + ::core::marker::Send + ::core::marker::Sync + 'static,
        Output: IntoFallible,
    {
        self.conjoin(description, callback)
            .label(StepLabel::And)
            .call()
    }

    pub fn but<Callback, Output>(self, description: impl Into<::std::borrow::Cow<'static, str>>, callback: Callback) -> BackgroundBuilder<World, self::background::SetGiven<self::background::SetGiven<InnerState>>>
    where
        Callback: Fn(&mut World) -> Output + ::core::marker::Send + ::core::marker::Sync + 'static,
        Output: IntoFallible,
    {
        self.conjoin(description, callback)
            .label(StepLabel::But)
            .call()
    }

    #[builder]
    fn conjoin<Callback, Output>(mut self, #[builder(start_fn)] description: impl Into<::std::borrow::Cow<'static, str>>, #[builder(start_fn)] callback: Callback, label: StepLabel) -> BackgroundBuilder<World, self::background::SetGiven<self::background::SetGiven<InnerState>>>
    where
        Callback: Fn(&mut World) -> Output + ::core::marker::Send + ::core::marker::Sync + 'static,
        Output: IntoFallible,
    {
        let callback  = aliases::sync::Arc::new(move |world: &mut World| (callback)(world).into_fallible())
            as aliases::sync::Arc<dyn Fn(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync>;

        let step = Step::builder()
            .label(label)
            .description(description)
            .callback(callback)
            .build();

        self.given.push(step);

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
            
            given: self.given,
        }
    }
}

impl<World, State: self::background::BuilderState> From<BackgroundBuilder<World, State>> for Background<World>
where
    State: self::background::IsComplete,
{
    fn from(builder: BackgroundBuilder<World, State>) -> Self {
        builder.build()
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

pub struct HookBuilder<Callback, State: self::hook::BuilderState = self::hook::Empty> {
    tags: ::core::option::Option<Tags>,
    callback: ::core::option::Option<Callback>,

    __phantom: self::marker::PhantomCovariant<State>,
}

impl<Callback> Hook<Callback> {
    pub(crate) fn builder() -> HookBuilder<Callback> {
        HookBuilder {
            tags: ::core::default::Default::default(),
            callback: ::core::default::Default::default(),

            __phantom: ::core::default::Default::default(),
        }
    }
}

impl<Callback, State: self::hook::BuilderState> HookBuilder<Callback, State> {
    pub(crate) fn tags(mut self, value: impl Into<Tags>) -> HookBuilder<Callback, self::hook::SetTags<State>>
    where
        State::Tags: self::marker::IsUnset,
    {
        self.tags = ::core::option::Option::from(value.into());

        HookBuilder {
            tags: self.tags,
            callback: self.callback,

            __phantom: ::core::default::Default::default(),
        }
    }

    pub(crate) fn callback(mut self, value: Callback) -> HookBuilder<Callback, self::hook::SetCallback<State>>
    where
        State::Callback: self::marker::IsUnset,
    {
        self.callback = ::core::option::Option::from(value);

        HookBuilder {
            tags: self.tags,
            callback: self.callback,

            __phantom: ::core::default::Default::default(),
        }
    }
}

impl<Callback, State: self::hook::BuilderState> HookBuilder<Callback, State>
where
    State: self::hook::IsComplete,
{
    pub(crate) fn build(self) -> Hook<Callback> {
        Hook {
            tags: self.tags,
            callback: unsafe { self.callback.unwrap_unchecked() },
        }
    }
}

mod hook {
    pub(super) use super::*;

    #[sealed]
    pub trait BuilderState: ::core::marker::Sized {
        type Tags;
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

    pub struct SetTags<State: BuilderState = Empty>(self::marker::PhantomCovariant<State>);
    pub struct SetCallback<State: BuilderState = Empty>(self::marker::PhantomCovariant<State>);

    #[sealed]
    impl BuilderState for Empty {
        type Tags = self::marker::Unset<self::members::Tags>;
        type Callback = self::marker::Unset<self::members::Callback>;
    }

    #[sealed]
    impl<State: BuilderState> BuilderState for SetTags<State> {
        type Tags = self::marker::Set<self::members::Tags>;
        type Callback = State::Callback;
    }

    #[sealed]
    impl<State: BuilderState> BuilderState for SetCallback<State> {
        type Tags = State::Tags;
        type Callback = self::marker::Set<self::members::Callback>;
    }

    mod members {
        pub struct Tags;
        pub struct Callback;
    }
}

struct StepBuilder<Callback, State: self::step::BuilderState = self::step::Empty> {
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
}

impl<Callback, State: self::step::BuilderState> StepBuilder<Callback, State>
where 
    State: self::step::IsComplete,
{
    fn build(self) -> Step<Callback> {
        Step {
            label: unsafe { self.label.unwrap_unchecked() },
            description: unsafe { self.description.unwrap_unchecked() },
            callback: unsafe { self.callback.unwrap_unchecked() },
        }
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
    pub trait IsComplete: BuilderState<Label: self::marker::IsSet, Description: self::marker::IsSet, Callback: self::marker::IsSet> {}

    #[sealed]
    impl<State: BuilderState> IsComplete for State
    where
        State::Label: self::marker::IsSet,
        State::Description: self::marker::IsSet,
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

impl Tags {
    fn extend(&mut self, tags: Tags) {
        self.0.extend(tags.0.into_iter());
    }
}

impl<I, T> From<I> for Tags
where
    I: IntoIterator<Item = T>,
    T: Into<::std::borrow::Cow<'static, str>>,
{
    fn from(values: I) -> Self {
        Self(values.into_iter().map(Into::into).collect())
    }
}

#[sealed]
pub trait IntoFallible<T = ()>: ::core::marker::Sized {
    fn into_fallible(self) -> Fallible<T>;
}

#[sealed]
impl<T> IntoFallible<T> for Fallible<T> {
    fn into_fallible(self) -> Fallible<T> {
        self
    }
}

#[sealed]
impl IntoFallible for () {
    fn into_fallible(self) -> Fallible {
        Ok(())
    }
}

impl<Message> From<Message> for Failed
where
    Message: Into<::std::borrow::Cow<'static, str>>,
{
    fn from(message: Message) -> Failed {
        Failed {
            message: message.into(),
        }
    }
}

pub(crate) mod marker {
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
