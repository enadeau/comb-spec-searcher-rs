use crate::errors::SpecificationNotFoundError;
use crate::pack::Rule;
use crate::pack::Strategy;
use crate::searcher::equiv_db;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::{HashMap, HashSet, VecDeque};

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

    pub fn get_specification_rules(&mut self, root: usize) -> Vec<Rule<S>> {
        let s = self.find_specification(root).expect("NO spec found");
        todo!();
    }

    fn find_specification(
        &mut self,
        label: usize,
    ) -> Result<Vec<LabelRule>, SpecificationNotFoundError> {
        let rules = self.rule_up_to_equivalence();
        let rules = prune(rules);
        random_proof_tree(&rules, label)
    }

    fn rule_up_to_equivalence(&mut self) -> HashMap<usize, HashSet<Vec<usize>>> {
        let mut rules = HashMap::new();
        for ((start, ends), _) in self.rule_to_strategy.iter() {
            let start = self.equiv_db.find(*start);
            let mut ends: Vec<usize> = ends.iter().map(|e| self.equiv_db.find(*e)).collect();
            ends.sort();
            rules.entry(start).or_insert(HashSet::new()).insert(ends);
        }
        rules
    }
}

fn prune(mut rules: HashMap<usize, HashSet<Vec<usize>>>) -> HashMap<usize, HashSet<Vec<usize>>> {
    let mut changed = true;
    let mut keys: HashSet<_> = rules.keys().cloned().collect();
    while changed {
        changed = false;
        for (start, ends_set) in rules.iter_mut() {
            ends_set.retain(|ends| ends.iter().all(|e| keys.contains(e)));
            if ends_set.is_empty() {
                keys.remove(start);
                changed = true;
            }
        }
        rules.retain(|k, _| keys.contains(k));
    }
    rules
}

#[derive(Debug)]
struct LabelRule {
    parent: usize,
    children: Vec<usize>,
}

fn random_proof_tree(
    rules: &HashMap<usize, HashSet<Vec<usize>>>,
    root: usize,
) -> Result<Vec<LabelRule>, SpecificationNotFoundError> {
    let mut seen = HashSet::new();
    let mut queue = VecDeque::new();
    let mut res = vec![];
    let mut rng = thread_rng();
    queue.push_back(root);
    while let Some(parent) = queue.pop_front() {
        if seen.contains(&parent) {
            continue;
        } else {
            seen.insert(parent);
        }
        let children_list: Vec<_> = rules
            .get(&parent)
            .ok_or(SpecificationNotFoundError {})?
            .iter()
            .collect();
        let children = children_list
            .choose(&mut rng)
            .ok_or(SpecificationNotFoundError {})?;
        queue.extend(children.iter());
        res.push(LabelRule {
            parent,
            children: (*children).clone(),
        });
    }
    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prune_verification_rule_test() {
        let mut rules = HashMap::new();
        rules.insert(0, HashSet::from_iter(vec![vec![]]));
        let rules = prune(rules);
        assert_eq!(rules.len(), 1);
    }

    #[test]
    fn prune_simple_tree_test() {
        let mut rules = HashMap::new();
        rules.insert(0, HashSet::from_iter(vec![vec![1, 2]]));
        rules.insert(1, HashSet::from_iter(vec![vec![]]));
        rules.insert(2, HashSet::from_iter(vec![vec![]]));
        let rules = prune(rules);
        assert_eq!(rules.len(), 3);
    }

    #[test]
    fn prune_nothing_test() {
        let mut rules = HashMap::new();
        rules.insert(0, HashSet::from_iter(vec![vec![1, 2]]));
        rules.insert(4, HashSet::from_iter(vec![vec![]]));
        rules.insert(2, HashSet::from_iter(vec![vec![]]));
        let rules = prune(rules);
        assert_eq!(rules.len(), 2);
        assert!(!rules.contains_key(&0));
    }
}
