use serde::{Deserialize, Serialize};

pub type Matrix = Vec<Vec<f64>>;

#[derive(Debug, Clone)]
pub struct Chain {
    pub name: String,
    pub scores: Vec<f64>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct QuantumProfile {
    pub key_size_bits: usize,
    pub hash_bits: usize,
    pub signature_scheme: String,
    pub validator_count: usize,
    pub upgrade_latency_days: usize,
    pub pq_migration_progress: f64,
}

#[derive(Debug, Clone)]
pub struct ChainProfile {
    pub name: String,
    pub base_scores: Vec<f64>,
    pub quantum_profile: QuantumProfile,
}

#[derive(Debug, Deserialize)]
pub struct AhpInput {
    pub criteria: Vec<String>,
    pub matrix: Matrix,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum AttackType {
    #[serde(rename = "shor_signature")]
    ShorSignature,
    #[serde(rename = "grover_hash")]
    GroverHash,
    #[serde(rename = "hybrid")]
    Hybrid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttackScenario {
    pub name: String,
    pub attack_type: AttackType,
    pub adversary_capability_qubits: usize,
    pub start_year: usize,
    pub duration_months: usize,
    pub migration_assumption: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct QuantumImpact {
    pub security_drop: f64,
    pub liveness_drop: f64,
    pub economic_penalty: f64,
}

#[derive(Debug, Clone)]
pub struct SimulationResult {
    pub chain: String,
    pub scenario: String,
    pub month: usize,
    pub base_score: f64,
    pub adjusted_score: f64,
    pub rank: usize,
}
