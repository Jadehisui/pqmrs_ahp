# Methodology

## Stage 1: AHP Theory Layer
- Define the post-quantum migration criteria.
- Build a pairwise comparison matrix.
- Normalize and derive criteria weights.

## Stage 2: Consistency Validation
- Compute $\lambda_{max}$, CI, and CR.
- Accept model if CR < 0.10.
- Redo pairwise judgments if CR > 0.10.

## Stage 3: Chain Scoring Layer
- Each chain receives the vector:
  [crypto_flex, governance, runtime, throughput, wallet, validator]

## Stage 4: PQMRS Final Score
- $PQMRS = 10 \sum_i w_i s_i$
- Sort chains by descending PQMRS.
