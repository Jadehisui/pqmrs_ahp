mod ahp;
mod io;
mod models;
mod pqmrs;

use std::error::Error;

use ahp::compute_weights;
use io::load_chains;
use pqmrs::rank_chains;

fn main() -> Result<(), Box<dyn Error>> {
    let (criteria, weights, lambda_max, ci, cr) = compute_weights("data/ahp_matrix.json")?;

    println!("\n=== AHP RESULTS ===\n");
    for (label, weight) in criteria.iter().zip(&weights) {
        println!("{:<16} {:.4}", label, weight);
    }
    println!("\nlambda_max: {:.4}", lambda_max);
    println!("CI: {:.4}", ci);
    println!("CR: {:.4}", cr);

    if cr > 0.10 {
        println!("\nInconsistent judgments (CR > 0.10). Redo pairwise comparisons.");
        return Ok(());
    }

    let chains = load_chains("data/chains.csv")?;
    let ranked = rank_chains(&chains, &weights);

    println!("\n=== PQMRS RANKING ===\n");
    for (idx, (name, score)) in ranked.iter().enumerate() {
        println!("{:>2}. {:<12} {:.2}", idx + 1, name, score);
    }

    Ok(())
}