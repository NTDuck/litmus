// use crate::{utils::aliases::{MaybeOwnedString, Vec}, Fallible, IntoTags, Scenario};

// trait Executor<Unit> {
//     type Output;

//     fn execute(&self, unit: Unit) -> Self::Output;
// }

// struct LibtestMimicExecutor;

// impl<Given, When, Then, World> Executor<Scenario<Given, When, Then>> for LibtestMimicExecutor
// where
//     Given: FnOnce() -> Fallible<World>,
//     When: FnOnce(&mut World) -> Fallible,
//     Then: FnOnce(&World) -> Fallible,
//     World: ::core::marker::Send + ::core::marker::Sync,
// {
//     type Output = ::libtest_mimic::Trial;
    
//     fn execute(&self, scenario: Scenario<Given, When, Then>) -> Self::Output {
//         // let description = scenario.description
//         //     .unwrap_or_else(|| )

//         todo!()
//     }
// }

// #[::bon::bon]
// impl LibtestMimicExecutor {
//     #[builder]
//     fn execute<Callback>(&self, #[builder(start_fn)] callback: Callback, description: impl Into<MaybeOwnedString>, ignored: ::core::option::Option<impl Into<bool>>, tags: ::core::option::Option<impl IntoTags>) -> ::libtest_mimic::Trial
//     where
//         Callback: FnOnce() -> Fallible + ::core::marker::Send + ::core::marker::Sync + 'static,
//     {
//         let callback = move || {
//             (callback)()
//                 .map_err(|err| err.message.into())
//         };

//         let description = description.into();
//         let ignored = ignored.map(Into::into);

//         let tags = tags
//             .map(IntoTags::into_tags)
//             .map(|tags| tags.0
//                 .into_iter()
//                 .collect::<Vec<_>>()
//                 .join(","));

//         let trial = ::libtest_mimic::Trial::test(description, callback);
        
//         let trial = match ignored {
//             Some(ignored) => trial.with_ignored_flag(ignored),
//             None => trial,
//         };

//         let trial = match tags {
//             Some(tags) => trial.with_kind(tags),
//             None => trial,
//         };

//         trial
//     }
// }
