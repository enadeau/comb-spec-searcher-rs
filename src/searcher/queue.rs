use crate::pack::{StrategyFactory, StrategyPack};
use std::collections::{HashSet, VecDeque};

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
