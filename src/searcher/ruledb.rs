use crate::pack::Rule;
use crate::pack::Strategy;
use crate::searcher::equiv_db;
use std::collections::HashMap;

pub struct RuleDB<S: Strategy> {
    rule_to_strategy: HashMap<(usize, Vec<usize>), S>,
    equiv_db: equiv_db::EquivDB,
}

impl<S: Strategy> RuleDB<S> {
    pub fn new() -> Self {
        Self {
            rule_to_strategy: HashMap::new(),
            equiv_db: equiv_db::EquivDB::new(),
        }
    }

    pub fn add(&mut self, start: usize, mut ends: Vec<usize>, rule: Rule<S>) {
        ends.sort();
        if ends.len() == 1 && rule.is_equivalence() {
            self.equiv_db.union(start, ends[0]);
        }
        self.rule_to_strategy
            .insert((start, ends), rule.get_strategy());
    }
}
