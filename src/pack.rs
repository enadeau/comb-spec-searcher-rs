use crate::combinatorial_class::CombinatorialClass;

pub struct Rule<C, S>
where
    C: CombinatorialClass,
    S: Strategy<ClassType = C>,
{
    comb_class: C,
    strategy: S,
}

pub trait Strategy: Sized {
    type ClassType: CombinatorialClass;

    fn apply(&self, comb_class: &Self::ClassType) -> Rule<Self::ClassType, Self>;
}

pub trait StrategyFactory {
    type ClassType: CombinatorialClass;
    type StrategyType: Strategy<ClassType = Self::ClassType>;

    fn apply(&self, class: &Self::ClassType) -> Vec<Self::StrategyType>;
}

pub struct StrategyPack<F: StrategyFactory> {
    pub initials: Vec<F>,
    pub inferrals: Vec<F>,
    pub expansions: Vec<F>,
    pub verifications: Vec<F>,
}

impl<F> StrategyPack<F>
where
    F: StrategyFactory,
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
            return &self.verifications[index];
        } else {
            index -= self.verifications.len();
        }
        if index < self.inferrals.len() {
            return &self.inferrals[index];
        } else {
            index -= self.inferrals.len();
        }
        if index < self.initials.len() {
            return &self.initials[index];
        } else {
            index -= self.initials.len();
        }
        if index < self.expansions.len() {
            return &self.expansions[index];
        } else {
            panic!("Index of strategy out of bound!");
        }
    }
}
