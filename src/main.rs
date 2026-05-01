mod ahp;
mod io;
mod models;
mod pqmrs;
mod quantum;

use std::error::Error;
use std::env;

use ahp::compute_weights;
use io::{load_chains, load_quantum_profiles, load_attack_scenarios};
use pqmrs::rank_chains;

fn baseline_mode() -> Result<(), Box<dyn Error>> {
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

    println!("\n=== PQMRS RANKING (BASELINE) ===\n");
    for (idx, (name, score)) in ranked.iter().enumerate() {
        println!("{:>2}. {:<12} {:.2}", idx + 1, name, score);
    }

    Ok(())
}

fn scenario_mode(scenario_name: &str) -> Result<(), Box<dyn Error>> {
    let (criteria, weights, lambda_max, ci, cr) = compute_weights("data/ahp_matrix.json")?;

    println!("\n=== AHP RESULTS ===\n");
    for (label, weight) in criteria.iter().zip(&weights) {
        println!("{:<16} {:.4}", label, weight);
    }
    println!("\nlambda_max: {:.4}", lambda_max);
    println!("CI: {:.4}", ci);
    println!("CR: {:.4}", cr);

    if cr > 0.10 {
        println!("\nInconsistent judgments (CR > 0.10).");
        return Ok(());
    }

    let chains = load_chains("data/chains.csv")?;
    let profiles = load_quantum_profiles("data/quantum_profiles.json")?;
    let scenarios = load_attack_scenarios("data/scenarios.json")?;

    let scenario = scenarios
        .iter()
        .find(|s| s.name == scenario_name)
        .ok_or(format!("Scenario '{}' not found", scenario_name))?;

    println!("\n=== QUANTUM ATTACK SCENARIO: {} ===", scenario.name);
    println!(
        "Attack Type: {:?}, Adversary Capability: {} qubits",
        scenario.attack_type, scenario.adversary_capability_qubits
    );
    println!(
        "Duration: {} months (starts year {})\n",
        scenario.duration_months, scenario.start_year
    );

    let results = pqmrs::simulate_scenario(&chains, scenario, &profiles, &weights);

    // Group by month and display
    let mut current_month = 0;
    for result in &results {
        if result.month != current_month {
            current_month = result.month;
            println!("\n--- Month {} ---", current_month);
        }
        println!(
            "  {:>2}. {:<12} base: {:.2} → adjusted: {:.2}",
            result.rank, result.chain, result.base_score, result.adjusted_score
        );
    }

    Ok(())
}

fn print_usage() {
    println!("Usage: pqmrs_ahp [MODE] [ARGS]");
    println!("  baseline                    - Run baseline AHP ranking");
    println!("  scenario <scenario_name>    - Simulate quantum attack scenario");
    println!("  list-scenarios              - List available scenarios");
}

fn list_scenarios() -> Result<(), Box<dyn Error>> {
    let scenarios = load_attack_scenarios("data/scenarios.json")?;
    println!("\n=== AVAILABLE SCENARIOS ===\n");
    for scenario in scenarios {
        println!("  {} ({:?})", scenario.name, scenario.attack_type);
        println!(
            "    Capability: {} qubits | Duration: {} months",
            scenario.adversary_capability_qubits, scenario.duration_months
        );
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        baseline_mode()?;
    } else {
        match args[1].as_str() {
            "baseline" => baseline_mode()?,
            "scenario" => {
                if args.len() < 3 {
                    eprintln!("Error: scenario requires a scenario name");
                    print_usage();
                    return Ok(());
                }
                scenario_mode(&args[2])?;
            }
            "list-scenarios" => list_scenarios()?,
            _ => {
                eprintln!("Unknown mode: {}", args[1]);
                print_usage();
            }
        }
    }

    Ok(())
}