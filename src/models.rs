use ::sealed::sealed;

pub struct Suite<World, RandomState: ::core::hash::BuildHasher = ::std::hash::RandomState> {
    pub(crate) features: ::std::vec::Vec<Feature<World, RandomState>>,
}

pub struct Feature<World, RandomState: ::core::hash::BuildHasher = ::std::hash::RandomState> {
    pub(crate) description: ::core::option::Option<::std::borrow::Cow<'static, str>>,
    pub(crate) ignored: ::core::option::Option<bool>,
    pub(crate) tags: ::core::option::Option<Tags<RandomState>>,

    pub(crate) background: ::core::option::Option<Background<World>>,
    pub(crate) scenarios: ::std::vec::Vec<Scenario<World>>,
    pub(crate) rules: ::std::vec::Vec<Rule<World>>,
}

pub struct Rule<World, RandomState: ::core::hash::BuildHasher = ::std::hash::RandomState> {
    pub(crate) description: ::core::option::Option<::std::borrow::Cow<'static, str>>,
    pub(crate) ignored: ::core::option::Option<bool>,
    pub(crate) tags: ::core::option::Option<Tags<RandomState>>,

    pub(crate) background: ::core::option::Option<Background<World>>,
    pub(crate) scenarios: ::std::vec::Vec<Scenario<World>>,
}

pub struct Scenario<World, RandomState: ::core::hash::BuildHasher = ::std::hash::RandomState> {
    pub(crate) description: ::core::option::Option<::std::borrow::Cow<'static, str>>,
    pub(crate) ignored: ::core::option::Option<bool>,
    pub(crate) tags: ::core::option::Option<Tags<RandomState>>,

    pub(crate) given: (
        Step<::std::boxed::Box<dyn FnOnce() -> Fallible<World> + ::core::marker::Send + ::core::marker::Sync>>,
        ::core::option::Option<Steps<::std::boxed::Box<dyn FnOnce(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync>>>,
    ),
    pub(crate) when: Steps<Box<dyn FnOnce(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync>>,
    pub(crate) then: Steps<Box<dyn FnOnce(&World) -> Fallible + ::core::marker::Send + ::core::marker::Sync>>,
}

pub struct Background<World> {
    pub(crate) description: ::core::option::Option<::std::borrow::Cow<'static, str>>,
    pub(crate) ignored: ::core::option::Option<bool>,

    pub(crate) given: (
        Step<::std::rc::Rc<dyn Fn() -> Fallible<World> + ::core::marker::Send + ::core::marker::Sync>>,
        ::core::option::Option<Steps<::std::rc::Rc<dyn Fn(&mut World) -> Fallible + ::core::marker::Send + ::core::marker::Sync>>>,
    ),
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
pub trait IntoFeature<World, RandomState: ::core::hash::BuildHasher = ::std::hash::RandomState>: ::core::marker::Sized {
    fn into_feature(self) -> Feature<World, RandomState>;
}

#[sealed(pub(crate))]
pub trait IntoRule<World, RandomState: ::core::hash::BuildHasher = ::std::hash::RandomState>: ::core::marker::Sized {
    fn into_rule(self) -> Rule<World, RandomState>;
}

#[sealed(pub(crate))]
pub trait IntoScenario<World, RandomState: ::core::hash::BuildHasher = ::std::hash::RandomState>: ::core::marker::Sized {
    fn into_scenario(self) -> Scenario<World, RandomState>;
}

#[sealed(pub(crate))]
pub trait IntoBackground<World>: ::core::marker::Sized {
    fn into_background(self) -> Background<World>;
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