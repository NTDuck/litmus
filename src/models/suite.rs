use crate::models::gherkin::{Feature, Hooks};

pub struct Suite<WorldImpl> {
    features: Vec<Feature<WorldImpl>>,
    hooks: Hooks<WorldImpl>,
}
