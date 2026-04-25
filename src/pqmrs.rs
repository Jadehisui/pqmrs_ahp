use crate::models::Chain;

pub fn pqmrs(chain: &Chain, weights: &[f64]) -> f64 {
    chain
        .scores
        .iter()
        .zip(weights.iter())
        .map(|(score, weight)| score * weight)
        .sum::<f64>()
        * 10.0
}

pub fn rank_chains(chains: &[Chain], weights: &[f64]) -> Vec<(String, f64)> {
    let mut results: Vec<(String, f64)> = chains
        .iter()
        .map(|chain| (chain.name.clone(), pqmrs(chain, weights)))
        .collect();

    results.sort_by(|a, b| b.1.total_cmp(&a.1));
    results
}
