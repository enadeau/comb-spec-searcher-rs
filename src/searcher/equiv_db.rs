use super::ruledb::RuleLabel;
use pathfinding::prelude::bfs;
use std::cmp;

/// Equivalence DB that keeps track of equivalence set
/// using a union find datastructure.
///
/// Implemented for the python version before introducing the notion of one way an two way rule.
/// Specifically this commit.
/// https://github.com/PermutaTriangle/comb_spec_searcher/blob/c4e169e057bb54dd2453901e668dcaf5fc358e90/comb_spec_searcher/equiv_db.py
use std::collections::HashMap;

#[derive(Debug)]
struct UnionFind {
    parent: HashMap<usize, usize>,
    weight: HashMap<usize, usize>,
}

impl UnionFind {
    pub fn new() -> Self {
        Self {
            parent: HashMap::new(),
            weight: HashMap::new(),
        }
    }

    pub fn find(&mut self, label: usize) -> usize {
        match self.parent.get(&label) {
            None => {
                self.parent.insert(label, label);
                self.weight.insert(label, 1);
                label
            }
            Some(&(mut root)) => {
                let mut path = vec![label];
                let mut last = label;
                // Find the path to the root
                while root != last {
                    path.push(root);
                    last = root;
                    root = *self.parent.get(&root).expect("Broken chain in union find");
                }
                // Perform path compression
                for ancestor in path.into_iter() {
                    self.parent.insert(ancestor, root);
                }
                root
            }
        }
    }

    pub fn union(&mut self, label1: usize, label2: usize) {
        let (root1, root2) = (self.find(label1), self.find(label2));
        let heaviest = cmp::max_by_key(root1, root2, |r| self.weight.get(r).unwrap());
        let lightest = cmp::min_by_key(root1, root2, |r| self.weight.get(r).unwrap());
        *self.weight.get_mut(&heaviest).unwrap() += *self.weight.get(&lightest).unwrap();
        self.parent.insert(lightest, heaviest);
    }

    fn are_equivalent(&mut self, label1: usize, label2: usize) -> bool {
        self.find(label1) == self.find(label2)
    }
}

#[derive(Debug)]
pub struct EquivDB {
    union_find: UnionFind,
    edges: Vec<(usize, usize)>,
}

impl EquivDB {
    pub fn new() -> Self {
        Self {
            union_find: UnionFind::new(),
            edges: Vec::new(),
        }
    }

    pub fn find(&mut self, label: usize) -> usize {
        self.union_find.find(label)
    }

    pub fn union(&mut self, label1: usize, label2: usize) {
        self.edges.push((label1, label2));
        self.union_find.union(label1, label2)
    }

    /// Convert the given rule to it's equivalence label version
    pub fn rule_up_to_equivalence(&mut self, rule: &RuleLabel) -> RuleLabel {
        let eqv_parent = self.find(*rule.get_parent());
        let eqv_children: Vec<_> = rule.get_children().iter().map(|c| self.find(*c)).collect();
        RuleLabel::new(eqv_parent, eqv_children)
    }

    /// Find a sequence equivalence path between to class
    pub fn find_path(&mut self, start: usize, end: usize) -> Vec<usize> {
        let connected_component: Vec<_> = self
            .edges
            .iter()
            .filter(|(v1, _)| self.union_find.are_equivalent(*v1, start))
            .collect();
        bfs(
            &start,
            |v| {
                connected_component
                    .iter()
                    .filter(|(v1, v2)| v1 == v || v2 == v)
                    .map(|(v1, v2)| if v1 == v { *v2 } else { *v1 })
                    .collect::<Vec<_>>()
            },
            |v| *v == end,
        )
        .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equiv_db_test() {
        let mut db = EquivDB::new();
        assert_eq!(1, db.find(1));
        assert_eq!(2, db.find(2));
        assert_eq!(3, db.find(3));
        assert_eq!(4, db.find(4));
        db.union(1, 3);
        println!("{:?}", db);
        assert_eq!(2, db.find(2));
        assert_eq!(db.find(1), db.find(3));
        assert!(db.find(1) == 1 || db.find(1) == 3);
        assert_eq!(4, db.find(4));
        db.union(2, 4);
        assert_eq!(db.find(1), db.find(3));
        assert_eq!(db.find(2), db.find(4));
        db.union(1, 2);
        assert_eq!(db.find(1), db.find(2));
        assert_eq!(db.find(2), db.find(3));
        assert_eq!(db.find(3), db.find(4));
    }
}
