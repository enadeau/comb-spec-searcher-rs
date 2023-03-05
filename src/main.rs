use comb_spec_searcher::pack::StrategyPack;
use comb_spec_searcher::word;
use comb_spec_searcher::CombinatorialSpecificationSearcher;
use serde_json;

fn main() {
    let prefix = String::from("");
    // let patterns = vec![String::from("ababa"), String::from("babb")];
    let patterns = vec![String::from("b")];
    let alphabet = vec!['a', 'b'];
    let start_class = word::AvoidingWithPrefix::new(prefix, patterns, alphabet);
    let pack = StrategyPack {
        initials: vec![word::WordStrategy::RemoveFrontOfPrefix],
        inferrals: vec![],
        expansions: vec![word::WordStrategy::Expansion],
        verifications: vec![word::WordStrategy::Atom],
    };

    let mut searcher = CombinatorialSpecificationSearcher::new(start_class, pack);
    let spec = searcher.auto_search().expect("No spec");
    for rule in spec.rules.into_iter() {
        let parent = rule.get_parent();
        println!("{}", serde_json::to_string(parent).unwrap());
        let strategy = rule.get_strategy();
        println!("{}", serde_json::to_string(&strategy).unwrap());
    }
    println!("{}", serde_json::to_string(&spec.root).unwrap());
}
