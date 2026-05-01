use crate::models::{AttackScenario, AttackType, QuantumProfile};

/// Compute probability that Shor's algorithm can break signature scheme
/// Pure quantum threat: P_sig(t) = 1 - exp(-alpha * Q_capability * vuln_factor)
pub fn shor_threat_probability(
    scenario: &AttackScenario,
    profile: &QuantumProfile,
    month: usize,
) -> f64 {
    let alpha = 0.00001; // coupling constant
    let months_elapsed = month as f64;
    
    // Vulnerability factor based on key size and migration progress
    let base_vuln = 1.0 / (1.0 + (profile.key_size_bits as f64 / 2048.0).sqrt());
    let migration_protection = profile.pq_migration_progress; // 0 to 1
    let vuln_factor = base_vuln * (1.0 - migration_protection);
    
    // Threat ramps up over time as adversary accumulates quantum resources
    let capability_growth = (months_elapsed / 12.0).min(1.0); // Scales over ~year
    let effective_qubits = scenario.adversary_capability_qubits as f64 * capability_growth;
    
    let threat_exponent = -alpha * effective_qubits * vuln_factor;
    (1.0 - threat_exponent.exp()).max(0.0).min(1.0)
}

/// Compute Grover's algorithm speedup impact on hash-based security
/// Grover halves effective hash bits: sec_eff(t) = hash_bits - sqrt(hash_bits) * speedup_factor
pub fn grover_speedup_factor(
    scenario: &AttackScenario,
    profile: &QuantumProfile,
    month: usize,
) -> f64 {
    let months_elapsed = month as f64;
    let capability_growth = (months_elapsed / 12.0).min(1.0);
    
    // Grover gives quadratic speedup; cap at 0.5 of hash strength
    let speedup = 0.5 * capability_growth * (scenario.adversary_capability_qubits as f64 / 1000.0).min(1.0);
    
    // Effective security reduction as fraction of original
    let hash_bits_f64 = profile.hash_bits as f64;
    let original_strength = 2.0_f64.powf(hash_bits_f64);
    let reduced_strength = 2.0_f64.powf((hash_bits_f64 * 0.5).max(0.0));
    let reduction_fraction = 1.0 - (reduced_strength / original_strength);
    
    (reduction_fraction * speedup).min(1.0)
}

/// Hybrid attack: signature compromise + hash weakening + validator compromise chain
pub fn hybrid_impact(
    scenario: &AttackScenario,
    profile: &QuantumProfile,
    month: usize,
) -> f64 {
    let sig_threat = shor_threat_probability(scenario, profile, month);
    let hash_speedup = grover_speedup_factor(scenario, profile, month);
    
    // Validator compromise risk: if validators < threshold, chain liveness threatened
    let validator_risk = if profile.validator_count < 13 { 0.3 } else { 0.0 };
    
    // Combine threats: assume attacker needs any one vector to succeed
    (sig_threat + hash_speedup + validator_risk).min(1.0) / 3.0
}

/// Map quantum threat to decision criterion impacts
/// Returns impact on each AHP criterion score
pub fn compute_impact(
    scenario: &AttackScenario,
    profile: &QuantumProfile,
    month: usize,
    criteria_count: usize,
) -> Vec<f64> {
    let threat = match scenario.attack_type {
        AttackType::ShorSignature => shor_threat_probability(scenario, profile, month),
        AttackType::GroverHash => grover_speedup_factor(scenario, profile, month),
        AttackType::Hybrid => hybrid_impact(scenario, profile, month),
    };
    
    // Distribute threat across criteria
    // Assume common index structure: crypto_flex, governance, runtime, throughput, wallet, validator
    let mut drops = vec![0.0; criteria_count];
    
    if criteria_count >= 6 {
        drops[0] = threat * 0.8;           // crypto_flex: high impact
        drops[1] = threat * 0.2;           // governance: low impact
        drops[2] = threat * 0.1;           // runtime: minimal
        drops[3] = threat * 0.1;           // throughput: minimal
        drops[4] = threat * 0.5;           // wallet (security): medium-high
        drops[5] = threat * 0.4;           // validator: medium
    }
    
    drops
}

/// Adjust chain scores under quantum attack at given time
pub fn apply_quantum_adjustment(
    base_scores: &[f64],
    scenario: &AttackScenario,
    profile: &QuantumProfile,
    month: usize,
) -> Vec<f64> {
    let impacts = compute_impact(scenario, profile, month, base_scores.len());
    base_scores
        .iter()
        .zip(impacts.iter())
        .map(|(score, impact)| {
            let adjustment = 1.0 - impact;
            (score * adjustment).max(0.0)
        })
        .collect()
}

/// Simulate a single trial at a given month for all chains
pub fn simulate_month(
    chains: &[(String, Vec<f64>)],
    scenario: &AttackScenario,
    profiles: &[(String, QuantumProfile)],
    month: usize,
) -> Vec<(String, Vec<f64>)> {
    chains
        .iter()
        .map(|(name, scores)| {
            let profile = profiles
                .iter()
                .find(|(pname, _)| pname == name)
                .map(|(_, p)| p.clone())
                .unwrap_or_else(|| QuantumProfile {
                    key_size_bits: 2048,
                    hash_bits: 256,
                    signature_scheme: "RSA-2048".to_string(),
                    validator_count: 10,
                    upgrade_latency_days: 30,
                    pq_migration_progress: 0.0,
                });
            
            let adjusted = apply_quantum_adjustment(scores, scenario, &profile, month);
            (name.clone(), adjusted)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shor_threat_increases_over_time() {
        let scenario = AttackScenario {
            name: "Test".to_string(),
            attack_type: AttackType::ShorSignature,
            adversary_capability_qubits: 5000,
            start_year: 2026,
            duration_months: 12,
            migration_assumption: 0.1,
        };
        
        let profile = QuantumProfile {
            key_size_bits: 2048,
            hash_bits: 256,
            signature_scheme: "RSA-2048".to_string(),
            validator_count: 10,
            upgrade_latency_days: 30,
            pq_migration_progress: 0.0,
        };
        
        let p0 = shor_threat_probability(&scenario, &profile, 0);
        let p6 = shor_threat_probability(&scenario, &profile, 6);
        assert!(p6 > p0, "Threat should increase over time");
    }

    #[test]
    fn test_grover_bounded_at_one() {
        let scenario = AttackScenario {
            name: "Test".to_string(),
            attack_type: AttackType::GroverHash,
            adversary_capability_qubits: 10000,
            start_year: 2026,
            duration_months: 24,
            migration_assumption: 0.0,
        };
        
        let profile = QuantumProfile {
            key_size_bits: 2048,
            hash_bits: 128,
            signature_scheme: "RSA-2048".to_string(),
            validator_count: 10,
            upgrade_latency_days: 30,
            pq_migration_progress: 0.0,
        };
        
        let speedup = grover_speedup_factor(&scenario, &profile, 24);
        assert!(speedup <= 1.0, "Speedup should be bounded at 1.0");
    }
}
