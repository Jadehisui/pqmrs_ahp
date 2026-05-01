use std::error::Error;
use std::fs;

use csv::ReaderBuilder;
use serde::Deserialize;

use crate::models::{AhpInput, AttackScenario, Chain, QuantumProfile};

#[derive(Debug, Deserialize)]
struct ChainRow {
    chain: String,
    crypto_flex: f64,
    governance: f64,
    runtime: f64,
    throughput: f64,
    wallet: f64,
    validator: f64,
}

#[derive(Debug, Deserialize)]
pub struct QuantumProfileJson {
    pub name: String,
    pub key_size_bits: usize,
    pub hash_bits: usize,
    pub signature_scheme: String,
    pub validator_count: usize,
    pub upgrade_latency_days: usize,
    pub pq_migration_progress: f64,
}

pub fn load_ahp_input(path: &str) -> Result<AhpInput, Box<dyn Error>> {
    let raw = fs::read_to_string(path)?;
    let parsed: AhpInput = serde_json::from_str(&raw)?;
    Ok(parsed)
}

pub fn load_chains(path: &str) -> Result<Vec<Chain>, Box<dyn Error>> {
    let mut reader = ReaderBuilder::new().trim(csv::Trim::All).from_path(path)?;
    let mut chains = Vec::new();

    for row in reader.deserialize::<ChainRow>() {
        let record = row?;
        chains.push(Chain {
            name: record.chain,
            scores: vec![
                record.crypto_flex,
                record.governance,
                record.runtime,
                record.throughput,
                record.wallet,
                record.validator,
            ],
        });
    }

    Ok(chains)
}

pub fn load_quantum_profiles(path: &str) -> Result<Vec<(String, QuantumProfile)>, Box<dyn Error>> {
    let raw = fs::read_to_string(path)?;
    let profiles: Vec<QuantumProfileJson> = serde_json::from_str(&raw)?;
    
    Ok(profiles
        .into_iter()
        .map(|p| {
            (
                p.name.clone(),
                QuantumProfile {
                    key_size_bits: p.key_size_bits,
                    hash_bits: p.hash_bits,
                    signature_scheme: p.signature_scheme,
                    validator_count: p.validator_count,
                    upgrade_latency_days: p.upgrade_latency_days,
                    pq_migration_progress: p.pq_migration_progress,
                },
            )
        })
        .collect())
}

pub fn load_attack_scenarios(path: &str) -> Result<Vec<AttackScenario>, Box<dyn Error>> {
    let raw = fs::read_to_string(path)?;
    let scenarios: Vec<AttackScenario> = serde_json::from_str(&raw)?;
    Ok(scenarios)
}
