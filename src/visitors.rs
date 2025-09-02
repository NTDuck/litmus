use ::sealed::sealed;

use crate::models::*;
use crate::utils::aliases;

pub struct Runner {
    trials: ::std::vec::Vec<::libtest_mimic::Trial>,

    configurations: self::configurations::RunnerConfigurations,

    before_global_hooks: ::std::vec::Vec<Hook<::std::boxed::Box<dyn FnOnce() -> Fallible + ::core::marker::Send + ::core::marker::Sync>>>,
    after_global_hooks: ::std::vec::Vec<Hook<::std::boxed::Box<dyn FnOnce() -> Fallible + ::core::marker::Send + ::core::marker::Sync>>>,
}

pub use configurations as config;

pub mod configurations {
    pub(super) use super::*;

    #[derive(::core::default::Default)]
    pub(crate) struct RunnerConfigurations {
        pub(crate) ignore_policy: IgnorePolicy,
        pub(crate) tags_filter: ::core::option::Option<::std::boxed::Box<dyn Fn(&Tags) -> bool>>,
        
        /* Used by `::libtest_mimic::Arguments` */
        pub(crate) format: Format,
        pub(crate) color: Color,
        pub(crate) threads: ::core::option::Option<ThreadsCount>,
        pub(crate) logfile: ::core::option::Option<::std::borrow::Cow<'static, ::std::path::Path>>,
    }

    #[derive(::core::default::Default, ::core::clone::Clone, ::core::marker::Copy)]
    pub(crate) enum IgnorePolicy {
        RetainIgnored,

        #[default]
        RetainUnignored,
        
        None,
    }

    #[derive(::core::default::Default)]
    pub enum Format {
        #[default]
        Pretty,
        Terse,
        Json,
    }

    #[derive(::core::default::Default)]
    pub enum Color {
        #[default]
        Auto,
        Always,
        Never,
    }

    pub enum ThreadsCount {
        #[cfg(feature = "num-cpus")]
        LogicalCores,

        #[cfg(feature = "num-cpus")]
        PhysicalCores,

        Custom(u64),
    }
}

pub struct Suite<World> {
    pub(crate) features: ::std::vec::Vec<Feature<World>>,

    pub(crate) before_scenario_hooks: ::std::vec::Vec<Hook<aliases::sync::Arc<dyn Fn(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync>>>,
    pub(crate) after_scenario_hooks: ::std::vec::Vec<Hook<aliases::sync::Arc<dyn Fn(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync>>>,

    pub(crate) before_step_hooks: ::std::vec::Vec<Hook<aliases::sync::Arc<dyn Fn(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync>>>,
    pub(crate) after_step_hooks: ::std::vec::Vec<Hook<aliases::sync::Arc<dyn Fn(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync>>>,
}

mod builder {
    pub(super) use super::*;

    use crate::builders::*;

    pub struct RunnerBuilder<State: self::runner::BuilderState = self::runner::Empty> {
        trials: ::std::vec::Vec<::libtest_mimic::Trial>,

        configurations: self::configurations::RunnerConfigurations,

        before_global_hooks: ::std::vec::Vec<Hook<::std::boxed::Box<dyn FnOnce() -> Fallible + ::core::marker::Send + ::core::marker::Sync>>>,
        after_global_hooks: ::std::vec::Vec<Hook<::std::boxed::Box<dyn FnOnce() -> Fallible + ::core::marker::Send + ::core::marker::Sync>>>,

        __phantom: self::marker::PhantomCovariant<State>,
    }

    impl Runner {
        pub fn builder() -> RunnerBuilder {
            RunnerBuilder {
                trials: ::core::default::Default::default(),

                configurations: ::core::default::Default::default(),

                before_global_hooks: ::core::default::Default::default(),
                after_global_hooks: ::core::default::Default::default(),

                __phantom: ::core::default::Default::default(),
            }
        }
    }

    impl<State: self::runner::BuilderState> RunnerBuilder<State>
    where
        State::Hooks: self::marker::IsUnset,
        State::Trials: self::marker::IsUnset,
    {
        pub fn include_only_ignored(mut self) -> RunnerBuilder<self::runner::SetIgnorePolicy<State>>
        where
            State::IgnorePolicy: self::marker::IsUnset,
        {
            self.configurations.ignore_policy = self::configurations::IgnorePolicy::RetainIgnored;

            RunnerBuilder {
                trials: self.trials,

                configurations: self.configurations,

                before_global_hooks: self.before_global_hooks,
                after_global_hooks: self.after_global_hooks,

                __phantom: ::core::default::Default::default(),
            }
        }

        pub fn include_ignored(mut self) -> RunnerBuilder<self::runner::SetIgnorePolicy<State>>
        where
            State::IgnorePolicy: self::marker::IsUnset,
        {
            self.configurations.ignore_policy = self::configurations::IgnorePolicy::None;

            RunnerBuilder {
                trials: self.trials,

                configurations: self.configurations,

                before_global_hooks: self.before_global_hooks,
                after_global_hooks: self.after_global_hooks,

                __phantom: ::core::default::Default::default(),
            }
        }

        pub fn filter(mut self, value: impl IntoTagsFilter + 'static) -> RunnerBuilder<self::runner::SetTagsFilter<State>> {
            self.configurations.tags_filter = ::core::option::Option::from(
                ::std::boxed::Box::from(
                    self.configurations.tags_filter.take()
                        .unwrap_or_else(|| ::std::boxed::Box::new(|_| true))
                        .chain(value)
                ) as ::std::boxed::Box<dyn Fn(&Tags) -> bool>
            );

            RunnerBuilder {
                trials: self.trials,

                configurations: self.configurations,

                before_global_hooks: self.before_global_hooks,
                after_global_hooks: self.after_global_hooks,

                __phantom: ::core::default::Default::default(),
            }
        }

        pub fn format(mut self, value: self::configurations::Format) -> RunnerBuilder<self::runner::SetFormat<State>>
        where
            State::Format: self::marker::IsUnset,
        {
            self.configurations.format = value;

            RunnerBuilder {
                trials: self.trials,

                configurations: self.configurations,

                before_global_hooks: self.before_global_hooks,
                after_global_hooks: self.after_global_hooks,

                __phantom: ::core::default::Default::default(),
            }
        }

        pub fn color(mut self, value: self::configurations::Color) -> RunnerBuilder<self::runner::SetColor<State>>
        where
            State::Color: self::marker::IsUnset,
        {
            self.configurations.color = value;

            RunnerBuilder {
                trials: self.trials,

                configurations: self.configurations,

                before_global_hooks: self.before_global_hooks,
                after_global_hooks: self.after_global_hooks,

                __phantom: ::core::default::Default::default(),
            }
        }

        pub fn threads(mut self, value: impl Into<self::configurations::ThreadsCount>) -> RunnerBuilder<self::runner::SetThreads<State>>
        where
            State::Threads: self::marker::IsUnset,
        {
            self.configurations.threads = ::core::option::Option::from(value.into());

            RunnerBuilder {
                trials: self.trials,

                configurations: self.configurations,

                before_global_hooks: self.before_global_hooks,
                after_global_hooks: self.after_global_hooks,

                __phantom: ::core::default::Default::default(),
            }
        }

        pub fn logfile(mut self, value: impl Into<::std::borrow::Cow<'static, ::std::path::Path>>) -> RunnerBuilder<self::runner::SetLogFile<State>>
        where
            State::LogFile: self::marker::IsUnset,
        {
            self.configurations.logfile = ::core::option::Option::from(value.into());

            RunnerBuilder {
                trials: self.trials,

                configurations: self.configurations,

                before_global_hooks: self.before_global_hooks,
                after_global_hooks: self.after_global_hooks,

                __phantom: ::core::default::Default::default(),
            }
        }
    }

    impl<State: self::runner::BuilderState> RunnerBuilder<State> {
        pub fn before_all<Callback, Output>(mut self, tags: impl Into<Tags>, callback: Callback) -> RunnerBuilder<self::runner::SetHooks<State>>
        where
            Callback: FnOnce() -> Output + ::core::marker::Send + ::core::marker::Sync + 'static,
            Output: IntoFallible,
        {
            let callback = ::std::boxed::Box::new(move || (callback)().into_fallible())
                as ::std::boxed::Box<dyn FnOnce() -> Fallible + ::core::marker::Send + ::core::marker::Sync>;

            let hook = Hook::builder()
                .tags(tags)
                .callback(callback)
                .build();

            self.before_global_hooks.push(hook);

            RunnerBuilder {
                trials: self.trials,

                configurations: self.configurations,

                before_global_hooks: self.before_global_hooks,
                after_global_hooks: self.after_global_hooks,

                __phantom: ::core::default::Default::default(),
            }
        }

        pub fn after_all<Callback, Output>(mut self, tags: impl Into<Tags>, callback: Callback) -> RunnerBuilder<self::runner::SetHooks<State>>
        where
            Callback: FnOnce() -> Output + ::core::marker::Send + ::core::marker::Sync + 'static,
            Output: IntoFallible,
        {
            let callback = ::std::boxed::Box::new(move || (callback)().into_fallible())
                as ::std::boxed::Box<dyn FnOnce() -> Fallible + ::core::marker::Send + ::core::marker::Sync>;

            let hook = Hook::builder()
                .tags(tags)
                .callback(callback)
                .build();

            self.after_global_hooks.push(hook);

            RunnerBuilder {
                trials: self.trials,

                configurations: self.configurations,

                before_global_hooks: self.before_global_hooks,
                after_global_hooks: self.after_global_hooks,

                __phantom: ::core::default::Default::default(),
            }
        }

        pub fn suite<World>(mut self, suite: impl Into<Suite<World>>) -> RunnerBuilder<self::runner::SetTrials<State>>
        where
            World: ::core::default::Default + 'static,
        {
            let mut suite = suite.into();
            
            RetainByIgnorePolicy::retain(&mut suite, self.configurations.ignore_policy);

            self.configurations.tags_filter.as_ref()
                .map(|filter| RetainByTagsFilter::retain(&mut suite, filter));

            self.trials.extend(suite.into_trials());

            RunnerBuilder {
                trials: self.trials,

                configurations: self.configurations,

                before_global_hooks: self.before_global_hooks,
                after_global_hooks: self.after_global_hooks,

                __phantom: ::core::default::Default::default(),
            }
        }

        pub fn feature<World>(mut self, feature: impl Into<Feature<World>>) -> RunnerBuilder<self::runner::SetTrials<State>>
        where
            World: ::core::default::Default + 'static,
        {
            let mut feature = feature.into();
            
            RetainByIgnorePolicy::retain(&mut feature, self.configurations.ignore_policy);

            self.configurations.tags_filter.as_ref()
                .map(|filter| RetainByTagsFilter::retain(&mut feature, filter));

            self.trials.extend(feature.into_trials());

            RunnerBuilder {
                trials: self.trials,

                configurations: self.configurations,

                before_global_hooks: self.before_global_hooks,
                after_global_hooks: self.after_global_hooks,

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
                trials: self.trials,

                configurations: self.configurations,

                before_global_hooks: self.before_global_hooks,
                after_global_hooks: self.after_global_hooks,
            }
        }
    }

    mod runner {
        pub(super) use super::*;

        #[sealed]
        pub trait BuilderState: ::core::marker::Sized {
            type Trials;
            type Hooks;

            type IgnorePolicy;
            type TagsFilter;

            type Format;
            type Color;
            type Threads;
            type LogFile;
        }

        #[sealed]
        pub trait IsComplete: BuilderState<Trials: self::marker::IsSet> {}

        #[sealed]
        impl<State: BuilderState> IsComplete for State
        where
            State::Trials: self::marker::IsSet,
        {
        }

        pub struct Empty;

        pub struct SetTrials<State: BuilderState = Empty>(self::marker::PhantomCovariant<State>);
        pub struct SetHooks<State: BuilderState = Empty>(self::marker::PhantomCovariant<State>);

        pub struct SetIgnorePolicy<State: BuilderState = Empty>(self::marker::PhantomCovariant<State>);
        pub struct SetTagsFilter<State: BuilderState = Empty>(self::marker::PhantomCovariant<State>);

        pub struct SetFormat<State: BuilderState = Empty>(self::marker::PhantomCovariant<State>);
        pub struct SetColor<State: BuilderState = Empty>(self::marker::PhantomCovariant<State>);
        pub struct SetThreads<State: BuilderState = Empty>(self::marker::PhantomCovariant<State>);
        pub struct SetLogFile<State: BuilderState = Empty>(self::marker::PhantomCovariant<State>);

        #[sealed]
        impl BuilderState for Empty {
            type Trials = self::marker::Unset<self::members::Trials>;
            type Hooks = self::marker::Unset<self::members::Hooks>;

            type IgnorePolicy = self::marker::Unset<self::members::IgnorePolicy>;
            type TagsFilter = self::marker::Unset<self::members::TagsFilter>;

            type Format = self::marker::Unset<self::members::Format>;
            type Color = self::marker::Unset<self::members::Color>;
            type Threads = self::marker::Unset<self::members::Threads>;
            type LogFile = self::marker::Unset<self::members::LogFile>;
        }

        #[sealed]
        impl<State: BuilderState> BuilderState for SetTrials<State> {
            type Trials = self::marker::Set<self::members::Trials>;
            type Hooks = State::Hooks;

            type IgnorePolicy = State::IgnorePolicy;
            type TagsFilter = State::TagsFilter;

            type Format = State::Format;
            type Color = State::Color;
            type Threads = State::Threads;
            type LogFile = State::LogFile;
        }

        #[sealed]
        impl<State: BuilderState> BuilderState for SetHooks<State> {
            type Trials = State::Trials;
            type Hooks = self::marker::Set<self::members::Hooks>;

            type IgnorePolicy = State::IgnorePolicy;
            type TagsFilter = State::TagsFilter;

            type Format = State::Format;
            type Color = State::Color;
            type Threads = State::Threads;
            type LogFile = State::LogFile;
        }

        #[sealed]
        impl<State: BuilderState> BuilderState for SetIgnorePolicy<State> {
            type Trials = State::Trials;
            type Hooks = State::Hooks;

            type IgnorePolicy = self::marker::Set<self::members::IgnorePolicy>;
            type TagsFilter = State::TagsFilter;

            type Format = State::Format;
            type Color = State::Color;
            type Threads = State::Threads;
            type LogFile = State::LogFile;
        }

        #[sealed]
        impl<State: BuilderState> BuilderState for SetTagsFilter<State> {
            type Trials = State::Trials;
            type Hooks = State::Hooks;

            type IgnorePolicy = State::IgnorePolicy;
            type TagsFilter = self::marker::Set<self::members::TagsFilter>;

            type Format = State::Format;
            type Color = State::Color;
            type Threads = State::Threads;
            type LogFile = State::LogFile;
        }

        #[sealed]
        impl<State: BuilderState> BuilderState for SetFormat<State> {
            type Trials = State::Trials;
            type Hooks = State::Hooks;
            
            type IgnorePolicy = State::IgnorePolicy;
            type TagsFilter = State::TagsFilter;

            type Format = self::marker::Set<self::members::Format>;
            type Color = State::Color;
            type Threads = State::Threads;
            type LogFile = State::LogFile;
        }

        #[sealed]
        impl<State: BuilderState> BuilderState for SetColor<State> {
            type Trials = State::Trials;
            type Hooks = State::Hooks;
            
            type IgnorePolicy = State::IgnorePolicy;
            type TagsFilter = State::TagsFilter;

            type Format = State::Format;
            type Color = self::marker::Set<self::members::Color>;
            type Threads = State::Threads;
            type LogFile = State::LogFile;
        }

        #[sealed]
        impl<State: BuilderState> BuilderState for SetThreads<State> {
            type Trials = State::Trials;
            type Hooks = State::Hooks;
            
            type IgnorePolicy = State::IgnorePolicy;
            type TagsFilter = State::TagsFilter;

            type Format = State::Format;
            type Color = State::Color;
            type Threads = self::marker::Set<self::members::Threads>;
            type LogFile = State::LogFile;
        }

        #[sealed]
        impl<State: BuilderState> BuilderState for SetLogFile<State> {
            type Trials = State::Trials;
            type Hooks = State::Hooks;
            
            type IgnorePolicy = State::IgnorePolicy;
            type TagsFilter = State::TagsFilter;

            type Format = State::Format;
            type Color = State::Color;
            type Threads = State::Threads;
            type LogFile = self::marker::Set<self::members::LogFile>;
        }

        mod members {
            pub struct Trials;
            pub struct Hooks;

            pub struct IgnorePolicy;
            pub struct TagsFilter;
            
            pub struct Format;
            pub struct Color;
            pub struct Threads;
            pub struct LogFile;
        }
    }

    #[sealed]
    pub trait IntoTagsFilter {
        fn into_filter(self) -> impl Fn(&Tags) -> bool;

        fn chain(self, other: impl IntoTagsFilter) -> impl Fn(&Tags) -> bool
        where
            Self: ::core::marker::Sized,
        {
            let this = self.into_filter();
            let other = other.into_filter();

            move |tags| this(tags) && other(tags)
        }
    }

    #[sealed]
    impl<F> IntoTagsFilter for F
    where
        F: Fn(&Tags) -> bool,
    {
        fn into_filter(self) -> impl Fn(&Tags) -> bool {
            self
        }
    }

    #[sealed]
    impl<'a, T> IntoTagsFilter for &'a [T]
    where
        T: Into<::std::borrow::Cow<'static, str>> + ::core::clone::Clone,
    {
        fn into_filter(self) -> impl Fn(&Tags) -> bool {
            move |tags| {
                let filter = Tags::from(self.iter().cloned().map(Into::into));
                !filter.is_disjoint(tags)
            }
        }
    }

    #[sealed]
    impl<T, const N: usize> IntoTagsFilter for [T; N]
    where
        T: Into<::std::borrow::Cow<'static, str>> + ::core::clone::Clone,
    {
        fn into_filter(self) -> impl Fn(&Tags) -> bool {
            move |tags| {
                let filter = Tags::from(self.iter().cloned().map(Into::into));
                !filter.is_disjoint(tags)
            }
        }
    }

    pub struct SuiteBuilder<World> {
        features: ::std::vec::Vec<Feature<World>>,

        before_scenario_hooks: ::std::vec::Vec<Hook<aliases::sync::Arc<dyn Fn(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync>>>,
        after_scenario_hooks: ::std::vec::Vec<Hook<aliases::sync::Arc<dyn Fn(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync>>>,

        before_step_hooks: ::std::vec::Vec<Hook<aliases::sync::Arc<dyn Fn(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync>>>,
        after_step_hooks: ::std::vec::Vec<Hook<aliases::sync::Arc<dyn Fn(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync>>>,
    }

    impl<World> Suite<World> {
        pub fn builder() -> SuiteBuilder<World> {
            SuiteBuilder {
                features: ::core::default::Default::default(),

                before_scenario_hooks: ::core::default::Default::default(),
                after_scenario_hooks: ::core::default::Default::default(),

                before_step_hooks: ::core::default::Default::default(),
                after_step_hooks: ::core::default::Default::default(),
            }
        }
    }

    impl<World> SuiteBuilder<World> {
        pub fn feature(mut self, value: impl Into<Feature<World>>) -> Self {
            self.features.push(value.into());
            self
        }

        pub fn features<T>(mut self, values: impl IntoIterator<Item = T>) -> Self
        where
            T: Into<Feature<World>>,
        {
            self.features.extend(values.into_iter().map(Into::into));
            self
        }

        pub fn before_scenario<Callback, Output>(mut self, tags: impl Into<Tags>, callback: Callback) -> Self
        where
            Callback: Fn(&mut World) -> Output + ::core::marker::Send + ::core::marker::Sync + 'static,
            Output: IntoFallible,
        {
            let callback  = aliases::sync::Arc::new(move |world: &mut World| (callback)(world).into_fallible())
                as aliases::sync::Arc<dyn Fn(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync>;

            let hook = Hook::builder()
                .tags(tags)
                .callback(callback)
                .build();

            self.before_scenario_hooks.push(hook);
            self
        }

        pub fn after_scenario<Callback, Output>(mut self, tags: impl Into<Tags>, callback: Callback) -> Self
        where
            Callback: Fn(&mut World) -> Output + ::core::marker::Send + ::core::marker::Sync + 'static,
            Output: IntoFallible,
        {
            let callback  = aliases::sync::Arc::new(move |world: &mut World| (callback)(world).into_fallible())
                as aliases::sync::Arc<dyn Fn(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync>;

            let hook = Hook::builder()
                .tags(tags)
                .callback(callback)
                .build();

            self.after_scenario_hooks.push(hook);
            self
        }

        pub fn before_step<Callback, Output>(mut self, tags: impl Into<Tags>, callback: Callback) -> Self
        where
            Callback: Fn(&mut World) -> Output + ::core::marker::Send + ::core::marker::Sync + 'static,
            Output: IntoFallible,
        {
            let callback  = aliases::sync::Arc::new(move |world: &mut World| (callback)(world).into_fallible())
                as aliases::sync::Arc<dyn Fn(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync>;

            let hook = Hook::builder()
                .tags(tags)
                .callback(callback)
                .build();

            self.before_step_hooks.push(hook);
            self
        }

        pub fn after_step<Callback, Output>(mut self, tags: impl Into<Tags>, callback: Callback) -> Self
        where
            Callback: Fn(&mut World) -> Output + ::core::marker::Send + ::core::marker::Sync + 'static,
            Output: IntoFallible,
        {
            let callback  = aliases::sync::Arc::new(move |world: &mut World| (callback)(world).into_fallible())
                as aliases::sync::Arc<dyn Fn(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync>;

            let hook = Hook::builder()
                .tags(tags)
                .callback(callback)
                .build();

            self.after_step_hooks.push(hook);
            self
        }
    }

    impl<World> SuiteBuilder<World> {
        pub fn build(self) -> Suite<World> {
            Suite {
                features: self.features,

                before_scenario_hooks: self.before_scenario_hooks,
                after_scenario_hooks: self.after_scenario_hooks,

                before_step_hooks: self.before_step_hooks,
                after_step_hooks: self.after_step_hooks,
            }
        }
    }

    impl<World> From<SuiteBuilder<World>> for Suite<World> {
        fn from(builder: SuiteBuilder<World>) -> Self {
            builder.build()
        }
    }
}

trait RetainByIgnorePolicy {
    fn retain(&mut self, policy: self::configurations::IgnorePolicy);
}

impl<World> RetainByIgnorePolicy for Suite<World> {
    fn retain(&mut self, policy: self::configurations::IgnorePolicy) {
        match policy {
            self::configurations::IgnorePolicy::RetainIgnored => self.features.retain(|features| features.ignored.as_ref().is_some_and(|ignored| *ignored)),
            self::configurations::IgnorePolicy::RetainUnignored => self.features.retain(|features| features.ignored.as_ref().is_none_or(|ignored| !ignored)),
            _ => {},
        }

        self.features.iter_mut()
            .for_each(|feature| RetainByIgnorePolicy::retain(feature, policy));
    }
}

impl<World> RetainByIgnorePolicy for Feature<World> {
    fn retain(&mut self, policy: self::configurations::IgnorePolicy) {
        match policy {
            self::configurations::IgnorePolicy::RetainIgnored => {
                self.background = self.background.take()
                    .filter(|background| background.ignored.as_ref().is_some_and(|ignored| *ignored));
                self.scenarios.retain(|scenario| scenario.ignored.as_ref().is_some_and(|ignored| *ignored));
                self.rules.retain(|rule| rule.ignored.as_ref().is_some_and(|ignored| *ignored));
            },

            self::configurations::IgnorePolicy::RetainUnignored => {
                self.background = self.background.take()
                    .filter(|background| background.ignored.as_ref().is_none_or(|ignored| !ignored));
                self.scenarios.retain(|scenario| scenario.ignored.as_ref().is_none_or(|ignored| !ignored));
                self.rules.retain(|rule| rule.ignored.as_ref().is_none_or(|ignored| !ignored));
            },

            _ => {},
        }

        self.rules.iter_mut()
            .for_each(|rule| RetainByIgnorePolicy::retain(rule, policy));
    }
}

impl<World> RetainByIgnorePolicy for Rule<World> {
    fn retain(&mut self, policy: self::configurations::IgnorePolicy) {
        match policy {
            self::configurations::IgnorePolicy::RetainIgnored => {
                self.background = self.background.take()
                    .filter(|background| background.ignored.as_ref().is_some_and(|ignored| *ignored));
                self.scenarios.retain(|scenario| scenario.ignored.as_ref().is_some_and(|ignored| *ignored));
            },

            self::configurations::IgnorePolicy::RetainUnignored => {
                self.background = self.background.take()
                    .filter(|background| background.ignored.as_ref().is_none_or(|ignored| !ignored));
                self.scenarios.retain(|scenario| scenario.ignored.as_ref().is_none_or(|ignored| !ignored));
            },

            _ => {},
        }
    }
}

trait RetainByTagsFilter {
    fn retain<Callback>(&mut self, filter: impl ::std::ops::Deref<Target = Callback> + ::core::clone::Clone)
    where
        Callback: Fn(&Tags) -> bool;
}

impl<World> RetainByTagsFilter for Suite<World> {
    fn retain<Callback>(&mut self, filter: impl ::std::ops::Deref<Target = Callback> + ::core::clone::Clone)
    where
        Callback: Fn(&Tags) -> bool,
    {
        self.features.retain(|feature| feature.tags.as_ref().is_some_and(|tags| (filter)(&tags)));

        self.features.iter_mut()
            .for_each(|feature| RetainByTagsFilter::retain(feature, filter.clone()));

        self.before_scenario_hooks.retain(|hook| hook.tags.as_ref().is_some_and(|tags| (filter)(&tags)));
        self.after_scenario_hooks.retain(|hook| hook.tags.as_ref().is_some_and(|tags| (filter)(&tags)));

        self.before_step_hooks.retain(|hook| hook.tags.as_ref().is_some_and(|tags| (filter)(&tags)));
        self.after_step_hooks.retain(|hook| hook.tags.as_ref().is_some_and(|tags| (filter)(&tags)));
    }
}

impl<World> RetainByTagsFilter for Feature<World> {
    fn retain<Callback>(&mut self, filter: impl ::std::ops::Deref<Target = Callback> + ::core::clone::Clone)
    where
        Callback: Fn(&Tags) -> bool,
    {
        self.scenarios.retain(|scenario| scenario.tags.as_ref().is_some_and(|tags| (filter)(&tags)));
        self.rules.retain(|rule| rule.tags.as_ref().is_some_and(|tags| (filter)(&tags)));

        self.rules.iter_mut()
            .for_each(|rule| RetainByTagsFilter::retain(rule, filter.clone()));
    }
}

impl<World> RetainByTagsFilter for Rule<World> {
    fn retain<Callback>(&mut self, filter: impl ::std::ops::Deref<Target = Callback> + ::core::clone::Clone)
    where
        Callback: Fn(&Tags) -> bool,
    {
        self.scenarios.retain(|scenario| scenario.tags.as_ref().is_some_and(|tags| (filter)(&tags)));
    }
}
#[sealed]
pub trait IntoTrials {
    fn into_trials(self) -> impl IntoIterator<Item = ::libtest_mimic::Trial>;
}

#[sealed]
impl<World> IntoTrials for Suite<World>
where
    World: ::core::default::Default + 'static,
{
    fn into_trials(self) -> impl IntoIterator<Item = ::libtest_mimic::Trial> {
        self.features.into_iter()
            .zip(::core::iter::repeat([
                self.before_scenario_hooks.clone(),
                self.after_scenario_hooks.clone(),
                self.before_step_hooks.clone(),
                self.after_step_hooks.clone(),
            ]))
            .map(|(feature, context)| ::core::iter::Iterator::chain(
                feature.scenarios.into_iter()
                    .zip(::core::iter::repeat((context.clone(), [
                        feature.background.as_ref().map(|background| background.given.clone()),
                    ])))
                    .map(|(scenario, context)| scenario.into_trial_with_context(context)),

                feature.rules.into_iter()
                    .map(|rule| (rule.scenarios, rule.background))
                    .map(move |(rule_scenarios, rule_background)| (rule_scenarios, (context.clone(), [
                        feature.background.as_ref().map(|background| background.given.clone()),
                        rule_background.as_ref().map(|background| background.given.clone()),
                    ])))
                    .map(|(rule_scenarios, context)| rule_scenarios.into_iter()
                        .zip(::core::iter::repeat(context))
                        .map(|(scenario, context)| scenario.into_trial_with_context(context)))
                    .flatten()
            ))
            .flatten()
    }
}

#[sealed]
impl<World> IntoTrials for Feature<World>
where
    World: ::core::default::Default + 'static,
{
    fn into_trials(self) -> impl IntoIterator<Item = ::libtest_mimic::Trial> {
        let feature = self;

        ::core::iter::Iterator::chain(
            feature.scenarios.into_iter()
                .zip(::core::iter::repeat([
                    feature.background.as_ref().map(|background| background.given.clone()),
                ]))
                .map(|(scenario, context)| scenario.into_trial_with_context(context)),

            feature.rules.into_iter()
                .map(|rule| (rule.scenarios, rule.background))
                .map(move |(rule_scenarios, rule_background)| (rule_scenarios, [
                    feature.background.as_ref().map(|background| background.given.clone()),
                    rule_background.as_ref().map(|background| background.given.clone()),
                ]))
                .map(|(rule_scenarios, context)| rule_scenarios.into_iter()
                    .zip(::core::iter::repeat(context))
                    .map(|(scenario, context)| scenario.into_trial_with_context(context)))
                .flatten()
        )
    }
}

trait ScenarioExt<Context> {
    fn into_trial_with_context(self, context: Context) -> ::libtest_mimic::Trial;
}

impl<const N: usize, World> ScenarioExt<(
    [::std::vec::Vec<Hook<aliases::sync::Arc<dyn Fn(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync>>>; 4],
    [::core::option::Option<::std::vec::Vec<Step<aliases::sync::Arc<dyn Fn(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync>>>>; N],
)> for Scenario<World>
where
    World: ::core::default::Default + 'static,
{
    fn into_trial_with_context(self, ([before_scenario_hooks, after_scenario_hooks, before_step_hooks, after_step_hooks], backgrounds): (
        [::std::vec::Vec<Hook<aliases::sync::Arc<dyn Fn(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync>>>; 4],
        [::core::option::Option<::std::vec::Vec<Step<aliases::sync::Arc<dyn Fn(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync>>>>; N],
    )) -> ::libtest_mimic::Trial {
        let description = self.to_description();

        let callback = move || {
            let mut world = ::core::default::Default::default();

            before_scenario_hooks.to_callback()(&mut world)?;

            backgrounds
                .into_iter()
                .flatten()
                .try_for_each(|background| background.to_callback_with_context([
                    before_step_hooks.clone(), after_step_hooks.clone(),
                ])(&mut world))?;

            self.given.into_callback_with_context([
                before_step_hooks.clone(), after_step_hooks.clone(),
            ])(&mut world)?;

            self.when.into_callback_with_context([
                before_step_hooks.clone(), after_step_hooks.clone(),
            ])(&mut world)?;

            self.then.into_callback_with_context([
                before_step_hooks.clone(), after_step_hooks.clone(),
            ])(&mut world)?;

            after_scenario_hooks.to_callback()(&mut world)?;

            Ok(())
        };

        into_trial()
            .description(description)
            .tags(self.tags)
            .callback(callback)
            .call()
    }
}

impl<const N: usize, World> ScenarioExt<
    [::core::option::Option<::std::vec::Vec<Step<aliases::sync::Arc<dyn Fn(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync>>>>; N],
> for Scenario<World>
where
    World: ::core::default::Default + 'static,
{
    fn into_trial_with_context(self, backgrounds: [::core::option::Option<::std::vec::Vec<Step<aliases::sync::Arc<dyn Fn(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync>>>>; N]) -> ::libtest_mimic::Trial {
        let description = self.to_description();

        let callback = move || {
            let mut world = ::core::default::Default::default();

            backgrounds
                .into_iter()
                .flatten()
                .try_for_each(|background| background.to_callback()(&mut world))?;

            self.given.into_callback()(&mut world)?;
            self.when.into_callback()(&mut world)?;
            self.then.into_callback()(&mut world)?;

            Ok(())
        };

        into_trial()
            .description(description)
            .tags(self.tags)
            .callback(callback)
            .call()
    }
}

#[::bon::builder]
#[builder(on(_, required))]
fn into_trial(description: impl Into<::std::borrow::Cow<'static, str>>, tags: ::core::option::Option<impl Into<Tags>>, callback: impl FnOnce() -> Fallible + ::core::marker::Send + ::core::marker::Sync + 'static) -> ::libtest_mimic::Trial {
    let callback = move || (callback)().map_err(|err| err.message.into());

    let description = description.into();

    let tags = tags
        .map(Into::into)
        .map(|tags| tags.to_description());

    let trial = ::libtest_mimic::Trial::test(description, callback);
    
    let trial = match tags {
        Some(tags) => trial.with_kind(tags),
        None => trial,
    };

    trial
}

trait ToDescription {
    fn to_description(&self) -> ::std::borrow::Cow<'static, str>;
}

impl<World> ToDescription for Scenario<World> {
    fn to_description(&self) -> ::std::borrow::Cow<'static, str> {
        match self.description {
            Some(ref description) => description.clone(),
            None => ::std::format!("{}; {}; {}", self.given.to_description(), self.when.to_description(), self.then.to_description()).into()
        }
    }
}

impl<Callback> ToDescription for ::std::vec::Vec<Step<Callback>> {
    fn to_description(&self) -> ::std::borrow::Cow<'static, str> {
        self
            .iter()
            .map(ToDescription::to_description)
            .collect::<::std::vec::Vec<_>>()
            .join(", ")
            .into()
    }
}

impl<Callback> ToDescription for Step<Callback> {
    fn to_description(&self) -> ::std::borrow::Cow<'static, str> {
        ::std::format!("{} {}", self.label.to_description(), self.description).into()
    }
}

impl ToDescription for Tags {
    fn to_description(&self) -> ::std::borrow::Cow<'static, str> {
        self.0
            .iter()
            .cloned()
            .collect::<::std::vec::Vec<_>>()
            .join(",")
            .into()
    }
}

impl ToDescription for StepLabel {
    fn to_description(&self) -> ::std::borrow::Cow<'static, str> {
        match self {
            Self::Given => "Given".into(),
            Self::When => "When".into(),
            Self::Then => "Then".into(),

            Self::And => "and".into(),
            Self::But => "but".into(),
        }
    }
}

trait ScenarioStepsExt<World> {
    fn into_callback(self) -> impl FnOnce(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync;

    fn into_callback_with_context(self, context: [::std::vec::Vec<Hook<aliases::sync::Arc<dyn Fn(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync>>>; 2]) -> impl FnOnce(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync;
}

impl<World> ScenarioStepsExt<World> for ::std::vec::Vec<Step<::std::boxed::Box<dyn FnOnce(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync>>>
where
    World: 'static,
{
    fn into_callback(self) -> impl FnOnce(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync {
        move |world: &mut World| self.into_iter()
            .try_for_each(|step| (step.callback)(world))
    }

    fn into_callback_with_context(self, [before_step_hooks, after_step_hooks]: [::std::vec::Vec<Hook<aliases::sync::Arc<dyn Fn(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync>>>; 2]) -> impl FnOnce(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync {
        move |world: &mut World| self.into_iter()
            .try_for_each(|step| {
                (before_step_hooks.to_callback())(world)?;
                (step.callback)(world)?;
                (after_step_hooks.to_callback())(world)?;

                Ok(())
            })
    }
}

trait BackgroundStepsExt<World> {
    fn to_callback(&self) -> impl FnOnce(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync;

    fn to_callback_with_context(&self, context: [::std::vec::Vec<Hook<aliases::sync::Arc<dyn Fn(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync>>>; 2]) -> impl FnOnce(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync;
}


impl<World> BackgroundStepsExt<World> for ::std::vec::Vec<Step<aliases::sync::Arc<dyn Fn(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync>>>
where
    World: 'static,
{
    fn to_callback(&self) -> impl FnOnce(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync {
        move |world: &mut World| self.iter()
            .try_for_each(|step| (step.callback)(world))
    }

    fn to_callback_with_context(&self, [before_step_hooks, after_step_hooks]: [::std::vec::Vec<Hook<aliases::sync::Arc<dyn Fn(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync>>>; 2]) -> impl FnOnce(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync {
        move |world: &mut World| self.iter()
            .try_for_each(|step| {
                (before_step_hooks.to_callback())(world)?;
                (step.callback)(world)?;
                (after_step_hooks.to_callback())(world)?;

                Ok(())
            })
    }
}

trait NonGlobalHooksExt<World> {
    fn to_callback(&self) -> impl Fn(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync;
}

impl<World> NonGlobalHooksExt<World> for ::std::vec::Vec<Hook<aliases::sync::Arc<dyn Fn(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync>>>
where
    World: 'static,
{
    fn to_callback(&self) -> impl Fn(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync {
        move |world: &mut World| self.iter()
            .try_for_each(|hook| (hook.callback)(world))
    }
}

trait GlobalHooksExt {
    fn to_callback(self) -> impl FnOnce() -> Fallible + ::core::marker::Send + ::core::marker::Sync;
}

impl GlobalHooksExt for ::std::vec::Vec<Hook<::std::boxed::Box<dyn FnOnce() -> Fallible + ::core::marker::Send + ::core::marker::Sync>>> {
    fn to_callback(self) -> impl FnOnce() -> Fallible + ::core::marker::Send + ::core::marker::Sync {
        move || self.into_iter()
            .try_for_each(|hook| (hook.callback)())
    }
}

impl Runner {
    pub fn run(self) -> ::std::process::ExitCode {
        let mut args = ::libtest_mimic::Arguments::from_args();
        self.configurations.update(&mut args);

        let _ = self.before_global_hooks.to_callback()();

        let conclusion = ::libtest_mimic::run(&args, self.trials);
        let exit_code = conclusion.exit_code();

        let _ = self.after_global_hooks.to_callback()();

        exit_code
    }
}

impl self::configurations::RunnerConfigurations {
    fn update(self, args: &mut ::libtest_mimic::Arguments) {
        args.format = ::core::option::Option::from(self.format).map(Into::into);
        args.color = ::core::option::Option::from(self.color).map(Into::into);
        args.test_threads = self.threads.map(Into::into);
        args.logfile = self.logfile.map(|path| path.to_string_lossy().into_owned());
    }
}

impl From<self::configurations::Format> for ::libtest_mimic::FormatSetting {
    fn from(format: self::configurations::Format) -> Self {
        match format {
            self::configurations::Format::Pretty => Self::Pretty,
            self::configurations::Format::Terse => Self::Terse,
            self::configurations::Format::Json => Self::Json,
        }
    }
}

impl From<self::configurations::Color> for ::libtest_mimic::ColorSetting {
    fn from(color: self::configurations::Color) -> Self {
        match color {
            self::configurations::Color::Auto => Self::Auto,
            self::configurations::Color::Always => Self::Always,
            self::configurations::Color::Never => Self::Never,
        }
    }
}

impl From<self::configurations::ThreadsCount> for usize {
    fn from(threads: self::configurations::ThreadsCount) -> Self {
        match threads {
            #[cfg(feature = "num-cpus")]
            configurations::ThreadsCount::LogicalCores => ::num_cpus::get(),

            #[cfg(feature = "num-cpus")]
            configurations::ThreadsCount::PhysicalCores => ::num_cpus::get_physical(),

            configurations::ThreadsCount::Custom(threads) => threads as usize,
        }
    }
}
