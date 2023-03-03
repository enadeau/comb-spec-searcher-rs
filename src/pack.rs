use crate::combinatorial_class::CombinatorialClass;
use std::fmt::Debug;

#[derive(Debug)]
pub struct Rule<S: Strategy> {
    parent: S::ClassType,
    strategy: S,
    children: Vec<S::ClassType>,
}

impl<S: Strategy> Rule<S> {
    pub fn new(parent: S::ClassType, strategy: S) -> Rule<S> {
        let children = strategy.decompose(&parent);
        Rule {
            parent,
            strategy,
            children,
        }
    }

    pub fn get_strategy(self) -> S {
        self.strategy
    }

    pub fn is_equivalence(&self) -> bool {
        self.strategy.is_equivalence()
    }

    pub fn get_parent(&self) -> &S::ClassType {
        &self.parent
    }

    pub fn get_children(&self) -> &Vec<S::ClassType> {
        &self.children
    }
}

pub trait Strategy: Debug + Sized + Clone {
    type ClassType: CombinatorialClass;

    fn decompose(&self, comb_class: &Self::ClassType) -> Vec<Self::ClassType>;
    fn is_equivalence(&self) -> bool;
}

pub trait StrategyFactory {
    type ClassType: CombinatorialClass;
    type StrategyType: Strategy<ClassType = Self::ClassType>;

    fn apply(&self, class: &Self::ClassType) -> Vec<Rule<Self::StrategyType>>;
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
