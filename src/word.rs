use crate::combinatorial_class::CombinatorialClass;
use crate::pack::{Rule, Strategy, StrategyFactory};

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

impl CombinatorialClass for AvoidingWithPrefix {}

#[derive(Debug)]
pub enum WordStrategy {
    Atom,
    RemoveFrontOfPrefix,
    Expansion,
}

impl StrategyFactory for WordStrategy {
    type ClassType = AvoidingWithPrefix;
    type StrategyType = WordStrategy;

    fn apply(&self, comb_class: &AvoidingWithPrefix) -> Vec<WordStrategy> {
        todo!();
    }
}

impl Strategy for WordStrategy {
    type ClassType = AvoidingWithPrefix;

    fn apply(&self, comb_class: &AvoidingWithPrefix) -> Rule<AvoidingWithPrefix, WordStrategy> {
        todo!();
    }
}
