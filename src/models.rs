use ::sealed::sealed;

pub struct Suite<World, RandomState: ::core::hash::BuildHasher = ::std::hash::RandomState> {
    pub(crate) features: ::std::vec::Vec<Feature<World, RandomState>>,

    pub(crate) before_scenario_hooks: Hooks<::std::rc::Rc<dyn Fn(&mut World) + ::core::marker::Send + ::core::marker::Sync>, RandomState>,
    pub(crate) after_scenario_hooks: Hooks<::std::rc::Rc<dyn Fn(&mut World) + ::core::marker::Send + ::core::marker::Sync>, RandomState>,

    pub(crate) before_step_hooks: Hooks<::std::rc::Rc<dyn Fn(&mut World) + ::core::marker::Send + ::core::marker::Sync>, RandomState>,
    pub(crate) after_step_hooks: Hooks<::std::rc::Rc<dyn Fn(&mut World) + ::core::marker::Send + ::core::marker::Sync>, RandomState>,

    pub(crate) before_global_hooks: Hooks<::std::boxed::Box<dyn FnOnce() + ::core::marker::Send + ::core::marker::Sync>, RandomState>,
    pub(crate) after_global_hooks: Hooks<::std::boxed::Box<dyn FnOnce() + ::core::marker::Send + ::core::marker::Sync>, RandomState>,
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

    pub(crate) given: Steps<::std::boxed::Box<dyn FnOnce(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync>>,
    pub(crate) when: Steps<Box<dyn FnOnce(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync>>,
    pub(crate) then: Steps<Box<dyn FnOnce(&World) -> Fallible + ::core::marker::Send + ::core::marker::Sync>>,
}

pub struct Background<World> {
    pub(crate) description: ::core::option::Option<::std::borrow::Cow<'static, str>>,
    pub(crate) ignored: ::core::option::Option<bool>,

    pub(crate) given: Steps<::std::rc::Rc<dyn Fn(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync>>,
}

#[derive(::core::fmt::Debug)]
pub(crate) struct Hooks<Callback, RandomState: ::core::hash::BuildHasher = ::std::hash::RandomState>(pub(crate) ::std::vec::Vec<Hook<Callback, RandomState>>);

#[derive(::core::fmt::Debug)]
pub(crate) struct Hook<Callback, RandomState: ::core::hash::BuildHasher = ::std::hash::RandomState> {
    pub(crate) description: ::std::borrow::Cow<'static, str>,
    pub(crate) tags: Tags<RandomState>,
    pub(crate) callback: Callback,
}

#[derive(::core::fmt::Debug)]
pub struct Tags<RandomState: ::core::hash::BuildHasher = ::std::hash::RandomState>(pub(crate) ::std::collections::HashSet<::std::borrow::Cow<'static, str>, RandomState>);

#[derive(::core::fmt::Debug)]
pub(crate) struct Steps<Callback>(pub(crate) ::std::vec::Vec<Step<Callback>>);

#[derive(::core::fmt::Debug)]
pub(crate) struct Step<Callback> {
    pub(crate) label: StepLabel,
    pub(crate) description: ::std::borrow::Cow<'static, str>,
    pub(crate) callback: Callback,
}

#[derive(::core::fmt::Debug)]
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

#[sealed(pub(crate))]
pub trait IntoSuite<World, RandomState: ::core::hash::BuildHasher = ::std::hash::RandomState>: ::core::marker::Sized {
    fn into_suite(self) -> Suite<World, RandomState>;
}

#[sealed]
impl<World, RandomState: ::core::hash::BuildHasher> IntoSuite<World, RandomState> for Suite<World, RandomState> {
    fn into_suite(self) -> Suite<World, RandomState> {
        self
    }
}

#[sealed(pub(crate))]
pub trait IntoFeature<World, RandomState: ::core::hash::BuildHasher = ::std::hash::RandomState>: ::core::marker::Sized {
    fn into_feature(self) -> Feature<World, RandomState>;
}

#[sealed]
impl<World, RandomState: ::core::hash::BuildHasher> IntoFeature<World, RandomState> for Feature<World, RandomState> {
    fn into_feature(self) -> Feature<World, RandomState> {
        self
    }
}

#[sealed(pub(crate))]
pub trait IntoRule<World, RandomState: ::core::hash::BuildHasher = ::std::hash::RandomState>: ::core::marker::Sized {
    fn into_rule(self) -> Rule<World, RandomState>;
}

#[sealed]
impl<World, RandomState: ::core::hash::BuildHasher> IntoRule<World, RandomState> for Rule<World, RandomState> {
    fn into_rule(self) -> Rule<World, RandomState> {
        self
    }
}

#[sealed(pub(crate))]
pub trait IntoScenario<World, RandomState: ::core::hash::BuildHasher = ::std::hash::RandomState>: ::core::marker::Sized {
    fn into_scenario(self) -> Scenario<World, RandomState>;
}

#[sealed]
impl<World, RandomState: ::core::hash::BuildHasher> IntoScenario<World, RandomState> for Scenario<World, RandomState> {
    fn into_scenario(self) -> Scenario<World, RandomState> {
        self
    }
}

#[sealed(pub(crate))]
pub trait IntoBackground<World>: ::core::marker::Sized {
    fn into_background(self) -> Background<World>;
}

#[sealed]
impl<World> IntoBackground<World> for Background<World> {
    fn into_background(self) -> Background<World> {
        self
    }
}

#[sealed(pub(crate))]
pub trait IntoTags<RandomState: ::core::hash::BuildHasher = ::std::hash::RandomState>: ::core::marker::Sized {
    fn into_tags(self) -> Tags<RandomState>;
}

#[sealed]
impl<RandomState: ::core::hash::BuildHasher> IntoTags<RandomState> for Tags<RandomState> {
    fn into_tags(self) -> Tags<RandomState> {
        self
    }
}
