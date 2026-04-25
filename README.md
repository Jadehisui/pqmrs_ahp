# PQMRS: Post-Quantum Migration Readiness Score

A reproducible, research-grade framework for evaluating blockchain readiness for post-quantum cryptographic migration.

## Features
- Analytic Hierarchy Process (AHP) criteria weighting
- Consistency Ratio (CR) validation
- Cross-chain PQMRS scoring and ranking
- CSV + JSON data ingestion
- Python sensitivity and plotting utilities

## Repository Structure
- `src/ahp.rs`: matrix normalization, weight extraction, CI/CR
- `src/pqmrs.rs`: weighted scoring and ranking
- `src/io.rs`: CSV and JSON loaders
- `src/models.rs`: shared models and types
- `data/chains.csv`: chain benchmark inputs
- `data/ahp_matrix.json`: pairwise criteria matrix
- `analysis/`: plotting and sensitivity scripts
- `docs/`: methodology, results, paper draft scaffold

## Method
1. Stage 1 - AHP theory layer:
   derive factor weights from pairwise comparisons.
2. Stage 2 - validation:
   enforce CR < 0.10 for model consistency.
3. Stage 3 - chain scoring:
   use score vectors [crypto, governance, runtime, throughput, wallet, validator].
4. Stage 4 - final score:
   `PQMRS = 10 * sum(w_i * s_i)`.

## Run
```bash
cargo run
```

## Python Analysis (optional)
```bash
python3 analysis/sensitivity.py
python3 analysis/plots.py
```

## License
MIT
