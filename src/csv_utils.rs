//! Utility functions for exporting ODE solutions and errors to CSV files.
//!
//! This module provides functions to write the analytic and Euler method results, as well as their errors,
//! to CSV files for further analysis or plotting in external tools.

use std::error::Error;

/// Writes the analytic solution, Euler n=20, and Euler n=1000 results to a CSV file in wide format.
///
/// # Arguments
/// * `t_1000` - Time points for the analytic and Euler n=1000 solutions (fine grid)
/// * `y_analytic_1000` - Analytic solution values at `t_1000`
/// * `t_20` - Time points for Euler n=20
/// * `y_euler_20` - Euler method values for n=20
/// * `y_euler_1000` - Euler method values for n=1000
/// * `filename` - Output CSV file name
///
/// # Output
/// Writes a CSV file with columns: t, analytic, euler_n20, euler_n1000
pub fn write_solutions_csv(
    t_1000: &[f64],
    y_analytic_1000: &[f64],
    t_20: &[f64],
    y_euler_20: &[f64],
    y_euler_1000: &[f64],
    filename: &str,
) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_path(filename)?;
    wtr.write_record(["t", "analytic", "euler_n20", "euler_n1000"])?;
    let mut idx_20 = 0;
    for (i, &t) in t_1000.iter().enumerate() {
        let analytic = y_analytic_1000[i];
        let euler_1000 = y_euler_1000[i];
        let euler_20 = if idx_20 < t_20.len() && (t - t_20[idx_20]).abs() < 1e-8 {
            let val = y_euler_20[idx_20];
            idx_20 += 1;
            Some(val)
        } else {
            None
        };
        wtr.write_record(&[
            t.to_string(),
            analytic.to_string(),
            euler_20.map(|v| v.to_string()).unwrap_or_default(),
            euler_1000.to_string(),
        ])?;
    }
    wtr.flush()?;
    Ok(())
}

/// Writes the errors (Euler - analytic) for n=20 and n=1000 to a CSV file in wide format.
///
/// # Arguments
/// * `t_1000` - Time points for the analytic and Euler n=1000 solutions (fine grid)
/// * `y_analytic_1000` - Analytic solution values at `t_1000`
/// * `t_20` - Time points for Euler n=20
/// * `y_euler_20` - Euler method values for n=20
/// * `y_euler_1000` - Euler method values for n=1000
/// * `analytic_fn` - Function to compute the analytic solution at a given t (for n=20 error)
/// * `filename` - Output CSV file name
///
/// # Output
/// Writes a CSV file with columns: t, error_n20, error_n1000
pub fn write_errors_csv(
    t_1000: &[f64],
    y_analytic_1000: &[f64],
    t_20: &[f64],
    y_euler_20: &[f64],
    y_euler_1000: &[f64],
    analytic_fn: &dyn Fn(f64) -> f64,
    filename: &str,
) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_path(filename)?;
    wtr.write_record(["t", "error_n20", "error_n1000"])?;
    let mut idx_20 = 0;
    for (i, &t) in t_1000.iter().enumerate() {
        let analytic = y_analytic_1000[i];
        let error_1000 = y_euler_1000[i] - analytic;
        let error_20 = if idx_20 < t_20.len() && (t - t_20[idx_20]).abs() < 1e-8 {
            let val = y_euler_20[idx_20] - analytic_fn(t_20[idx_20]);
            idx_20 += 1;
            Some(val)
        } else {
            None
        };
        wtr.write_record(&[
            t.to_string(),
            error_20.map(|v| v.to_string()).unwrap_or_default(),
            error_1000.to_string(),
        ])?;
    }
    wtr.flush()?;
    Ok(())
} 