use ::sealed::sealed;

use crate::models::*;
use crate::utils::aliases;

pub struct Runner {
    pub(crate) configurations: self::configurations::RunnerConfigurations,

    pub(crate) before_global_hooks: ::std::vec::Vec<GlobalHook>,
    pub(crate) after_global_hooks: ::std::vec::Vec<GlobalHook>,

    pub(crate) trials: ::std::vec::Vec<::std::boxed::Box<dyn IntoTrialsWithConfigurations>>,
}

pub struct Suite<World> {
    pub(crate) before_scenario_hooks: ::std::vec::Vec<ScenarioOrStepHook<World>>,
    pub(crate) after_scenario_hooks: ::std::vec::Vec<ScenarioOrStepHook<World>>,

    pub(crate) before_step_hooks: ::std::vec::Vec<ScenarioOrStepHook<World>>,
    pub(crate) after_step_hooks: ::std::vec::Vec<ScenarioOrStepHook<World>>,

    pub(crate) features: ::std::vec::Vec<Feature<World>>,
}

pub struct AsyncSuite<World> {
    pub(crate) before_scenario_hooks: ::std::vec::Vec<AsyncScenarioOrStepHook<World>>,
    pub(crate) after_scenario_hooks: ::std::vec::Vec<AsyncScenarioOrStepHook<World>>,

    pub(crate) before_step_hooks: ::std::vec::Vec<AsyncScenarioOrStepHook<World>>,
    pub(crate) after_step_hooks: ::std::vec::Vec<AsyncScenarioOrStepHook<World>>,

    pub(crate) features: ::std::vec::Vec<AsyncFeature<World>>,
}

pub mod configurations {
    pub(super) use super::*;

    #[derive(::core::default::Default)]
    pub(crate) struct RunnerConfigurations {
        pub(crate) ignore_policy: IgnorePolicy,
        pub(crate) tags_filter: ::core::option::Option<TagsFilter>,

        /* Used by `::libtest_mimic::Arguments` */
        pub(crate) format: Format,
        pub(crate) color: Color,
        pub(crate) threads: ::core::option::Option<ThreadsCount>,
        pub(crate) logfile: ::core::option::Option<aliases::path::Path>,
    }

    #[derive(::core::default::Default, ::core::clone::Clone, ::core::marker::Copy)]
    pub(crate) enum IgnorePolicy {
        RetainIgnored,

        #[default]
        RetainUnignored,

        None,
    }

    pub(crate) type TagsFilter = ::std::boxed::Box<dyn Fn(&Tags) -> bool>;

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

pub use configurations as config;

#[sealed]
pub trait IntoTrialsWithConfigurations: 'static {
    #[allow(private_interfaces)]
    fn into_trials_with_configurations(
        self: ::std::boxed::Box<Self>,
        configurations: &self::configurations::RunnerConfigurations,
    ) -> ::std::vec::Vec<libtest_mimic::Trial>;
}

#[sealed]
impl<T> IntoTrialsWithConfigurations for T
where
    T: IntoTrials + RetainByConfigurations + 'static,
{
    #[allow(private_interfaces)]
    fn into_trials_with_configurations(
        mut self: ::std::boxed::Box<Self>,
        configurations: &self::configurations::RunnerConfigurations,
    ) -> ::std::vec::Vec<libtest_mimic::Trial> {
        self.retain(configurations);
        self.into_trials()
    }
}

trait RetainByConfigurations {
    fn retain(&mut self, configurations: &self::configurations::RunnerConfigurations);
}

impl<T> RetainByConfigurations for T
where
    T: RetainByIgnorePolicy + RetainByTagsFilter,
{
    fn retain(&mut self, configurations: &self::configurations::RunnerConfigurations) {
        RetainByIgnorePolicy::retain(self, configurations.ignore_policy);

        if let Some(filter) = configurations.tags_filter.as_ref() {
            RetainByTagsFilter::retain(self, filter)
        }
    }
}

trait RetainByIgnorePolicy {
    fn retain(&mut self, policy: self::configurations::IgnorePolicy);
}

impl<World> RetainByIgnorePolicy for Suite<World> {
    fn retain(&mut self, policy: self::configurations::IgnorePolicy) {
        match policy {
            self::configurations::IgnorePolicy::RetainIgnored =>
                self.features.retain(|features| features.ignored.as_ref().is_some_and(|ignored| *ignored)),
            self::configurations::IgnorePolicy::RetainUnignored =>
                self.features.retain(|features| features.ignored.as_ref().is_none_or(|ignored| !ignored)),
            _ => {},
        }

        self.features.iter_mut().for_each(|feature| RetainByIgnorePolicy::retain(feature, policy));
    }
}

impl<World> RetainByIgnorePolicy for Feature<World> {
    fn retain(&mut self, policy: self::configurations::IgnorePolicy) {
        match policy {
            self::configurations::IgnorePolicy::RetainIgnored => {
                self.background = self
                    .background
                    .take()
                    .filter(|background| background.ignored.as_ref().is_some_and(|ignored| *ignored));
                self.scenarios.retain(|scenario| scenario.ignored.as_ref().is_some_and(|ignored| *ignored));
                self.rules.retain(|rule| rule.ignored.as_ref().is_some_and(|ignored| *ignored));
            },

            self::configurations::IgnorePolicy::RetainUnignored => {
                self.background = self
                    .background
                    .take()
                    .filter(|background| background.ignored.as_ref().is_none_or(|ignored| !ignored));
                self.scenarios.retain(|scenario| scenario.ignored.as_ref().is_none_or(|ignored| !ignored));
                self.rules.retain(|rule| rule.ignored.as_ref().is_none_or(|ignored| !ignored));
            },

            _ => {},
        }

        self.rules.iter_mut().for_each(|rule| RetainByIgnorePolicy::retain(rule, policy));
    }
}

impl<World> RetainByIgnorePolicy for Rule<World> {
    fn retain(&mut self, policy: self::configurations::IgnorePolicy) {
        match policy {
            self::configurations::IgnorePolicy::RetainIgnored => {
                self.background = self
                    .background
                    .take()
                    .filter(|background| background.ignored.as_ref().is_some_and(|ignored| *ignored));
                self.scenarios.retain(|scenario| scenario.ignored.as_ref().is_some_and(|ignored| *ignored));
            },

            self::configurations::IgnorePolicy::RetainUnignored => {
                self.background = self
                    .background
                    .take()
                    .filter(|background| background.ignored.as_ref().is_none_or(|ignored| !ignored));
                self.scenarios.retain(|scenario| scenario.ignored.as_ref().is_none_or(|ignored| !ignored));
            },

            _ => {},
        }
    }
}

impl<World> RetainByIgnorePolicy for AsyncSuite<World> {
    fn retain(&mut self, policy: self::configurations::IgnorePolicy) {
        match policy {
            self::configurations::IgnorePolicy::RetainIgnored =>
                self.features.retain(|features| features.ignored.as_ref().is_some_and(|ignored| *ignored)),
            self::configurations::IgnorePolicy::RetainUnignored =>
                self.features.retain(|features| features.ignored.as_ref().is_none_or(|ignored| !ignored)),
            _ => {},
        }

        self.features.iter_mut().for_each(|feature| RetainByIgnorePolicy::retain(feature, policy));
    }
}

impl<World> RetainByIgnorePolicy for AsyncFeature<World> {
    fn retain(&mut self, policy: self::configurations::IgnorePolicy) {
        match policy {
            self::configurations::IgnorePolicy::RetainIgnored => {
                self.background = self
                    .background
                    .take()
                    .filter(|background| background.ignored.as_ref().is_some_and(|ignored| *ignored));
                self.scenarios.retain(|scenario| scenario.ignored.as_ref().is_some_and(|ignored| *ignored));
                self.rules.retain(|rule| rule.ignored.as_ref().is_some_and(|ignored| *ignored));
            },

            self::configurations::IgnorePolicy::RetainUnignored => {
                self.background = self
                    .background
                    .take()
                    .filter(|background| background.ignored.as_ref().is_none_or(|ignored| !ignored));
                self.scenarios.retain(|scenario| scenario.ignored.as_ref().is_none_or(|ignored| !ignored));
                self.rules.retain(|rule| rule.ignored.as_ref().is_none_or(|ignored| !ignored));
            },

            _ => {},
        }

        self.rules.iter_mut().for_each(|rule| RetainByIgnorePolicy::retain(rule, policy));
    }
}

impl<World> RetainByIgnorePolicy for AsyncRule<World> {
    fn retain(&mut self, policy: self::configurations::IgnorePolicy) {
        match policy {
            self::configurations::IgnorePolicy::RetainIgnored => {
                self.background = self
                    .background
                    .take()
                    .filter(|background| background.ignored.as_ref().is_some_and(|ignored| *ignored));
                self.scenarios.retain(|scenario| scenario.ignored.as_ref().is_some_and(|ignored| *ignored));
            },

            self::configurations::IgnorePolicy::RetainUnignored => {
                self.background = self
                    .background
                    .take()
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
        self.features.retain(|feature| feature.tags.as_ref().is_some_and(&*filter));

        self.features.iter_mut().for_each(|feature| RetainByTagsFilter::retain(feature, filter.clone()));

        self.before_scenario_hooks.retain(|hook| hook.tags.as_ref().is_some_and(&*filter));
        self.after_scenario_hooks.retain(|hook| hook.tags.as_ref().is_some_and(&*filter));

        self.before_step_hooks.retain(|hook| hook.tags.as_ref().is_some_and(&*filter));
        self.after_step_hooks.retain(|hook| hook.tags.as_ref().is_some_and(&*filter));
    }
}

impl<World> RetainByTagsFilter for Feature<World> {
    fn retain<Callback>(&mut self, filter: impl ::std::ops::Deref<Target = Callback> + ::core::clone::Clone)
    where
        Callback: Fn(&Tags) -> bool,
    {
        self.scenarios.retain(|scenario| scenario.tags.as_ref().is_some_and(&*filter));
        self.rules.retain(|rule| rule.tags.as_ref().is_some_and(&*filter));

        self.rules.iter_mut().for_each(|rule| RetainByTagsFilter::retain(rule, filter.clone()));
    }
}

impl<World> RetainByTagsFilter for Rule<World> {
    fn retain<Callback>(&mut self, filter: impl ::std::ops::Deref<Target = Callback> + ::core::clone::Clone)
    where
        Callback: Fn(&Tags) -> bool,
    {
        self.scenarios.retain(|scenario| scenario.tags.as_ref().is_some_and(&*filter));
    }
}

impl<World> RetainByTagsFilter for AsyncSuite<World> {
    fn retain<Callback>(&mut self, filter: impl ::std::ops::Deref<Target = Callback> + ::core::clone::Clone)
    where
        Callback: Fn(&Tags) -> bool,
    {
        self.features.retain(|feature| feature.tags.as_ref().is_some_and(&*filter));

        self.features.iter_mut().for_each(|feature| RetainByTagsFilter::retain(feature, filter.clone()));

        self.before_scenario_hooks.retain(|hook| hook.tags.as_ref().is_some_and(&*filter));
        self.after_scenario_hooks.retain(|hook| hook.tags.as_ref().is_some_and(&*filter));

        self.before_step_hooks.retain(|hook| hook.tags.as_ref().is_some_and(&*filter));
        self.after_step_hooks.retain(|hook| hook.tags.as_ref().is_some_and(&*filter));
    }
}

impl<World> RetainByTagsFilter for AsyncFeature<World> {
    fn retain<Callback>(&mut self, filter: impl ::std::ops::Deref<Target = Callback> + ::core::clone::Clone)
    where
        Callback: Fn(&Tags) -> bool,
    {
        self.scenarios.retain(|scenario| scenario.tags.as_ref().is_some_and(&*filter));
        self.rules.retain(|rule| rule.tags.as_ref().is_some_and(&*filter));

        self.rules.iter_mut().for_each(|rule| RetainByTagsFilter::retain(rule, filter.clone()));
    }
}

impl<World> RetainByTagsFilter for AsyncRule<World> {
    fn retain<Callback>(&mut self, filter: impl ::std::ops::Deref<Target = Callback> + ::core::clone::Clone)
    where
        Callback: Fn(&Tags) -> bool,
    {
        self.scenarios.retain(|scenario| scenario.tags.as_ref().is_some_and(&*filter));
    }
}

trait IntoTrials {
    fn into_trials(self) -> ::std::vec::Vec<::libtest_mimic::Trial>;
}

impl<World> IntoTrials for Suite<World>
where
    World: ::core::default::Default + 'static,
{
    fn into_trials(self) -> ::std::vec::Vec<::libtest_mimic::Trial> {
        self.features
            .into_iter()
            .zip(::core::iter::repeat([
                self.before_scenario_hooks.clone(),
                self.after_scenario_hooks.clone(),
                self.before_step_hooks.clone(),
                self.after_step_hooks.clone(),
            ]))
            .flat_map(|(feature, hooks)| {
                ::core::iter::Iterator::chain(
                    feature
                        .scenarios
                        .into_iter()
                        .zip(::core::iter::repeat((hooks.clone(), [feature
                            .background
                            .as_ref()
                            .map(|background| background.given.clone())])))
                        .map(|(scenario, context)| scenario.into_trial_with_context(context)),
                    feature
                        .rules
                        .into_iter()
                        .map(|rule| (rule.scenarios, rule.background))
                        .map(move |(rule_scenarios, rule_background)| {
                            (
                                rule_scenarios,
                                (hooks.clone(), [
                                    feature.background.as_ref().map(|background| background.given.clone()),
                                    rule_background.as_ref().map(|background| background.given.clone()),
                                ]),
                            )
                        })
                        .flat_map(|(rule_scenarios, context)| {
                            rule_scenarios
                                .into_iter()
                                .zip(::core::iter::repeat(context))
                                .map(|(scenario, context)| scenario.into_trial_with_context(context))
                        }),
                )
            })
            .collect()
    }
}

impl<World> IntoTrials for Feature<World>
where
    World: ::core::default::Default + 'static,
{
    fn into_trials(self) -> ::std::vec::Vec<::libtest_mimic::Trial> {
        let feature = self;

        ::core::iter::Iterator::chain(
            feature
                .scenarios
                .into_iter()
                .zip(::core::iter::repeat([feature.background.as_ref().map(|background| background.given.clone())]))
                .map(|(scenario, context)| scenario.into_trial_with_context(context)),
            feature
                .rules
                .into_iter()
                .map(|rule| (rule.scenarios, rule.background))
                .map(move |(rule_scenarios, rule_background)| {
                    (rule_scenarios, [
                        feature.background.as_ref().map(|background| background.given.clone()),
                        rule_background.as_ref().map(|background| background.given.clone()),
                    ])
                })
                .flat_map(|(rule_scenarios, context)| {
                    rule_scenarios
                        .into_iter()
                        .zip(::core::iter::repeat(context))
                        .map(|(scenario, context)| scenario.into_trial_with_context(context))
                }),
        )
        .collect()
    }
}

trait ScenarioExt<Context> {
    fn into_trial_with_context(self, context: Context) -> ::libtest_mimic::Trial;
}

impl<const N: usize, World>
    ScenarioExt<(
        [::std::vec::Vec<ScenarioOrStepHook<World>>; 4],
        [::core::option::Option<::std::vec::Vec<BackgroundGivenStep<World>>>; N],
    )> for Scenario<World>
where
    World: ::core::default::Default + 'static,
{
    fn into_trial_with_context(
        self,
        ([before_scenario_hooks, after_scenario_hooks, before_step_hooks, after_step_hooks], backgrounds): (
            [::std::vec::Vec<ScenarioOrStepHook<World>>; 4],
            [::core::option::Option<::std::vec::Vec<BackgroundGivenStep<World>>>; N],
        ),
    ) -> ::libtest_mimic::Trial {
        let description = self.to_description();

        let context = [before_step_hooks.clone(), after_step_hooks.clone()];

        let callback = move || {
            let mut world = ::core::default::Default::default();

            before_scenario_hooks.to_callback()(&mut world)?;

            backgrounds
                .into_iter()
                .flatten()
                .try_for_each(|background| background.to_callback_with_context(context.clone())(&mut world))?;

            self.given.into_callback_with_context(context.clone())(&mut world)?;
            self.when.into_callback_with_context(context.clone())(&mut world)?;
            self.then.into_callback_with_context(context.clone())(&mut world)?;

            after_scenario_hooks.to_callback()(&mut world)?;

            Ok(())
        };

        into_trial(description, self.tags, callback)
    }
}

impl<const N: usize, World> ScenarioExt<[::core::option::Option<::std::vec::Vec<BackgroundGivenStep<World>>>; N]>
    for Scenario<World>
where
    World: ::core::default::Default + 'static,
{
    fn into_trial_with_context(
        self,
        backgrounds: [::core::option::Option<::std::vec::Vec<BackgroundGivenStep<World>>>; N],
    ) -> ::libtest_mimic::Trial {
        let description = self.to_description();

        let callback = move || {
            let mut world = ::core::default::Default::default();

            backgrounds.into_iter().flatten().try_for_each(|background| background.to_callback()(&mut world))?;

            self.given.into_callback()(&mut world)?;
            self.when.into_callback()(&mut world)?;
            self.then.into_callback()(&mut world)?;

            Ok(())
        };

        into_trial(description, self.tags, callback)
    }
}

fn into_trial(
    description: impl Into<::std::borrow::Cow<'static, str>>,
    tags: ::core::option::Option<impl Into<Tags>>,
    callback: impl FnOnce() -> Fallible + ::core::marker::Send + ::core::marker::Sync + 'static,
) -> ::libtest_mimic::Trial {
    let callback = move || (callback)().map_err(|err| err.message.into());

    let description = description.into();

    let tags = tags.map(Into::into).map(|tags| tags.to_description());

    let trial = ::libtest_mimic::Trial::test(description, callback);

    let trial = match tags {
        Some(tags) => trial.with_kind(tags),
        None => trial,
    };

    trial
}

#[cfg(not(any(feature = "tokio")))]
fn into_trial_async(
    description: impl Into<::std::borrow::Cow<'static, str>>,
    tags: ::core::option::Option<impl Into<Tags>>,
    callback: impl FnOnce() -> ::futures::future::BoxFuture<'static, Fallible> + ::core::marker::Send + ::core::marker::Sync + 'static,
) -> ::libtest_mimic::Trial {
    let callback = move || ::futures::executor::block_on((callback)());

    into_trial(description, tags, callback)
}

#[cfg(feature = "tokio")]
fn into_trial_async(
    description: impl Into<::std::borrow::Cow<'static, str>>,
    tags: ::core::option::Option<impl Into<Tags>>,
    callback: impl FnOnce() -> ::futures::future::BoxFuture<'static, Fallible> + ::core::marker::Send + ::core::marker::Sync + 'static,
) -> ::libtest_mimic::Trial {
    let handle = ::tokio::runtime::Handle::current();

    let callback = move || handle.block_on((callback)());

    into_trial(description, tags, callback)
}

trait ToDescription {
    fn to_description(&self) -> ::std::borrow::Cow<'static, str>;
}

impl<World> ToDescription for Scenario<World> {
    fn to_description(&self) -> ::std::borrow::Cow<'static, str> {
        match self.description {
            Some(ref description) => description.clone(),
            None => ::std::format!(
                "{}; {}; {}",
                self.given.to_description(),
                self.when.to_description(),
                self.then.to_description()
            )
            .into(),
        }
    }
}

impl<Callback> ToDescription for ::std::vec::Vec<Step<Callback>> {
    fn to_description(&self) -> ::std::borrow::Cow<'static, str> {
        self.iter().map(ToDescription::to_description).collect::<::std::vec::Vec<_>>().join(", ").into()
    }
}

impl<Callback> ToDescription for Step<Callback> {
    fn to_description(&self) -> ::std::borrow::Cow<'static, str> {
        ::std::format!("{} {}", self.label.to_description(), self.description).into()
    }
}

impl ToDescription for Tags {
    fn to_description(&self) -> ::std::borrow::Cow<'static, str> {
        self.iter().cloned().collect::<::std::vec::Vec<_>>().join(",").into()
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

    fn into_callback_with_context(
        self,
        context: [::std::vec::Vec<ScenarioOrStepHook<World>>; 2],
    ) -> impl FnOnce(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync;
}

impl<World> ScenarioStepsExt<World> for ::std::vec::Vec<ScenarioGivenOrWhenStep<World>>
where
    World: 'static,
{
    fn into_callback(self) -> impl FnOnce(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync {
        move |world: &mut World| self.into_iter().try_for_each(|step| (step.callback)(world))
    }

    fn into_callback_with_context(
        self,
        [before_step_hooks, after_step_hooks]: [::std::vec::Vec<ScenarioOrStepHook<World>>; 2],
    ) -> impl FnOnce(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync {
        move |world: &mut World| {
            self.into_iter().try_for_each(|step| {
                (before_step_hooks.to_callback())(world)?;
                (step.callback)(world)?;
                (after_step_hooks.to_callback())(world)?;

                Ok(())
            })
        }
    }
}

impl<World> ScenarioStepsExt<World> for ::std::vec::Vec<ScenarioThenStep<World>>
where
    World: 'static,
{
    fn into_callback(self) -> impl FnOnce(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync {
        move |world: &mut World| self.into_iter().try_for_each(|step| (step.callback)(world))
    }

    fn into_callback_with_context(
        self,
        [before_step_hooks, after_step_hooks]: [::std::vec::Vec<ScenarioOrStepHook<World>>; 2],
    ) -> impl FnOnce(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync {
        move |world: &mut World| {
            self.into_iter().try_for_each(|step| {
                (before_step_hooks.to_callback())(world)?;
                (step.callback)(world)?;
                (after_step_hooks.to_callback())(world)?;

                Ok(())
            })
        }
    }
}

trait BackgroundGivenStepsExt<World> {
    fn to_callback(&self) -> impl FnOnce(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync;

    fn to_callback_with_context(
        &self,
        context: [::std::vec::Vec<ScenarioOrStepHook<World>>; 2],
    ) -> impl FnOnce(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync;
}

impl<World> BackgroundGivenStepsExt<World> for ::std::vec::Vec<BackgroundGivenStep<World>>
where
    World: 'static,
{
    fn to_callback(&self) -> impl FnOnce(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync {
        move |world: &mut World| self.iter().try_for_each(|step| (step.callback)(world))
    }

    fn to_callback_with_context(
        &self,
        [before_step_hooks, after_step_hooks]: [::std::vec::Vec<ScenarioOrStepHook<World>>; 2],
    ) -> impl FnOnce(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync {
        move |world: &mut World| {
            self.iter().try_for_each(|step| {
                (before_step_hooks.to_callback())(world)?;
                (step.callback)(world)?;
                (after_step_hooks.to_callback())(world)?;

                Ok(())
            })
        }
    }
}

trait AsyncScenarioGivenOrWhenStepsExt<World> {
    fn into_callback(self) -> impl for<'a> FnOnce(&'a mut World) -> ::futures::future::BoxFuture<'a, Fallible> + ::core::marker::Send + ::core::marker::Sync;

    fn into_callback_with_context(
        self,
        context: [::std::vec::Vec<AsyncScenarioOrStepHook<World>>; 2],
    ) -> impl for<'a> FnOnce(&'a mut World) -> ::futures::future::BoxFuture<'a, Fallible> + ::core::marker::Send + ::core::marker::Sync;
}

impl<World> AsyncScenarioGivenOrWhenStepsExt<World> for ::std::vec::Vec<AsyncScenarioGivenOrWhenStep<World>>
where
    World: ::core::marker::Send + ::core::marker::Sync + 'static,
{
    fn into_callback(self) -> impl for<'a> FnOnce(&'a mut World) -> ::futures::future::BoxFuture<'a, Fallible> + ::core::marker::Send + ::core::marker::Sync {
        move |world: &mut World| ::std::boxed::Box::pin(async move {
            let step_callbacks = self.into_iter().map(|step| step.callback);

            for step_callback in step_callbacks {
                (step_callback)(world).await?;
            }

            Ok(())
        })
    }

    fn into_callback_with_context(
        self,
        [before_step_hooks, after_step_hooks]: [::std::vec::Vec<AsyncScenarioOrStepHook<World>>; 2],
    ) -> impl for<'a> FnOnce(&'a mut World) -> ::futures::future::BoxFuture<'a, Fallible> + ::core::marker::Send + ::core::marker::Sync {
        move |world: &mut World| ::std::boxed::Box::pin(async move {
            let step_callbacks = self.into_iter().map(|step| step.callback);

            for step_callback in step_callbacks {
                (before_step_hooks.to_callback())(world).await?;
                (step_callback)(world).await?;
                (after_step_hooks.to_callback())(world).await?;
            }

            Ok(())
        })
    }
}

impl<World> AsyncScenarioGivenOrWhenStepsExt<World> for ::std::vec::Vec<AsyncScenarioThenStep<World>>
where
    World: ::core::marker::Send + ::core::marker::Sync + 'static,
{
    fn into_callback(self) -> impl for<'a> FnOnce(&'a mut World) -> ::futures::future::BoxFuture<'a, Fallible> + ::core::marker::Send + ::core::marker::Sync {
        move |world: &mut World| ::std::boxed::Box::pin(async move {
            let step_callbacks = self.into_iter().map(|step| step.callback);

            for step_callback in step_callbacks {
                (step_callback)(world).await?;
            }

            Ok(())
        })
    }

    fn into_callback_with_context(
        self,
        [before_step_hooks, after_step_hooks]: [::std::vec::Vec<AsyncScenarioOrStepHook<World>>; 2],
    ) -> impl for<'a> FnOnce(&'a mut World) -> ::futures::future::BoxFuture<'a, Fallible> + ::core::marker::Send + ::core::marker::Sync {
        move |world: &mut World| ::std::boxed::Box::pin(async move {
            let step_callbacks = self.into_iter().map(|step| step.callback);

            for step_callback in step_callbacks {
                (before_step_hooks.to_callback())(world).await?;
                (step_callback)(world).await?;
                (after_step_hooks.to_callback())(world).await?;
            }

            Ok(())
        })
    }
}

trait AsyncBackgroundGivenStepsExt<World> {
    fn to_callback(&self) -> impl for<'a> FnOnce(&'a mut World) -> ::futures::future::BoxFuture<'a, Fallible> + ::core::marker::Send + ::core::marker::Sync;

    fn to_callback_with_context(
        &self,
        context: [::std::vec::Vec<AsyncScenarioOrStepHook<World>>; 2],
    ) -> impl for<'a> FnOnce(&'a mut World) -> ::futures::future::BoxFuture<'a, Fallible> + ::core::marker::Send + ::core::marker::Sync;
}

impl<World> AsyncBackgroundGivenStepsExt<World> for ::std::vec::Vec<AsyncBackgroundGivenStep<World>>
where
    World: ::core::marker::Send + ::core::marker::Sync + 'static,
{
    fn to_callback(&self) -> impl for<'a> FnOnce(&'a mut World) -> ::futures::future::BoxFuture<'a, Fallible> + ::core::marker::Send + ::core::marker::Sync {
        move |world: &mut World| {
            let step_callbacks = self.iter().map(|step| step.callback.clone()).collect::<::std::vec::Vec<_>>();

            ::std::boxed::Box::pin(async move {
                for step_callback in step_callbacks {
                    (step_callback)(world).await?;
                }

                Ok(())
            })
        }
    }

    fn to_callback_with_context(
        &self,
        [before_step_hooks, after_step_hooks]: [::std::vec::Vec<AsyncScenarioOrStepHook<World>>; 2],
    ) -> impl for<'a> FnOnce(&'a mut World) -> ::futures::future::BoxFuture<'a, Fallible> + ::core::marker::Send + ::core::marker::Sync {
        move |world: &mut World| {
            let step_callbacks = self.iter().map(|step| step.callback.clone()).collect::<::std::vec::Vec<_>>();

            ::std::boxed::Box::pin(async move {
                for step_callback in step_callbacks {
                    (before_step_hooks.to_callback())(world).await?;
                    (step_callback)(world).await?;
                    (after_step_hooks.to_callback())(world).await?;
                }

                Ok(())
            })
        }
    }
}

trait ScenarioOrStepHooksExt<World> {
    fn to_callback(&self) -> impl Fn(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync;
}

impl<World> ScenarioOrStepHooksExt<World> for ::std::vec::Vec<ScenarioOrStepHook<World>>
where
    World: 'static,
{
    fn to_callback(&self) -> impl Fn(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync {
        move |world: &mut World| self.iter().try_for_each(|hook| (hook.callback)(world))
    }
}

trait GlobalHooksExt {
    fn into_callback(self) -> impl FnOnce() -> Fallible + ::core::marker::Send + ::core::marker::Sync;
}

impl GlobalHooksExt for ::std::vec::Vec<GlobalHook> {
    fn into_callback(self) -> impl FnOnce() -> Fallible + ::core::marker::Send + ::core::marker::Sync {
        move || self.into_iter().try_for_each(|hook| (hook.callback)())
    }
}

trait AsyncScenarioOrStepHooksExt<World> {
    fn to_callback(&self) -> impl for<'a> Fn(&'a mut World) -> ::futures::future::BoxFuture<'a, Fallible> + ::core::marker::Send + ::core::marker::Sync;
}

impl<World> AsyncScenarioOrStepHooksExt<World> for ::std::vec::Vec<AsyncScenarioOrStepHook<World>>
where
    World: ::core::marker::Send + ::core::marker::Sync + 'static,
{
    fn to_callback(&self) -> impl for<'a> Fn(&'a mut World) -> ::futures::future::BoxFuture<'a, Fallible> + ::core::marker::Send + ::core::marker::Sync + '_ {
        move |world: &mut World| {
            let hook_callbacks = self.iter()
                .map(|hook| hook.callback.clone())
                .collect::<::std::vec::Vec<_>>();

            ::std::boxed::Box::pin(async move {
                for hook_callback in hook_callbacks {
                    (hook_callback)(world).await?
                }

                Ok(())
            })
        }
    }
}

trait AsyncGlobalHooksExt {
    fn into_callback(self) -> impl FnOnce() -> ::futures::future::BoxFuture<'static, Fallible> + ::core::marker::Send + ::core::marker::Sync;
}

impl AsyncGlobalHooksExt for ::std::vec::Vec<AsyncGlobalHook> {
    fn into_callback(self) -> impl FnOnce() -> ::futures::future::BoxFuture<'static, Fallible> + ::core::marker::Send + ::core::marker::Sync {
        move || ::std::boxed::Box::pin(async move {
            let hook_callbacks = self.into_iter()
                .map(|hook| hook.callback);

            for hook_callback in hook_callbacks {
                (hook_callback)().await?
            }

            Ok(())
        })
    }
}

impl Runner {
    pub fn run(self) -> ::std::process::ExitCode {
        let trials = self
            .trials
            .into_iter()
            .flat_map(|trials| trials.into_trials_with_configurations(&self.configurations))
            .collect();

        let mut args = ::libtest_mimic::Arguments::from_args();
        self.configurations.update(&mut args);

        let _ = self.before_global_hooks.into_callback()();

        let conclusion = ::libtest_mimic::run(&args, trials);
        let exit_code = conclusion.exit_code();

        let _ = self.after_global_hooks.into_callback()();

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
