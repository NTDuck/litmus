use ::sealed::sealed;

use crate::builders::*;
use crate::engine::*;
use crate::models::*;
use crate::utils::aliases;

// pub struct RunnerBuilder {
//     trials: ::std::vec::Vec<::std::boxed::Box<dyn IntoTrials>>,

//     configurations: self::configurations::RunnerConfigurations,

//     before_global_hooks: ::std::vec::Vec<GlobalHook>,
//     after_global_hooks: ::std::vec::Vec<GlobalHook>,
// }

pub struct SuiteBuilder<World, State: self::suite::BuilderState = self::suite::Empty> {
    before_scenario_hooks: ::std::vec::Vec<NonGlobalHook<World>>,
    after_scenario_hooks: ::std::vec::Vec<NonGlobalHook<World>>,

    before_step_hooks: ::std::vec::Vec<NonGlobalHook<World>>,
    after_step_hooks: ::std::vec::Vec<NonGlobalHook<World>>,

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
    pub fn before_scenario(mut self, hook: impl IntoNonGlobalHook<World>) -> SuiteBuilder<World, self::suite::SetHooks<State>> {
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

    pub fn after_scenario(mut self, hook: impl IntoNonGlobalHook<World>) -> SuiteBuilder<World, self::suite::SetHooks<State>> {
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

    pub fn before_step(mut self, hook: impl IntoNonGlobalHook<World>) -> SuiteBuilder<World, self::suite::SetHooks<State>> {
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

    pub fn after_step(mut self, hook: impl IntoNonGlobalHook<World>) -> SuiteBuilder<World, self::suite::SetHooks<State>> {
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
