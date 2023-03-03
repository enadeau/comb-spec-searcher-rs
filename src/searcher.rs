use crate::errors::SpecificationNotFoundError;
use crate::pack::{Rule, StrategyFactory, StrategyPack};
use crate::specification::CombinatorialSpecification;
use std::time::{Duration, Instant};

mod classdb;
mod equiv_db;
mod queue;
mod ruledb;

pub struct CombinatorialSpecificationSearcher<F: StrategyFactory> {
    start_class: F::ClassType,
    start_label: usize,
    queue: queue::ClassQueue<F>,
    classdb: classdb::ClassDB<F::ClassType>,
    ruledb: ruledb::RuleDB<F::StrategyType>,
}

impl<F: StrategyFactory> CombinatorialSpecificationSearcher<F> {
    pub fn new(start_class: F::ClassType, pack: StrategyPack<F>) -> Self {
        let mut classdb = classdb::ClassDB::new();
        let start_label = classdb.get_label_from_class_or_add(&start_class);
        let queue = queue::ClassQueue::new(pack, start_label);
        let ruledb = ruledb::RuleDB::new();
        Self {
            start_class,
            start_label,
            queue,
            classdb,
            ruledb,
        }
    }

    pub fn auto_search(
        &mut self,
    ) -> Result<CombinatorialSpecification<F::StrategyType>, SpecificationNotFoundError> {
        self.expand_for(Duration::from_millis(1));
        self.ruledb.get_specification(
            self.classdb
                .get_label_from_class(&self.start_class)
                .expect("Start class label not found"),
            &self.classdb,
        )
    }

    fn expand_for(&mut self, expansion_time: Duration) {
        let start_time = Instant::now();
        while start_time.elapsed() < expansion_time {
            let (class_label, strategy_factory) = self.queue.next().expect("Queue is empty");
            let class = self
                .classdb
                .get_class_from_label(class_label)
                .expect("Class label not found");
            let rules = strategy_factory.apply(&class);
            for rule in rules.into_iter() {
                let start = self.classdb.get_label_from_class_or_add(rule.get_parent());
                let ends = rule
                    .get_children()
                    .iter()
                    .map(|c| self.classdb.get_label_from_class_or_add(c))
                    .collect();
                self.add_rule(start, ends, rule);
            }
        }
    }

    fn add_rule(&mut self, start: usize, ends: Vec<usize>, rule: Rule<F::StrategyType>) {
        ends.iter().map(|l| self.queue.add(*l)).last();
        self.ruledb.add(start, ends, rule);
    }
}
