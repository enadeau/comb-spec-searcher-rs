use crate::pack::{Rule, Strategy};

pub struct CombinatorialSpecification<S: Strategy> {
    pub rules: Vec<Rule<S>>,
    pub root: S::ClassType,
}
