use ::sealed::sealed;

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

pub struct Background<World> {
    pub(crate) description: ::core::option::Option<::std::borrow::Cow<'static, str>>,
    pub(crate) ignored: ::core::option::Option<bool>,

    pub(crate) given: (
        Step<::std::rc::Rc<dyn Fn() -> Fallible<World>>>,
        ::core::option::Option<Steps<::std::rc::Rc<dyn Fn(&mut World) -> Fallible>>>,
    ),
}

pub struct Scenario<World> {
    pub(crate) description: ::core::option::Option<::std::borrow::Cow<'static, str>>,
    pub(crate) ignored: ::core::option::Option<bool>,
    pub(crate) tags: ::core::option::Option<Tags>,

    pub(crate) given: (
        Step<::std::boxed::Box<dyn FnOnce() -> Fallible<World>>>,
        ::core::option::Option<Steps<::std::boxed::Box<dyn FnOnce(&mut World) -> Fallible>>>,
    ),
    pub(crate) when: Steps<Box<dyn FnOnce(&mut World) -> Fallible>>,
    pub(crate) then: Steps<Box<dyn FnOnce(&World) -> Fallible>>,
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
pub trait IntoFeature<World>: ::core::marker::Sized {
    fn into_feature(self) -> Feature<World>;
}

#[sealed(pub(crate))]
pub trait IntoRule<World>: ::core::marker::Sized {
    fn into_rule(self) -> Rule<World>;
}

#[sealed(pub(crate))]
pub trait IntoBackground<World>: ::core::marker::Sized {
    fn into_background(self) -> Background<World>;
}

#[sealed(pub(crate))]
pub trait IntoScenario<World>: ::core::marker::Sized {
    fn into_scenario(self) -> Scenario<World>;
}

#[sealed(pub(crate))]
pub trait IntoTags: ::core::marker::Sized {
    fn into_tags(self) -> Tags;
}

#[sealed(pub(crate))]
pub trait IntoStep<Callback> {
    fn into_step(self) -> Step<Callback>;
}
