use crate::pack::{StrategyFactory, StrategyPack};
use std::collections::{HashSet, VecDeque};

#[derive(Debug, PartialEq)]
pub struct WorkPacket<'a, F: StrategyFactory> {
    pub class_label: usize,
    pub factory: &'a F,
}

pub struct ClassQueue<F: StrategyFactory> {
    pack: StrategyPack<F>,
    queue: VecDeque<usize>,
    curr_label: usize,
    strat_index: usize,
    max_strat_index: usize,
    ignore: HashSet<usize>,
}

impl<F: StrategyFactory> ClassQueue<F> {
    pub fn new(pack: StrategyPack<F>, start_label: usize) -> Self {
        let pack_size = pack.len();
        Self {
            pack,
            queue: VecDeque::new(),
            curr_label: start_label,
            strat_index: 0,
            max_strat_index: pack_size,
            ignore: HashSet::new(),
        }
    }

    pub fn add(&mut self, label: usize) {
        self.queue.push_back(label);
    }

    pub fn ignore(&mut self, label: usize) {
        self.ignore.insert(label);
    }

    pub fn next(&mut self) -> Option<WorkPacket<F>> {
        loop {
            let next = self.next_no_ignore()?;
            if !self.ignore.contains(&next.0) {
                return Some(WorkPacket {
                    class_label: next.0,
                    factory: self.pack.get_strategy_factory(next.1),
                });
            }
        }
    }

    /// Return the next logical work packet
    fn next_no_ignore(&mut self) -> Option<(usize, usize)> {
        if self.max_strat_index == self.strat_index {
            self.ignore(self.curr_label);
            self.strat_index = 1;
            self.curr_label = self.queue.pop_front()?;
        } else {
            self.strat_index += 1;
        }
        Some((self.curr_label, self.strat_index - 1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::combinatorial_class::CombinatorialClass;
    use crate::pack::{Rule, Strategy, StrategyPack};

    #[derive(Debug, PartialEq, Clone)]
    struct MockClass {}

    impl CombinatorialClass for MockClass {}

    #[derive(Debug, PartialEq, Clone)]
    enum MockStrategy {
        Inferral1,
        Inferral2,
        Initial1,
        Initial2,
        Expansion1,
        Expansion2,
        Verification1,
        Verification2,
    }

    impl Strategy for MockStrategy {
        type ClassType = MockClass;

        fn decompose(&self, comb_class: &MockClass) -> Vec<MockClass> {
            unimplemented!();
        }

        fn is_equivalence(&self) -> bool {
            unimplemented!();
        }
    }

    impl StrategyFactory for MockStrategy {
        type ClassType = MockClass;
        type StrategyType = MockStrategy;

        fn apply(&self, class: &MockClass) -> Vec<Rule<MockStrategy>> {
            unimplemented!();
        }
    }

    fn pack() -> StrategyPack<MockStrategy> {
        StrategyPack {
            initials: vec![MockStrategy::Initial1, MockStrategy::Initial2],
            inferrals: vec![MockStrategy::Inferral1, MockStrategy::Inferral2],
            expansions: vec![MockStrategy::Expansion1, MockStrategy::Expansion2],
            verifications: vec![MockStrategy::Verification1, MockStrategy::Verification2],
        }
    }

    #[test]
    /// Test that the strategies are yielded in the right order for a single class
    fn queue_basic_one_class_test() {
        let mut queue = ClassQueue::new(pack(), 0);
        let expected_factory = [
            MockStrategy::Verification1,
            MockStrategy::Verification2,
            MockStrategy::Inferral1,
            MockStrategy::Inferral2,
            MockStrategy::Initial1,
            MockStrategy::Initial2,
            MockStrategy::Expansion1,
            MockStrategy::Expansion2,
        ];
        for factory in expected_factory {
            let wp = queue.next().unwrap();
            assert_eq!(wp.class_label, 0);
            assert_eq!(wp.factory, &factory);
        }
        assert_eq!(queue.next(), None);
    }
}
