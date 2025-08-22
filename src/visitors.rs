use crate::models::*;

trait Executor<Unit> {
    type Output;

    fn execute(&self, unit: Unit) -> Self::Output;
}

struct LibtestMimicExecutor;

impl<World> Executor<Scenario<World>> for LibtestMimicExecutor
where
    World: 'static,
{
    type Output = ::libtest_mimic::Trial;
    
    fn execute(&self, scenario: Scenario<World>) -> Self::Output {
        let description = scenario.description
            .unwrap_or_else(|| match scenario.given.1 {
                Some(ref steps) => ::std::format!("{} {}; {}; {}", scenario.given.0, steps, scenario.when, scenario.then).into(),
                None => ::std::format!("{}; {}; {}", scenario.given.0, scenario.when, scenario.then).into(),
            });

        let callback = move || {
            let mut world = (scenario.given.0.callback)()?;
            
            if let Some(steps) = scenario.given.1 {
                steps.0.into_iter()
                    .map(|step| step.callback)
                    .try_for_each(|callback| (callback)(&mut world))?;
            }

            scenario.when.0.into_iter()
                .map(|step| step.callback)
                .try_for_each(|callback| (callback)(&mut world))?;

            scenario.then.0.into_iter()
                .map(|step| step.callback)
                .try_for_each(|callback| (callback)(&world))?;

            Ok(())
        };

        self.execute(callback)
            .description(description)
            .ignored(scenario.ignored)
            .tags(scenario.tags)
            .call()
    }
}

#[::bon::bon]
impl LibtestMimicExecutor {
    #[builder(on(_, required))]
    fn execute<Callback>(&self, #[builder(start_fn)] callback: Callback, description: impl Into<::std::borrow::Cow<'static, str>>, ignored: ::core::option::Option<impl Into<bool>>, tags: ::core::option::Option<impl IntoTags>) -> ::libtest_mimic::Trial
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
