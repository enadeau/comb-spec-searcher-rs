use comb_spec_searcher::pack::StrategyPack;
use comb_spec_searcher::word;
use comb_spec_searcher::CombinatorialSpecificationSearcher;

fn main() {
    let prefix = String::from("");
    let patterns = vec![String::from("ababa"), String::from("babb")];
    let alphabet = vec!['a', 'b'];
    let start_class = word::AvoidingWithPrefix::new(prefix, patterns, alphabet);
    let pack = StrategyPack {
        initials: vec![word::WordStrategy::RemoveFrontOfPrefix],
        inferrals: vec![],
        expansions: vec![word::WordStrategy::Expansion],
        verifications: vec![word::WordStrategy::Atom],
    };

    let mut searcher = CombinatorialSpecificationSearcher::new(start_class, pack);
    let spec = searcher.auto_search();
}
