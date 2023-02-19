use crate::combinatorial_class::CombinatorialClass;

pub trait StrategyFactory {
    type ClassType: CombinatorialClass;

    fn apply(&self, class: &Self::ClassType);
}

pub struct StrategyPack<F>
where
    F: StrategyFactory
{
    pub initials: Vec<F>,
    pub inferrals: Vec<F>,
    pub expansions: Vec<F>,
    pub verifications: Vec<F>,
}

impl<F> StrategyPack<F>
where
    F: StrategyFactory
{
    pub fn len(&self) -> usize {
        self.initials.len()
            + self.inferrals.len()
            + self.expansions.len()
            + self.verifications.len()
    }

    pub fn get_strategy_factory(&self, index: usize) -> &F {
        let mut index = index;
        if index < self.verifications.len() {
            return &self.verifications[index]
        } else {
            index -= self.verifications.len();
        }
        if index < self.inferrals.len() {
            return &self.inferrals[index]
        } else {
            index -= self.inferrals.len();
        }
        if index < self.initials.len() {
            return &self.initials[index]
        } else {
            index -= self.initials.len();
        }
        if index < self.expansions.len() {
            return &self.expansions[index]
        } else {
            panic!("Index of strategy out of bound!");
        }
    }
}
