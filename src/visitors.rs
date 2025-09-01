use ::sealed::sealed;

use crate::models::*;
use crate::utils::aliases;

/*
ok ok so heres idea
runner -> suite becomes a part of visitors
suite accepts features + worldful hooks
runner accepts suites + individual features (will be exec-ed without hooks in mind)
there are hookful runner and hookless runner, will be impl in private API DEBUNKED
runner func is run
runner can be configured
child thingies get access to runner configuration, gherkin has none, visit libtest mimic for intel
whoopes!!!
*/

pub struct Runner {
    trials: ::std::vec::Vec<::libtest_mimic::Trial>,

    configurations: RunnerConfigurations,

    before_global_hooks: ::std::vec::Vec<Hook<::std::boxed::Box<dyn FnOnce() -> Fallible + ::core::marker::Send + ::core::marker::Sync>>>,
    after_global_hooks: ::std::vec::Vec<Hook<::std::boxed::Box<dyn FnOnce() -> Fallible + ::core::marker::Send + ::core::marker::Sync>>>,
}

struct RunnerConfigurations {
    ignore_filter: IgnoreFilter,
    tags_filter: ::core::option::Option<::std::boxed::Box<dyn Fn(&Tags) -> bool>>,

    format: FormatPolicy,
    color: ColorPolicy,

    threads: ::core::option::Option<u64>,
    logfile: ::core::option::Option<::std::borrow::Cow<'static, ::std::path::Path>>,
}

#[derive(::core::default::Default)]
pub enum IgnoreFilter {
    #[default]
    RunNonIgnoredOnly,
    RunIgnoredOnly,
    RunBothIgnoredAndUnignored,
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

impl Tags {
    fn is_disjoint(&self, other: &Tags) -> bool {
        self.0.is_disjoint(&other.0)
    }
}

#[derive(::core::default::Default)]
pub enum FormatPolicy {
    #[default]
    Pretty,
    Terse,
    Json,
}

impl From<FormatPolicy> for ::libtest_mimic::FormatSetting {
    fn from(policy: FormatPolicy) -> Self {
        match policy {
            FormatPolicy::Pretty => Self::Pretty,
            FormatPolicy::Terse => Self::Terse,
            FormatPolicy::Json => Self::Json,
        }
    }
}

#[derive(::core::default::Default)]
pub enum ColorPolicy {
    #[default]
    Auto,
    Always,
    Never,
}

impl From<ColorPolicy> for ::libtest_mimic::ColorSetting {
    fn from(policy: ColorPolicy) -> Self {
        match policy {
            ColorPolicy::Auto => Self::Auto,
            ColorPolicy::Always => Self::Always,
            ColorPolicy::Never => Self::Never,
        }
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

    mod suite {
        pub(super) use super::*;
    }

    pub struct RunnerBuilder {
        trials: ::std::vec::Vec<::libtest_mimic::Trial>,

        configurations: RunnerConfigurations,

        before_global_hooks: ::std::vec::Vec<Hook<::std::boxed::Box<dyn FnOnce() -> Fallible + ::core::marker::Send + ::core::marker::Sync>>>,
        after_global_hooks: ::std::vec::Vec<Hook<::std::boxed::Box<dyn FnOnce() -> Fallible + ::core::marker::Send + ::core::marker::Sync>>>,      
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
}

trait RetainByIgnored {
    fn retain(&mut self, filter: IgnoreFilter);
}

trait RetainByTags {
    fn retain<Callback>(&mut self, filter: impl ::std::ops::Deref<Target = Callback> + ::core::clone::Clone)
    where
        Callback: Fn(&Tags) -> bool;
}

impl<World> RetainByTags for Suite<World> {
    fn retain<Callback>(&mut self, filter: impl ::std::ops::Deref<Target = Callback> + ::core::clone::Clone)
    where
        Callback: Fn(&Tags) -> bool,
    {
        self.features.retain(|feature| feature.tags.as_ref().is_some_and(|tags| (filter)(&tags)));

        self.before_scenario_hooks.retain(|hook| hook.tags.as_ref().is_some_and(|tags| (filter)(&tags)));
        self.after_scenario_hooks.retain(|hook| hook.tags.as_ref().is_some_and(|tags| (filter)(&tags)));

        self.before_step_hooks.retain(|hook| hook.tags.as_ref().is_some_and(|tags| (filter)(&tags)));
        self.after_step_hooks.retain(|hook| hook.tags.as_ref().is_some_and(|tags| (filter)(&tags)));
    }
}

impl<World> RetainByTags for Feature<World> {
    fn retain<Callback>(&mut self, filter: impl ::std::ops::Deref<Target = Callback> + ::core::clone::Clone)
    where
        Callback: Fn(&Tags) -> bool,
    {
        self.scenarios.retain(|scenario| scenario.tags.as_ref().is_some_and(|tags| (filter)(&tags)));
        self.rules.retain(|rule| rule.tags.as_ref().is_some_and(|tags| (filter)(&tags)));

        self.rules.iter_mut()
            .for_each(|rule| rule.retain(filter.clone()));
    }
}

impl<World> RetainByTags for Rule<World> {
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
            .ignored(self.ignored)
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
            .ignored(self.ignored)
            .tags(self.tags)
            .callback(callback)
            .call()
    }
}

trait ToDescription {
    fn to_description(&self) -> ::std::borrow::Cow<'static, str>;
}

impl<World> ToDescription for Scenario<World> {
    fn to_description(&self) -> ::std::borrow::Cow<'static, str> {
        ::std::format!("{}; {}; {}", self.given.to_description(), self.when.to_description(), self.then.to_description()).into()
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

#[::bon::builder]
#[builder(on(_, required))]
fn into_trial(description: impl Into<::std::borrow::Cow<'static, str>>, ignored: ::core::option::Option<impl Into<bool>>, tags: ::core::option::Option<impl Into<Tags>>, callback: impl FnOnce() -> Fallible + ::core::marker::Send + ::core::marker::Sync + 'static) -> ::libtest_mimic::Trial {
    let callback = move || (callback)().map_err(|err| err.message.into());

    let description = description.into();
    let ignored = ignored.map(Into::into);

    let tags = tags
        .map(Into::into)
        .map(|tags| tags.to_description());

    let trial = ::libtest_mimic::Trial::test(description, callback);
    
    let trial = match ignored {
        Some(ignored) => trial.with_ignored_flag(ignored),
        None => trial,
    };

    let trial = match tags {
        Some(tags) => trial.with_kind(tags),
        None => trial,
    };

    trial
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
