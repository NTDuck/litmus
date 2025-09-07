use ::sealed::sealed;

use crate::models::*;
use crate::builders::marker;
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
    #[cfg(feature = "natural")]
    #[inline(always)]
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
        value: impl Into<aliases::string::String>,
    ) -> FeatureBuilder<World, self::feature::SetDescription<State>>
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

    pub fn background(
        mut self,
        value: impl Into<Background<World>>,
    ) -> FeatureBuilder<World, self::feature::SetBackground<State>>
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
            self.scenarios.iter_mut().for_each(|scenario| scenario.ignored = ::core::option::Option::from(*ignored));

            self.rules.iter_mut().for_each(|rule| rule.ignored = ::core::option::Option::from(*ignored));
        }
    }

    /// See also: [Tag inheritance](https://cucumber.io/docs/cucumber/api/#tag-inheritance)
    fn propagate_tags(&mut self) {
        if let Some(tags) = self.tags.as_ref() {
            self.scenarios.iter_mut().for_each(|scenario| {
                scenario
                    .tags
                    .get_or_insert_with(|| Tags::from(::std::iter::empty::<aliases::string::String>()))
                    .extend(tags.clone())
            });

            self.rules.iter_mut().for_each(|rule| {
                rule.tags
                    .get_or_insert_with(|| Tags::from(::std::iter::empty::<aliases::string::String>()))
                    .extend(tags.clone())
            });
        }
    }
}

#[cfg(feature = "natural")]
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

    pub struct SetDescription<State: BuilderState = Empty>(aliases::marker::PhantomCovariant<State>);
    pub struct SetIgnored<State: BuilderState = Empty>(aliases::marker::PhantomCovariant<State>);
    pub struct SetTags<State: BuilderState = Empty>(aliases::marker::PhantomCovariant<State>);

    pub struct SetBackground<State: BuilderState = Empty>(aliases::marker::PhantomCovariant<State>);

    #[sealed]
    impl BuilderState for Empty {
        type Background = self::marker::Unset<self::members::Background>;
        type Description = self::marker::Unset<self::members::Description>;
        type Ignored = self::marker::Unset<self::members::Ignored>;
        type Tags = self::marker::Unset<self::members::Tags>;
    }

    #[sealed]
    impl<State: BuilderState> BuilderState for SetDescription<State> {
        type Background = State::Background;
        type Description = self::marker::Set<self::members::Description>;
        type Ignored = State::Ignored;
        type Tags = State::Tags;
    }

    #[sealed]
    impl<State: BuilderState> BuilderState for SetIgnored<State> {
        type Background = State::Background;
        type Description = State::Description;
        type Ignored = self::marker::Set<self::members::Ignored>;
        type Tags = State::Tags;
    }

    #[sealed]
    impl<State: BuilderState> BuilderState for SetTags<State> {
        type Background = State::Background;
        type Description = State::Description;
        type Ignored = State::Ignored;
        type Tags = self::marker::Set<self::members::Tags>;
    }

    #[sealed]
    impl<State: BuilderState> BuilderState for SetBackground<State> {
        type Background = self::marker::Set<self::members::Background>;
        type Description = State::Description;
        type Ignored = State::Ignored;
        type Tags = State::Tags;
    }

    mod members {
        pub struct Description;
        pub struct Ignored;
        pub struct Tags;

        pub struct Background;
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
    #[cfg(feature = "natural")]
    #[inline(always)]
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
        value: impl Into<aliases::string::String>,
    ) -> RuleBuilder<World, self::rule::SetDescription<State>>
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

    pub fn background(
        mut self,
        value: impl Into<Background<World>>,
    ) -> RuleBuilder<World, self::rule::SetBackground<State>>
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
            self.scenarios.iter_mut().for_each(|scenario| scenario.ignored = ::core::option::Option::from(*ignored));
        }
    }

    /// See also: [Tag inheritance](https://cucumber.io/docs/cucumber/api/#tag-inheritance)
    fn propagate_tags(&mut self) {
        if let Some(tags) = self.tags.as_ref() {
            self.scenarios.iter_mut().for_each(|scenario| {
                scenario
                    .tags
                    .get_or_insert_with(|| Tags::from(::std::iter::empty::<aliases::string::String>()))
                    .extend(tags.clone())
            });
        }
    }
}

#[cfg(feature = "natural")]
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

    pub struct SetDescription<State: BuilderState = Empty>(aliases::marker::PhantomCovariant<State>);
    pub struct SetIgnored<State: BuilderState = Empty>(aliases::marker::PhantomCovariant<State>);
    pub struct SetTags<State: BuilderState = Empty>(aliases::marker::PhantomCovariant<State>);

    pub struct SetBackground<State: BuilderState = Empty>(aliases::marker::PhantomCovariant<State>);

    #[sealed]
    impl BuilderState for Empty {
        type Background = self::marker::Unset<self::members::Background>;
        type Description = self::marker::Unset<self::members::Description>;
        type Ignored = self::marker::Unset<self::members::Ignored>;
        type Tags = self::marker::Unset<self::members::Tags>;
    }

    #[sealed]
    impl<State: BuilderState> BuilderState for SetDescription<State> {
        type Background = State::Background;
        type Description = self::marker::Set<self::members::Description>;
        type Ignored = State::Ignored;
        type Tags = State::Tags;
    }

    #[sealed]
    impl<State: BuilderState> BuilderState for SetIgnored<State> {
        type Background = State::Background;
        type Description = State::Description;
        type Ignored = self::marker::Set<self::members::Ignored>;
        type Tags = State::Tags;
    }

    #[sealed]
    impl<State: BuilderState> BuilderState for SetTags<State> {
        type Background = State::Background;
        type Description = State::Description;
        type Ignored = State::Ignored;
        type Tags = self::marker::Set<self::members::Tags>;
    }

    #[sealed]
    impl<State: BuilderState> BuilderState for SetBackground<State> {
        type Background = self::marker::Set<self::members::Background>;
        type Description = State::Description;
        type Ignored = State::Ignored;
        type Tags = State::Tags;
    }

    mod members {
        pub struct Description;
        pub struct Ignored;
        pub struct Tags;

        pub struct Background;
    }
}

pub struct ScenarioBuilder<World, State: self::scenario::BuilderState = self::scenario::Empty> {
    description: ::core::option::Option<aliases::string::String>,
    ignored: ::core::option::Option<bool>,
    tags: ::core::option::Option<Tags>,

    pub(crate) given: ::std::vec::Vec<ScenarioGivenStep<World>>,
    pub(crate) when: ::std::vec::Vec<ScenarioWhenStep<World>>,
    pub(crate) then: ::std::vec::Vec<ScenarioThenStep<World>>,

    __phantom: aliases::marker::PhantomCovariant<State>,
}

impl<World> Scenario<World> {
    #[cfg(feature = "natural")]
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

    pub fn given<Callback, Output>(
        mut self,
        step: impl IntoScenarioGivenStep<World>,
    ) -> ScenarioBuilder<World, self::scenario::SetGiven<State>> {
        self.given.push(step.into_step(StepLabel::Given));

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
    pub fn and(
        mut self,
        step: impl IntoScenarioGivenStep<World>,
    ) -> ScenarioBuilder<World, self::scenario::SetGiven<self::scenario::SetGiven<InnerState>>> {
        self.given.push(step.into_step(StepLabel::And));

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

    pub fn but(
        mut self,
        step: impl IntoScenarioGivenStep<World>,
    ) -> ScenarioBuilder<World, self::scenario::SetGiven<self::scenario::SetGiven<InnerState>>> {
        self.given.push(step.into_step(StepLabel::But));

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

    pub fn when(
        mut self,
        step: impl IntoScenarioWhenStep<World>,
    ) -> ScenarioBuilder<World, self::scenario::SetWhen<self::scenario::SetGiven<InnerState>>> {
        self.when.push(step.into_step(StepLabel::When));

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
    pub fn and(
        mut self,
        step: impl IntoScenarioWhenStep<World>,
    ) -> ScenarioBuilder<World, self::scenario::SetWhen<self::scenario::SetWhen<InnerState>>> {
        self.when.push(step.into_step(StepLabel::And));

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

    pub fn but(
        mut self,
        step: impl IntoScenarioWhenStep<World>,
    ) -> ScenarioBuilder<World, self::scenario::SetWhen<self::scenario::SetWhen<InnerState>>> {
        self.when.push(step.into_step(StepLabel::But));

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

    pub fn then(
        mut self,
        step: impl IntoScenarioThenStep<World>,
    ) -> ScenarioBuilder<World, self::scenario::SetThen<self::scenario::SetWhen<InnerState>>> {
        self.then.push(step.into_step(StepLabel::Then));

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
    pub fn and(
        mut self,
        step: impl IntoScenarioThenStep<World>,
    ) -> ScenarioBuilder<World, self::scenario::SetThen<self::scenario::SetThen<InnerState>>> {
        self.then.push(step.into_step(StepLabel::And));

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

    pub fn but(
        mut self,
        step: impl IntoScenarioThenStep<World>,
    ) -> ScenarioBuilder<World, self::scenario::SetThen<self::scenario::SetThen<InnerState>>> {
        self.then.push(step.into_step(StepLabel::But));

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

    #[sealed]
    impl BuilderState for Empty {
        type Description = self::marker::Unset<self::members::Description>;
        type Given = self::marker::Unset<self::members::Given>;
        type Ignored = self::marker::Unset<self::members::Ignored>;
        type Tags = self::marker::Unset<self::members::Tags>;
        type Then = self::marker::Unset<self::members::Then>;
        type When = self::marker::Unset<self::members::When>;
    }

    #[sealed]
    impl<State: BuilderState> BuilderState for SetDescription<State> {
        type Description = self::marker::Set<self::members::Description>;
        type Given = State::Given;
        type Ignored = State::Ignored;
        type Tags = State::Tags;
        type Then = State::Then;
        type When = State::When;
    }

    #[sealed]
    impl<State: BuilderState> BuilderState for SetIgnored<State> {
        type Description = State::Description;
        type Given = State::Given;
        type Ignored = self::marker::Set<self::members::Ignored>;
        type Tags = State::Tags;
        type Then = State::Then;
        type When = State::When;
    }

    #[sealed]
    impl<State: BuilderState> BuilderState for SetTags<State> {
        type Description = State::Description;
        type Given = State::Given;
        type Ignored = State::Ignored;
        type Tags = self::marker::Set<self::members::Tags>;
        type Then = State::Then;
        type When = State::When;
    }

    #[sealed]
    impl<State: BuilderState> BuilderState for SetGiven<State> {
        type Description = State::Description;
        type Given = self::marker::Set<self::members::Given>;
        type Ignored = State::Ignored;
        type Tags = State::Tags;
        type Then = State::Then;
        type When = State::When;
    }

    #[sealed]
    impl<State: BuilderState> BuilderState for SetWhen<State> {
        type Description = State::Description;
        type Given = State::Given;
        type Ignored = State::Ignored;
        type Tags = State::Tags;
        type Then = State::Then;
        type When = self::marker::Set<self::members::When>;
    }

    #[sealed]
    impl<State: BuilderState> BuilderState for SetThen<State> {
        type Description = State::Description;
        type Given = State::Given;
        type Ignored = State::Ignored;
        type Tags = State::Tags;
        type Then = self::marker::Set<self::members::Then>;
        type When = State::When;
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

#[cfg(feature = "natural")]
#[sealed]
impl<World, State: self::scenario::BuilderState> IntoScenario<World> for ScenarioBuilder<World, State>
where
    State: self::scenario::IsComplete,
{
    fn into_scenario(self) -> Scenario<World> {
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
    #[cfg(feature = "natural")]
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

    pub fn given(mut self, step: impl IntoBackgroundGivenStep<World>) -> BackgroundBuilder<World, self::background::SetGiven<State>> {
        self.given.push(step.into_step(StepLabel::Given));

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
    pub fn and<Callback, Output>(
        mut self,
        step: impl IntoBackgroundGivenStep<World>
    ) -> BackgroundBuilder<World, self::background::SetGiven<self::background::SetGiven<InnerState>>> {
        self.given.push(step.into_step(StepLabel::And));

        BackgroundBuilder {
            description: self.description,
            ignored: self.ignored,

            given: self.given,

            __phantom: ::core::default::Default::default(),
        }
    }

    pub fn but<Callback, Output>(
        mut self,
        step: impl IntoBackgroundGivenStep<World>
    ) -> BackgroundBuilder<World, self::background::SetGiven<self::background::SetGiven<InnerState>>> {
        self.given.push(step.into_step(StepLabel::But));

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

    #[sealed]
    impl BuilderState for Empty {
        type Description = self::marker::Unset<self::members::Description>;
        type Given = self::marker::Unset<self::members::Given>;
        type Ignored = self::marker::Unset<self::members::Ignored>;
    }

    #[sealed]
    impl<State: BuilderState> BuilderState for SetDescription<State> {
        type Description = self::marker::Set<self::members::Description>;
        type Given = State::Given;
        type Ignored = State::Ignored;
    }

    #[sealed]
    impl<State: BuilderState> BuilderState for SetIgnored<State> {
        type Description = State::Description;
        type Given = State::Given;
        type Ignored = self::marker::Set<self::members::Ignored>;
    }

    #[sealed]
    impl<State: BuilderState> BuilderState for SetGiven<State> {
        type Description = State::Description;
        type Given = self::marker::Set<self::members::Given>;
        type Ignored = State::Ignored;
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

#[cfg(feature = "natural")]
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
    impl<State: BuilderState> IsComplete for State where State::Callback: self::marker::IsSet {}

    pub struct Empty;

    pub struct SetTags<State: BuilderState = Empty>(aliases::marker::PhantomCovariant<State>);
    pub struct SetCallback<State: BuilderState = Empty>(aliases::marker::PhantomCovariant<State>);

    #[sealed]
    impl BuilderState for Empty {
        type Callback = self::marker::Unset<self::members::Callback>;
        type Tags = self::marker::Unset<self::members::Tags>;
    }

    #[sealed]
    impl<State: BuilderState> BuilderState for SetTags<State> {
        type Callback = State::Callback;
        type Tags = self::marker::Set<self::members::Tags>;
    }

    #[sealed]
    impl<State: BuilderState> BuilderState for SetCallback<State> {
        type Callback = self::marker::Set<self::members::Callback>;
        type Tags = State::Tags;
    }

    mod members {
        pub struct Tags;
        pub struct Callback;
    }
}

#[sealed]
pub trait IntoNonGlobalHook<World> {
    #[allow(private_interfaces)]
    fn into_hook(self) -> NonGlobalHook<World>;
}

#[sealed]
impl<World, Callback, Output> IntoNonGlobalHook<World> for Callback
where
    Callback: Fn(&mut World) -> Output + ::core::marker::Send + ::core::marker::Sync + 'static,
    Output: IntoFallible,
{
    #[allow(private_interfaces)]
    fn into_hook(self) -> NonGlobalHook<World> {
        let callback = aliases::sync::Arc::new(move |world: &mut World| (self)(world).into_fallible())
            as aliases::sync::Arc<dyn Fn(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync>;

        Hook::builder()
            .callback(callback)
            .build()
    }
}

#[sealed]
impl<World, TagsLike, Callback, Output> IntoNonGlobalHook<World> for (TagsLike, Callback)
where
    TagsLike: IntoTags,
    Callback: Fn(&mut World) -> Output + ::core::marker::Send + ::core::marker::Sync + 'static,
    Output: IntoFallible,
{
    #[allow(private_interfaces)]
    fn into_hook(self) -> NonGlobalHook<World> {
        let (tags, callback) = self;

        let tags = tags.into_tags();
        let callback = aliases::sync::Arc::new(move |world: &mut World| (callback)(world).into_fallible())
            as aliases::sync::Arc<dyn Fn(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync>;

        Hook::builder()
            .tags(tags)
            .callback(callback)
            .build()
    }
}

#[sealed]
pub trait IntoGlobalHook {
    #[allow(private_interfaces)]
    fn into_hook(self) -> GlobalHook;
}

#[sealed]
impl<Callback, Output> IntoGlobalHook for Callback
where
    Callback: FnOnce() -> Output + ::core::marker::Send + ::core::marker::Sync + 'static,
    Output: IntoFallible,
{
    fn into_hook(self) -> GlobalHook {
        let callback = ::std::boxed::Box::new(move || (self)().into_fallible())
            as ::std::boxed::Box<dyn FnOnce() -> Fallible + ::core::marker::Send + ::core::marker::Sync>;

        Hook::builder()
            .callback(callback)
            .build()
    }
}

#[sealed]
impl<TagsLike, Callback, Output> IntoGlobalHook for (TagsLike, Callback)
where
    TagsLike: IntoTags,
    Callback: FnOnce() -> Output + ::core::marker::Send + ::core::marker::Sync + 'static,
    Output: IntoFallible,
{
    fn into_hook(self) -> GlobalHook {
        let (tags, callback) = self;

        let tags = tags.into_tags();
        let callback = ::std::boxed::Box::new(move || (callback)().into_fallible())
            as ::std::boxed::Box<dyn FnOnce() -> Fallible + ::core::marker::Send + ::core::marker::Sync>;

        Hook::builder()
            .tags(tags)
            .callback(callback)
            .build()
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

    #[sealed]
    impl BuilderState for Empty {
        type Callback = self::marker::Unset<self::members::Callback>;
        type Description = self::marker::Unset<self::members::Description>;
        type Label = self::marker::Unset<self::members::Label>;
    }

    #[sealed]
    impl<State: BuilderState> BuilderState for SetLabel<State> {
        type Callback = State::Callback;
        type Description = State::Description;
        type Label = self::marker::Set<self::members::Label>;
    }

    #[sealed]
    impl<State: BuilderState> BuilderState for SetDescription<State> {
        type Callback = State::Callback;
        type Description = self::marker::Set<self::members::Description>;
        type Label = State::Label;
    }

    #[sealed]
    impl<State: BuilderState> BuilderState for SetCallback<State> {
        type Callback = self::marker::Set<self::members::Callback>;
        type Description = State::Description;
        type Label = State::Label;
    }

    mod members {
        pub struct Label;
        pub struct Description;
        pub struct Callback;
    }
}

#[sealed]
pub trait IntoScenarioGivenStep<World> {
    #[allow(private_interfaces)]
    fn into_step(self, label: StepLabel) -> ScenarioGivenStep<World>;
}

#[sealed]
impl<World, Description, Callback, Output> IntoScenarioGivenStep<World> for (Description, Callback)
where
    Description: Into<aliases::string::String>,
    Callback: FnOnce(&mut World) -> Output + ::core::marker::Send + ::core::marker::Sync + 'static,
    Output: IntoFallible,
{
    #[allow(private_interfaces)]
    fn into_step(self, label: StepLabel) -> ScenarioGivenStep<World> {
        let (description, callback) = self;

        let callback = ::std::boxed::Box::new(move |world: &mut World| (callback)(world).into_fallible())
            as ::std::boxed::Box<dyn FnOnce(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync>;

        Step::builder()
            .label(label)
            .description(description)
            .callback(callback)
            .build()
    }
}

#[sealed]
pub trait IntoScenarioWhenStep<World> {
    #[allow(private_interfaces)]
    fn into_step(self, label: StepLabel) -> ScenarioWhenStep<World>;
}

#[sealed]
impl<World, Description, Callback, Output> IntoScenarioWhenStep<World> for (Description, Callback)
where
    Description: Into<aliases::string::String>,
    Callback: FnOnce(&mut World) -> Output + ::core::marker::Send + ::core::marker::Sync + 'static,
    Output: IntoFallible,
{
    #[allow(private_interfaces)]
    fn into_step(self, label: StepLabel) -> ScenarioWhenStep<World> {
        let (description, callback) = self;

        let callback = ::std::boxed::Box::new(move |world: &mut World| (callback)(world).into_fallible())
            as ::std::boxed::Box<dyn FnOnce(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync>;

        Step::builder()
            .label(label)
            .description(description)
            .callback(callback)
            .build()
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

        Step::builder()
            .label(label)
            .description(description)
            .callback(callback)
            .build()
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

        Step::builder()
            .label(label)
            .description(description)
            .callback(callback)
            .build()
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
        Failed { message: self.into() }
    }
}
