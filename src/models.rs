use crate::utils::aliases;

pub struct Feature<World> {
    #[allow(dead_code)]
    pub(crate) description: ::core::option::Option<aliases::string::String>,
    pub(crate) ignored: ::core::option::Option<bool>,
    pub(crate) tags: ::core::option::Option<Tags>,

    pub(crate) background: ::core::option::Option<Background<World>>,
    pub(crate) scenarios: ::std::vec::Vec<Scenario<World>>,
    pub(crate) rules: ::std::vec::Vec<Rule<World>>,
}

pub struct Rule<World> {
    pub(crate) description: ::core::option::Option<aliases::string::String>,
    pub(crate) ignored: ::core::option::Option<bool>,
    pub(crate) tags: ::core::option::Option<Tags>,

    pub(crate) background: ::core::option::Option<Background<World>>,
    pub(crate) scenarios: ::std::vec::Vec<Scenario<World>>,
}

pub struct Scenario<World> {
    pub(crate) description: ::core::option::Option<aliases::string::String>,
    pub(crate) ignored: ::core::option::Option<bool>,
    pub(crate) tags: ::core::option::Option<Tags>,

    pub(crate) given: ::std::vec::Vec<ScenarioStep<World>>,
    pub(crate) when: ::std::vec::Vec<ScenarioStep<World>>,
    pub(crate) then: ::std::vec::Vec<ScenarioStep<World>>,
}

pub struct Background<World> {
    pub(crate) description: ::core::option::Option<aliases::string::String>,
    pub(crate) ignored: ::core::option::Option<bool>,

    pub(crate) given: ::std::vec::Vec<BackgroundStep<World>>,
}

#[derive(::core::clone::Clone)]
pub(crate) struct Hook<Callback> {
    pub(crate) tags: ::core::option::Option<Tags>,
    pub(crate) callback: Callback,
}

pub(crate) type NonGlobalHook<World> = Hook<aliases::sync::Arc<dyn Fn(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync>>;
pub(crate) type GlobalHook = Hook<::std::boxed::Box<dyn FnOnce() -> Fallible + ::core::marker::Send + ::core::marker::Sync>>;

#[derive(::core::clone::Clone)]
pub(crate) struct Step<Callback> {
    pub(crate) label: StepLabel,
    pub(crate) description: aliases::string::String,
    
    pub(crate) callback: Callback,
}

pub(crate) type ScenarioStep<World> = Step<::std::boxed::Box<dyn FnOnce(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync>>;
pub(crate) type BackgroundStep<World> = Step<aliases::sync::Arc<dyn Fn(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync>>;

pub type Tags = ::std::collections::HashSet<aliases::string::String, aliases::hash::BuildHasher>;

#[derive(::core::clone::Clone, ::core::marker::Copy)]
pub(crate) enum StepLabel {
    Given,
    When,
    Then,

    And,
    But,
}

pub type Fallible<T = ()> = ::core::result::Result<T, Failed>;

pub struct Failed {
    pub(crate) message: aliases::string::String,
}
