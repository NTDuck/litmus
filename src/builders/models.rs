use ::sealed::sealed;

use crate::builders::*;
use crate::models::*;
use crate::utils::aliases;

pub struct FeatureBuilder<World, State: self::feature::BuilderState = self::feature::Empty> {
    description: ::core::option::Option<aliases::string::String>,
    ignored: ::core::option::Option<bool>,
    tags: ::core::option::Option<Tags>,

    background: ::core::option::Option<Background<World>>,
    scenarios: ::std::vec::Vec<Scenario<World>>,
    rules: ::std::vec::Vec<Rule<World>>,

    __phantom: aliases::marker::PhantomCovariant<State>,
}

impl<World> Feature<World> {
    #[cfg(feature = "allow-natural")]
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> FeatureBuilder<World> {
        Self::builder()
    }

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
    pub fn description(
        mut self,
        description: impl Into<aliases::string::String>,
    ) -> FeatureBuilder<World, self::feature::SetDescription<State>>
    where
        State::Description: self::marker::IsUnset,
    {
        self.description = ::core::option::Option::from(description.into());

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

    pub fn ignored(mut self, ignored: impl Into<bool>) -> FeatureBuilder<World, self::feature::SetIgnored<State>>
    where
        State::Ignored: self::marker::IsUnset,
    {
        self.ignored = ::core::option::Option::from(ignored.into());

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

    pub fn tags(mut self, tags: impl IntoTags) -> FeatureBuilder<World, self::feature::SetTags<State>>
    where
        State::Tags: self::marker::IsUnset,
    {
        self.tags = ::core::option::Option::from(tags.into_tags());

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

    pub fn background(
        mut self,
        background: impl IntoBackground<World>,
    ) -> FeatureBuilder<World, self::feature::SetBackground<State>>
    where
        State::Background: self::marker::IsUnset,
    {
        self.background = ::core::option::Option::from(background.into_background());

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

    pub fn scenario(
        mut self,
        scenario: impl IntoScenario<World>,
    ) -> FeatureBuilder<World, self::feature::SetScenarios<State>> {
        self.scenarios.push(scenario.into_scenario());

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

    pub fn scenarios<T>(
        mut self,
        scenarios: impl IntoIterator<Item = T>,
    ) -> FeatureBuilder<World, self::feature::SetScenarios<State>>
    where
        T: IntoScenario<World>,
    {
        self.scenarios.extend(scenarios.into_iter().map(IntoScenario::into_scenario));

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

    pub fn rule(mut self, rule: impl IntoRule<World>) -> FeatureBuilder<World, self::feature::SetRules<State>> {
        self.rules.push(rule.into_rule());

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

    pub fn rules<T>(
        mut self,
        rules: impl IntoIterator<Item = T>,
    ) -> FeatureBuilder<World, self::feature::SetRules<State>>
    where
        T: IntoRule<World>,
    {
        self.rules.extend(rules.into_iter().map(IntoRule::into_rule));

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

impl<World, State: self::feature::BuilderState> FeatureBuilder<World, State>
where
    State: self::feature::IsComplete,
{
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
            self.scenarios.iter_mut().for_each(|scenario| scenario.ignored = ::core::option::Option::from(*ignored))
        }
    }

    /// See also: [Tag inheritance](https://cucumber.io/docs/cucumber/api/#tag-inheritance)
    fn propagate_tags(&mut self) {
        if let Some(tags) = self.tags.as_ref() {
            self.scenarios.iter_mut().for_each(|scenario| scenario.tags.get_or_insert_default().extend(tags.clone()));
            self.rules.iter_mut().for_each(|rule| rule.tags.get_or_insert_default().extend(tags.clone()));
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
        type Scenarios;
        type Rules;

        type ScenariosOrRules;
    }

    #[cfg(feature = "allow-empty")]
    #[sealed]
    pub trait IsComplete: BuilderState {}

    #[cfg(feature = "allow-empty")]
    #[sealed]
    impl<State: BuilderState> IsComplete for State {}

    #[cfg(not(feature = "allow-empty"))]
    #[sealed]
    pub trait IsComplete: BuilderState<ScenariosOrRules: self::marker::IsSet> {}

    #[cfg(not(feature = "allow-empty"))]
    #[sealed]
    impl<State: BuilderState> IsComplete for State where State::ScenariosOrRules: self::marker::IsSet {}

    pub struct Empty;

    pub struct SetDescription<State: BuilderState = Empty>(aliases::marker::PhantomCovariant<State>);
    pub struct SetIgnored<State: BuilderState = Empty>(aliases::marker::PhantomCovariant<State>);
    pub struct SetTags<State: BuilderState = Empty>(aliases::marker::PhantomCovariant<State>);

    pub struct SetBackground<State: BuilderState = Empty>(aliases::marker::PhantomCovariant<State>);
    pub struct SetScenarios<State: BuilderState = Empty>(aliases::marker::PhantomCovariant<State>);
    pub struct SetRules<State: BuilderState = Empty>(aliases::marker::PhantomCovariant<State>);

    #[rustfmt::skip] // `reorder_impl_items`
    #[sealed]
    impl BuilderState for Empty {
        type Description = self::marker::Unset<self::members::Description>;
        type Ignored = self::marker::Unset<self::members::Ignored>;
        type Tags = self::marker::Unset<self::members::Tags>;

        type Background = self::marker::Unset<self::members::Background>;
        type Scenarios = self::marker::Unset<self::members::Scenarios>;
        type Rules = self::marker::Unset<self::members::Rules>;

        type ScenariosOrRules = self::marker::Unset<self::members::Scenarios>;
    }

    #[rustfmt::skip] // `reorder_impl_items`
    #[sealed]
    impl<State: BuilderState> BuilderState for SetDescription<State> {
        type Description = self::marker::Set<self::members::Description>;
        type Ignored = State::Ignored;
        type Tags = State::Tags;

        type Background = State::Background;
        type Scenarios = State::Scenarios;
        type Rules = State::Rules;

        type ScenariosOrRules = State::ScenariosOrRules;
    }

    #[rustfmt::skip] // `reorder_impl_items`
    #[sealed]
    impl<State: BuilderState> BuilderState for SetIgnored<State> {
        type Description = State::Description;
        type Ignored = self::marker::Set<self::members::Ignored>;
        type Tags = State::Tags;

        type Background = State::Background;
        type Scenarios = State::Scenarios;
        type Rules = State::Rules;

        type ScenariosOrRules = State::ScenariosOrRules;
    }

    #[rustfmt::skip] // `reorder_impl_items`
    #[sealed]
    impl<State: BuilderState> BuilderState for SetTags<State> {
        type Description = State::Description;
        type Ignored = State::Ignored;
        type Tags = self::marker::Set<self::members::Tags>;

        type Background = State::Background;
        type Scenarios = State::Scenarios;
        type Rules = State::Rules;

        type ScenariosOrRules = State::ScenariosOrRules;
    }

    #[rustfmt::skip] // `reorder_impl_items`
    #[sealed]
    impl<State: BuilderState> BuilderState for SetBackground<State> {
        type Description = State::Description;
        type Ignored = State::Ignored;
        type Tags = State::Tags;

        type Background = self::marker::Set<self::members::Background>;
        type Scenarios = State::Scenarios;
        type Rules = State::Rules;

        type ScenariosOrRules = State::ScenariosOrRules;
    }

    #[rustfmt::skip] // `reorder_impl_items`
    #[sealed]
    impl<State: BuilderState> BuilderState for SetScenarios<State> {
        type Description = State::Description;
        type Ignored = State::Ignored;
        type Tags = State::Tags;

        type Background = State::Background;
        type Scenarios = self::marker::Set<self::members::Scenarios>;
        type Rules = State::Rules;

        type ScenariosOrRules = self::marker::Set<self::members::ScenariosOrRules>;
    }

    #[rustfmt::skip] // `reorder_impl_items`
    #[sealed]
    impl<State: BuilderState> BuilderState for SetRules<State> {
        type Description = State::Description;
        type Ignored = State::Ignored;
        type Tags = State::Tags;

        type Background = State::Background;
        type Scenarios = State::Scenarios;
        type Rules = self::marker::Set<self::members::Rules>;

        type ScenariosOrRules = self::marker::Set<self::members::ScenariosOrRules>;
    }

    mod members {
        pub struct Description;
        pub struct Ignored;
        pub struct Tags;

        pub struct Background;
        pub struct Scenarios;
        pub struct Rules;

        pub struct ScenariosOrRules;
    }
}

#[sealed]
pub trait IntoFeature<World> {
    fn into_feature(self) -> Feature<World>;
}

#[sealed]
impl<World> IntoFeature<World> for Feature<World> {
    fn into_feature(self) -> Feature<World> {
        self
    }
}

#[cfg(feature = "allow-natural")]
#[sealed]
impl<World, State: self::feature::BuilderState> IntoFeature<World> for FeatureBuilder<World, State>
where
    State: self::feature::IsComplete,
{
    fn into_feature(self) -> Feature<World> {
        self.build()
    }
}

pub struct RuleBuilder<World, State: self::rule::BuilderState = self::rule::Empty> {
    description: ::core::option::Option<aliases::string::String>,
    ignored: ::core::option::Option<bool>,
    tags: ::core::option::Option<Tags>,

    background: ::core::option::Option<Background<World>>,
    scenarios: ::std::vec::Vec<Scenario<World>>,

    __phantom: aliases::marker::PhantomCovariant<State>,
}

impl<World> Rule<World> {
    #[cfg(feature = "allow-natural")]
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> RuleBuilder<World> {
        Self::builder()
    }

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
    pub fn description(
        mut self,
        description: impl Into<aliases::string::String>,
    ) -> RuleBuilder<World, self::rule::SetDescription<State>>
    where
        State::Description: self::marker::IsUnset,
    {
        self.description = ::core::option::Option::from(description.into());

        RuleBuilder {
            description: self.description,
            ignored: self.ignored,
            tags: self.tags,

            background: self.background,
            scenarios: self.scenarios,

            __phantom: ::core::default::Default::default(),
        }
    }

    pub fn ignored(mut self, ignored: impl Into<bool>) -> RuleBuilder<World, self::rule::SetIgnored<State>>
    where
        State::Ignored: self::marker::IsUnset,
    {
        self.ignored = ::core::option::Option::from(ignored.into());

        RuleBuilder {
            description: self.description,
            ignored: self.ignored,
            tags: self.tags,

            background: self.background,
            scenarios: self.scenarios,

            __phantom: ::core::default::Default::default(),
        }
    }

    pub fn tags(mut self, tags: impl IntoTags) -> RuleBuilder<World, self::rule::SetTags<State>>
    where
        State::Tags: self::marker::IsUnset,
    {
        self.tags = ::core::option::Option::from(tags.into_tags());

        RuleBuilder {
            description: self.description,
            ignored: self.ignored,
            tags: self.tags,

            background: self.background,
            scenarios: self.scenarios,

            __phantom: ::core::default::Default::default(),
        }
    }

    pub fn background(
        mut self,
        background: impl IntoBackground<World>,
    ) -> RuleBuilder<World, self::rule::SetBackground<State>>
    where
        State::Background: self::marker::IsUnset,
    {
        self.background = ::core::option::Option::from(background.into_background());

        RuleBuilder {
            description: self.description,
            ignored: self.ignored,
            tags: self.tags,

            background: self.background,
            scenarios: self.scenarios,

            __phantom: ::core::default::Default::default(),
        }
    }

    pub fn scenario(
        mut self,
        scenario: impl IntoScenario<World>,
    ) -> RuleBuilder<World, self::rule::SetScenarios<State>> {
        self.scenarios.push(scenario.into_scenario());

        RuleBuilder {
            description: self.description,
            ignored: self.ignored,
            tags: self.tags,

            background: self.background,
            scenarios: self.scenarios,

            __phantom: ::core::default::Default::default(),
        }
    }

    pub fn scenarios<T>(
        mut self,
        scenarios: impl IntoIterator<Item = T>,
    ) -> RuleBuilder<World, self::rule::SetScenarios<State>>
    where
        T: IntoScenario<World>,
    {
        self.scenarios.extend(scenarios.into_iter().map(IntoScenario::into_scenario));

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

impl<World, State: self::rule::BuilderState> RuleBuilder<World, State>
where
    State: self::rule::IsComplete,
{
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
            self.scenarios.iter_mut().for_each(|scenario| scenario.ignored = ::core::option::Option::from(*ignored));
        }
    }

    /// See also: [Tag inheritance](https://cucumber.io/docs/cucumber/api/#tag-inheritance)
    fn propagate_tags(&mut self) {
        if let Some(tags) = self.tags.as_ref() {
            self.scenarios.iter_mut().for_each(|scenario| scenario.tags.get_or_insert_default().extend(tags.clone()));
        }
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
        type Scenarios;
    }

    #[cfg(feature = "allow-empty")]
    #[sealed]
    pub trait IsComplete: BuilderState {}

    #[cfg(feature = "allow-empty")]
    #[sealed]
    impl<State: BuilderState> IsComplete for State {}

    #[cfg(not(feature = "allow-empty"))]
    pub trait IsComplete: BuilderState<Scenarios: self::marker::IsSet> {}

    #[cfg(not(feature = "allow-empty"))]
    impl<State: BuilderState> IsComplete for State where State::Scenarios: self::marker::IsSet {}

    pub struct Empty;

    pub struct SetDescription<State: BuilderState = Empty>(aliases::marker::PhantomCovariant<State>);
    pub struct SetIgnored<State: BuilderState = Empty>(aliases::marker::PhantomCovariant<State>);
    pub struct SetTags<State: BuilderState = Empty>(aliases::marker::PhantomCovariant<State>);

    pub struct SetBackground<State: BuilderState = Empty>(aliases::marker::PhantomCovariant<State>);
    pub struct SetScenarios<State: BuilderState = Empty>(aliases::marker::PhantomCovariant<State>);

    #[rustfmt::skip] // `reorder_impl_items`
    #[sealed]
    impl BuilderState for Empty {
        type Description = self::marker::Unset<self::members::Description>;
        type Ignored = self::marker::Unset<self::members::Ignored>;
        type Tags = self::marker::Unset<self::members::Tags>;

        type Background = self::marker::Unset<self::members::Background>;
        type Scenarios = self::marker::Unset<self::members::Scenarios>;
    }

    #[rustfmt::skip] // `reorder_impl_items`
    #[sealed]
    impl<State: BuilderState> BuilderState for SetDescription<State> {
        type Description = self::marker::Set<self::members::Description>;
        type Ignored = State::Ignored;
        type Tags = State::Tags;

        type Background = State::Background;
        type Scenarios = State::Scenarios;
    }

    #[rustfmt::skip] // `reorder_impl_items`
    #[sealed]
    impl<State: BuilderState> BuilderState for SetIgnored<State> {
        type Description = State::Description;
        type Ignored = self::marker::Set<self::members::Ignored>;
        type Tags = State::Tags;

        type Background = State::Background;
        type Scenarios = State::Scenarios;        
    }

    #[rustfmt::skip] // `reorder_impl_items`
    #[sealed]
    impl<State: BuilderState> BuilderState for SetTags<State> {
        type Description = State::Description;
        type Ignored = State::Ignored;
        type Tags = self::marker::Set<self::members::Tags>;

        type Background = State::Background;
        type Scenarios = State::Scenarios;
    }

    #[rustfmt::skip] // `reorder_impl_items`
    #[sealed]
    impl<State: BuilderState> BuilderState for SetBackground<State> {
        type Description = State::Description;
        type Ignored = State::Ignored;
        type Tags = State::Tags;

        type Background = self::marker::Set<self::members::Background>;
        type Scenarios = State::Scenarios;
    }

    #[rustfmt::skip] // `reorder_impl_items`
    #[sealed]
    impl<State: BuilderState> BuilderState for SetScenarios<State> {
        type Description = State::Description;
        type Ignored = State::Ignored;
        type Tags = State::Tags;

        type Background = State::Background;
        type Scenarios = self::marker::Set<self::members::Scenarios>;
    }

    mod members {
        pub struct Description;
        pub struct Ignored;
        pub struct Tags;

        pub struct Background;
        pub struct Scenarios;
    }
}

#[sealed]
pub trait IntoRule<World> {
    fn into_rule(self) -> Rule<World>;
}

#[sealed]
impl<World> IntoRule<World> for Rule<World> {
    fn into_rule(self) -> Rule<World> {
        self
    }
}

#[cfg(feature = "allow-natural")]
#[sealed]
impl<World, State: self::rule::BuilderState> IntoRule<World> for RuleBuilder<World, State>
where
    State: self::rule::IsComplete,
{
    fn into_rule(self) -> Rule<World> {
        self.build()
    }
}

pub struct ScenarioBuilder<World, State: self::scenario::BuilderState = self::scenario::Empty> {
    description: ::core::option::Option<aliases::string::String>,
    ignored: ::core::option::Option<bool>,
    tags: ::core::option::Option<Tags>,

    pub(crate) given: ::std::vec::Vec<ScenarioGivenOrWhenStep<World>>,
    pub(crate) when: ::std::vec::Vec<ScenarioGivenOrWhenStep<World>>,
    pub(crate) then: ::std::vec::Vec<ScenarioThenStep<World>>,

    __phantom: aliases::marker::PhantomCovariant<State>,
}

impl<World> Scenario<World> {
    #[cfg(feature = "allow-natural")]
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> ScenarioBuilder<World> {
        Self::builder()
    }

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
    pub fn description(
        mut self,
        description: impl Into<aliases::string::String>,
    ) -> ScenarioBuilder<World, self::scenario::SetDescription<State>>
    where
        State::Description: self::marker::IsUnset,
    {
        self.description = ::core::option::Option::from(description.into());

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

    pub fn ignored(mut self, ignored: impl Into<bool>) -> ScenarioBuilder<World, self::scenario::SetIgnored<State>>
    where
        State::Ignored: self::marker::IsUnset,
    {
        self.ignored = ::core::option::Option::from(ignored.into());

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

    pub fn tags(mut self, tags: impl IntoTags) -> ScenarioBuilder<World, self::scenario::SetTags<State>>
    where
        State::Tags: self::marker::IsUnset,
    {
        self.tags = ::core::option::Option::from(tags.into_tags());

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

    pub fn given<Description, Callback, Output>(
        mut self,
        description: Description,
        callback: Callback,
    ) -> ScenarioBuilder<World, self::scenario::SetGiven<State>>
    where
        Description: Into<aliases::string::String>,
        Callback: Fn(&mut World) -> Output + ::core::marker::Send + ::core::marker::Sync + 'static,
        Output: IntoFallible,
    {
        let step = IntoScenarioGivenOrWhenStep::into_step((description, callback), StepLabel::Given);
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

impl<World, InnerState: self::scenario::BuilderState> ScenarioBuilder<World, self::scenario::SetGiven<InnerState>>
where
    <self::scenario::SetGiven<InnerState> as self::scenario::BuilderState>::Given: self::marker::IsSet,
    <self::scenario::SetGiven<InnerState> as self::scenario::BuilderState>::When: self::marker::IsUnset,
    <self::scenario::SetGiven<InnerState> as self::scenario::BuilderState>::Then: self::marker::IsUnset,
{
    pub fn and<Description, Callback, Output>(
        mut self,
        description: Description,
        callback: Callback,
    ) -> ScenarioBuilder<World, self::scenario::SetGiven<self::scenario::SetGiven<InnerState>>>
    where
        Description: Into<aliases::string::String>,
        Callback: Fn(&mut World) -> Output + ::core::marker::Send + ::core::marker::Sync + 'static,
        Output: IntoFallible,
    {
        let step = IntoScenarioGivenOrWhenStep::into_step((description, callback), StepLabel::And);
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

    pub fn but<Description, Callback, Output>(
        mut self,
        description: Description,
        callback: Callback,
    ) -> ScenarioBuilder<World, self::scenario::SetGiven<self::scenario::SetGiven<InnerState>>>
    where
        Description: Into<aliases::string::String>,
        Callback: Fn(&mut World) -> Output + ::core::marker::Send + ::core::marker::Sync + 'static,
        Output: IntoFallible,
    {
        let step = IntoScenarioGivenOrWhenStep::into_step((description, callback), StepLabel::But);
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

    pub fn when<Description, Callback, Output>(
        mut self,
        description: Description,
        callback: Callback,
    ) -> ScenarioBuilder<World, self::scenario::SetWhen<self::scenario::SetGiven<InnerState>>>
    where
        Description: Into<aliases::string::String>,
        Callback: Fn(&mut World) -> Output + ::core::marker::Send + ::core::marker::Sync + 'static,
        Output: IntoFallible,
    {
        let step = IntoScenarioGivenOrWhenStep::into_step((description, callback), StepLabel::When);
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

impl<World, InnerState: self::scenario::BuilderState> ScenarioBuilder<World, self::scenario::SetWhen<InnerState>>
where
    <self::scenario::SetWhen<InnerState> as self::scenario::BuilderState>::Given: self::marker::IsSet,
    <self::scenario::SetWhen<InnerState> as self::scenario::BuilderState>::When: self::marker::IsSet,
    <self::scenario::SetWhen<InnerState> as self::scenario::BuilderState>::Then: self::marker::IsUnset,
{
    pub fn and<Description, Callback, Output>(
        mut self,
        description: Description,
        callback: Callback,
    ) -> ScenarioBuilder<World, self::scenario::SetWhen<self::scenario::SetWhen<InnerState>>>
    where
        Description: Into<aliases::string::String>,
        Callback: Fn(&mut World) -> Output + ::core::marker::Send + ::core::marker::Sync + 'static,
        Output: IntoFallible,
    {
        let step = IntoScenarioGivenOrWhenStep::into_step((description, callback), StepLabel::And);
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

    pub fn but<Description, Callback, Output>(
        mut self,
        description: Description,
        callback: Callback,
    ) -> ScenarioBuilder<World, self::scenario::SetWhen<self::scenario::SetWhen<InnerState>>>
    where
        Description: Into<aliases::string::String>,
        Callback: Fn(&mut World) -> Output + ::core::marker::Send + ::core::marker::Sync + 'static,
        Output: IntoFallible,
    {
        let step = IntoScenarioGivenOrWhenStep::into_step((description, callback), StepLabel::But);
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

    pub fn then<Description, Callback, Output>(
        mut self,
        description: Description,
        callback: Callback,
    ) -> ScenarioBuilder<World, self::scenario::SetThen<self::scenario::SetWhen<InnerState>>>
    where
        Description: Into<aliases::string::String>,
        Callback: Fn(&World) -> Output + ::core::marker::Send + ::core::marker::Sync + 'static,
        Output: IntoFallible,
    {
        let step = IntoScenarioThenStep::into_step((description, callback), StepLabel::Then);
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

impl<World, InnerState: self::scenario::BuilderState> ScenarioBuilder<World, self::scenario::SetThen<InnerState>>
where
    <self::scenario::SetThen<InnerState> as self::scenario::BuilderState>::Given: self::marker::IsSet,
    <self::scenario::SetThen<InnerState> as self::scenario::BuilderState>::When: self::marker::IsSet,
    <self::scenario::SetThen<InnerState> as self::scenario::BuilderState>::Then: self::marker::IsSet,
{
    pub fn and<Description, Callback, Output>(
        mut self,
        description: Description,
        callback: Callback,
    ) -> ScenarioBuilder<World, self::scenario::SetThen<self::scenario::SetThen<InnerState>>>
    where
        Description: Into<aliases::string::String>,
        Callback: Fn(&World) -> Output + ::core::marker::Send + ::core::marker::Sync + 'static,
        Output: IntoFallible,
    {
        let step = IntoScenarioThenStep::into_step((description, callback), StepLabel::And);
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

    pub fn but<Description, Callback, Output>(
        mut self,
        description: Description,
        callback: Callback,
    ) -> ScenarioBuilder<World, self::scenario::SetThen<self::scenario::SetThen<InnerState>>>
    where
        Description: Into<aliases::string::String>,
        Callback: Fn(&World) -> Output + ::core::marker::Send + ::core::marker::Sync + 'static,
        Output: IntoFallible,
    {
        let step = IntoScenarioThenStep::into_step((description, callback), StepLabel::But);
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
    pub trait IsComplete:
        BuilderState<Given: self::marker::IsSet, When: self::marker::IsSet, Then: self::marker::IsSet>
    {
    }

    #[sealed]
    impl<State: BuilderState> IsComplete for State
    where
        State::Given: self::marker::IsSet,
        State::When: self::marker::IsSet,
        State::Then: self::marker::IsSet,
    {
    }

    pub struct Empty;

    pub struct SetDescription<State: BuilderState = Empty>(aliases::marker::PhantomCovariant<State>);
    pub struct SetIgnored<State: BuilderState = Empty>(aliases::marker::PhantomCovariant<State>);
    pub struct SetTags<State: BuilderState = Empty>(aliases::marker::PhantomCovariant<State>);

    pub struct SetGiven<State: BuilderState = Empty>(aliases::marker::PhantomCovariant<State>);
    pub struct SetWhen<State: BuilderState = Empty>(aliases::marker::PhantomCovariant<State>);
    pub struct SetThen<State: BuilderState = Empty>(aliases::marker::PhantomCovariant<State>);

    #[rustfmt::skip] // `reorder_impl_items`
    #[sealed]
    impl BuilderState for Empty {
        type Description = self::marker::Unset<self::members::Description>;
        type Ignored = self::marker::Unset<self::members::Ignored>;
        type Tags = self::marker::Unset<self::members::Tags>;

        type Given = self::marker::Unset<self::members::Given>;
        type When = self::marker::Unset<self::members::When>;
        type Then = self::marker::Unset<self::members::Then>;
    }

    #[rustfmt::skip] // `reorder_impl_items`
    #[sealed]
    impl<State: BuilderState> BuilderState for SetDescription<State> {
        type Description = self::marker::Set<self::members::Description>;
        type Ignored = State::Ignored;
        type Tags = State::Tags;

        type Given = State::Given;
        type When = State::When;
        type Then = State::Then;
    }

    #[rustfmt::skip] // `reorder_impl_items`
    #[sealed]
    impl<State: BuilderState> BuilderState for SetIgnored<State> {
        type Description = State::Description;
        type Ignored = self::marker::Set<self::members::Ignored>;
        type Tags = State::Tags;

        type Given = State::Given;
        type When = State::When;
        type Then = State::Then;
    }

    #[rustfmt::skip] // `reorder_impl_items`
    #[sealed]
    impl<State: BuilderState> BuilderState for SetTags<State> {
        type Description = State::Description;
        type Ignored = State::Ignored;
        type Tags = self::marker::Set<self::members::Tags>;

        type Given = State::Given;
        type When = State::When;
        type Then = State::Then;
    }

    #[rustfmt::skip] // `reorder_impl_items`
    #[sealed]
    impl<State: BuilderState> BuilderState for SetGiven<State> {
        type Description = State::Description;
        type Ignored = State::Ignored;
        type Tags = State::Tags;

        type Given = self::marker::Set<self::members::Given>;
        type When = State::When;
        type Then = State::Then;
    }

    #[rustfmt::skip] // `reorder_impl_items`
    #[sealed]
    impl<State: BuilderState> BuilderState for SetWhen<State> {
        type Description = State::Description;
        type Ignored = State::Ignored;
        type Tags = State::Tags;

        type Given = State::Given;
        type When = self::marker::Set<self::members::When>;
        type Then = State::Then;
    }

    #[rustfmt::skip] // `reorder_impl_items`
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

#[sealed]
pub trait IntoScenario<World> {
    fn into_scenario(self) -> Scenario<World>;
}

#[sealed]
impl<World> IntoScenario<World> for Scenario<World> {
    fn into_scenario(self) -> Scenario<World> {
        self
    }
}

#[cfg(feature = "allow-natural")]
#[sealed]
impl<World, State: self::scenario::BuilderState> IntoScenario<World> for ScenarioBuilder<World, State>
where
    State: self::scenario::IsComplete,
{
    fn into_scenario(self) -> Scenario<World> {
        self.build()
    }
}

pub struct ScenarioOutlineBuilder<World, Example, State: self::scenario_outline::BuilderState = self::scenario_outline::Empty> {
    description: ::core::option::Option<aliases::string::String>,
    ignored: ::core::option::Option<bool>,
    tags: ::core::option::Option<Tags>,

    given: ::std::vec::Vec<ScenarioOutlineGivenOrWhenStep<World, Example>>,
    when: ::std::vec::Vec<ScenarioOutlineGivenOrWhenStep<World, Example>>,
    then: ::std::vec::Vec<ScenarioOutlineThenStep<World, Example>>,

    examples: ::std::vec::Vec<Example>,

    __phantom: aliases::marker::PhantomCovariant<State>,
}

impl<World, Example> ScenarioOutline<World, Example> {
    #[cfg(feature = "allow-natural")]
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> ScenarioOutlineBuilder<World, Example> {
        Self::builder()
    }

    pub fn builder() -> ScenarioOutlineBuilder<World, Example> {
        ScenarioOutlineBuilder {
            description: ::core::default::Default::default(),
            ignored: ::core::default::Default::default(),
            tags: ::core::default::Default::default(),

            given: ::core::default::Default::default(),
            when: ::core::default::Default::default(),
            then: ::core::default::Default::default(),

            examples: ::core::default::Default::default(),

            __phantom: ::core::default::Default::default(),
        }
    }
}

impl<World, Example, State: self::scenario_outline::BuilderState> ScenarioOutlineBuilder<World, Example, State>
where
    State::Given: self::marker::IsUnset,
    State::When: self::marker::IsUnset,
    State::Then: self::marker::IsUnset,
    State::Examples: self::marker::IsUnset,
{
    pub fn description(mut self, description: impl Into<aliases::string::String>) -> ScenarioOutlineBuilder<World, Example, self::scenario_outline::SetDescription<State>>
    where
        State::Description: self::marker::IsUnset,
    {
        self.description = ::core::option::Option::from(description.into());
        
        ScenarioOutlineBuilder {
            description: self.description,
            ignored: self.ignored,
            tags: self.tags,

            given: self.given,
            when: self.when,
            then: self.then,

            examples: self.examples,

            __phantom: ::core::default::Default::default(),
        }
    }

    pub fn ignored(mut self, ignored: impl Into<bool>) -> ScenarioOutlineBuilder<World, Example, self::scenario_outline::SetIgnored<State>>
    where
        State::Ignored: self::marker::IsUnset,
    {
        self.ignored = ::core::option::Option::from(ignored.into());

        ScenarioOutlineBuilder {
            description: self.description,
            ignored: self.ignored,
            tags: self.tags,

            given: self.given,
            when: self.when,
            then: self.then,

            examples: self.examples,

            __phantom: ::core::default::Default::default(),
        }
    }

    pub fn tags(mut self, tags: impl IntoTags) -> ScenarioOutlineBuilder<World, Example, self::scenario_outline::SetTags<State>>
    where
        State::Tags: self::marker::IsUnset,
    {
        self.tags = ::core::option::Option::from(tags.into_tags());

        ScenarioOutlineBuilder {
            description: self.description,
            ignored: self.ignored,
            tags: self.tags,

            given: self.given,
            when: self.when,
            then: self.then,

            examples: self.examples,

            __phantom: ::core::default::Default::default(),
        }
    }

    pub fn given<Description, Callback, Output>(
        mut self,
        description: Description,
        callback: Callback,
    ) -> ScenarioOutlineBuilder<World, Example, self::scenario_outline::SetGiven<State>>
    where
        Description: Into<aliases::string::String>,
        Callback: Fn(&mut World, Example) -> Output + ::core::marker::Send + ::core::marker::Sync + 'static,
        Output: IntoFallible,
    {
        let step = IntoScenarioOutlineGivenOrWhenStep::into_step((description, callback), StepLabel::Given);
        self.given.push(step);

        ScenarioOutlineBuilder {
            description: self.description,
            ignored: self.ignored,
            tags: self.tags,

            given: self.given,
            when: self.when,
            then: self.then,

            examples: self.examples,

            __phantom: ::core::default::Default::default(),
        }
    }
}

impl<World, Example, InnerState: self::scenario_outline::BuilderState> ScenarioOutlineBuilder<World, Example, self::scenario_outline::SetGiven<InnerState>>
where
    <self::scenario_outline::SetGiven<InnerState> as self::scenario_outline::BuilderState>::Given: self::marker::IsSet,
    <self::scenario_outline::SetGiven<InnerState> as self::scenario_outline::BuilderState>::When: self::marker::IsUnset,
    <self::scenario_outline::SetGiven<InnerState> as self::scenario_outline::BuilderState>::Then: self::marker::IsUnset,
    <self::scenario_outline::SetGiven<InnerState> as self::scenario_outline::BuilderState>::Examples: self::marker::IsUnset,
{
    pub fn and<Description, Callback, Output>(
        mut self, description: Description, callback: Callback,
    ) -> ScenarioOutlineBuilder<World, Example, self::scenario_outline::SetGiven<self::scenario_outline::SetGiven<InnerState>>>
    where
        Description: Into<aliases::string::String>,
        Callback: Fn(&mut World, Example) -> Output + ::core::marker::Send + ::core::marker::Sync + 'static,
        Output: IntoFallible,
    {
        let step = IntoScenarioOutlineGivenOrWhenStep::into_step((description, callback), StepLabel::And);
        self.given.push(step);

        ScenarioOutlineBuilder {
            description: self.description,
            ignored: self.ignored,
            tags: self.tags,

            given: self.given,
            when: self.when,
            then: self.then,

            examples: self.examples,

            __phantom: ::core::default::Default::default(),
        }
    }

    pub fn but<Description, Callback, Output>(
        mut self, description: Description, callback: Callback,
    ) -> ScenarioOutlineBuilder<World, Example, self::scenario_outline::SetGiven<self::scenario_outline::SetGiven<InnerState>>>
    where
        Description: Into<aliases::string::String>,
        Callback: Fn(&mut World, Example) -> Output + ::core::marker::Send + ::core::marker::Sync + 'static,
        Output: IntoFallible,
    {
        let step = IntoScenarioOutlineGivenOrWhenStep::into_step((description, callback), StepLabel::But);
        self.given.push(step);

        ScenarioOutlineBuilder {
            description: self.description,
            ignored: self.ignored,
            tags: self.tags,

            given: self.given,
            when: self.when,
            then: self.then,

            examples: self.examples,

            __phantom: ::core::default::Default::default(),
        }
    }

    pub fn when<Description, Callback, Output>(
        mut self, description: Description, callback: Callback,
    ) -> ScenarioOutlineBuilder<World, Example, self::scenario_outline::SetWhen<self::scenario_outline::SetGiven<InnerState>>>
    where
        Description: Into<aliases::string::String>,
        Callback: Fn(&mut World, Example) -> Output + ::core::marker::Send + ::core::marker::Sync + 'static,
        Output: IntoFallible,
    {
        let step = IntoScenarioOutlineGivenOrWhenStep::into_step((description, callback), StepLabel::When);
        self.when.push(step);

        ScenarioOutlineBuilder {
            description: self.description,
            ignored: self.ignored,
            tags: self.tags,

            given: self.given,
            when: self.when,
            then: self.then,

            examples: self.examples,

            __phantom: ::core::default::Default::default(),
        }
    }
}

impl<World, Example, InnerState: self::scenario_outline::BuilderState> ScenarioOutlineBuilder<World, Example, self::scenario_outline::SetWhen<InnerState>>
where
    <self::scenario_outline::SetWhen<InnerState> as self::scenario_outline::BuilderState>::Given: self::marker::IsSet,
    <self::scenario_outline::SetWhen<InnerState> as self::scenario_outline::BuilderState>::When: self::marker::IsSet,
    <self::scenario_outline::SetWhen<InnerState> as self::scenario_outline::BuilderState>::Then: self::marker::IsUnset,
    <self::scenario_outline::SetWhen<InnerState> as self::scenario_outline::BuilderState>::Examples: self::marker::IsUnset,
{
    pub fn and<Description, Callback, Output>(
        mut self, description: Description, callback: Callback,
    ) -> ScenarioOutlineBuilder<World, Example, self::scenario_outline::SetWhen<self::scenario_outline::SetWhen<InnerState>>>
    where
        Description: Into<aliases::string::String>,
        Callback: Fn(&mut World, Example) -> Output + ::core::marker::Send + ::core::marker::Sync + 'static,
        Output: IntoFallible,
    {
        let step = IntoScenarioOutlineGivenOrWhenStep::into_step((description, callback), StepLabel::And);
        self.when.push(step);

        ScenarioOutlineBuilder {
            description: self.description,
            ignored: self.ignored,
            tags: self.tags,

            given: self.given,
            when: self.when,
            then: self.then,

            examples: self.examples,

            __phantom: ::core::default::Default::default(),
        }
    }

    pub fn but<Description, Callback, Output>(
        mut self, description: Description, callback: Callback,
    ) -> ScenarioOutlineBuilder<World, Example, self::scenario_outline::SetWhen<self::scenario_outline::SetWhen<InnerState>>>
    where
        Description: Into<aliases::string::String>,
        Callback: Fn(&mut World, Example) -> Output + ::core::marker::Send + ::core::marker::Sync + 'static,
        Output: IntoFallible,
    {
        let step = IntoScenarioOutlineGivenOrWhenStep::into_step((description, callback), StepLabel::But);
        self.when.push(step);

        ScenarioOutlineBuilder {
            description: self.description,
            ignored: self.ignored,
            tags: self.tags,

            given: self.given,
            when: self.when,
            then: self.then,

            examples: self.examples,

            __phantom: ::core::default::Default::default(),
        }
    }

    pub fn then<Description, Callback, Output>(
        mut self, description: Description, callback: Callback,
    ) -> ScenarioOutlineBuilder<World, Example, self::scenario_outline::SetThen<self::scenario_outline::SetWhen<InnerState>>>
    where
        Description: Into<aliases::string::String>,
        Callback: Fn(&World, Example) -> Output + ::core::marker::Send + ::core::marker::Sync + 'static,
        Output: IntoFallible,
    {
        let step = IntoScenarioOutlineThenStep::into_step((description, callback), StepLabel::Then);
        self.then.push(step);

        ScenarioOutlineBuilder {
            description: self.description,
            ignored: self.ignored,
            tags: self.tags,

            given: self.given,
            when: self.when,
            then: self.then,

            examples: self.examples,

            __phantom: ::core::default::Default::default(),
        }
    }
}

impl<World, Example, InnerState: self::scenario_outline::BuilderState> ScenarioOutlineBuilder<World, Example, self::scenario_outline::SetThen<InnerState>>
where
    <self::scenario_outline::SetThen<InnerState> as self::scenario_outline::BuilderState>::Given: self::marker::IsSet,
    <self::scenario_outline::SetThen<InnerState> as self::scenario_outline::BuilderState>::When: self::marker::IsSet,
    <self::scenario_outline::SetThen<InnerState> as self::scenario_outline::BuilderState>::Then: self::marker::IsSet,
    <self::scenario_outline::SetThen<InnerState> as self::scenario_outline::BuilderState>::Examples: self::marker::IsUnset,
{
    pub fn and<Description, Callback, Output>(
        mut self, description: Description, callback: Callback,
    ) -> ScenarioOutlineBuilder<World, Example, self::scenario_outline::SetThen<self::scenario_outline::SetThen<InnerState>>>
    where
        Description: Into<aliases::string::String>,
        Callback: Fn(&World, Example) -> Output + ::core::marker::Send + ::core::marker::Sync + 'static,
        Output: IntoFallible,
    {
        let step = IntoScenarioOutlineThenStep::into_step((description, callback), StepLabel::And);
        self.then.push(step);

        ScenarioOutlineBuilder {
            description: self.description,
            ignored: self.ignored,
            tags: self.tags,

            given: self.given,
            when: self.when,
            then: self.then,

            examples: self.examples,

            __phantom: ::core::default::Default::default(),
        }
    }

    pub fn but<Description, Callback, Output>(
        mut self, description: Description, callback: Callback,
    ) -> ScenarioOutlineBuilder<World, Example, self::scenario_outline::SetThen<self::scenario_outline::SetThen<InnerState>>>
    where
        Description: Into<aliases::string::String>,
        Callback: Fn(&World, Example) -> Output + ::core::marker::Send + ::core::marker::Sync + 'static,
        Output: IntoFallible,
    {
        let step = IntoScenarioOutlineThenStep::into_step((description, callback), StepLabel::But);
        self.then.push(step);

        ScenarioOutlineBuilder {
            description: self.description,
            ignored: self.ignored,
            tags: self.tags,

            given: self.given,
            when: self.when,
            then: self.then,

            examples: self.examples,

            __phantom: ::core::default::Default::default(),
        }
    }

    pub fn example(mut self, example: Example) -> ScenarioOutlineBuilder<World, Example, self::scenario_outline::SetExamples<self::scenario_outline::SetThen<InnerState>>> {
        self.examples.push(example);

        ScenarioOutlineBuilder {
            description: self.description,
            ignored: self.ignored,
            tags: self.tags,

            given: self.given,
            when: self.when,
            then: self.then,

            examples: self.examples,

            __phantom: ::core::default::Default::default(),
        }
    }

    pub fn examples(mut self, examples: impl IntoIterator<Item = Example>) -> ScenarioOutlineBuilder<World, Example, self::scenario_outline::SetExamples<self::scenario_outline::SetThen<InnerState>>> {
        self.examples.extend(examples);

        ScenarioOutlineBuilder {
            description: self.description,
            ignored: self.ignored,
            tags: self.tags,

            given: self.given,
            when: self.when,
            then: self.then,

            examples: self.examples,

            __phantom: ::core::default::Default::default(),
        }
    }
}

impl<World, Example, InnerState: self::scenario_outline::BuilderState> ScenarioOutlineBuilder<World, Example, self::scenario_outline::SetExamples<InnerState>> {
    pub fn example(mut self, example: Example) -> ScenarioOutlineBuilder<World, Example, self::scenario_outline::SetExamples<self::scenario_outline::SetExamples<InnerState>>> {
        self.examples.push(example);

        ScenarioOutlineBuilder {
            description: self.description,
            ignored: self.ignored,
            tags: self.tags,

            given: self.given,
            when: self.when,
            then: self.then,

            examples: self.examples,

            __phantom: ::core::default::Default::default(),
        }
    }

    pub fn examples(mut self, examples: impl IntoIterator<Item = Example>) -> ScenarioOutlineBuilder<World, Example, self::scenario_outline::SetExamples<self::scenario_outline::SetExamples<InnerState>>> {
        self.examples.extend(examples);

        ScenarioOutlineBuilder {
            description: self.description,
            ignored: self.ignored,
            tags: self.tags,

            given: self.given,
            when: self.when,
            then: self.then,

            examples: self.examples,

            __phantom: ::core::default::Default::default(),
        }
    }
}

impl<World, Example, State: self::scenario_outline::BuilderState> ScenarioOutlineBuilder<World, Example, State>
where
    State: self::scenario_outline::IsComplete,
{
    pub fn build(self) -> ScenarioOutline<World, Example> {
        ScenarioOutline {
            description: self.description,
            ignored: self.ignored,
            tags: self.tags,

            given: self.given,
            when: self.when,
            then: self.then,
            
            examples: self.examples,
        }
    }
}

mod scenario_outline {
    pub(super) use super::*;

    #[sealed]
    pub trait BuilderState: ::core::marker::Sized {
        type Description;
        type Ignored;
        type Tags;

        type Given;
        type When;
        type Then;

        type Examples;
    }

    #[sealed]
    pub trait IsComplete:
        BuilderState<Given: self::marker::IsSet, When: self::marker::IsSet, Then: self::marker::IsSet, Examples: self::marker::IsSet>
    {
    }

    #[sealed]
    impl<State: BuilderState> IsComplete for State
    where
        State::Given: self::marker::IsSet,
        State::When: self::marker::IsSet,
        State::Then: self::marker::IsSet,
        State::Examples: self::marker::IsSet,
    {
    }

    pub struct Empty;

    pub struct SetDescription<State: BuilderState = Empty>(aliases::marker::PhantomCovariant<State>);
    pub struct SetIgnored<State: BuilderState = Empty>(aliases::marker::PhantomCovariant<State>);
    pub struct SetTags<State: BuilderState = Empty>(aliases::marker::PhantomCovariant<State>);

    pub struct SetGiven<State: BuilderState = Empty>(aliases::marker::PhantomCovariant<State>);
    pub struct SetWhen<State: BuilderState = Empty>(aliases::marker::PhantomCovariant<State>);
    pub struct SetThen<State: BuilderState = Empty>(aliases::marker::PhantomCovariant<State>);

    pub struct SetExamples<State: BuilderState = Empty>(aliases::marker::PhantomCovariant<State>);

    #[rustfmt::skip] // `reorder_impl_items`
    #[sealed]
    impl BuilderState for Empty {
        type Description = self::marker::Unset<self::members::Description>;
        type Ignored = self::marker::Unset<self::members::Ignored>;
        type Tags = self::marker::Unset<self::members::Tags>;

        type Given = self::marker::Unset<self::members::Given>;
        type When = self::marker::Unset<self::members::When>;
        type Then = self::marker::Unset<self::members::Then>;

        type Examples = self::marker::Unset<self::members::Examples>;
    }

    #[rustfmt::skip] // `reorder_impl_items`
    #[sealed]
    impl<State: BuilderState> BuilderState for SetDescription<State> {
        type Description = self::marker::Set<self::members::Description>;
        type Ignored = State::Ignored;
        type Tags = State::Tags;

        type Given = State::Given;
        type When = State::When;
        type Then = State::Then;

        type Examples = State::Examples;
    }

    #[rustfmt::skip] // `reorder_impl_items`
    #[sealed]
    impl<State: BuilderState> BuilderState for SetIgnored<State> {
        type Description = State::Description;
        type Ignored = self::marker::Set<self::members::Ignored>;
        type Tags = State::Tags;

        type Given = State::Given;
        type When = State::When;
        type Then = State::Then;

        type Examples = State::Examples;
    }

    #[rustfmt::skip] // `reorder_impl_items`
    #[sealed]
    impl<State: BuilderState> BuilderState for SetTags<State> {
        type Description = State::Description;
        type Ignored = State::Ignored;
        type Tags = self::marker::Set<self::members::Tags>;

        type Given = State::Given;
        type When = State::When;
        type Then = State::Then;

        type Examples = State::Examples;
    }

    #[rustfmt::skip] // `reorder_impl_items`
    #[sealed]
    impl<State: BuilderState> BuilderState for SetGiven<State> {
        type Description = State::Description;
        type Ignored = State::Ignored;
        type Tags = State::Tags;

        type Given = self::marker::Set<self::members::Given>;
        type When = State::When;
        type Then = State::Then;

        type Examples = State::Examples;
    }

    #[rustfmt::skip] // `reorder_impl_items`
    #[sealed]
    impl<State: BuilderState> BuilderState for SetWhen<State> {
        type Description = State::Description;
        type Ignored = State::Ignored;
        type Tags = State::Tags;

        type Given = State::Given;
        type When = self::marker::Set<self::members::When>;
        type Then = State::Then;

        type Examples = State::Examples;
    }

    #[rustfmt::skip] // `reorder_impl_items`
    #[sealed]
    impl<State: BuilderState> BuilderState for SetThen<State> {
        type Description = State::Description;
        type Ignored = State::Ignored;
        type Tags = State::Tags;

        type Given = State::Given;
        type When = State::When;
        type Then = self::marker::Set<self::members::Then>;

        type Examples = State::Examples;
    }

    #[rustfmt::skip] // `reorder_impl_items`
    #[sealed]
    impl<State: BuilderState> BuilderState for SetExamples<State> {
        type Description = State::Description;
        type Ignored = State::Ignored;
        type Tags = State::Tags;

        type Given = State::Given;
        type When = State::When;
        type Then = State::Then;

        type Examples = self::marker::Set<self::members::Examples>;
    }

    mod members {
        pub struct Description;
        pub struct Ignored;
        pub struct Tags;

        pub struct Given;
        pub struct When;
        pub struct Then;

        pub struct Examples;
    }
}

#[sealed]
pub trait IntoScenarioOutline<World, Example> {
    fn into_scenario_outline(self) -> ScenarioOutline<World, Example>;
}

#[sealed]
impl<World, Example> IntoScenarioOutline<World, Example> for ScenarioOutline<World, Example> {
    fn into_scenario_outline(self) -> ScenarioOutline<World, Example> {
        self
    }
}

#[cfg(feature = "allow-natural")]
#[sealed]
impl<World, Example, State: self::scenario_outline::BuilderState> IntoScenarioOutline<World, Example> for ScenarioOutlineBuilder<World, Example, State>
where
    State: self::scenario_outline::IsComplete,
{
    fn into_scenario_outline(self) -> ScenarioOutline<World, Example> {
        self.build()
    }
}

pub struct BackgroundBuilder<World, State: self::background::BuilderState = self::background::Empty> {
    description: ::core::option::Option<aliases::string::String>,
    ignored: ::core::option::Option<bool>,

    given: ::std::vec::Vec<BackgroundGivenStep<World>>,

    __phantom: aliases::marker::PhantomCovariant<State>,
}

impl<World> Background<World> {
    #[cfg(feature = "allow-natural")]
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> BackgroundBuilder<World> {
        Self::builder()
    }

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
    pub fn description(
        mut self,
        description: impl Into<aliases::string::String>,
    ) -> BackgroundBuilder<World, self::background::SetDescription<State>>
    where
        State::Description: self::marker::IsUnset,
    {
        self.description = ::core::option::Option::from(description.into());

        BackgroundBuilder {
            description: self.description,
            ignored: self.ignored,

            given: self.given,

            __phantom: ::core::default::Default::default(),
        }
    }

    pub fn ignored(mut self, ignored: impl Into<bool>) -> BackgroundBuilder<World, self::background::SetIgnored<State>>
    where
        State::Ignored: self::marker::IsUnset,
    {
        self.ignored = ::core::option::Option::from(ignored.into());

        BackgroundBuilder {
            description: self.description,
            ignored: self.ignored,

            given: self.given,

            __phantom: ::core::default::Default::default(),
        }
    }

    pub fn given<Description, Callback, Output>(
        mut self,
        description: Description,
        callback: Callback,
    ) -> BackgroundBuilder<World, self::background::SetGiven<State>>
    where
        Description: Into<aliases::string::String>,
        Callback: Fn(&mut World) -> Output + ::core::marker::Send + ::core::marker::Sync + 'static,
        Output: IntoFallible,
    {
        let step = IntoBackgroundGivenStep::into_step((description, callback), StepLabel::Given);
        self.given.push(step);

        BackgroundBuilder {
            description: self.description,
            ignored: self.ignored,

            given: self.given,

            __phantom: ::core::default::Default::default(),
        }
    }
}

impl<World, InnerState: self::background::BuilderState> BackgroundBuilder<World, self::background::SetGiven<InnerState>>
where
    <self::background::SetGiven<InnerState> as self::background::BuilderState>::Given: self::marker::IsSet,
{
    pub fn and<Description, Callback, Output>(
        mut self,
        description: Description,
        callback: Callback,
    ) -> BackgroundBuilder<World, self::background::SetGiven<self::background::SetGiven<InnerState>>>
    where
        Description: Into<aliases::string::String>,
        Callback: Fn(&mut World) -> Output + ::core::marker::Send + ::core::marker::Sync + 'static,
        Output: IntoFallible,
    {
        let step = IntoBackgroundGivenStep::into_step((description, callback), StepLabel::And);
        self.given.push(step);

        BackgroundBuilder {
            description: self.description,
            ignored: self.ignored,

            given: self.given,

            __phantom: ::core::default::Default::default(),
        }
    }

    pub fn but<Description, Callback, Output>(
        mut self,
        description: Description,
        callback: Callback,
    ) -> BackgroundBuilder<World, self::background::SetGiven<self::background::SetGiven<InnerState>>>
    where
        Description: Into<aliases::string::String>,
        Callback: Fn(&mut World) -> Output + ::core::marker::Send + ::core::marker::Sync + 'static,
        Output: IntoFallible,
    {
        let step = IntoBackgroundGivenStep::into_step((description, callback), StepLabel::But);
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
    impl<State: BuilderState> IsComplete for State where State::Given: self::marker::IsSet {}

    pub struct Empty;

    pub struct SetDescription<State: BuilderState = Empty>(aliases::marker::PhantomCovariant<State>);
    pub struct SetIgnored<State: BuilderState = Empty>(aliases::marker::PhantomCovariant<State>);
    pub struct SetGiven<State: BuilderState = Empty>(aliases::marker::PhantomCovariant<State>);

    #[rustfmt::skip] // `reorder_impl_items`
    #[sealed]
    impl BuilderState for Empty {
        type Description = self::marker::Unset<self::members::Description>;
        type Ignored = self::marker::Unset<self::members::Ignored>;
        type Given = self::marker::Unset<self::members::Given>;
    }

    #[rustfmt::skip] // `reorder_impl_items`
    #[sealed]
    impl<State: BuilderState> BuilderState for SetDescription<State> {
        type Description = self::marker::Set<self::members::Description>;
        type Ignored = State::Ignored;
        type Given = State::Given;
    }

    #[rustfmt::skip] // `reorder_impl_items`
    #[sealed]
    impl<State: BuilderState> BuilderState for SetIgnored<State> {
        type Description = State::Description;
        type Ignored = self::marker::Set<self::members::Ignored>;
        type Given = State::Given;
    }

    #[rustfmt::skip] // `reorder_impl_items`
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

#[sealed]
pub trait IntoBackground<World> {
    fn into_background(self) -> Background<World>;
}

#[sealed]
impl<World> IntoBackground<World> for Background<World> {
    fn into_background(self) -> Background<World> {
        self
    }
}

#[cfg(feature = "allow-natural")]
#[sealed]
impl<World, State: self::background::BuilderState> IntoBackground<World> for BackgroundBuilder<World, State>
where
    State: self::background::IsComplete,
{
    fn into_background(self) -> Background<World> {
        self.build()
    }
}

pub struct HookBuilder<Callback, State: self::hook::BuilderState = self::hook::Empty> {
    tags: ::core::option::Option<Tags>,
    callback: ::core::option::Option<Callback>,

    __phantom: aliases::marker::PhantomCovariant<State>,
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
    pub(crate) fn tags(mut self, tags: impl IntoTags) -> HookBuilder<Callback, self::hook::SetTags<State>>
    where
        State::Tags: self::marker::IsUnset,
    {
        self.tags = ::core::option::Option::from(tags.into_tags());

        HookBuilder {
            tags: self.tags,
            callback: self.callback,

            __phantom: ::core::default::Default::default(),
        }
    }

    pub(crate) fn callback(mut self, callback: Callback) -> HookBuilder<Callback, self::hook::SetCallback<State>>
    where
        State::Callback: self::marker::IsUnset,
    {
        self.callback = ::core::option::Option::from(callback);

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
    impl<State: BuilderState> IsComplete for State where State::Callback: self::marker::IsSet {}

    pub struct Empty;

    pub struct SetTags<State: BuilderState = Empty>(aliases::marker::PhantomCovariant<State>);
    pub struct SetCallback<State: BuilderState = Empty>(aliases::marker::PhantomCovariant<State>);

    #[rustfmt::skip] // `reorder_impl_items`
    #[sealed]
    impl BuilderState for Empty {
        type Tags = self::marker::Unset<self::members::Tags>;
        type Callback = self::marker::Unset<self::members::Callback>;
    }

    #[rustfmt::skip] // `reorder_impl_items`
    #[sealed]
    impl<State: BuilderState> BuilderState for SetTags<State> {
        type Tags = self::marker::Set<self::members::Tags>;
        type Callback = State::Callback;
    }

    #[rustfmt::skip] // `reorder_impl_items`
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

#[sealed]
pub trait IntoScenarioOrStepHook<World> {
    #[allow(private_interfaces)]
    fn into_hook(self) -> ScenarioOrStepHook<World>;
}

#[sealed]
impl<World> IntoScenarioOrStepHook<World> for ScenarioOrStepHook<World> {
    #[allow(private_interfaces)]
    fn into_hook(self) -> ScenarioOrStepHook<World> {
        self
    }
}

#[sealed]
impl<World, Callback, Output> IntoScenarioOrStepHook<World> for Callback
where
    Callback: Fn(&mut World) -> Output + ::core::marker::Send + ::core::marker::Sync + 'static,
    Output: IntoFallible,
{
    #[allow(private_interfaces)]
    fn into_hook(self) -> ScenarioOrStepHook<World> {
        let callback = aliases::sync::Arc::new(move |world: &mut World| (self)(world).into_fallible())
            as aliases::sync::Arc<dyn Fn(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync>;

        Hook::builder().callback(callback).build()
    }
}

#[sealed]
pub trait ScenarioOrStepHookCallbackExt<World> {
    fn tags(self, tags: impl IntoTags) -> impl IntoScenarioOrStepHook<World>;
}

#[sealed]
impl<World, Callback, Output> ScenarioOrStepHookCallbackExt<World> for Callback
where
    Callback: Fn(&mut World) -> Output + ::core::marker::Send + ::core::marker::Sync + 'static,
    Output: IntoFallible,
{
    fn tags(self, tags: impl IntoTags) -> impl IntoScenarioOrStepHook<World> {
        let tags = tags.into_tags();
        let callback = aliases::sync::Arc::new(move |world: &mut World| (self)(world).into_fallible())
            as aliases::sync::Arc<dyn Fn(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync>;

        Hook::builder().tags(tags).callback(callback).build()
    }
}

#[sealed]
pub trait IntoGlobalHook {
    #[allow(private_interfaces)]
    fn into_hook(self) -> GlobalHook;
}

#[sealed]
impl IntoGlobalHook for GlobalHook {
    #[allow(private_interfaces)]
    fn into_hook(self) -> GlobalHook {
        self
    }
}

#[sealed]
impl<Callback, Output> IntoGlobalHook for Callback
where
    Callback: FnOnce() -> Output + ::core::marker::Send + ::core::marker::Sync + 'static,
    Output: IntoFallible,
{
    #[allow(private_interfaces)]
    fn into_hook(self) -> GlobalHook {
        let callback = ::std::boxed::Box::new(move || (self)().into_fallible())
            as ::std::boxed::Box<dyn FnOnce() -> Fallible + ::core::marker::Send + ::core::marker::Sync>;

        Hook::builder().callback(callback).build()
    }
}

#[sealed]
pub trait GlobalHookCallbackExt {
    fn tags(self, tags: impl IntoTags) -> impl IntoGlobalHook;
}

#[sealed]
impl<Callback, Output> GlobalHookCallbackExt for Callback
where
    Callback: FnOnce() -> Output + ::core::marker::Send + ::core::marker::Sync + 'static,
    Output: IntoFallible,
{
    fn tags(self, tags: impl IntoTags) -> impl IntoGlobalHook {
        let tags = tags.into_tags();
        let callback = ::std::boxed::Box::new(move || (self)().into_fallible())
            as ::std::boxed::Box<dyn FnOnce() -> Fallible + ::core::marker::Send + ::core::marker::Sync>;

        Hook::builder().tags(tags).callback(callback).build()
    }
}

struct StepBuilder<Callback, State: self::step::BuilderState = self::step::Empty> {
    label: ::core::option::Option<StepLabel>,
    description: ::core::option::Option<aliases::string::String>,
    callback: ::core::option::Option<Callback>,

    __phantom: aliases::marker::PhantomCovariant<State>,
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
    fn label(mut self, label: StepLabel) -> StepBuilder<Callback, self::step::SetLabel<State>>
    where
        State::Label: self::marker::IsUnset,
    {
        self.label = ::core::option::Option::from(label);

        StepBuilder {
            label: self.label,
            description: self.description,
            callback: self.callback,

            __phantom: ::core::default::Default::default(),
        }
    }

    fn description(
        mut self,
        description: impl Into<aliases::string::String>,
    ) -> StepBuilder<Callback, self::step::SetDescription<State>>
    where
        State::Description: self::marker::IsUnset,
    {
        self.description = ::core::option::Option::from(description.into());

        StepBuilder {
            label: self.label,
            description: self.description,
            callback: self.callback,

            __phantom: ::core::default::Default::default(),
        }
    }

    fn callback(mut self, callback: Callback) -> StepBuilder<Callback, self::step::SetCallback<State>>
    where
        State::Callback: self::marker::IsUnset,
    {
        self.callback = ::core::option::Option::from(callback);

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
    pub trait IsComplete:
        BuilderState<Label: self::marker::IsSet, Description: self::marker::IsSet, Callback: self::marker::IsSet>
    {
    }

    #[sealed]
    impl<State: BuilderState> IsComplete for State
    where
        State::Label: self::marker::IsSet,
        State::Description: self::marker::IsSet,
        State::Callback: self::marker::IsSet,
    {
    }

    pub struct Empty;

    pub struct SetLabel<State: BuilderState = Empty>(aliases::marker::PhantomCovariant<State>);
    pub struct SetDescription<State: BuilderState = Empty>(aliases::marker::PhantomCovariant<State>);
    pub struct SetCallback<State: BuilderState = Empty>(aliases::marker::PhantomCovariant<State>);

    #[rustfmt::skip] // `reorder_impl_items`
    #[sealed]
    impl BuilderState for Empty {
        type Label = self::marker::Unset<self::members::Label>;
        type Description = self::marker::Unset<self::members::Description>;
        type Callback = self::marker::Unset<self::members::Callback>;
    }

    #[rustfmt::skip] // `reorder_impl_items`
    #[sealed]
    impl<State: BuilderState> BuilderState for SetLabel<State> {
        type Label = self::marker::Set<self::members::Label>;
        type Description = State::Description;
        type Callback = State::Callback;
    }

    #[rustfmt::skip] // `reorder_impl_items`
    #[sealed]
    impl<State: BuilderState> BuilderState for SetDescription<State> {
        type Label = State::Label;
        type Description = self::marker::Set<self::members::Description>;
        type Callback = State::Callback;
    }

    #[rustfmt::skip] // `reorder_impl_items`
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

#[sealed]
pub trait IntoScenarioGivenOrWhenStep<World> {
    #[allow(private_interfaces)]
    fn into_step(self, label: StepLabel) -> ScenarioGivenOrWhenStep<World>;
}

#[sealed]
impl<World, Description, Callback, Output> IntoScenarioGivenOrWhenStep<World> for (Description, Callback)
where
    Description: Into<aliases::string::String>,
    Callback: FnOnce(&mut World) -> Output + ::core::marker::Send + ::core::marker::Sync + 'static,
    Output: IntoFallible,
{
    #[allow(private_interfaces)]
    fn into_step(self, label: StepLabel) -> ScenarioGivenOrWhenStep<World> {
        let (description, callback) = self;

        let callback = ::std::boxed::Box::new(move |world: &mut World| (callback)(world).into_fallible())
            as ::std::boxed::Box<dyn FnOnce(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync>;

        Step::builder().label(label).description(description).callback(callback).build()
    }
}

#[sealed]
pub trait IntoScenarioThenStep<World> {
    #[allow(private_interfaces)]
    fn into_step(self, label: StepLabel) -> ScenarioThenStep<World>;
}

#[sealed]
impl<World, Description, Callback, Output> IntoScenarioThenStep<World> for (Description, Callback)
where
    Description: Into<aliases::string::String>,
    Callback: FnOnce(&World) -> Output + ::core::marker::Send + ::core::marker::Sync + 'static,
    Output: IntoFallible,
{
    #[allow(private_interfaces)]
    fn into_step(self, label: StepLabel) -> ScenarioThenStep<World> {
        let (description, callback) = self;

        let callback = ::std::boxed::Box::new(move |world: &World| (callback)(world).into_fallible())
            as ::std::boxed::Box<dyn FnOnce(&World) -> Fallible + ::core::marker::Send + ::core::marker::Sync>;

        Step::builder().label(label).description(description).callback(callback).build()
    }
}

#[sealed]
pub trait IntoScenarioOutlineGivenOrWhenStep<World, Example> {
    #[allow(private_interfaces)]
    fn into_step(self, label: StepLabel) -> ScenarioOutlineGivenOrWhenStep<World, Example>;
}

#[sealed]
impl<World, Example, Description, Callback, Output> IntoScenarioOutlineGivenOrWhenStep<World, Example>
    for (Description, Callback)
where
    Description: Into<aliases::string::String>,
    Callback: Fn(&mut World, Example) -> Output + ::core::marker::Send + ::core::marker::Sync + 'static,
    Output: IntoFallible,
{
    #[allow(private_interfaces)]
    fn into_step(self, label: StepLabel) -> ScenarioOutlineGivenOrWhenStep<World, Example> {
        let (description, callback) = self;

        let callback = aliases::sync::Arc::new(move |world: &mut World, example: Example| {
            (callback)(world, example).into_fallible()
        })
            as aliases::sync::Arc<dyn Fn(&mut World, Example) -> Fallible + ::core::marker::Send + ::core::marker::Sync>;

        Step::builder().label(label).description(description).callback(callback).build()
    }
}

#[sealed]
pub trait IntoScenarioOutlineThenStep<World, Example> {
    #[allow(private_interfaces)]
    fn into_step(self, label: StepLabel) -> ScenarioOutlineThenStep<World, Example>;
}

#[sealed]
impl<World, Example, Description, Callback, Output> IntoScenarioOutlineThenStep<World, Example>
    for (Description, Callback)
where
    Description: Into<aliases::string::String>,
    Callback: Fn(&World, Example) -> Output + ::core::marker::Send + ::core::marker::Sync + 'static,
    Output: IntoFallible,
{
    #[allow(private_interfaces)]
    fn into_step(self, label: StepLabel) -> ScenarioOutlineThenStep<World, Example> {
        let (description, callback) = self;

        let callback = aliases::sync::Arc::new(move |world: &World, example: Example| {
            (callback)(world, example).into_fallible()
        })
            as aliases::sync::Arc<dyn Fn(&World, Example) -> Fallible + ::core::marker::Send + ::core::marker::Sync>;

        Step::builder().label(label).description(description).callback(callback).build()
    }
}

#[sealed]
pub trait IntoBackgroundGivenStep<World> {
    #[allow(private_interfaces)]
    fn into_step(self, label: StepLabel) -> BackgroundGivenStep<World>;
}

#[sealed]
impl<World, Description, Callback, Output> IntoBackgroundGivenStep<World> for (Description, Callback)
where
    Description: Into<aliases::string::String>,
    Callback: Fn(&mut World) -> Output + ::core::marker::Send + ::core::marker::Sync + 'static,
    Output: IntoFallible,
{
    #[allow(private_interfaces)]
    fn into_step(self, label: StepLabel) -> BackgroundGivenStep<World> {
        let (description, callback) = self;

        let callback = aliases::sync::Arc::new(move |world: &mut World| (callback)(world).into_fallible())
            as aliases::sync::Arc<dyn Fn(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync>;

        Step::builder().label(label).description(description).callback(callback).build()
    }
}

#[sealed]
pub trait IntoTags {
    fn into_tags(self) -> Tags;
}

#[sealed]
impl<I, T> IntoTags for I
where
    I: IntoIterator<Item = T>,
    T: Into<aliases::string::String>,
{
    fn into_tags(self) -> Tags {
        self.into_iter().map(Into::into).collect()
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

#[sealed]
pub trait IntoFailed {
    fn into_failed(self) -> Failed;
}

#[sealed]
impl<T> IntoFailed for T
where
    T: Into<aliases::string::String>,
{
    fn into_failed(self) -> Failed {
        Failed {
            message: self.into(),
        }
    }
}
