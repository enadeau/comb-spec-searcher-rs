use crate::pack::StrategyFactory;
use crate::combinatorial_class::CombinatorialClass;

#[derive(Clone, Debug, PartialEq)]
pub struct AvoidingWithPrefix {
    prefix: String,
    patterns: Vec<String>,
    alphabet: Vec<char>,
}

impl AvoidingWithPrefix {
    pub fn new(prefix: String, patterns: Vec<String>, alphabet: Vec<char>) -> Self {
        Self {
            prefix,
            patterns,
            alphabet,
        }
    }
}

impl CombinatorialClass for AvoidingWithPrefix {
}

pub enum WordStrategyFactory {
    Atom,
    RemoveFrontOfPrefix,
    Expansion,
}

impl StrategyFactory for WordStrategyFactory {
    type ClassType = AvoidingWithPrefix;

    fn apply(&self, class: &AvoidingWithPrefix) {
        todo!();
    }
}
