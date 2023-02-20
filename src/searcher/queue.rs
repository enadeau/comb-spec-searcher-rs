use crate::pack::{StrategyFactory, StrategyPack, Strategy};
use crate::combinatorial_class::CombinatorialClass;
use std::collections::VecDeque;

pub struct ClassQueue<C, S, F>
where
    C: CombinatorialClass,
    S: Strategy<ClassType=C>,
    F: StrategyFactory<ClassType=C, StrategyType=S>,
{
    pack: StrategyPack<F>,
    queue: VecDeque<usize>,
    curr_label: usize,
    strat_index: usize,
    max_strat_index: usize,
}

impl<C, S, F> ClassQueue<C, S, F>
where
    C: CombinatorialClass,
    S: Strategy<ClassType=C>,
    F: StrategyFactory<ClassType=C, StrategyType=S>,
{
    pub fn new(pack: StrategyPack<F>, start_label: usize) -> Self {
        let pack_size = pack.len();
        Self {
            pack,
            queue: VecDeque::new(),
            curr_label: start_label,
            strat_index: 0,
            max_strat_index: pack_size,
        }
    }

    pub fn next(&mut self) -> Option<(usize, &F)> {
        if self.max_strat_index == self.strat_index {
            self.strat_index = 0;
            self.curr_label = self.queue.pop_front()?;
        } else {
            self.strat_index += 1;
        }
        Some((self.curr_label, self.pack.get_strategy_factory(self.strat_index)))
    }
}
