use crate::pack::{StrategyFactory, StrategyPack};
use std::collections::{HashSet, VecDeque};

#[derive(Debug, PartialEq)]
pub struct WorkPacket<'a, F: StrategyFactory> {
    pub class_label: usize,
    pub factory: &'a F,
}

#[derive(Debug, PartialEq)]
struct WorkPacketInternal {
    class_label: usize,
    factory_index: usize,
}

impl WorkPacketInternal {
    fn make_external<F: StrategyFactory>(self, pack: &StrategyPack<F>) -> WorkPacket<F> {
        WorkPacket {
            class_label: self.class_label,
            factory: pack.get_strategy_factory(self.factory_index),
        }
    }
}

pub struct ClassQueue<F: StrategyFactory> {
    pack: StrategyPack<F>,
    verification_queue: VecDeque<WorkPacketInternal>,
    inferral_queue: VecDeque<WorkPacketInternal>,
    initial_queue: VecDeque<WorkPacketInternal>,
    expansion_queue: VecDeque<WorkPacketInternal>,
    ignore: HashSet<usize>, // Classes that should not be yielded anymore
    added: HashSet<usize>,  // Classes already added to the queue
}

impl<F: StrategyFactory> ClassQueue<F> {
    pub fn new(pack: StrategyPack<F>, start_label: usize) -> Self {
        let mut queue = Self {
            pack,
            verification_queue: VecDeque::new(),
            inferral_queue: VecDeque::new(),
            initial_queue: VecDeque::new(),
            expansion_queue: VecDeque::new(),
            ignore: HashSet::new(),
            added: HashSet::new(),
        };
        queue.add(start_label);
        queue
    }

    pub fn add(&mut self, class_label: usize) {
        if !self.added.insert(class_label) {
            return;
        }
        let mut factory_index = 0;
        while let Some(_) = self.pack.verifications.get(factory_index) {
            self.verification_queue.push_back(WorkPacketInternal {
                class_label,
                factory_index,
            });
            factory_index += 1;
        }
        let mut cum = factory_index;
        while let Some(_) = self.pack.inferrals.get(factory_index - cum) {
            self.inferral_queue.push_back(WorkPacketInternal {
                class_label,
                factory_index,
            });
            factory_index += 1;
        }
        cum = factory_index;
        while let Some(_) = self.pack.initials.get(factory_index - cum) {
            self.initial_queue.push_back(WorkPacketInternal {
                class_label,
                factory_index,
            });
            factory_index += 1;
        }
        cum = factory_index;
        while let Some(_) = self.pack.expansions.get(factory_index - cum) {
            self.expansion_queue.push_back(WorkPacketInternal {
                class_label,
                factory_index,
            });
            factory_index += 1;
        }
    }

    pub fn ignore(&mut self, label: usize) {
        self.ignore.insert(label);
    }

    pub fn next(&mut self) -> Option<WorkPacket<F>> {
        loop {
            let next = self.next_no_ignore()?;
            if !self.ignore.contains(&next.class_label) {
                return Some(next.make_external(&self.pack));
            }
        }
    }

    /// Return the next logical work packet
    fn next_no_ignore(&mut self) -> Option<WorkPacketInternal> {
        if let Some(wp) = self.verification_queue.pop_front() {
            return Some(wp);
        } else if let Some(wp) = self.inferral_queue.pop_front() {
            return Some(wp);
        } else if let Some(wp) = self.initial_queue.pop_front() {
            return Some(wp);
        }
        self.expansion_queue.pop_front()
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

    #[test]
    fn queue_basic_two_classes_test() {
        let mut queue = ClassQueue::new(pack(), 0);
        queue.add(1);
        let expected_wps = [
            WorkPacket {
                class_label: 0,
                factory: &MockStrategy::Verification1,
            },
            WorkPacket {
                class_label: 0,
                factory: &MockStrategy::Verification2,
            },
            WorkPacket {
                class_label: 1,
                factory: &MockStrategy::Verification1,
            },
            WorkPacket {
                class_label: 1,
                factory: &MockStrategy::Verification2,
            },
            WorkPacket {
                class_label: 0,
                factory: &MockStrategy::Inferral1,
            },
            WorkPacket {
                class_label: 0,
                factory: &MockStrategy::Inferral2,
            },
            WorkPacket {
                class_label: 1,
                factory: &MockStrategy::Inferral1,
            },
            WorkPacket {
                class_label: 1,
                factory: &MockStrategy::Inferral2,
            },
            WorkPacket {
                class_label: 0,
                factory: &MockStrategy::Initial1,
            },
            WorkPacket {
                class_label: 0,
                factory: &MockStrategy::Initial2,
            },
            WorkPacket {
                class_label: 1,
                factory: &MockStrategy::Initial1,
            },
            WorkPacket {
                class_label: 1,
                factory: &MockStrategy::Initial2,
            },
            WorkPacket {
                class_label: 0,
                factory: &MockStrategy::Expansion1,
            },
            WorkPacket {
                class_label: 0,
                factory: &MockStrategy::Expansion2,
            },
            WorkPacket {
                class_label: 1,
                factory: &MockStrategy::Expansion1,
            },
            WorkPacket {
                class_label: 1,
                factory: &MockStrategy::Expansion2,
            },
        ];
        for expected_wp in expected_wps {
            assert_eq!(expected_wp, queue.next().unwrap());
        }
        assert_eq!(queue.next(), None);
    }

    #[test]
    fn queue_add_class_while_working() {
        let mut queue = ClassQueue::new(pack(), 0);
        for _ in 0..3 {
            queue.next().unwrap();
        }
        queue.add(3);
        assert_eq!(
            queue.next(),
            Some(WorkPacket {
                class_label: 3,
                factory: &MockStrategy::Verification1
            })
        );
        assert_eq!(
            queue.next(),
            Some(WorkPacket {
                class_label: 3,
                factory: &MockStrategy::Verification2
            })
        );
        assert_eq!(
            queue.next(),
            Some(WorkPacket {
                class_label: 0,
                factory: &MockStrategy::Inferral2
            })
        );
    }

    #[test]
    fn add_same_class_twice() {
        let mut queue = ClassQueue::new(pack(), 0);
        for _ in 0..3 {
            queue.next().unwrap();
        }
        queue.add(0);
        for _ in 3..8 {
            queue.next().unwrap();
        }
        assert_eq!(queue.next(), None);
    }
}
