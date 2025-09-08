use ::sealed::sealed;

use crate::builders::*;
use crate::engine::*;
use crate::models::*;
use crate::utils::aliases;

pub struct RunnerBuilder<State: self::runner::BuilderState = self::runner::Empty> {
    configurations: self::configurations::RunnerConfigurations,
    
    before_global_hooks: ::std::vec::Vec<GlobalHook>,
    after_global_hooks: ::std::vec::Vec<GlobalHook>,
    
    trials: ::std::vec::Vec<::std::boxed::Box<dyn IntoTrialsWithConfigurations>>,

    __phantom: aliases::marker::PhantomCovariant<State>,
}

impl Runner {
    #[cfg(feature = "natural")]
    pub fn new() -> RunnerBuilder {
        Self::builder()
    }

    pub fn builder() -> RunnerBuilder {
        RunnerBuilder {
            configurations: ::core::default::Default::default(),

            before_global_hooks: ::core::default::Default::default(),
            after_global_hooks: ::core::default::Default::default(),

            trials: ::core::default::Default::default(),

            __phantom: ::core::default::Default::default(),
        }
    }
}

impl<State: self::runner::BuilderState> RunnerBuilder<State> {
    pub fn include_only_ignored(mut self) -> RunnerBuilder<self::runner::SetIgnorePolicy<State>>
    where
        State::IgnorePolicy: self::marker::IsUnset,
    {
        self.configurations.ignore_policy = self::configurations::IgnorePolicy::RetainIgnored;

        RunnerBuilder {
            configurations: self.configurations,

            before_global_hooks: self.before_global_hooks,
            after_global_hooks: self.after_global_hooks,

            trials: self.trials,

            __phantom: ::core::default::Default::default(),
        }
    }

    pub fn include_ignored(mut self) -> RunnerBuilder<self::runner::SetIgnorePolicy<State>>
    where
        State::IgnorePolicy: self::marker::IsUnset,
    {
        self.configurations.ignore_policy = self::configurations::IgnorePolicy::None;

        RunnerBuilder {
            configurations: self.configurations,

            before_global_hooks: self.before_global_hooks,
            after_global_hooks: self.after_global_hooks,

            trials: self.trials,

            __phantom: ::core::default::Default::default(),
        }
    }

    pub fn filter(mut self, filter: impl IntoTagsFilter) -> RunnerBuilder<self::runner::SetTagsFilter<State>>
    where
        State::TagsFilter: self::marker::IsUnset,
    {
        self.configurations.tags_filter = ::core::option::Option::from(::std::boxed::Box::new(filter.into_filter()) as self::configurations::TagsFilter);

        RunnerBuilder {
            configurations: self.configurations,

            before_global_hooks: self.before_global_hooks,
            after_global_hooks: self.after_global_hooks,

            trials: self.trials,

            __phantom: ::core::default::Default::default(),
        }
    }

    pub fn and(mut self, filter: impl IntoTagsFilter) -> RunnerBuilder<self::runner::SetTagsFilter<State>>
    where
        State::TagsFilter: self::marker::IsSet,
        State::IsInTagsFilterChain: self::marker::IsSet,
    {
        let filter = unsafe { self.configurations.tags_filter.unwrap_unchecked() }
            .and(filter);

        self.configurations.tags_filter = ::core::option::Option::from(::std::boxed::Box::new(filter) as self::configurations::TagsFilter);

        RunnerBuilder {
            configurations: self.configurations,

            before_global_hooks: self.before_global_hooks,
            after_global_hooks: self.after_global_hooks,

            trials: self.trials,

            __phantom: ::core::default::Default::default(),
        }
    }

    pub fn or(mut self, filter: impl IntoTagsFilter) -> RunnerBuilder<self::runner::SetTagsFilter<State>>
    where
        State::TagsFilter: self::marker::IsSet,
        State::IsInTagsFilterChain: self::marker::IsSet,
    {
        let filter = unsafe { self.configurations.tags_filter.unwrap_unchecked() }
            .or(filter);

        self.configurations.tags_filter = ::core::option::Option::from(::std::boxed::Box::new(filter) as self::configurations::TagsFilter);

        RunnerBuilder {
            configurations: self.configurations,

            before_global_hooks: self.before_global_hooks,
            after_global_hooks: self.after_global_hooks,

            trials: self.trials,

            __phantom: ::core::default::Default::default(),
        }
    }

    pub fn format(mut self, format: self::configurations::Format) -> RunnerBuilder<self::runner::SetFormat<State>>
    where
        State::Format: self::marker::IsUnset,
    {
        self.configurations.format = format;

        RunnerBuilder {
            configurations: self.configurations,

            before_global_hooks: self.before_global_hooks,
            after_global_hooks: self.after_global_hooks,

            trials: self.trials,

            __phantom: ::core::default::Default::default(),
        }
    }

    pub fn color(mut self, color: self::configurations::Color) -> RunnerBuilder<self::runner::SetColor<State>>
    where
        State::Color: self::marker::IsUnset,
    {
        self.configurations.color = color;

        RunnerBuilder {
            configurations: self.configurations,

            before_global_hooks: self.before_global_hooks,
            after_global_hooks: self.after_global_hooks,

            trials: self.trials,

            __phantom: ::core::default::Default::default(),
        }
    }

    pub fn threads(mut self, threads: impl Into<self::configurations::ThreadsCount>) -> RunnerBuilder<self::runner::SetThreads<State>>
    where
        State::Threads: self::marker::IsUnset,
    {
        self.configurations.threads = ::core::option::Option::from(threads.into());

        RunnerBuilder {
            configurations: self.configurations,

            before_global_hooks: self.before_global_hooks,
            after_global_hooks: self.after_global_hooks,

            trials: self.trials,

            __phantom: ::core::default::Default::default(),
        }
    }

    pub fn logfile(mut self, logfile: impl Into<aliases::path::Path>) -> RunnerBuilder<self::runner::SetLogFile<State>>
    where
        State::LogFile: self::marker::IsUnset,
    {
        self.configurations.logfile = ::core::option::Option::from(logfile.into());

        RunnerBuilder {
            configurations: self.configurations,

            before_global_hooks: self.before_global_hooks,
            after_global_hooks: self.after_global_hooks,

            trials: self.trials,

            __phantom: ::core::default::Default::default(),
        }        
    }

    pub fn before_all(mut self, hook: impl IntoGlobalHook) -> RunnerBuilder<self::runner::SetHooks<State>> {
        self.before_global_hooks.push(hook.into_hook());

        RunnerBuilder {
            configurations: self.configurations,

            before_global_hooks: self.before_global_hooks,
            after_global_hooks: self.after_global_hooks,

            trials: self.trials,

            __phantom: ::core::default::Default::default(),
        }
    }

    pub fn after_all(mut self, hook: impl IntoGlobalHook) -> RunnerBuilder<self::runner::SetHooks<State>> {
        self.after_global_hooks.push(hook.into_hook());

        RunnerBuilder {
            configurations: self.configurations,

            before_global_hooks: self.before_global_hooks,
            after_global_hooks: self.after_global_hooks,

            trials: self.trials,

            __phantom: ::core::default::Default::default(),
        }
    }

    pub fn suite<World>(self, suite: impl IntoSuite<World>) -> RunnerBuilder<self::runner::SetTrials<State>>
    where
        World: ::core::default::Default + 'static,
    {
        self.add(suite.into_suite())
    }

    pub fn feature<World>(self, feature: impl IntoFeature<World>) -> RunnerBuilder<self::runner::SetTrials<State>>
    where
        World: ::core::default::Default + 'static,
    {
        self.add(feature.into_feature())
    }

    fn add(mut self, trials: impl IntoTrialsWithConfigurations) -> RunnerBuilder<self::runner::SetTrials<State>> {
        self.trials.push(::std::boxed::Box::new(trials));

        RunnerBuilder {
            configurations: self.configurations,

            before_global_hooks: self.before_global_hooks,
            after_global_hooks: self.after_global_hooks,

            trials: self.trials,

            __phantom: ::core::default::Default::default(),
        }
    }
}

impl<State: self::runner::BuilderState> RunnerBuilder<State>
where
    State: self::runner::IsComplete,
{
    pub fn build(self) -> Runner {
        Runner {
            configurations: self.configurations,

            before_global_hooks: self.before_global_hooks,
            after_global_hooks: self.after_global_hooks,

            trials: self.trials,
        }
    }

    #[cfg(feature = "natural")]
    pub fn run(self) -> ::std::process::ExitCode {
        self.build().run()
    }
}

mod runner {
    pub(super) use super::*;

    #[sealed]
    pub trait BuilderState: ::core::marker::Sized {
        type IgnorePolicy;
        type TagsFilter;

        type Format;
        type Color;
        type Threads;
        type LogFile;

        type Hooks;
        type Trials;

        type IsInTagsFilterChain;
    }

    #[sealed]
    pub trait IsComplete: BuilderState<Trials: self::marker::IsSet> {}

    #[sealed]
    impl<State: BuilderState> IsComplete for State where State::Trials: self::marker::IsSet {}

    pub struct Empty;

    pub struct SetIgnorePolicy<State: BuilderState = Empty>(aliases::marker::PhantomCovariant<State>);
    pub struct SetTagsFilter<State: BuilderState = Empty>(aliases::marker::PhantomCovariant<State>);

    pub struct SetFormat<State: BuilderState = Empty>(aliases::marker::PhantomCovariant<State>);
    pub struct SetColor<State: BuilderState = Empty>(aliases::marker::PhantomCovariant<State>);
    pub struct SetThreads<State: BuilderState = Empty>(aliases::marker::PhantomCovariant<State>);
    pub struct SetLogFile<State: BuilderState = Empty>(aliases::marker::PhantomCovariant<State>);

    pub struct SetHooks<State: BuilderState = Empty>(aliases::marker::PhantomCovariant<State>);
    pub struct SetTrials<State: BuilderState = Empty>(aliases::marker::PhantomCovariant<State>);

    #[sealed]
    impl BuilderState for Empty {
        type IgnorePolicy = self::marker::Unset<self::members::IgnorePolicy>;
        type TagsFilter = self::marker::Unset<self::members::TagsFilter>;

        type Format = self::marker::Unset<self::members::Format>;
        type Color = self::marker::Unset<self::members::Color>;
        type Threads = self::marker::Unset<self::members::Threads>;
        type LogFile = self::marker::Unset<self::members::LogFile>;

        type Hooks = self::marker::Unset<self::members::Hooks>;
        type Trials = self::marker::Unset<self::members::Trials>;

        type IsInTagsFilterChain = self::marker::Unset<self::members::IsInTagsFilterChain>;
    }

    #[sealed]
    impl<State: BuilderState> BuilderState for SetIgnorePolicy<State> {
        type IgnorePolicy = self::marker::Set<self::members::IgnorePolicy>;
        type TagsFilter = State::TagsFilter;

        type Format = State::Format;
        type Color = State::Color;
        type Threads = State::Threads;
        type LogFile = State::LogFile;

        type Hooks = State::Hooks;
        type Trials = State::Trials;

        type IsInTagsFilterChain = self::marker::Unset<self::members::IsInTagsFilterChain>;
    }

    #[sealed]
    impl<State: BuilderState> BuilderState for SetTagsFilter<State> {
        type IgnorePolicy = State::IgnorePolicy;
        type TagsFilter = self::marker::Set<self::members::TagsFilter>;

        type Format = State::Format;
        type Color = State::Color;
        type Threads = State::Threads;
        type LogFile = State::LogFile;

        type Hooks = State::Hooks;
        type Trials = State::Trials;

        type IsInTagsFilterChain = self::marker::Set<self::members::IsInTagsFilterChain>;
    }

    #[sealed]
    impl<State: BuilderState> BuilderState for SetFormat<State> {
        type IgnorePolicy = State::IgnorePolicy;
        type TagsFilter = State::TagsFilter;

        type Format = self::marker::Set<self::members::Format>;
        type Color = State::Color;
        type Threads = State::Threads;
        type LogFile = State::LogFile;

        type Hooks = State::Hooks;
        type Trials = State::Trials;
        
        type IsInTagsFilterChain = self::marker::Unset<self::members::IsInTagsFilterChain>;
    }

    #[sealed]
    impl<State: BuilderState> BuilderState for SetColor<State> {
        type IgnorePolicy = State::IgnorePolicy;
        type TagsFilter = State::TagsFilter;

        type Format = State::Format;
        type Color = self::marker::Set<self::members::Color>;
        type Threads = State::Threads;
        type LogFile = State::LogFile;

        type Hooks = State::Hooks;
        type Trials = State::Trials;
        
        type IsInTagsFilterChain = self::marker::Unset<self::members::IsInTagsFilterChain>;
    }

    #[sealed]
    impl<State: BuilderState> BuilderState for SetThreads<State> {
        type IgnorePolicy = State::IgnorePolicy;
        type TagsFilter = State::TagsFilter;

        type Format = State::Format;
        type Color = State::Color;
        type Threads = self::marker::Set<self::members::Threads>;
        type LogFile = State::LogFile;

        type Hooks = State::Hooks;
        type Trials = State::Trials;
        
        type IsInTagsFilterChain = self::marker::Unset<self::members::IsInTagsFilterChain>;
    }

    #[sealed]
    impl<State: BuilderState> BuilderState for SetLogFile<State> {
        type IgnorePolicy = State::IgnorePolicy;
        type TagsFilter = State::TagsFilter;

        type Format = State::Format;
        type Color = State::Color;
        type Threads = State::Threads;
        type LogFile = self::marker::Set<self::members::LogFile>;

        type Hooks = State::Hooks;
        type Trials = State::Trials;
        
        type IsInTagsFilterChain = self::marker::Unset<self::members::IsInTagsFilterChain>;
    }

    #[sealed]
    impl<State: BuilderState> BuilderState for SetHooks<State> {
        type IgnorePolicy = State::IgnorePolicy;
        type TagsFilter = State::TagsFilter;

        type Format = State::Format;
        type Color = State::Color;
        type Threads = State::Threads;
        type LogFile = State::LogFile;

        type Hooks = self::marker::Set<self::members::Hooks>;
        type Trials = State::Trials;
        
        type IsInTagsFilterChain = self::marker::Unset<self::members::IsInTagsFilterChain>;
    }

    #[sealed]
    impl<State: BuilderState> BuilderState for SetTrials<State> {
        type IgnorePolicy = State::IgnorePolicy;
        type TagsFilter = State::TagsFilter;

        type Format = State::Format;
        type Color = State::Color;
        type Threads = State::Threads;
        type LogFile = State::LogFile;

        type Hooks = State::Hooks;
        type Trials = self::marker::Set<self::members::Trials>;
        
        type IsInTagsFilterChain = self::marker::Unset<self::members::IsInTagsFilterChain>;
    }

    mod members {
        pub struct IgnorePolicy;
        pub struct TagsFilter;

        pub struct Format;
        pub struct Color;
        pub struct Threads;
        pub struct LogFile;

        pub struct Hooks;
        pub struct Trials;

        pub struct IsInTagsFilterChain;
    }
}

#[sealed]
pub trait IntoTagsFilter {
    fn into_filter(self) -> impl Fn(&Tags) -> bool + 'static;

    fn and(self, other: impl IntoTagsFilter) -> impl Fn(&Tags) -> bool + 'static
    where
        Self: ::core::marker::Sized,
    {
        let this = self.into_filter();
        let other = other.into_filter();

        move |tags| this(tags) && other(tags)
    }

    fn or(self, other: impl IntoTagsFilter) -> impl Fn(&Tags) -> bool + 'static
    where
        Self: ::core::marker::Sized,
    {
        let this = self.into_filter();
        let other = other.into_filter();

        move |tags| this(tags) || other(tags)
    }
}

#[sealed]
impl<F> IntoTagsFilter for F
where
    F: Fn(&Tags) -> bool + 'static,
{
    fn into_filter(self) -> impl Fn(&Tags) -> bool + 'static {
        self
    }
}

#[sealed]
impl<T> IntoTagsFilter for &'static [T]
where
    T: Into<aliases::string::String> + ::core::clone::Clone + 'static,
{
    fn into_filter(self) -> impl Fn(&Tags) -> bool + 'static {
        move |tags| {
            let filter = self.iter().cloned().map(Into::into).collect::<Tags>();
            !filter.is_disjoint(tags)
        }
    }
}

#[sealed]
impl<T, const N: usize> IntoTagsFilter for [T; N]
where
    T: Into<aliases::string::String> + ::core::clone::Clone + 'static,
{
    fn into_filter(self) -> impl Fn(&Tags) -> bool + 'static {
        move |tags| {
            let filter = self.iter().cloned().map(Into::into).collect::<Tags>();
            !filter.is_disjoint(tags)
        }
    }
}

impl From<u64> for self::configurations::ThreadsCount {
    fn from(value: u64) -> Self {
        Self::Custom(value)
    }
}

pub struct SuiteBuilder<World, State: self::suite::BuilderState = self::suite::Empty> {
    before_scenario_hooks: ::std::vec::Vec<ScenarioOrStepHook<World>>,
    after_scenario_hooks: ::std::vec::Vec<ScenarioOrStepHook<World>>,

    before_step_hooks: ::std::vec::Vec<ScenarioOrStepHook<World>>,
    after_step_hooks: ::std::vec::Vec<ScenarioOrStepHook<World>>,

    features: ::std::vec::Vec<Feature<World>>,

    __phantom: aliases::marker::PhantomCovariant<State>,
}

impl<World> Suite<World> {
    #[cfg(feature = "natural")]
    pub fn new() -> SuiteBuilder<World> {
        Self::builder()
    }

    pub fn builder() -> SuiteBuilder<World> {
        SuiteBuilder {
            before_scenario_hooks: ::core::default::Default::default(),
            after_scenario_hooks: ::core::default::Default::default(),
            
            before_step_hooks: ::core::default::Default::default(),
            after_step_hooks: ::core::default::Default::default(),
            
            features: ::core::default::Default::default(),

            __phantom: ::core::default::Default::default(),
        }
    }
}

impl<World, State: self::suite::BuilderState> SuiteBuilder<World, State> {
    pub fn before_scenario(mut self, hook: impl IntoScenarioOrStepHook<World>) -> SuiteBuilder<World, self::suite::SetHooks<State>> {
        self.before_scenario_hooks.push(hook.into_hook());
        
        SuiteBuilder {
            before_scenario_hooks: self.before_scenario_hooks,
            after_scenario_hooks: self.after_scenario_hooks,
            
            before_step_hooks: self.before_step_hooks,
            after_step_hooks: self.after_step_hooks,
            
            features: self.features,

            __phantom: ::core::default::Default::default(),
        }
    }

    pub fn after_scenario(mut self, hook: impl IntoScenarioOrStepHook<World>) -> SuiteBuilder<World, self::suite::SetHooks<State>> {
        self.after_scenario_hooks.push(hook.into_hook());
        
        SuiteBuilder {
            before_scenario_hooks: self.before_scenario_hooks,
            after_scenario_hooks: self.after_scenario_hooks,
            
            before_step_hooks: self.before_step_hooks,
            after_step_hooks: self.after_step_hooks,
            
            features: self.features,

            __phantom: ::core::default::Default::default(),
        }
    }

    pub fn before_step(mut self, hook: impl IntoScenarioOrStepHook<World>) -> SuiteBuilder<World, self::suite::SetHooks<State>> {
        self.before_step_hooks.push(hook.into_hook());

        SuiteBuilder {
            before_scenario_hooks: self.before_scenario_hooks,
            after_scenario_hooks: self.after_scenario_hooks,
            
            before_step_hooks: self.before_step_hooks,
            after_step_hooks: self.after_step_hooks,
            
            features: self.features,

            __phantom: ::core::default::Default::default(),
        }
    }

    pub fn after_step(mut self, hook: impl IntoScenarioOrStepHook<World>) -> SuiteBuilder<World, self::suite::SetHooks<State>> {
        self.after_step_hooks.push(hook.into_hook());

        SuiteBuilder {
            before_scenario_hooks: self.before_scenario_hooks,
            after_scenario_hooks: self.after_scenario_hooks,
            
            before_step_hooks: self.before_step_hooks,
            after_step_hooks: self.after_step_hooks,
            
            features: self.features,

            __phantom: ::core::default::Default::default(),
        }
    }

    pub fn feature(mut self, feature: impl IntoFeature<World>) -> SuiteBuilder<World, self::suite::SetFeatures<State>> {
        self.features.push(feature.into_feature());

        SuiteBuilder {
            before_scenario_hooks: self.before_scenario_hooks,
            after_scenario_hooks: self.after_scenario_hooks,
            
            before_step_hooks: self.before_step_hooks,
            after_step_hooks: self.after_step_hooks,
            
            features: self.features,

            __phantom: ::core::default::Default::default(),
        }
    }

    pub fn features<T>(mut self, features: impl IntoIterator<Item = T>) -> SuiteBuilder<World, self::suite::SetFeatures<State>>
    where
        T: IntoFeature<World>,
    {
        self.features.extend(features.into_iter().map(IntoFeature::into_feature));

        SuiteBuilder {
            before_scenario_hooks: self.before_scenario_hooks,
            after_scenario_hooks: self.after_scenario_hooks,
            
            before_step_hooks: self.before_step_hooks,
            after_step_hooks: self.after_step_hooks,
            
            features: self.features,

            __phantom: ::core::default::Default::default(),
        }
    }
}

impl<World, State: self::suite::BuilderState> SuiteBuilder<World, State>
where
    State: self::suite::IsComplete,
{
    pub fn build(self) -> Suite<World> {
        Suite {
            before_scenario_hooks: self.before_scenario_hooks,
            after_scenario_hooks: self.after_scenario_hooks,

            before_step_hooks: self.before_step_hooks,
            after_step_hooks: self.after_step_hooks,

            features: self.features,
        }
    }
}

mod suite {
    pub(super) use super::*;

    #[sealed]
    pub trait BuilderState: ::core::marker::Sized {
        type Hooks;
        type Features;
    }

    #[sealed]
    pub trait IsComplete: BuilderState {}

    #[sealed]
    impl<State: BuilderState> IsComplete for State {}

    pub struct Empty;

    pub struct SetHooks<State: BuilderState = Empty>(aliases::marker::PhantomCovariant<State>);
    pub struct SetFeatures<State: BuilderState = Empty>(aliases::marker::PhantomCovariant<State>);

    #[sealed]
    impl BuilderState for Empty {
        type Hooks = self::marker::Unset<self::members::Hooks>;
        type Features = self::marker::Unset<self::members::Features>;
    }

    #[sealed]
    impl<State: BuilderState> BuilderState for SetHooks<State> {
        type Hooks = self::marker::Set<self::members::Hooks>;
        type Features = State::Features;
    }

    #[sealed]
    impl<State: BuilderState> BuilderState for SetFeatures<State> {
        type Hooks = State::Hooks;
        type Features = self::marker::Set<self::members::Features>;
    }

    mod members {
        pub struct Hooks;
        pub struct Features;
    }
}

#[sealed]
pub trait IntoSuite<World> {
    fn into_suite(self) -> Suite<World>;
}

#[sealed]
impl<World> IntoSuite<World> for Suite<World> {
    fn into_suite(self) -> Suite<World> {
        self
    }
}

#[cfg(feature = "natural")]
#[sealed]
impl<World, State: self::suite::BuilderState> IntoSuite<World> for SuiteBuilder<World, State>
where
    State: self::suite::IsComplete,
{
    fn into_suite(self) -> Suite<World> {
        self.build()
    }
}
