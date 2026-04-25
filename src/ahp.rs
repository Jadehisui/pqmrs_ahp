use std::error::Error;

use crate::io::load_ahp_input;
use crate::models::Matrix;

fn column_sums(m: &Matrix) -> Vec<f64> {
    let n = m.len();
    let mut sums = vec![0.0; n];

    for row in m {
        for (j, value) in row.iter().enumerate() {
            sums[j] += value;
        }
    }

    sums
}

fn normalize_matrix(m: &Matrix) -> Matrix {
    let sums = column_sums(m);
    let n = m.len();
    let mut norm = vec![vec![0.0; n]; n];

    for i in 0..n {
        for (j, sum) in sums.iter().enumerate() {
            norm[i][j] = m[i][j] / sum;
        }
    }

    norm
}

fn row_avg(m: &Matrix) -> Vec<f64> {
    m.iter()
        .map(|row| row.iter().sum::<f64>() / row.len() as f64)
        .collect()
}

fn mat_vec(m: &Matrix, v: &[f64]) -> Vec<f64> {
    m.iter()
        .map(|row| row.iter().zip(v).map(|(a, b)| a * b).sum())
        .collect()
}

fn consistency(n: usize, lambda_max: f64) -> (f64, f64) {
    let ci = if n <= 1 {
        0.0
    } else {
        (lambda_max - n as f64) / (n as f64 - 1.0)
    };

    let ri = match n {
        1 | 2 => 0.0,
        3 => 0.58,
        4 => 0.90,
        5 => 1.12,
        6 => 1.24,
        7 => 1.32,
        8 => 1.41,
        9 => 1.45,
        10 => 1.49,
        _ => 1.49,
    };

    let cr = if ri == 0.0 { 0.0 } else { ci / ri };
    (ci, cr)
}

fn validate_matrix(matrix: &Matrix, criteria_len: usize) -> Result<(), Box<dyn Error>> {
    let n = matrix.len();
    if n == 0 {
        return Err("AHP matrix cannot be empty".into());
    }

    if n != criteria_len {
        return Err("criteria count must match matrix dimensions".into());
    }

    for row in matrix {
        if row.len() != n {
            return Err("AHP matrix must be square".into());
        }
        if row.iter().any(|v| *v <= 0.0) {
            return Err("AHP matrix values must be positive".into());
        }
    }

    Ok(())
}

pub fn compute_weights(path: &str) -> Result<(Vec<String>, Vec<f64>, f64, f64, f64), Box<dyn Error>> {
    let input = load_ahp_input(path)?;
    validate_matrix(&input.matrix, input.criteria.len())?;

    let norm = normalize_matrix(&input.matrix);
    let weights = row_avg(&norm);
    let aw = mat_vec(&input.matrix, &weights);

    let lambda_vals: Vec<f64> = aw.iter().zip(&weights).map(|(a, w)| a / w).collect();
    let lambda_max = lambda_vals.iter().sum::<f64>() / lambda_vals.len() as f64;
    let (ci, cr) = consistency(input.matrix.len(), lambda_max);

    Ok((input.criteria, weights, lambda_max, ci, cr))
}
