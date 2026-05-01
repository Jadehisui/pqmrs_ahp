use crate::models::{AttackScenario, Chain, QuantumProfile, SimulationResult};
use crate::quantum;

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

/// Simulate quantum attack scenario over time
pub fn simulate_scenario(
    chains: &[Chain],
    scenario: &AttackScenario,
    profiles: &[(String, QuantumProfile)],
    weights: &[f64],
) -> Vec<SimulationResult> {
    let mut results = Vec::new();
    
    for month in 0..=scenario.duration_months {
        // Convert base chain scores to tuples for quantum module
        let chain_data: Vec<(String, Vec<f64>)> = chains
            .iter()
            .map(|c| (c.name.clone(), c.scores.clone()))
            .collect();
        
        // Apply quantum impacts
        let adjusted_chains = quantum::simulate_month(&chain_data, scenario, profiles, month);
        
        // Compute adjusted scores and rank
        let mut month_results: Vec<(String, f64)> = adjusted_chains
            .iter()
            .map(|(name, adjusted_scores)| {
                let score = adjusted_scores
                    .iter()
                    .zip(weights.iter())
                    .map(|(s, w)| s * w)
                    .sum::<f64>()
                    * 10.0;
                (name.clone(), score)
            })
            .collect();
        
        month_results.sort_by(|a, b| b.1.total_cmp(&a.1));
        
        // Log base scores before adjustment for reference
        let base_chains: Vec<(String, f64)> = chains
            .iter()
            .map(|c| (c.name.clone(), pqmrs(c, weights)))
            .collect();
        
        for (rank, (name, adjusted_score)) in month_results.iter().enumerate() {
            let base_score = base_chains
                .iter()
                .find(|(n, _)| n == name)
                .map(|(_, s)| s)
                .unwrap_or(&0.0);
            
            results.push(SimulationResult {
                chain: name.clone(),
                scenario: scenario.name.clone(),
                month,
                base_score: *base_score,
                adjusted_score: *adjusted_score,
                rank: rank + 1,
            });
        }
    }
    
    results
}
