use serde::Deserialize;

pub type Matrix = Vec<Vec<f64>>;

#[derive(Debug, Clone)]
pub struct Chain {
    pub name: String,
    pub scores: Vec<f64>,
}

#[derive(Debug, Deserialize)]
pub struct AhpInput {
    pub criteria: Vec<String>,
    pub matrix: Matrix,
}
