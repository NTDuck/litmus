use ::sealed::sealed;

use crate::models::*;

pub fn run(trials: impl IntoTrials) -> ::std::process::ExitCode {
    let args = ::libtest_mimic::Arguments::from_args();
    let trials = trials.into_trials().into_iter().collect();
    let conclusion = ::libtest_mimic::run(&args, trials);
    conclusion.exit_code()
}

#[sealed]
impl<World, RandomState: ::core::hash::BuildHasher> IntoTrials for Suite<World, RandomState> {
    fn into_trials(self) -> impl IntoIterator<Item =  ::libtest_mimic::Trial>  {
        vec![]
    }
}

#[sealed]
impl<World, RandomState: ::core::hash::BuildHasher> IntoTrial for (
    Scenario<World, RandomState>,
    &Feature<World, RandomState>,
)
where
    World: ::core::default::Default + 'static,
{
    fn into_trial(self) ->  ::libtest_mimic::Trial {
        let (scenario, feature) = self;

        todo!()
    }
}

// impl<World> Into<::libtest_mimic::Trial> for Scenario<World>
// where
//     World: ::core::default::Default + 'static,
// {
//     fn into(self) -> ::libtest_mimic::Trial {
//         let description = self.description
//             .unwrap_or_else(|| ::std::format!("{}; {}; {}", self.given, self.when, self.then).into());

//         let callback = move || {
//             let mut world = ::core::default::Default::default();
            
//             self.given.0.into_iter()
//                 .map(|step| step.callback)
//                 .try_for_each(|callback| (callback)(&mut world))?;

//             self.when.0.into_iter()
//                 .map(|step| step.callback)
//                 .try_for_each(|callback| (callback)(&mut world))?;

//             self.then.0.into_iter()
//                 .map(|step| step.callback)
//                 .try_for_each(|callback| (callback)(&world))?;

//             Ok(())
//         };

//         into_trial(callback)
//             .description(description)
//             .ignored(self.ignored)
//             .tags(self.tags)
//             .call()
//     }
// }

#[sealed]
pub trait IntoTrial {
    fn into_trial(self) -> ::libtest_mimic::Trial;
}

#[sealed]
pub trait IntoTrials {
    fn into_trials(self) -> impl IntoIterator<Item = ::libtest_mimic::Trial>;
}

#[::bon::builder]
#[builder(on(_, required))]
fn into_trial<Callback>(#[builder(start_fn)] callback: Callback, description: impl Into<::std::borrow::Cow<'static, str>>, ignored: ::core::option::Option<impl Into<bool>>, tags: ::core::option::Option<impl Into<Tags>>) -> ::libtest_mimic::Trial
where
    Callback: FnOnce() -> Fallible + ::core::marker::Send + ::core::marker::Sync + 'static,
{
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
