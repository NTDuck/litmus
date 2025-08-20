use std::rc::Rc;

use ::sealed::sealed;

use crate::utils::aliases::{borrow::MaybeOwnedString, collections::Set};

#[derive(::core::fmt::Debug)]
pub struct Suite {

}

#[derive(::core::fmt::Debug)]
pub struct Feature {

}

#[derive(::core::fmt::Debug)]
pub struct Rule {

}

pub struct Background<World> {
    pub(crate) description: Option<MaybeOwnedString>,
    pub(crate) ignored: Option<bool>,

    pub(crate) given: (
        Step<Rc<dyn FnOnce() -> Fallible<World>>>,
        Steps<Rc<dyn FnOnce(&mut World) -> Fallible>>,
    ),
}

pub struct Scenario<World> {
    pub(crate) description: Option<MaybeOwnedString>,
    pub(crate) ignored: Option<bool>,
    pub(crate) tags: Option<Tags>,

    pub(crate) given: (
        Step<Box<dyn FnOnce() -> Fallible<World>>>,
        Steps<Box<dyn FnOnce(&mut World) -> Fallible>>,
    ),
    pub(crate) when: Steps<Box<dyn FnOnce(&mut World) -> Fallible>>,
    pub(crate) then: Steps<Box<dyn FnOnce(&World) -> Fallible>>,
}

#[derive(::core::fmt::Debug)]
pub struct Tags(pub(crate) Set<MaybeOwnedString>);

#[derive(::core::fmt::Debug)]
pub(crate) struct Steps<Callback: ::core::marker::Sized>(pub(crate) Vec<Step<Callback>>);

#[derive(::core::fmt::Debug)]
pub(crate) struct Step<Callback: ::core::marker::Sized> {
    pub(crate) label: StepLabel,
    pub(crate) description: MaybeOwnedString,

    pub(crate) callback: Callback,
}

#[derive(::core::fmt::Debug, ::core::clone::Clone)]
pub(crate) enum StepLabel {
    Given,
    When,
    Then,

    And,
    But,
}

pub type Fallible<T = ()> = ::core::result::Result<T, Failed>;

pub struct Failed {
    pub message: MaybeOwnedString,
}

#[sealed(pub(crate))]
pub trait IntoBackground<Given>: ::core::marker::Sized {
    fn into_background(self) -> Background<Given>;
}

#[sealed(pub(crate))]
pub trait IntoScenario<World>: ::core::marker::Sized {
    fn into_scenario(self) -> Scenario<World>;
}

#[sealed(pub(crate))]
pub trait IntoTags: ::core::marker::Sized {
    fn into_tags(self) -> Tags;
}
