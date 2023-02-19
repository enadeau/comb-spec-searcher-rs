use comb_spec_searcher::{CombinatorialSpecificationSearcher, StrategyPack};
use comb_spec_searcher::word;

fn main() {
    let prefix = String::from("");
    let patterns = vec![String::from("ababa"), String::from("babb")];
    let alphabet = vec!['a', 'b'];
    let start_class = word::AvoidingWithPrefix::new(
        prefix, patterns, alphabet
    );
    let pack = StrategyPack{
        initials: vec![word::WordStrategyFactory::RemoveFrontOfPrefix],
        inferrals: vec![],
        expansions: vec![word::WordStrategyFactory::Expansion],
        verifications: vec![word::WordStrategyFactory::Atom],
    };

    let mut searcher = CombinatorialSpecificationSearcher::new(
        start_class, pack
    );
    let spec = searcher.auto_search();
}
