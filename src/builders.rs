use ::sealed::sealed;

use crate::models::*;

pub struct FeatureBuilder<World, RandomState: ::core::hash::BuildHasher = ::std::hash::RandomState, State: self::feature::BuilderState = self::feature::Empty> {
    description: ::core::option::Option<::std::borrow::Cow<'static, str>>,
    ignored: ::core::option::Option<bool>,
    tags: ::core::option::Option<Tags<RandomState>>,

    background: ::core::option::Option<Background<World>>,
    scenarios: ::std::vec::Vec<Scenario<World, RandomState>>,
    rules: ::std::vec::Vec<Rule<World, RandomState>>,

    __phantom: self::marker::PhantomCovariant<State>,
}

impl<World, RandomState: ::core::hash::BuildHasher> Feature<World, RandomState> {
    pub fn builder() -> FeatureBuilder<World, RandomState> {
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

impl<World, RandomState: ::core::hash::BuildHasher, State: self::feature::BuilderState> FeatureBuilder<World, RandomState, State> {
    pub fn description(mut self, value: impl Into<::std::borrow::Cow<'static, str>>) -> FeatureBuilder<World, RandomState, self::feature::SetDescription<State>>
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

    pub fn ignored(mut self, value: impl Into<bool>) -> FeatureBuilder<World, RandomState, self::feature::SetIgnored<State>>
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

    pub fn tags(mut self, value: impl IntoTags<RandomState>) -> FeatureBuilder<World, RandomState, self::feature::SetTags<State>>
    where
        State::Tags: self::marker::IsUnset,
    {
        self.tags = ::core::option::Option::from(value.into_tags());

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

    pub fn background(mut self, value: Background<World>) -> FeatureBuilder<World, RandomState, self::feature::SetBackground<State>>
    where
        State::Background: self::marker::IsUnset,
    {
        self.background = ::core::option::Option::from(value);

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

    pub fn scenario(mut self, value: Scenario<World, RandomState>) -> FeatureBuilder<World, RandomState, self::feature::SetScenario<State>>
    where
        State::Scenario: self::marker::IsUnset,
    {
        self.scenarios.push(value);

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

    pub fn scenarios<T>(mut self, values: impl IntoIterator<Item = T>) -> FeatureBuilder<World, RandomState, self::feature::SetScenario<State>>
    where
        T: Into<Scenario<World, RandomState>>,
    {
        self.scenarios.extend(values.into_iter().map(Into::into));

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

    pub fn rule(mut self, value: impl IntoRule<World, RandomState>) -> FeatureBuilder<World, RandomState, self::feature::SetRule<State>> {
        self.rules.push(value.into_rule());

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

    pub fn rules<T>(mut self, values: impl IntoIterator<Item = T>) -> FeatureBuilder<World, RandomState, self::feature::SetRule<State>>
    where
        T: IntoRule<World, RandomState>,
    {
        self.rules.extend(values.into_iter().map(IntoRule::into_rule));

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
}

mod feature {
    pub(super) use super::*;

    #[sealed]
    pub trait BuilderState: ::core::marker::Sized {
        type Description;
        type Ignored;
        type Tags;

        type Background;
        type Scenario;
        type Rule;
    }

    #[sealed]
    pub trait IsComplete: BuilderState<Scenario: self::marker::IsSet> {}

    #[sealed]
    impl<State: BuilderState> IsComplete for State
    where
        State::Scenario: self::marker::IsSet,
    {
    }

    pub struct Empty;

    pub struct SetDescription<State: BuilderState = Empty>(self::marker::PhantomCovariant<State>);
    pub struct SetIgnored<State: BuilderState = Empty>(self::marker::PhantomCovariant<State>);
    pub struct SetTags<State: BuilderState = Empty>(self::marker::PhantomCovariant<State>);

    pub struct SetBackground<State: BuilderState = Empty>(self::marker::PhantomCovariant<State>);
    pub struct SetScenario<State: BuilderState = Empty>(self::marker::PhantomCovariant<State>);
    pub struct SetRule<State: BuilderState = Empty>(self::marker::PhantomCovariant<State>);

    #[sealed]
    impl BuilderState for Empty {
        type Description = self::marker::Unset<self::members::Description>;
        type Ignored = self::marker::Unset<self::members::Ignored>;
        type Tags = self::marker::Unset<self::members::Tags>;

        type Background = self::marker::Unset<self::members::Background>;
        type Scenario = self::marker::Unset<self::members::Scenario>;
        type Rule = self::marker::Unset<self::members::Rule>;
    }

    #[sealed]
    impl<State: BuilderState> BuilderState for SetDescription<State> {
        type Description = self::marker::Set<self::members::Description>;
        type Ignored = State::Ignored;
        type Tags = State::Tags;

        type Background = State::Background;
        type Scenario = State::Scenario;
        type Rule = State::Rule;
    }

    #[sealed]
    impl<State: BuilderState> BuilderState for SetIgnored<State> {
        type Description = State::Description;
        type Ignored = self::marker::Set<self::members::Ignored>;
        type Tags = State::Tags;

        type Background = State::Background;
        type Scenario = State::Scenario;
        type Rule = State::Rule;
    }

    #[sealed]
    impl<State: BuilderState> BuilderState for SetTags<State> {
        type Description = State::Description;
        type Ignored = State::Ignored;
        type Tags = self::marker::Set<self::members::Tags>;

        type Background = State::Background;
        type Scenario = State::Scenario;
        type Rule = State::Rule;
    }

    #[sealed]
    impl<State: BuilderState> BuilderState for SetBackground<State> {
        type Description = State::Description;
        type Ignored = State::Ignored;
        type Tags = State::Tags;

        type Background = self::marker::Set<self::members::Background>;
        type Scenario = State::Scenario;
        type Rule = State::Rule;
    }

    #[sealed]
    impl<State: BuilderState> BuilderState for SetScenario<State> {
        type Description = State::Description;
        type Ignored = State::Ignored;
        type Tags = State::Tags;

        type Background = State::Background;
        type Scenario = self::marker::Set<self::members::Scenario>;
        type Rule = State::Rule;
    }

    #[sealed]
    impl<State: BuilderState> BuilderState for SetRule<State> {
        type Description = State::Description;
        type Ignored = State::Ignored;
        type Tags = State::Tags;

        type Background = State::Background;
        type Scenario = State::Scenario;
        type Rule = self::marker::Set<self::members::Rule>;
    }

    mod members {
        pub struct Description;
        pub struct Ignored;
        pub struct Tags;

        pub struct Background;
        pub struct Scenario;
        pub struct Rule;
    }
}

pub struct RuleBuilder<World, RandomState: ::core::hash::BuildHasher = ::std::hash::RandomState, State: self::rule::BuilderState = self::rule::Empty> {
    description: ::core::option::Option<::std::borrow::Cow<'static, str>>,
    ignored: ::core::option::Option<bool>,
    tags: ::core::option::Option<Tags<RandomState>>,

    background: ::core::option::Option<Background<World>>,
    scenarios: ::std::vec::Vec<Scenario<World, RandomState>>,

    __phantom: self::marker::PhantomCovariant<State>,
}

impl<World, RandomState: ::core::hash::BuildHasher> Rule<World, RandomState> {
    pub fn builder() -> RuleBuilder<World, RandomState> {
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

impl<World, RandomState: ::core::hash::BuildHasher, State: self::rule::BuilderState> RuleBuilder<World, RandomState, State> {
    pub fn description(mut self, value: impl Into<::std::borrow::Cow<'static, str>>) -> RuleBuilder<World, RandomState, self::rule::SetDescription<State>>
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

    pub fn ignored(mut self, value: impl Into<bool>) -> RuleBuilder<World, RandomState, self::rule::SetIgnored<State>>
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

    pub fn tags(mut self, value: impl IntoTags<RandomState>) -> RuleBuilder<World, RandomState, self::rule::SetTags<State>>
    where
        State::Tags: self::marker::IsUnset,
    {
        self.tags = ::core::option::Option::from(value.into_tags());

                RuleBuilder {
            description: self.description,
            ignored: self.ignored,
            tags: self.tags,

            background: self.background,
            scenarios: self.scenarios,

            __phantom: ::core::default::Default::default(),
        }
    }

    pub fn background(mut self, value: impl IntoBackground<World>) -> RuleBuilder<World, RandomState, self::rule::SetBackground<State>>
    where
        State::Background: self::marker::IsUnset,
    {
        self.background = ::core::option::Option::from(value.into_background());

                RuleBuilder {
            description: self.description,
            ignored: self.ignored,
            tags: self.tags,

            background: self.background,
            scenarios: self.scenarios,

            __phantom: ::core::default::Default::default(),
        }
    }

    pub fn scenario(mut self, value: impl IntoScenario<World, RandomState>) -> RuleBuilder<World, RandomState, self::rule::SetScenario<State>> {
        self.scenarios.push(value.into_scenario());

        RuleBuilder {
            description: self.description,
            ignored: self.ignored,
            tags: self.tags,

            background: self.background,
            scenarios: self.scenarios,

            __phantom: ::core::default::Default::default(),
        }
    }

    pub fn scenarios<T>(mut self, values: impl IntoIterator<Item = T>) -> RuleBuilder<World, RandomState, self::rule::SetScenario<State>>
    where
        T: IntoScenario<World, RandomState>,
    {
        self.scenarios.extend(values.into_iter().map(IntoScenario::into_scenario));

        RuleBuilder {
            description: self.description,
            ignored: self.ignored,
            tags: self.tags,

            background: self.background,
            scenarios: self.scenarios,

            __phantom: ::core::default::Default::default(),
        }
    }
}

impl<World, RandomState: ::core::hash::BuildHasher, State: self::rule::BuilderState> RuleBuilder<World, RandomState, State>
where
    State: self::rule::IsComplete,
{
    pub fn build(self) -> Rule<World, RandomState> {
        Rule {
            description: self.description,
            ignored: self.ignored,
            tags: self.tags,

            background: self.background,
            scenarios: self.scenarios,
        }
    }
}

#[sealed]
impl<World, RandomState: ::core::hash::BuildHasher, State: self::rule::BuilderState> IntoRule<World, RandomState> for RuleBuilder<World, RandomState, State>
where
    State: self::rule::IsComplete,
{
    fn into_rule(self) -> Rule<World, RandomState> {
        self.build()
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
        type Scenario;
    }

    #[sealed]
    pub trait IsComplete: BuilderState<Scenario: self::marker::IsSet> {}

    #[sealed]
    impl<State: BuilderState> IsComplete for State
    where
        State::Scenario: self::marker::IsSet,
    {
    }

    pub struct Empty;

    pub struct SetDescription<State: BuilderState = Empty>(self::marker::PhantomCovariant<State>);
    pub struct SetIgnored<State: BuilderState = Empty>(self::marker::PhantomCovariant<State>);
    pub struct SetTags<State: BuilderState = Empty>(self::marker::PhantomCovariant<State>);

    pub struct SetBackground<State: BuilderState = Empty>(self::marker::PhantomCovariant<State>);
    pub struct SetScenario<State: BuilderState = Empty>(self::marker::PhantomCovariant<State>);

    #[sealed]
    impl BuilderState for Empty {
        type Description = self::marker::Unset<self::members::Description>;
        type Ignored = self::marker::Unset<self::members::Ignored>;
        type Tags = self::marker::Unset<self::members::Tags>;

        type Background = self::marker::Unset<self::members::Background>;
        type Scenario = self::marker::Unset<self::members::Scenario>;
    }

    #[sealed]
    impl<State: BuilderState> BuilderState for SetDescription<State> {
        type Description = self::marker::Set<self::members::Description>;
        type Ignored = State::Ignored;
        type Tags = State::Tags;

        type Background = State::Background;
        type Scenario = State::Scenario;
    }

    #[sealed]
    impl<State: BuilderState> BuilderState for SetIgnored<State> {
        type Description = State::Description;
        type Ignored = self::marker::Set<self::members::Ignored>;
        type Tags = State::Tags;

        type Background = State::Background;
        type Scenario = State::Scenario;
    }

    #[sealed]
    impl<State: BuilderState> BuilderState for SetTags<State> {
        type Description = State::Description;
        type Ignored = State::Ignored;
        type Tags = self::marker::Set<self::members::Tags>;

        type Background = State::Background;
        type Scenario = State::Scenario;
    }

    #[sealed]
    impl<State: BuilderState> BuilderState for SetBackground<State> {
        type Description = State::Description;
        type Ignored = State::Ignored;
        type Tags = State::Tags;

        type Background = self::marker::Set<self::members::Background>;
        type Scenario = State::Scenario;
    }

    #[sealed]
    impl<State: BuilderState> BuilderState for SetScenario<State> {
        type Description = State::Description;
        type Ignored = State::Ignored;
        type Tags = State::Tags;

        type Background = State::Background;
        type Scenario = self::marker::Set<self::members::Scenario>;
    }

    mod members {
        pub struct Description;
        pub struct Ignored;
        pub struct Tags;

        pub struct Background;
        pub struct Scenario;
    }
}

pub struct ScenarioBuilder<World, RandomState: ::core::hash::BuildHasher = ::std::hash::RandomState, State: self::scenario::BuilderState = self::scenario::Empty> {
    description: ::core::option::Option<::std::borrow::Cow<'static, str>>,
    ignored: ::core::option::Option<bool>,
    tags: ::core::option::Option<Tags<RandomState>>,

    given: ::core::option::Option<Steps<::std::boxed::Box<dyn FnOnce(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync>>>,
    when: ::core::option::Option<Steps<Box<dyn FnOnce(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync>>>,
    then: ::core::option::Option<Steps<Box<dyn FnOnce(&World) -> Fallible + ::core::marker::Send + ::core::marker::Sync>>>,

    __phantom: self::marker::PhantomCovariant<State>,
}

impl<World, RandomState: ::core::hash::BuildHasher> Scenario<World, RandomState> {
    pub fn builder() -> ScenarioBuilder<World, RandomState> {
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

impl<World, RandomState: ::core::hash::BuildHasher, State: self::scenario::BuilderState> ScenarioBuilder<World, RandomState, State>
where
    State::Given: self::marker::IsUnset,
    State::When: self::marker::IsUnset,
    State::Then: self::marker::IsUnset,
{
    pub fn description(mut self, value: impl Into<::std::borrow::Cow<'static, str>>) -> ScenarioBuilder<World, RandomState, self::scenario::SetDescription<State>>
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

    pub fn ignored(mut self, value: impl Into<bool>) -> ScenarioBuilder<World, RandomState, self::scenario::SetIgnored<State>>
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

    pub fn tags(mut self, value: impl IntoTags<RandomState>) -> ScenarioBuilder<World, RandomState, self::scenario::SetTags<State>>
    where
        State::Tags: self::marker::IsUnset,
    {
        self.tags = ::core::option::Option::from(value.into_tags());
        
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

    pub fn given(mut self, description: impl Into<::std::borrow::Cow<'static, str>>, callback: impl FnOnce(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync + 'static) -> ScenarioBuilder<World, RandomState, self::scenario::SetGiven<State>> {
        let step = Step::builder()
            .label(StepLabel::Given)
            .description(description)
            .callback(::std::boxed::Box::new(callback) as ::std::boxed::Box<dyn FnOnce(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync>)
            .build();

        let steps = Steps::builder()
            .step(step)
            .build();

        self.given = ::core::option::Option::from(steps);

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
impl<World, RandomState: ::core::hash::BuildHasher, InnerState: self::scenario::BuilderState> ScenarioBuilder<World, RandomState, self::scenario::SetGiven<InnerState>>
where
    <self::scenario::SetGiven<InnerState> as self::scenario::BuilderState>::Given: self::marker::IsSet,
    <self::scenario::SetGiven<InnerState> as self::scenario::BuilderState>::When: self::marker::IsUnset,
    <self::scenario::SetGiven<InnerState> as self::scenario::BuilderState>::Then: self::marker::IsUnset,
{
    pub fn and(self, description: impl Into<::std::borrow::Cow<'static, str>>, callback: impl FnOnce(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync + 'static) -> ScenarioBuilder<World, RandomState, self::scenario::SetGiven<self::scenario::SetGiven<InnerState>>> {
        self.conjoin_given(description, callback)
            .label(StepLabel::And)
            .call()
    }

    pub fn but(self, description: impl Into<::std::borrow::Cow<'static, str>>, callback: impl FnOnce(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync + 'static) -> ScenarioBuilder<World, RandomState, self::scenario::SetGiven<self::scenario::SetGiven<InnerState>>> {
        self.conjoin_given(description, callback)
            .label(StepLabel::But)
            .call()
    }

    #[builder]
    fn conjoin_given(mut self, #[builder(start_fn)] description: impl Into<::std::borrow::Cow<'static, str>>, #[builder(start_fn)] callback: impl FnOnce(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync + 'static, label: StepLabel) -> ScenarioBuilder<World, RandomState, self::scenario::SetGiven<self::scenario::SetGiven<InnerState>>> {
        let step = Step::builder()
            .label(label)
            .description(description)
            .callback(::std::boxed::Box::new(callback) as ::std::boxed::Box<dyn FnOnce(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync>)
            .build();

        unsafe { self.given.as_mut().unwrap_unchecked() }.0.push(step);

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

    pub fn when(mut self, description: impl Into<::std::borrow::Cow<'static, str>>, callback: impl FnOnce(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync + 'static) -> ScenarioBuilder<World, RandomState, self::scenario::SetWhen<self::scenario::SetGiven<InnerState>>> {
        let step = Step::builder()
            .label(StepLabel::When)
            .description(description)
            .callback(::std::boxed::Box::new(callback) as ::std::boxed::Box<dyn FnOnce(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync>)
            .build();

        let steps = Steps::builder()
            .step(step)
            .build();

        self.when = ::core::option::Option::from(steps);

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
impl <World, RandomState: ::core::hash::BuildHasher, InnerState: self::scenario::BuilderState> ScenarioBuilder<World, RandomState, self::scenario::SetWhen<InnerState>>
where
    <self::scenario::SetWhen<InnerState> as self::scenario::BuilderState>::Given: self::marker::IsSet,
    <self::scenario::SetWhen<InnerState> as self::scenario::BuilderState>::When: self::marker::IsSet,
    <self::scenario::SetWhen<InnerState> as self::scenario::BuilderState>::Then: self::marker::IsUnset,
{
    pub fn and(self, description: impl Into<::std::borrow::Cow<'static, str>>, callback: impl FnOnce(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync + 'static) -> ScenarioBuilder<World, RandomState, self::scenario::SetWhen<self::scenario::SetWhen<InnerState>>> {
        self.conjoin_when(description, callback)
            .label(StepLabel::And)
            .call()
    }

    pub fn but(self, description: impl Into<::std::borrow::Cow<'static, str>>, callback: impl FnOnce(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync + 'static) -> ScenarioBuilder<World, RandomState, self::scenario::SetWhen<self::scenario::SetWhen<InnerState>>> {
        self.conjoin_when(description, callback)
            .label(StepLabel::But)
            .call()
    }

    #[builder]
    fn conjoin_when(mut self, #[builder(start_fn)] description: impl Into<::std::borrow::Cow<'static, str>>, #[builder(start_fn)] callback: impl FnOnce(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync + 'static, label: StepLabel) -> ScenarioBuilder<World, RandomState, self::scenario::SetWhen<self::scenario::SetWhen<InnerState>>> {
        let step = Step::builder()
            .label(label)
            .description(description)
            .callback(::std::boxed::Box::new(callback) as ::std::boxed::Box<dyn FnOnce(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync>)
            .build();

        unsafe { self.when.as_mut().unwrap_unchecked() }.0.push(step);

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

    pub fn then(mut self, description: impl Into<::std::borrow::Cow<'static, str>>, callback: impl FnOnce(&World) -> Fallible + ::core::marker::Send + ::core::marker::Sync + 'static) -> ScenarioBuilder<World, RandomState, self::scenario::SetThen<self::scenario::SetWhen<InnerState>>> {
        let step = Step::builder()
            .label(StepLabel::Then)
            .description(description)
            .callback(::std::boxed::Box::new(callback) as ::std::boxed::Box<dyn FnOnce(&World) -> Fallible + ::core::marker::Send + ::core::marker::Sync>)
            .build();

        let steps = Steps::builder()
            .step(step)
            .build();

        self.then = ::core::option::Option::from(steps);

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
impl<World, RandomState: ::core::hash::BuildHasher, InnerState: self::scenario::BuilderState> ScenarioBuilder<World, RandomState, self::scenario::SetThen<InnerState>>
where
    <self::scenario::SetThen<InnerState> as self::scenario::BuilderState>::Given: self::marker::IsSet,
    <self::scenario::SetThen<InnerState> as self::scenario::BuilderState>::When: self::marker::IsSet,
    <self::scenario::SetThen<InnerState> as self::scenario::BuilderState>::Then: self::marker::IsSet,
{
    pub fn and(self, description: impl Into<::std::borrow::Cow<'static, str>>, callback: impl FnOnce(&World) -> Fallible + ::core::marker::Send + ::core::marker::Sync + 'static) -> ScenarioBuilder<World, RandomState, self::scenario::SetThen<self::scenario::SetThen<InnerState>>> {
        self.conjoin_then(description, callback)
            .label(StepLabel::And)
            .call()
    }

    pub fn but(self, description: impl Into<::std::borrow::Cow<'static, str>>, callback: impl FnOnce(&World) -> Fallible + ::core::marker::Send + ::core::marker::Sync + 'static) -> ScenarioBuilder<World, RandomState, self::scenario::SetThen<self::scenario::SetThen<InnerState>>> {
        self.conjoin_then(description, callback)
            .label(StepLabel::But)
            .call()
    }

    #[builder]
    fn conjoin_then(mut self, #[builder(start_fn)] description: impl Into<::std::borrow::Cow<'static, str>>, #[builder(start_fn)] callback: impl FnOnce(&World) -> Fallible + ::core::marker::Send + ::core::marker::Sync + 'static, label: StepLabel) -> ScenarioBuilder<World, RandomState, self::scenario::SetThen<self::scenario::SetThen<InnerState>>> {
        let step = Step::builder()
            .label(label)
            .description(description)
            .callback(::std::boxed::Box::new(callback) as ::std::boxed::Box<dyn FnOnce(&World) -> Fallible + ::core::marker::Send + ::core::marker::Sync>)
            .build();

        unsafe { self.then.as_mut().unwrap_unchecked() }.0.push(step);

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

impl<World, RandomState: ::core::hash::BuildHasher, State: self::scenario::BuilderState> ScenarioBuilder<World, RandomState, State>
where
    State: self::scenario::IsComplete,
{
    pub fn build(self) -> Scenario<World, RandomState> {
        Scenario {
            description: self.description,
            ignored: self.ignored,
            tags: self.tags,

            given: unsafe { self.given.unwrap_unchecked() },
            when: unsafe { self.when.unwrap_unchecked() },
            then: unsafe { self.then.unwrap_unchecked() },
        }
    }
}

#[sealed]
impl<World, RandomState: ::core::hash::BuildHasher, State: self::scenario::BuilderState> IntoScenario<World, RandomState> for ScenarioBuilder<World, RandomState, State>
where
    State: self::scenario::IsComplete,
{
    fn into_scenario(self) -> Scenario<World, RandomState> {
        self.build()
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

    given: ::core::option::Option<Steps<::std::rc::Rc<dyn Fn(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync>>>,

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

    pub fn given(mut self, description: impl Into<::std::borrow::Cow<'static, str>>, callback: impl Fn(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync + 'static) -> BackgroundBuilder<World, self::background::SetGiven<State>> {
        let step = Step::builder()
            .label(StepLabel::Given)
            .description(description)
            .callback(::std::rc::Rc::new(callback) as ::std::rc::Rc<dyn Fn(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync>)
            .build();

        let steps = Steps::builder()
            .step(step)
            .build();

        self.given = ::core::option::Option::from(steps);

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
    pub fn and(self, description: impl Into<::std::borrow::Cow<'static, str>>, callback: impl Fn(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync + 'static) -> BackgroundBuilder<World, self::background::SetGiven<self::background::SetGiven<InnerState>>> {
        self.conjoin(description, callback)
            .label(StepLabel::And)
            .call()
    }

    pub fn but(self, description: impl Into<::std::borrow::Cow<'static, str>>, callback: impl Fn(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync + 'static) -> BackgroundBuilder<World, self::background::SetGiven<self::background::SetGiven<InnerState>>> {
        self.conjoin(description, callback)
            .label(StepLabel::But)
            .call()
    }

    #[builder]
    fn conjoin(mut self, #[builder(start_fn)] description: impl Into<::std::borrow::Cow<'static, str>>, #[builder(start_fn)] callback: impl Fn(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync + 'static, label: StepLabel) -> BackgroundBuilder<World, self::background::SetGiven<self::background::SetGiven<InnerState>>> {
        let step = Step::builder()
            .label(label)
            .description(description)
            .callback(::std::rc::Rc::new(callback) as ::std::rc::Rc<dyn Fn(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync>)
            .build();

        unsafe { self.given.as_mut().unwrap_unchecked() }.0.push(step);

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
            given: unsafe { self.given.unwrap_unchecked() },
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

mod hook {
    pub(super) use super::*;

    #[sealed]
    pub trait BuilderState: ::core::marker::Sized {
        type Description;
        type Tags;
        type Callback;
    }

    #[sealed]
    pub trait IsComplete: BuilderState<Description: self::marker::IsSet, Tags: self::marker::IsSet, Callback: self::marker::IsSet> {}

    #[sealed]
    impl<State: BuilderState> IsComplete for State
    where
        State::Description: self::marker::IsSet,
        State::Tags: self::marker::IsSet,
        State::Callback: self::marker::IsSet,
    {
    }

    pub struct Empty;

    pub struct SetDescription<State: BuilderState = Empty>(self::marker::PhantomCovariant<State>);
    pub struct SetTags<State: BuilderState = Empty>(self::marker::PhantomCovariant<State>);
    pub struct SetCallback<State: BuilderState = Empty>(self::marker::PhantomCovariant<State>);

    #[sealed]
    impl BuilderState for Empty {
        type Description = self::marker::Unset<self::members::Description>;
        type Tags = self::marker::Unset<self::members::Tags>;
        type Callback = self::marker::Unset<self::members::Callback>;
    }

    #[sealed]
    impl<State: BuilderState> BuilderState for SetDescription<State> {
        type Description = self::marker::Set<self::members::Description>;
        type Tags = State::Tags;
        type Callback = State::Callback;
    }

    #[sealed]
    impl<State: BuilderState> BuilderState for SetTags<State> {
        type Description = State::Description;
        type Tags = self::marker::Set<self::members::Tags>;
        type Callback = State::Callback;
    }

    #[sealed]
    impl<State: BuilderState> BuilderState for SetCallback<State> {
        type Description = State::Description;
        type Tags = State::Tags;
        type Callback = self::marker::Set<self::members::Callback>;
    }

    mod members {
        pub struct Description;
        pub struct Tags;
        pub struct Callback;
    }
}

#[sealed]
impl<I, T> IntoTags for I
where
    I: IntoIterator<Item = T>,
    T: Into<::std::borrow::Cow<'static, str>>,
{
    fn into_tags(self) -> Tags {
        Tags(self.into_iter().map(Into::into).collect())
    }
}

pub(crate) struct StepsBuilder<Callback>(::std::vec::Vec<Step<Callback>>);

impl<Callback> Steps<Callback> {
    fn builder() -> StepsBuilder<Callback> {
        StepsBuilder(::core::default::Default::default())
    }
}

impl<Callback> StepsBuilder<Callback> {
    fn step(mut self, value: Step<Callback>) -> Self {
        self.0.push(value);
        self
    }

    #[allow(dead_code)]
    fn steps(mut self, values: impl IntoIterator<Item = Step<Callback>>) -> Self {
        self.0.extend(values.into_iter());
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

impl<T> From<T> for Failed
where
    T: Into<::std::borrow::Cow<'static, str>>,
{
    fn from(message: T) -> Failed {
        Failed {
            message: message.into(),
        }
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
