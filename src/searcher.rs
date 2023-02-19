use std::time::{Duration, Instant};
use crate::pack::{StrategyPack, StrategyFactory};
use crate::combinatorial_class::CombinatorialClass;

mod queue;
mod classdb;

pub struct CombinatorialSpecification {}

pub struct CombinatorialSpecificationSearcher<C, F>
where
    C: CombinatorialClass,
    F: StrategyFactory<ClassType = C>
{
    start_class: C,
    queue: queue::ClassQueue<F>,
    classdb: classdb::ClassDB<C>,
}

impl<C, F> CombinatorialSpecificationSearcher<C, F>
where
    C: CombinatorialClass,
    F: StrategyFactory<ClassType = C>
{
    pub fn new(start_class: C, pack: StrategyPack<F>) -> Self {
        let mut classdb = classdb::ClassDB::new();
        let start_label = classdb.get_label_from_class(&start_class);
        let queue = queue::ClassQueue::new(pack, start_label);
        Self { start_class, queue, classdb}
    }

    pub fn auto_search(&mut self) -> CombinatorialSpecification {
        self.expand_for(Duration::from_secs(10));
        CombinatorialSpecification {}
    }

    fn expand_for(&mut self, expansion_time: Duration) {
        let start_time = Instant::now();
        while start_time.elapsed() < expansion_time {
            let (class_label, strategy_factory) = self.queue.next().unwrap();
            let class = self.classdb.get_class_from_label(class_label).expect("Class label not found");
            strategy_factory.apply(class);
        }
    }
}
