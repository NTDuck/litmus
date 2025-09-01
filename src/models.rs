use crate::utils::aliases;

pub struct Feature<World> {
    pub(crate) description: ::core::option::Option<::std::borrow::Cow<'static, str>>,
    pub(crate) ignored: ::core::option::Option<bool>,
    pub(crate) tags: ::core::option::Option<Tags>,

    pub(crate) background: ::core::option::Option<Background<World>>,
    pub(crate) scenarios: ::std::vec::Vec<Scenario<World>>,
    pub(crate) rules: ::std::vec::Vec<Rule<World>>,
}

pub struct Rule<World> {
    pub(crate) description: ::core::option::Option<::std::borrow::Cow<'static, str>>,
    pub(crate) ignored: ::core::option::Option<bool>,
    pub(crate) tags: ::core::option::Option<Tags>,

    pub(crate) background: ::core::option::Option<Background<World>>,
    pub(crate) scenarios: ::std::vec::Vec<Scenario<World>>,
}

pub struct Scenario<World> {
    pub(crate) description: ::core::option::Option<::std::borrow::Cow<'static, str>>,
    pub(crate) ignored: ::core::option::Option<bool>,
    pub(crate) tags: ::core::option::Option<Tags>,

    pub(crate) given: ::std::vec::Vec<Step<::std::boxed::Box<dyn FnOnce(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync>>>,
    pub(crate) when: ::std::vec::Vec<Step<::std::boxed::Box<dyn FnOnce(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync>>>,
    pub(crate) then: ::std::vec::Vec<Step<::std::boxed::Box<dyn FnOnce(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync>>>,
}

pub struct Background<World> {
    pub(crate) description: ::core::option::Option<::std::borrow::Cow<'static, str>>,
    pub(crate) ignored: ::core::option::Option<bool>,

    pub(crate) given: ::std::vec::Vec<Step<aliases::sync::Arc<dyn Fn(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync>>>,
}

#[derive(::core::clone::Clone)]
pub(crate) struct Hook<Callback> {
    pub(crate) tags: ::core::option::Option<Tags>,
    pub(crate) callback: Callback,
}

#[derive(::core::clone::Clone)]
pub(crate) struct Step<Callback> {
    pub(crate) label: StepLabel,
    pub(crate) description: ::std::borrow::Cow<'static, str>,
    pub(crate) callback: Callback,
}

#[derive(::core::clone::Clone)]
pub struct Tags(pub(crate) ::std::collections::HashSet<::std::borrow::Cow<'static, str>, aliases::hash::BuildHasher>);

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
    pub message: ::std::borrow::Cow<'static, str>,
}
