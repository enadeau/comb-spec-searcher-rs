use crate::combinatorial_class::CombinatorialClass;
use crate::pack::{Rule, Strategy, StrategyFactory};
use serde::ser::{Serialize, SerializeStruct, Serializer};

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

    pub fn new_just_prefix(prefix: String, patterns: Vec<String>, alphabet: Vec<char>) -> Self {
        Self {
            prefix,
            patterns,
            alphabet,
            just_prefix: true,
        }
    }

    pub fn is_just_prefix(&self) -> bool {
        self.just_prefix
    }
}

impl CombinatorialClass for AvoidingWithPrefix {}

impl Serialize for AvoidingWithPrefix {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("AvoidingWithPrefix", 5)?;
        state.serialize_field("prefix", &self.prefix)?;
        state.serialize_field("patterns", &self.patterns)?;
        state.serialize_field("alphabet", &self.alphabet)?;
        state.serialize_field("just_prefix", &self.just_prefix)?;
        state.serialize_field("class_module", "example")?;
        state.serialize_field("comb_class", "AvoidingWithPrefix")?;
        state.end()
    }
}

#[derive(Debug, Clone)]
pub enum WordStrategy {
    Atom,
    RemoveFrontOfPrefix,
    Expansion,
}

impl StrategyFactory for WordStrategy {
    type ClassType = AvoidingWithPrefix;
    type StrategyType = WordStrategy;

    fn apply(&self, comb_class: &AvoidingWithPrefix) -> Vec<Rule<WordStrategy>> {
        match self {
            WordStrategy::Atom => atom_strategy::apply(comb_class),
            WordStrategy::RemoveFrontOfPrefix => remove_front_of_prefix_strategy::apply(comb_class),
            WordStrategy::Expansion => expansion_strategy::apply(comb_class),
        }
    }
}

impl Strategy for WordStrategy {
    type ClassType = AvoidingWithPrefix;

    fn decompose(&self, comb_class: &Self::ClassType) -> Vec<Self::ClassType> {
        match self {
            WordStrategy::Atom => atom_strategy::decompose(comb_class),
            WordStrategy::RemoveFrontOfPrefix => {
                remove_front_of_prefix_strategy::decompose(comb_class)
            }
            WordStrategy::Expansion => expansion_strategy::decompose(comb_class),
        }
    }

    fn is_equivalence(&self) -> bool {
        match self {
            WordStrategy::Atom => false,
            WordStrategy::RemoveFrontOfPrefix => false,
            WordStrategy::Expansion => true,
        }
    }
}

impl Serialize for WordStrategy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("WordStrategy", 2)?;
        match self {
            WordStrategy::Expansion => {
                state.serialize_field("class_module", "example")?;
                state.serialize_field("strategy_class", "ExpansionStrategy")?;
            }
            WordStrategy::RemoveFrontOfPrefix => {
                state.serialize_field("class_module", "example")?;
                state.serialize_field("strategy_class", "RemoveFrontOfPrefix")?;
            }
            WordStrategy::Atom => {
                state.serialize_field("class_module", "comb_spec_searcher")?;
                state.serialize_field("strategy_class", "AtomStrategy")?;
            }
        }
        state.end()
    }
}

mod atom_strategy {
    use super::{AvoidingWithPrefix, Rule, WordStrategy};
    pub fn apply(comb_class: &AvoidingWithPrefix) -> Vec<Rule<WordStrategy>> {
        let mut res = vec![];
        if comb_class.is_just_prefix() {
            let strategy = WordStrategy::Atom;
            res.push(Rule::new(comb_class.clone(), strategy));
        }
        res
    }

    pub fn decompose(word: &AvoidingWithPrefix) -> Vec<AvoidingWithPrefix> {
        vec![]
    }
}

mod remove_front_of_prefix_strategy {
    use super::{AvoidingWithPrefix, Rule, WordStrategy};
    use std::cmp;

    pub fn apply(comb_class: &AvoidingWithPrefix) -> Vec<Rule<WordStrategy>> {
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
        let mut safe = if word.prefix.len() > m {
            word.prefix.len() - m + 0
        } else {
            0
        };
        for i in safe..word.prefix.len() {
            let end = &word.prefix[i..];
            if word
                .patterns
                .iter()
                .any(|patt| end == &patt[..cmp::min(end.len(), patt.len())])
            {
                break;
            }
            safe = i + 1;
        }
        safe
    }

    pub fn decompose(word: &AvoidingWithPrefix) -> Vec<AvoidingWithPrefix> {
        let safe = removable_prefix_length(word);
        let start_prefix = &word.prefix[..safe];
        let end_prefix = &word.prefix[safe..];
        let start = AvoidingWithPrefix::new_just_prefix(
            start_prefix.to_string(),
            word.patterns.clone(),
            word.alphabet.clone(),
        );
        let end = AvoidingWithPrefix::new(
            end_prefix.to_string(),
            word.patterns.clone(),
            word.alphabet.clone(),
        );
        vec![start, end]
    }
}

mod expansion_strategy {
    use super::{AvoidingWithPrefix, Rule, WordStrategy};
    pub fn apply(word: &AvoidingWithPrefix) -> Vec<Rule<WordStrategy>> {
        let mut res = vec![];
        if !word.is_just_prefix() {
            res.push(Rule::new(word.clone(), WordStrategy::Expansion))
        }
        res
    }

    pub fn decompose(word: &AvoidingWithPrefix) -> Vec<AvoidingWithPrefix> {
        let mut children = vec![AvoidingWithPrefix::new_just_prefix(
            word.prefix.clone(),
            word.patterns.clone(),
            word.alphabet.clone(),
        )];
        for letter in word.alphabet.iter() {
            let mut new_prefix = word.prefix.clone();
            new_prefix.push(*letter);
            children.push(AvoidingWithPrefix::new(
                new_prefix,
                word.patterns.clone(),
                word.alphabet.clone(),
            ));
        }
        children
    }
}
