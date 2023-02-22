use crate::combinatorial_class::CombinatorialClass;
use crate::pack::{Rule, Strategy, StrategyFactory};

#[derive(Clone, Debug, PartialEq)]
pub struct AvoidingWithPrefix {
    prefix: String,
    patterns: Vec<String>,
    alphabet: Vec<char>,
    just_prefix: bool,
}

impl AvoidingWithPrefix {
    pub fn new(prefix: String, patterns: Vec<String>, alphabet: Vec<char>) -> Self {
        Self {
            prefix,
            patterns,
            alphabet,
            just_prefix: false,
        }
    }

    pub fn is_just_prefix(&self) -> bool {
        self.just_prefix
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

    fn apply(
        &self,
        comb_class: &AvoidingWithPrefix,
    ) -> Vec<Rule<AvoidingWithPrefix, WordStrategy>> {
        match self {
            WordStrategy::Atom => AtomStrategy::apply(comb_class),
            WordStrategy::RemoveFrontOfPrefix => RemoveFrontOfPrefixStrategy::apply(comb_class),
            WordStrategy::Expansion => ExpansionStrategy::apply(comb_class),
        }
    }
}

impl Strategy for WordStrategy {
    type ClassType = AvoidingWithPrefix;

    fn apply(&self, comb_class: &AvoidingWithPrefix) -> Rule<AvoidingWithPrefix, WordStrategy> {
        todo!();
    }
}

mod AtomStrategy {
    use super::{AvoidingWithPrefix, Rule, WordStrategy};
    pub fn apply(comb_class: &AvoidingWithPrefix) -> Vec<Rule<AvoidingWithPrefix, WordStrategy>> {
        let mut res = vec![];
        if comb_class.is_just_prefix() {
            let strategy = WordStrategy::Atom;
            res.push(Rule::new(comb_class.clone(), strategy));
        }
        res
    }
}

mod RemoveFrontOfPrefixStrategy {
    use super::{AvoidingWithPrefix, Rule, WordStrategy};
    use std::cmp;

    pub fn apply(comb_class: &AvoidingWithPrefix) -> Vec<Rule<AvoidingWithPrefix, WordStrategy>> {
        let mut res = vec![];
        if !comb_class.is_just_prefix() {
            let safe = removable_prefix_length(comb_class);
            if safe > 0 {
                res.push(Rule::new(
                    comb_class.clone(),
                    WordStrategy::RemoveFrontOfPrefix,
                ));
            }
        }
        res
    }

    fn removable_prefix_length(word: &AvoidingWithPrefix) -> usize {
        let m = word.patterns.iter().map(|s| s.len()).max().unwrap_or(1);
        let mut safe = cmp::max(0, word.prefix.len() - m + 1);
        for i in safe..word.prefix.len() {
            let end = &word.prefix[i..];
            if word.patterns.iter().any(|patt| end == &patt[..end.len()]) {
                break;
            }
            safe = i + 1;
        }
        safe
    }
}

mod ExpansionStrategy {
    use super::{AvoidingWithPrefix, Rule, WordStrategy};
    pub fn apply(comb_class: &AvoidingWithPrefix) -> Vec<Rule<AvoidingWithPrefix, WordStrategy>> {
        vec![Rule::new(comb_class.clone(), WordStrategy::Expansion)]
    }
}
