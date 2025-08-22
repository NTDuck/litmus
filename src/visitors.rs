use crate::models::*;

pub fn run<Trial>(trials: impl IntoIterator<Item = Trial>) -> ::std::process::ExitCode
where
    Trial: Into<::libtest_mimic::Trial>,
{
    let args = ::libtest_mimic::Arguments::from_args();
    let trials = trials.into_iter().map(Into::into).collect();
    let conclusion = ::libtest_mimic::run(&args, trials);
    conclusion.exit_code()
}

impl<World> Into<::libtest_mimic::Trial> for Scenario<World>
where
    World: ::core::default::Default + 'static,
{
    fn into(self) -> ::libtest_mimic::Trial {
        let description = self.description
            .unwrap_or_else(|| ::std::format!("{}; {}; {}", self.given, self.when, self.then).into());

        let callback = move || {
            let mut world = ::core::default::Default::default();
            
            self.given.0.into_iter()
                .map(|step| step.callback)
                .try_for_each(|callback| (callback)(&mut world))?;

            self.when.0.into_iter()
                .map(|step| step.callback)
                .try_for_each(|callback| (callback)(&mut world))?;

            self.then.0.into_iter()
                .map(|step| step.callback)
                .try_for_each(|callback| (callback)(&world))?;

            Ok(())
        };

        into_trial(callback)
            .description(description)
            .ignored(self.ignored)
            .tags(self.tags)
            .call()
    }
}

#[::bon::builder]
#[builder(on(_, required))]
fn into_trial<Callback>(#[builder(start_fn)] callback: Callback, description: impl Into<::std::borrow::Cow<'static, str>>, ignored: ::core::option::Option<impl Into<bool>>, tags: ::core::option::Option<impl IntoTags>) -> ::libtest_mimic::Trial
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
        .map(IntoTags::into_tags)
        .map(|tags| ::std::format!("{}", tags));

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

impl<RandomState: ::core::hash::BuildHasher> ::std::fmt::Display for Tags<RandomState> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let repr = self.0
            .iter()
            .cloned()
            .collect::<::std::vec::Vec<_>>()
            .join(",");

        ::core::write!(formatter, "{}", repr)
    }
}

impl<Callback> ::std::fmt::Display for Steps<Callback> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let repr = self.0
            .iter()
            .map(|step| ::std::format!("{}", step))
            .collect::<::std::vec::Vec<_>>()
            .join(", ");

        ::core::write!(formatter, "{}", repr)
    }
}

impl<Callback> ::std::fmt::Display for Step<Callback> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        ::core::write!(formatter, "{} {}", self.label, self.description)
    }
}

impl ::std::fmt::Display for StepLabel {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let repr = match self {
            Self::Given => "Given",
            Self::When => "When",
            Self::Then => "Then",

            Self::And => "and",
            Self::But => "but",
        };

        ::core::write!(formatter, "{}", repr)
    }
}
