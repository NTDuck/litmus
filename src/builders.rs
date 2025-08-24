use ::sealed::sealed;

use crate::models::*;

pub struct SuiteBuilder<World, RandomState: ::core::hash::BuildHasher = ::std::hash::RandomState> {
    features: ::std::vec::Vec<Feature<World, RandomState>>,

    before_scenario_hooks: ::std::vec::Vec<Hook<::std::rc::Rc<dyn Fn(&mut World) + ::core::marker::Send + ::core::marker::Sync>, RandomState>>,
    after_scenario_hooks: ::std::vec::Vec<Hook<::std::rc::Rc<dyn Fn(&mut World) + ::core::marker::Send + ::core::marker::Sync>, RandomState>>,

    before_step_hooks: ::std::vec::Vec<Hook<::std::rc::Rc<dyn Fn(&mut World) + ::core::marker::Send + ::core::marker::Sync>, RandomState>>,
    after_step_hooks: ::std::vec::Vec<Hook<::std::rc::Rc<dyn Fn(&mut World) + ::core::marker::Send + ::core::marker::Sync>, RandomState>>,

    before_global_hooks: ::std::vec::Vec<Hook<::std::boxed::Box<dyn FnOnce() + ::core::marker::Send + ::core::marker::Sync>, RandomState>>,
    after_global_hooks: ::std::vec::Vec<Hook<::std::boxed::Box<dyn FnOnce() + ::core::marker::Send + ::core::marker::Sync>, RandomState>>,
}

impl<World, RandomState: ::core::hash::BuildHasher> Suite<World, RandomState> {
    pub fn builder() -> SuiteBuilder<World, RandomState> {
        SuiteBuilder {
            features: ::core::default::Default::default(),

            before_scenario_hooks: ::core::default::Default::default(),
            after_scenario_hooks: ::core::default::Default::default(),

            before_step_hooks: ::core::default::Default::default(),
            after_step_hooks: ::core::default::Default::default(),

            before_global_hooks: ::core::default::Default::default(),
            after_global_hooks: ::core::default::Default::default(),
        }
    }
}

impl<World, RandomState: ::core::hash::BuildHasher> SuiteBuilder<World, RandomState> {
    pub fn feature(mut self, value: impl Into<Feature<World, RandomState>>) -> Self {
        self.features.push(value.into());
        self
    }

    pub fn features<T>(mut self, values: impl IntoIterator<Item = T>) -> Self
    where
        T: Into<Feature<World, RandomState>>,
    {
        self.features.extend(values.into_iter().map(Into::into));
        self
    }

    pub fn before_scenario(mut self, tags: impl Into<Tags<RandomState>>, callback: impl Fn(&mut World) + ::core::marker::Send + ::core::marker::Sync + 'static) -> Self {
        let hook = Hook::builder()
            .tags(tags)
            .callback(::std::rc::Rc::new(callback) as ::std::rc::Rc<dyn Fn(&mut World) + ::core::marker::Send + ::core::marker::Sync>)
            .build();

        self.before_scenario_hooks.push(hook);
        self
    }

    pub fn after_scenario(mut self, tags: impl Into<Tags<RandomState>>, callback: impl Fn(&mut World) + ::core::marker::Send + ::core::marker::Sync + 'static) -> Self {
        let hook = Hook::builder()
            .tags(tags)
            .callback(::std::rc::Rc::new(callback) as ::std::rc::Rc<dyn Fn(&mut World) + ::core::marker::Send + ::core::marker::Sync>)
            .build();

        self.after_scenario_hooks.push(hook);
        self
    }

    pub fn before_step(mut self, tags: impl Into<Tags<RandomState>>, callback: impl Fn(&mut World) + ::core::marker::Send + ::core::marker::Sync + 'static) -> Self {
        let hook = Hook::builder()
            .tags(tags)
            .callback(::std::rc::Rc::new(callback) as ::std::rc::Rc<dyn Fn(&mut World) + ::core::marker::Send + ::core::marker::Sync>)
            .build();

        self.before_step_hooks.push(hook);
        self
    }

    pub fn after_step(mut self, tags: impl Into<Tags<RandomState>>, callback: impl Fn(&mut World) + ::core::marker::Send + ::core::marker::Sync + 'static) -> Self {
        let hook = Hook::builder()
            .tags(tags)
            .callback(::std::rc::Rc::new(callback) as ::std::rc::Rc<dyn Fn(&mut World) + ::core::marker::Send + ::core::marker::Sync>)
            .build();

        self.after_step_hooks.push(hook);
        self
    }

    pub fn before_all(mut self, tags: impl Into<Tags<RandomState>>, callback: impl FnOnce() + ::core::marker::Send + ::core::marker::Sync + 'static) -> Self {
        let hook = Hook::builder()
            .tags(tags)
            .callback(::std::boxed::Box::new(callback) as ::std::boxed::Box<dyn FnOnce() + ::core::marker::Send + ::core::marker::Sync>)
            .build();

        self.before_global_hooks.push(hook);
        self
    }

    pub fn after_all(mut self, tags: impl Into<Tags<RandomState>>, callback: impl FnOnce() + ::core::marker::Send + ::core::marker::Sync + 'static) -> Self {
        let hook = Hook::builder()
            .tags(tags)
            .callback(::std::boxed::Box::new(callback) as ::std::boxed::Box<dyn FnOnce() + ::core::marker::Send + ::core::marker::Sync>)
            .build();

        self.after_global_hooks.push(hook);
        self
    }
}

impl<World, RandomState: ::core::hash::BuildHasher> SuiteBuilder<World, RandomState> {
    pub fn build(self) -> Suite<World, RandomState> {
        Suite {
            features: self.features,

            before_scenario_hooks: self.before_scenario_hooks,
            after_scenario_hooks: self.after_scenario_hooks,

            before_step_hooks: self.before_step_hooks,
            after_step_hooks: self.after_step_hooks,

            before_global_hooks: self.before_global_hooks,
            after_global_hooks: self.after_global_hooks,
        }
    }
}

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

    pub fn tags(mut self, value: impl Into<Tags<RandomState>>) -> FeatureBuilder<World, RandomState, self::feature::SetTags<State>>
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

    pub fn background(mut self, value: impl Into<Background<World>>) -> FeatureBuilder<World, RandomState, self::feature::SetBackground<State>>
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

    pub fn scenario(mut self, value: Scenario<World, RandomState>) -> Self {
        self.scenarios.push(value.into());
        self
    }

    pub fn scenarios<T>(mut self, values: impl IntoIterator<Item = T>) -> Self
    where
        T: Into<Scenario<World, RandomState>>,
    {
        self.scenarios.extend(values.into_iter().map(Into::into));
        self
    }

    pub fn rule(mut self, value: impl Into<Rule<World, RandomState>>) -> Self {
        self.rules.push(value.into());
        self
    }

    pub fn rules<T>(mut self, values: impl IntoIterator<Item = T>) -> Self
    where
        T: Into<Rule<World, RandomState>>,
    {
        self.rules.extend(values.into_iter().map(Into::into));
        self
    }

    pub fn build(self) -> Feature<World, RandomState> {
        Feature {
            description: self.description,
            ignored: self.ignored,
            tags: self.tags,

            background: self.background,
            scenarios: self.scenarios,
            rules: self.rules,
        }
    }
}

impl<World, RandomState: ::core::hash::BuildHasher, State: self::feature::BuilderState> From<FeatureBuilder<World, RandomState, State>> for Feature<World, RandomState> {
    fn from(builder: FeatureBuilder<World, RandomState, State>) -> Self {
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

    pub fn tags(mut self, value: impl Into<Tags<RandomState>>) -> RuleBuilder<World, RandomState, self::rule::SetTags<State>>
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

    pub fn background(mut self, value: impl Into<Background<World>>) -> RuleBuilder<World, RandomState, self::rule::SetBackground<State>>
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

    pub fn scenario(mut self, value: impl Into<Scenario<World, RandomState>>) -> Self {
        self.scenarios.push(value.into());
        self
    }

    pub fn scenarios<T>(mut self, values: impl IntoIterator<Item = T>) -> Self
    where
        T: Into<Scenario<World, RandomState>>,
    {
        self.scenarios.extend(values.into_iter().map(Into::into));
        self
    }

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

impl<World, RandomState: ::core::hash::BuildHasher, State: self::rule::BuilderState> From<RuleBuilder<World, RandomState, State>> for Rule<World, RandomState> {
    fn from(builder: RuleBuilder<World, RandomState, State>) -> Self {
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

pub struct ScenarioBuilder<World, RandomState: ::core::hash::BuildHasher = ::std::hash::RandomState, State: self::scenario::BuilderState = self::scenario::Empty> {
    description: ::core::option::Option<::std::borrow::Cow<'static, str>>,
    ignored: ::core::option::Option<bool>,
    tags: ::core::option::Option<Tags<RandomState>>,

    given: ::std::vec::Vec<Step<::std::boxed::Box<dyn FnOnce(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync>>>,
    when: ::std::vec::Vec<Step<Box<dyn FnOnce(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync>>>,
    then: ::std::vec::Vec<Step<Box<dyn FnOnce(&World) -> Fallible + ::core::marker::Send + ::core::marker::Sync>>>,

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

    pub fn tags(mut self, value: impl Into<Tags<RandomState>>) -> ScenarioBuilder<World, RandomState, self::scenario::SetTags<State>>
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

    pub fn given(mut self, description: impl Into<::std::borrow::Cow<'static, str>>, callback: impl FnOnce(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync + 'static) -> ScenarioBuilder<World, RandomState, self::scenario::SetGiven<State>> {
        let step = Step::builder()
            .label(StepLabel::Given)
            .description(description)
            .callback(::std::boxed::Box::new(callback) as ::std::boxed::Box<dyn FnOnce(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync>)
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

    pub fn when(mut self, description: impl Into<::std::borrow::Cow<'static, str>>, callback: impl FnOnce(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync + 'static) -> ScenarioBuilder<World, RandomState, self::scenario::SetWhen<self::scenario::SetGiven<InnerState>>> {
        let step = Step::builder()
            .label(StepLabel::When)
            .description(description)
            .callback(::std::boxed::Box::new(callback) as ::std::boxed::Box<dyn FnOnce(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync>)
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

    pub fn then(mut self, description: impl Into<::std::borrow::Cow<'static, str>>, callback: impl FnOnce(&World) -> Fallible + ::core::marker::Send + ::core::marker::Sync + 'static) -> ScenarioBuilder<World, RandomState, self::scenario::SetThen<self::scenario::SetWhen<InnerState>>> {
        let step = Step::builder()
            .label(StepLabel::Then)
            .description(description)
            .callback(::std::boxed::Box::new(callback) as ::std::boxed::Box<dyn FnOnce(&World) -> Fallible + ::core::marker::Send + ::core::marker::Sync>)
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

impl<World, RandomState: ::core::hash::BuildHasher, State: self::scenario::BuilderState> ScenarioBuilder<World, RandomState, State>
where
    State: self::scenario::IsComplete,
{
    pub fn build(self) -> Scenario<World, RandomState> {
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

impl<World, RandomState: ::core::hash::BuildHasher, State: self::scenario::BuilderState> From<ScenarioBuilder<World, RandomState, State>> for Scenario<World, RandomState>
where
    State: self::scenario::IsComplete,
{
    fn from(builder: ScenarioBuilder<World, RandomState, State>) -> Self {
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

    given: ::std::vec::Vec<Step<::std::rc::Rc<dyn Fn(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync>>>,

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

pub struct HookBuilder<Callback, RandomState: ::core::hash::BuildHasher = ::std::hash::RandomState, State: self::hook::BuilderState = self::hook::Empty> {
    tags: ::core::option::Option<Tags<RandomState>>,
    callback: ::core::option::Option<Callback>,

    __phantom: self::marker::PhantomCovariant<State>,
}

impl<Callback, RandomState: ::core::hash::BuildHasher> Hook<Callback, RandomState> {
    fn builder() -> HookBuilder<Callback, RandomState> {
        HookBuilder {
            tags: ::core::default::Default::default(),
            callback: ::core::default::Default::default(),

            __phantom: ::core::default::Default::default(),
        }
    }
}

impl<Callback, RandomState: ::core::hash::BuildHasher, State: self::hook::BuilderState> HookBuilder<Callback, RandomState, State> {
    fn tags(mut self, value: impl Into<Tags<RandomState>>) -> HookBuilder<Callback, RandomState, self::hook::SetTags<State>>
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

    fn callback(mut self, value: Callback) -> HookBuilder<Callback, RandomState, self::hook::SetCallback<State>>
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

impl<Callback, RandomState: ::core::hash::BuildHasher, State: self::hook::BuilderState> HookBuilder<Callback, RandomState, State>
where
    State: self::hook::IsComplete,
{
    fn build(self) -> Hook<Callback, RandomState> {
        Hook {
            tags: unsafe { self.tags.unwrap_unchecked() },
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
    pub trait IsComplete: BuilderState<Tags: self::marker::IsSet, Callback: self::marker::IsSet> {}

    #[sealed]
    impl<State: BuilderState> IsComplete for State
    where
        State::Tags: self::marker::IsSet,
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

impl<I, T> From<I> for Tags
where
    I: IntoIterator<Item = T>,
    T: Into<::std::borrow::Cow<'static, str>>,
{
    fn from(values: I) -> Self {
        Self(values.into_iter().map(Into::into).collect())
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
