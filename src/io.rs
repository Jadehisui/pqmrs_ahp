use std::error::Error;
use std::fs;

use csv::ReaderBuilder;
use serde::Deserialize;

use crate::models::{AhpInput, Chain};

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
