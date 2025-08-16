use crate::utils::aliases::{MaybeOwnedString, Set, Vec};

#[derive(::core::fmt::Debug)]
pub struct Background<Given> {
    pub(crate) description: Option<MaybeOwnedString>,
    pub(crate) ignored: Option<bool>,

    pub(crate) given: Steps<Given>,
}

#[derive(::core::fmt::Debug)]
pub(crate) struct Tags(Set<MaybeOwnedString>);

#[derive(::std::fmt::Debug)]
pub(crate) struct Steps<Callback> {
    pub(crate) labels: Vec<StepLabel>,
    pub(crate) descriptions: Vec<MaybeOwnedString>,
    
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
