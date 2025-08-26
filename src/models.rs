pub struct Suite<World, RandomState: ::core::hash::BuildHasher = ::std::hash::RandomState> {
    pub(crate) features: ::std::vec::Vec<Feature<World, RandomState>>,

    pub(crate) before_scenario_hooks: ::std::vec::Vec<Hook<::std::rc::Rc<dyn Fn(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync>, RandomState>>,
    pub(crate) after_scenario_hooks: ::std::vec::Vec<Hook<::std::rc::Rc<dyn Fn(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync>, RandomState>>,

    pub(crate) before_step_hooks: ::std::vec::Vec<Hook<::std::rc::Rc<dyn Fn(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync>, RandomState>>,
    pub(crate) after_step_hooks: ::std::vec::Vec<Hook<::std::rc::Rc<dyn Fn(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync>, RandomState>>,

    pub(crate) before_global_hooks: ::std::vec::Vec<Hook<::std::boxed::Box<dyn FnOnce() -> Fallible + ::core::marker::Send + ::core::marker::Sync>, RandomState>>,
    pub(crate) after_global_hooks: ::std::vec::Vec<Hook<::std::boxed::Box<dyn FnOnce() -> Fallible + ::core::marker::Send + ::core::marker::Sync>, RandomState>>,
}

pub struct Feature<World, RandomState: ::core::hash::BuildHasher = ::std::hash::RandomState> {
    pub(crate) description: ::core::option::Option<::std::borrow::Cow<'static, str>>,
    pub(crate) ignored: ::core::option::Option<bool>,
    pub(crate) tags: ::core::option::Option<Tags<RandomState>>,

    pub(crate) background: ::core::option::Option<Background<World>>,
    pub(crate) scenarios: ::std::vec::Vec<Scenario<World, RandomState>>,
    pub(crate) rules: ::std::vec::Vec<Rule<World, RandomState>>,
}

pub struct Rule<World, RandomState: ::core::hash::BuildHasher = ::std::hash::RandomState> {
    pub(crate) description: ::core::option::Option<::std::borrow::Cow<'static, str>>,
    pub(crate) ignored: ::core::option::Option<bool>,
    pub(crate) tags: ::core::option::Option<Tags<RandomState>>,

    pub(crate) background: ::core::option::Option<Background<World>>,
    pub(crate) scenarios: ::std::vec::Vec<Scenario<World, RandomState>>,
}

pub struct Scenario<World, RandomState: ::core::hash::BuildHasher = ::std::hash::RandomState> {
    pub(crate) description: ::core::option::Option<::std::borrow::Cow<'static, str>>,
    pub(crate) ignored: ::core::option::Option<bool>,
    pub(crate) tags: ::core::option::Option<Tags<RandomState>>,

    pub(crate) given: ::std::vec::Vec<Step<::std::boxed::Box<dyn FnOnce(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync>>>,
    pub(crate) when: ::std::vec::Vec<Step<Box<dyn FnOnce(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync>>>,
    pub(crate) then: ::std::vec::Vec<Step<Box<dyn FnOnce(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync>>>,
}

pub struct Background<World> {
    pub(crate) description: ::core::option::Option<::std::borrow::Cow<'static, str>>,
    pub(crate) ignored: ::core::option::Option<bool>,

    pub(crate) given: ::std::vec::Vec<Step<::std::rc::Rc<dyn Fn(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync>>>,
}

pub(crate) struct Hook<Callback, RandomState: ::core::hash::BuildHasher = ::std::hash::RandomState> {
    pub(crate) tags: ::core::option::Option<Tags<RandomState>>,
    pub(crate) callback: Callback,
}

pub(crate) struct Step<Callback> {
    pub(crate) label: StepLabel,
    pub(crate) description: ::std::borrow::Cow<'static, str>,
    pub(crate) callback: Callback,
}

#[derive(::core::clone::Clone)]
pub struct Tags<RandomState: ::core::hash::BuildHasher = ::std::hash::RandomState>(pub(crate) ::std::collections::HashSet<::std::borrow::Cow<'static, str>, RandomState>);

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
