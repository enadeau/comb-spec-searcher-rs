use super::{Rule, RuleDB, Strategy};
use crate::errors::SpecificationNotFoundError;
use crate::searcher::classdb;
use crate::CombinatorialSpecification;

mod function;
mod table_method;

pub use function::{Function, IntOrInf};
pub use table_method::TableMethod;

pub struct ForestRuleDB {
    reverse: bool,
    table_method: TableMethod,
}

impl<S: Strategy> RuleDB<S> for ForestRuleDB {
    fn add(&mut self, start: usize, ends: Vec<usize>, rule: Rule<S>) {
        unimplemented!()
    }

    fn get_specification(
        &mut self,
        root: usize,
        classdb: &classdb::ClassDB<S::ClassType>,
    ) -> Result<CombinatorialSpecification<S>, SpecificationNotFoundError> {
        unimplemented!()
    }
}
