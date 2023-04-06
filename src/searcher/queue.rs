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
    fn make_external<'a, F: StrategyFactory>(
        &self,
        pack: &'a StrategyPack<F>,
    ) -> WorkPacket<'a, F> {
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
    last_wp: Option<WorkPacketInternal>,
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
            last_wp: None,
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

    pub fn next(&mut self, last_wp_created_rule: Option<bool>) -> Option<WorkPacket<F>> {
        self.decide_if_ignore(last_wp_created_rule);
        loop {
            let next = self.next_no_ignore()?;
            if !self.ignore.contains(&next.class_label) {
                let external_wp = next.make_external(&self.pack);
                self.last_wp = Some(next);
                return Some(external_wp);
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

    /// Decide whether the class from the last work packet should now be ignore based on whether the
    /// work packet produced a rule.
    fn decide_if_ignore(&mut self, last_wp_created_rule: Option<bool>) {
        match (&self.last_wp, last_wp_created_rule) {
            (None, None) => (),
            (None, Some(_)) => panic!("There was not last packet"),
            (Some(_), None) => panic!("Need info about last packet"),
            (Some(wp), Some(wp_created_rule)) => {
                if wp_created_rule
                    && (self.pack.is_verification(wp.factory_index)
                        || self.pack.is_inferral(wp.factory_index))
                {
                    self.ignore(wp.class_label);
                }
            }
        }
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
        let mut expected_factory = [
            MockStrategy::Verification1,
            MockStrategy::Verification2,
            MockStrategy::Inferral1,
            MockStrategy::Inferral2,
            MockStrategy::Initial1,
            MockStrategy::Initial2,
            MockStrategy::Expansion1,
            MockStrategy::Expansion2,
        ]
        .iter();
        let wp = queue.next(None).unwrap();
        assert_eq!(wp.class_label, 0);
        assert_eq!(wp.factory, expected_factory.next().unwrap());
        for factory in expected_factory {
            let wp = queue.next(Some(false)).unwrap();
            assert_eq!(wp.class_label, 0);
            assert_eq!(wp.factory, factory);
        }
        assert_eq!(queue.next(Some(false)), None);
    }

    #[test]
    fn queue_basic_two_classes_test() {
        let mut queue = ClassQueue::new(pack(), 0);
        queue.add(1);
        let mut expected_wps = [
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
        ]
        .into_iter();
        assert_eq!(expected_wps.next().unwrap(), queue.next(None).unwrap());
        for expected_wp in expected_wps {
            assert_eq!(expected_wp, queue.next(Some(false)).unwrap());
        }
        assert_eq!(queue.next(Some(false)), None);
    }

    #[test]
    fn queue_add_class_while_working() {
        let mut queue = ClassQueue::new(pack(), 0);
        queue.next(None).unwrap();
        for _ in 0..2 {
            queue.next(Some(false)).unwrap();
        }
        queue.add(3);
        assert_eq!(
            queue.next(Some(false)),
            Some(WorkPacket {
                class_label: 3,
                factory: &MockStrategy::Verification1
            })
        );
        assert_eq!(
            queue.next(Some(false)),
            Some(WorkPacket {
                class_label: 3,
                factory: &MockStrategy::Verification2
            })
        );
        assert_eq!(
            queue.next(Some(false)),
            Some(WorkPacket {
                class_label: 0,
                factory: &MockStrategy::Inferral2
            })
        );
    }

    #[test]
    fn add_same_class_twice() {
        let mut queue = ClassQueue::new(pack(), 0);
        queue.next(None);
        for _ in 0..2 {
            queue.next(Some(false)).unwrap();
        }
        queue.add(0);
        for _ in 3..8 {
            queue.next(Some(false)).unwrap();
        }
        assert_eq!(queue.next(Some(false)), None);
    }

    #[test]
    fn stop_yielding_after_verification() {
        let mut queue = ClassQueue::new(pack(), 0);
        queue.next(None).unwrap();
        assert_eq!(queue.next(Some(true)), None);
    }

    #[test]
    fn stop_yielding_after_inferral() {
        let mut queue = ClassQueue::new(pack(), 0);
        queue.next(None).unwrap();
        queue.next(Some(false)).unwrap();
        queue.next(Some(false)).unwrap();
        assert_eq!(queue.next(Some(true)), None);
    }

    #[test]
    fn keep_yielding_after_initial_and_expansion() {
        let mut queue = ClassQueue::new(pack(), 0);
        queue.next(None).unwrap(); // ver1
        queue.next(Some(false)).unwrap(); // ver2
        queue.next(Some(false)).unwrap(); // inf1
        queue.next(Some(false)).unwrap(); // inf2
        queue.next(Some(false)).unwrap(); // init1
        queue
            .next(Some(true))
            .expect("Should yield after successful initial"); // init2
        queue
            .next(Some(true))
            .expect("Should yield after successful initial"); //exp1
        queue
            .next(Some(true))
            .expect("Should yield after successful initial"); //exp2
        assert_eq!(queue.next(Some(true)), None)
    }
}
