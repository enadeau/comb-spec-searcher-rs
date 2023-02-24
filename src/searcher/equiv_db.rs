use std::cmp;
/// Equivalence DB that keeps track of equivalence set
/// using a union find datastructure.
///
/// Implemented for the python version before introducing the notion of one way an two way rule.
/// Specifically this commit.
/// https://github.com/PermutaTriangle/comb_spec_searcher/blob/c4e169e057bb54dd2453901e668dcaf5fc358e90/comb_spec_searcher/equiv_db.py
use std::collections::HashMap;

#[derive(Debug)]
pub struct EquivDB {
    parent: HashMap<usize, usize>,
    weight: HashMap<usize, usize>,
}

impl EquivDB {
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
