use super::classdb;
use crate::errors::SpecificationNotFoundError;
use crate::pack::Rule;
use crate::pack::Strategy;
use crate::specification::CombinatorialSpecification;

mod simple;
pub use simple::SimpleRuleDB;
mod forest;
pub use forest::ForestRuleDB;

pub trait RuleDB<S: Strategy> {
    fn get_specification(
        &mut self,
        root: usize,
        classdb: &classdb::ClassDB<S::ClassType>,
    ) -> Result<CombinatorialSpecification<S>, SpecificationNotFoundError>;

    fn add(&mut self, start: usize, ends: Vec<usize>, rule: Rule<S>);
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct RuleLabel {
    parent: usize,
    children: Vec<usize>,
}

impl RuleLabel {
    pub fn new(parent: usize, mut children: Vec<usize>) -> Self {
        children.sort();
        Self { parent, children }
    }

    pub fn get_parent(&self) -> &usize {
        &self.parent
    }

    pub fn get_children(&self) -> &Vec<usize> {
        &self.children
    }
}
