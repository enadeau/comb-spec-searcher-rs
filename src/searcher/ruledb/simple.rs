use super::classdb;
use super::{RuleDB, RuleLabel};
use crate::errors::SpecificationNotFoundError;
use crate::pack::Rule;
use crate::pack::Strategy;
use crate::searcher::equiv_db;
use crate::specification::CombinatorialSpecification;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::{HashMap, HashSet, VecDeque};

pub struct SimpleRuleDB<S: Strategy> {
    rule_to_strategy: HashMap<RuleLabel, S>,
    equiv_db: equiv_db::EquivDB,
}

impl<S: Strategy> SimpleRuleDB<S> {
    pub fn new() -> Self {
        Self {
            rule_to_strategy: HashMap::new(),
            equiv_db: equiv_db::EquivDB::new(),
        }
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

    /// Convert a specification in term of equivalence labels in to
    /// specification in term of actual labels.
    fn eqv_specification_to_specification(
        &mut self,
        eqv_specification_rules: Vec<RuleLabel>,
    ) -> Vec<RuleLabel> {
        let mut children: Vec<usize> = vec![];
        let specification_rules_by_eqv_parent: HashMap<_, _> = eqv_specification_rules
            .into_iter()
            .map(|eqv_rule| {
                let rule = self.find_rule_from_eqv_rule(&eqv_rule).unwrap().clone();
                children.extend(rule.get_children());
                (*eqv_rule.get_parent(), rule)
            })
            .collect();
        let mut specification_rules = HashSet::new();
        for child in children {
            let child_eqv_label = self.equiv_db.find(child);
            let parent_to_connect =
                *specification_rules_by_eqv_parent[&child_eqv_label].get_parent();
            let path = self.equiv_db.find_path(child, parent_to_connect);
            for pair in path.windows(2) {
                specification_rules.insert(RuleLabel {
                    parent: pair[0],
                    children: vec![pair[1]],
                });
            }
        }
        specification_rules.extend(specification_rules_by_eqv_parent.into_values());
        specification_rules.into_iter().collect()
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

impl<S: Strategy> RuleDB<S> for SimpleRuleDB<S> {
    fn add(&mut self, start: usize, ends: Vec<usize>, rule: Rule<S>) {
        if ends.len() == 1 && rule.is_equivalence() {
            self.equiv_db.union(start, ends[0]);
        }
        self.rule_to_strategy
            .insert(RuleLabel::new(start, ends), rule.get_strategy());
    }

    fn get_specification(
        &mut self,
        root: usize,
        classdb: &classdb::ClassDB<S::ClassType>,
    ) -> Result<CombinatorialSpecification<S>, SpecificationNotFoundError> {
        let eqv_specification_rules = self.find_specification(root)?;
        let specification_rules = self.eqv_specification_to_specification(eqv_specification_rules);
        let actual_rules: Vec<Rule<S>> = specification_rules
            .into_iter()
            .map(|rule| {
                let parent = classdb
                    .get_class_from_label(*rule.get_parent())
                    .unwrap()
                    .clone();
                let strategy = self.rule_to_strategy.get(&rule).unwrap().clone();
                Rule::new(parent, strategy)
            })
            .collect();
        Ok(CombinatorialSpecification {
            rules: actual_rules,
            root: classdb.get_class_from_label(root).unwrap().clone(),
        })
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
