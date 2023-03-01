use crate::errors::SpecificationNotFoundError;
use crate::pack::Rule;
use crate::pack::Strategy;
use crate::searcher::equiv_db;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::{HashMap, HashSet, VecDeque};

pub struct RuleDB<S: Strategy> {
    rule_to_strategy: HashMap<RuleLabel, S>,
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
        if ends.len() == 1 && rule.is_equivalence() {
            self.equiv_db.union(start, ends[0]);
        }
        self.rule_to_strategy
            .insert(RuleLabel::new(start, ends), rule.get_strategy());
    }

    pub fn get_specification_rules(&mut self, root: usize) -> Vec<Rule<S>> {
        let s = self.find_specification(root).expect("NO spec found");
        todo!();
    }

    /// Find a specification in term of the equivalence label
    fn find_specification(
        &mut self,
        label: usize,
    ) -> Result<Vec<RuleLabel>, SpecificationNotFoundError> {
        let rules = self.rule_up_to_equivalence();
        let rules = prune(rules);
        random_proof_tree(&rules, label)
    }

    fn find_rule_from_eqv_rule(&mut self, eqv_rule: &RuleLabel) -> Option<&RuleLabel> {
        for rule in self.rule_to_strategy.keys() {
            if *eqv_rule == self.equiv_db.rule_up_to_equivalence(rule) {
                return Some(rule);
            }
        }
        None
    }

    fn rule_up_to_equivalence(&mut self) -> HashSet<RuleLabel> {
        let mut eqv_rules = HashSet::new();
        for rule in self.rule_to_strategy.keys() {
            let eqv_rule = RuleLabel::new(
                self.equiv_db.find(*rule.get_parent()),
                rule.get_children()
                    .iter()
                    .map(|e| self.equiv_db.find(*e))
                    .collect(),
            );
            eqv_rules.insert(eqv_rule);
        }
        eqv_rules
    }
}

fn prune(rules: HashSet<RuleLabel>) -> HashMap<usize, Vec<RuleLabel>> {
    let mut rules_by_parent = rules.into_iter().fold(HashMap::new(), |mut map, rule| {
        map.entry(*rule.get_parent())
            .or_insert_with(Vec::new)
            .push(rule);
        map
    });
    let mut keys: HashSet<_> = rules_by_parent.keys().cloned().collect();
    let mut changed = true;
    while changed {
        changed = false;
        for (parent, rules_for_parent) in rules_by_parent.iter_mut() {
            rules_for_parent.retain(|rule| rule.get_children().iter().all(|e| keys.contains(e)));
            if rules_for_parent.is_empty() {
                keys.remove(parent);
                changed = true;
            }
        }
        rules_by_parent.retain(|k, _| keys.contains(k));
    }
    rules_by_parent
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

fn random_proof_tree(
    rules_by_parent: &HashMap<usize, Vec<RuleLabel>>,
    root: usize,
) -> Result<Vec<RuleLabel>, SpecificationNotFoundError> {
    let mut seen = HashSet::new();
    let mut queue = VecDeque::new();
    let mut proof_tree_rules: Vec<RuleLabel> = vec![];
    let mut rng = thread_rng();
    queue.push_back(root);
    while let Some(parent) = queue.pop_front() {
        if seen.contains(&parent) {
            continue;
        } else {
            seen.insert(parent);
        }
        let rules_for_parent = rules_by_parent
            .get(&parent)
            .ok_or(SpecificationNotFoundError {})?;
        let chosen_rule_for_parent = rules_for_parent
            .choose(&mut rng)
            .ok_or(SpecificationNotFoundError {})?;
        queue.extend(chosen_rule_for_parent.get_children().iter());
        proof_tree_rules.push(chosen_rule_for_parent.clone());
    }
    Ok(proof_tree_rules)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prune_verification_rule_test() {
        let mut rules = HashSet::new();
        rules.insert(RuleLabel::new(0, vec![]));
        let rules = prune(rules);
        assert_eq!(rules.len(), 1);
    }

    #[test]
    fn prune_simple_tree_test() {
        let mut rules = HashSet::new();
        rules.insert(RuleLabel::new(0, vec![1, 2]));
        rules.insert(RuleLabel::new(1, vec![]));
        rules.insert(RuleLabel::new(2, vec![]));
        let rules = prune(rules);
        assert_eq!(rules.len(), 3);
    }

    #[test]
    fn prune_nothing_test() {
        let mut rules = HashSet::new();
        rules.insert(RuleLabel::new(0, vec![1, 2]));
        rules.insert(RuleLabel::new(2, vec![]));
        rules.insert(RuleLabel::new(4, vec![]));
        let rules = prune(rules);
        assert_eq!(rules.len(), 2);
        assert!(!rules.contains_key(&0));
    }
}
