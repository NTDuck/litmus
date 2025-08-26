use ::sealed::sealed;

use crate::models::*;

pub fn run<RandomState: ::core::hash::BuildHasher>(trials: impl IntoTrials<RandomState>, filter: impl IntoTagsFilter<RandomState>) -> ::std::process::ExitCode {
    let args = ::libtest_mimic::Arguments::from_args();

    let filter = filter.into_filter();
    let trials = trials.into_trials(filter).into_iter().collect();

    let conclusion = ::libtest_mimic::run(&args, trials);
    conclusion.exit_code()
}

#[sealed]
pub trait IntoTagsFilter<RandomState: ::core::hash::BuildHasher> {
    fn into_filter(self) -> impl Fn(&Tags<RandomState>) -> bool + ::core::clone::Clone;
}

#[sealed]
impl<F, RandomState: ::core::hash::BuildHasher> IntoTagsFilter<RandomState> for F
where
    F: Fn(&Tags<RandomState>) -> bool + ::core::clone::Clone,
{
    fn into_filter(self) -> impl Fn(&Tags<RandomState>) -> bool + ::core::clone::Clone {
        self
    }
}

#[sealed]
impl<'a, T, RandomState: ::core::hash::BuildHasher> IntoTagsFilter<RandomState> for &'a [T]
where
    T: Into<::std::borrow::Cow<'static, str>> + ::core::clone::Clone,
    RandomState: ::core::default::Default + ::core::clone::Clone,
{
    fn into_filter(self) -> impl Fn(&Tags<RandomState>) -> bool + ::core::clone::Clone {
        move |tags| {
            let filter = Tags::from(self.iter().cloned().map(Into::into));
            !filter.is_disjoint(tags)
        }
    }
}

#[sealed]
impl<T, const N: usize, RandomState: ::core::hash::BuildHasher> IntoTagsFilter<RandomState> for [T; N]
where
    T: Into<::std::borrow::Cow<'static, str>> + ::core::clone::Clone,
    RandomState: ::core::default::Default,
{
    fn into_filter(self) -> impl Fn(&Tags<RandomState>) -> bool + ::core::clone::Clone {
        move |tags| {
            let filter = Tags::from(self.iter().cloned().map(Into::into));
            !filter.is_disjoint(tags)
        }
    }
}

impl<RandomState: ::core::hash::BuildHasher> Tags<RandomState> {
    fn is_disjoint(&self, other: &Tags<RandomState>) -> bool {
        self.0.is_disjoint(&other.0)
    }
}

#[sealed]
pub trait IntoTrial<RandomState: ::core::hash::BuildHasher> {
    fn into_trial(self, filter: impl Fn(&Tags<RandomState>) -> bool + ::core::clone::Clone) -> ::libtest_mimic::Trial;
}

#[sealed]
pub trait IntoTrials<RandomState: ::core::hash::BuildHasher> {
    fn into_trials(self, filter: impl Fn(&Tags<RandomState>) -> bool + ::core::clone::Clone) -> impl IntoIterator<Item = ::libtest_mimic::Trial>;
}

#[sealed]
impl<World, RandomState: ::core::hash::BuildHasher> IntoTrials<RandomState> for Suite<World, RandomState> {
    fn into_trials(mut self, filter: impl Fn(&Tags<RandomState>) -> bool + ::core::clone::Clone) -> impl IntoIterator<Item = ::libtest_mimic::Trial> {
        self.retain(filter);

        let scenario_step_transformer = move |step: Step<::std::boxed::Box<dyn FnOnce(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync>>| |world: &mut World| {
            self.before_step_hooks.into_iter()
                .map(|hook| hook.callback)
                .try_for_each(|hook| (hook)(world))?;

            (step.callback)(world)?;

            self.after_step_hooks.into_iter()
                .map(|hook| hook.callback)
                .try_for_each(|hook| (hook)(world))?;

            Fallible::Ok(())
        };

        vec![]
    }
}

trait Retain<RandomState: ::core::hash::BuildHasher> {
    fn retain(&mut self, filter: impl Fn(&Tags<RandomState>) -> bool + ::core::clone::Clone);
}

impl<World, RandomState: ::core::hash::BuildHasher> Retain<RandomState> for Suite<World, RandomState> {
    fn retain(&mut self, filter: impl Fn(&Tags<RandomState>) -> bool + ::core::clone::Clone) {
        self.features.retain(|feature| feature.tags.as_ref().is_some_and(|tags| (filter)(&tags)));
    }
}

impl<World, RandomState: ::core::hash::BuildHasher> Retain<RandomState> for Feature<World, RandomState> {
    fn retain(&mut self, filter: impl Fn(&Tags<RandomState>) -> bool + ::core::clone::Clone) {
        self.scenarios.retain(|scenario| scenario.tags.as_ref().is_some_and(|tags| (filter)(&tags)));
        self.rules.retain(|rule| rule.tags.as_ref().is_some_and(|tags| (filter)(&tags)));

        self.rules.iter_mut()
            .for_each(|rule| rule.retain(filter.clone()));
    }
}

impl<World, RandomState: ::core::hash::BuildHasher> Retain<RandomState> for Rule<World, RandomState> {
    fn retain(&mut self, filter: impl Fn(&Tags<RandomState>) -> bool + ::core::clone::Clone) {
        self.scenarios.retain(|scenario| scenario.tags.as_ref().is_some_and(|tags| (filter)(&tags)));
    }
}

impl<World, RandomState: ::core::hash::BuildHasher> Suite<World, RandomState> {
    fn to_scenario_step_transformer(&self, step: Step<::std::boxed::Box<dyn FnOnce(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync + 'static>>) -> impl FnOnce(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync + 'static {
        move |world: &mut World| {
            self.before_step_hooks.into_iter()
                .map(|hook| hook.callback)
                .try_for_each(|hook| (hook)(world))?;

            (step.callback)(world)?;

            self.after_step_hooks.into_iter()
                .map(|hook| hook.callback)
                .try_for_each(|hook| (hook)(world))?;

            Ok(())
        }
    }
}

#[::bon::builder]
#[builder(on(_, required))]
fn into_trial(#[builder(start_fn)] callback: impl FnOnce() -> Fallible + ::core::marker::Send + ::core::marker::Sync + 'static, description: impl Into<::std::borrow::Cow<'static, str>>, ignored: ::core::option::Option<impl Into<bool>>, tags: ::core::option::Option<impl Into<Tags>>) -> ::libtest_mimic::Trial {
    let callback = move || {
        (callback)()
            .map_err(|err| err.message.into())
    };

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

#[sealed]
trait ToDescription {
    fn to_description(&self) -> ::std::borrow::Cow<'static, str>;
}

#[sealed]
impl<World, RandomState: ::core::hash::BuildHasher> ToDescription for Scenario<World, RandomState> {
    fn to_description(&self) -> ::std::borrow::Cow<'static, str> {
        ::std::format!("{}; {}; {}", self.given.to_description(), self.when.to_description(), self.then.to_description()).into()
    }
}

#[sealed]
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

#[sealed]
impl<Callback> ToDescription for Step<Callback> {
    fn to_description(&self) -> ::std::borrow::Cow<'static, str> {
        ::std::format!("{} {}", self.label.to_description(), self.description).into()
    }
}

#[sealed]
impl<RandomState: ::core::hash::BuildHasher> ToDescription for Tags<RandomState> {
    fn to_description(&self) -> ::std::borrow::Cow<'static, str> {
        self.0
            .iter()
            .cloned()
            .collect::<::std::vec::Vec<_>>()
            .join(",")
            .into()
    }
}

#[sealed]
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

#[sealed]
trait IntoCallback<RandomState: ::core::hash::BuildHasher> {
    fn into_callback(self, filter: impl IntoTagsFilter<RandomState>) -> impl FnOnce() -> Fallible + ::core::marker::Send + ::core::marker::Sync + 'static;
}

#[sealed]
impl<World, RandomState: ::core::hash::BuildHasher> IntoCallback<RandomState> for (
    Scenario<World, RandomState>,
    &Suite<World, RandomState>,
    &Feature<World, RandomState>,
)
where
    World: ::core::default::Default + 'static,
{
    fn into_callback(self, filter: impl IntoTagsFilter<RandomState>) -> impl FnOnce() -> Fallible + ::core::marker::Send + ::core::marker::Sync + 'static {
        let filter = filter.into_filter();

        let (scenario, suite, feature) = self;

        ::core::assert!(feature.tags.as_ref().is_none_or(|tags| filter(&tags)));
        ::core::assert!(scenario.tags.as_ref().is_none_or(|tags| filter(&tags)));

        move || {
            let mut world = ::core::default::Default::default();
            
            scenario.given.into_iter()
                .map(|step| step.callback)
                .try_for_each(|callback| (callback)(&mut world))?;

            scenario.when.into_iter()
                .map(|step| step.callback)
                .try_for_each(|callback| (callback)(&mut world))?;

            scenario.then.into_iter()
                .map(|step| step.callback)
                .try_for_each(|callback| (callback)(&mut world))?;

            Ok(())
        }
    }
}
