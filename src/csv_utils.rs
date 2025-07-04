//! Provides functions to write analytic and Euler results, and their errors, to CSV files.

use std::error::Error;

/// Write analytic and Euler solutions to a CSV file.
pub fn write_solutions_csv(
    t_1000: &[f64],
    y_analytic_1000: &[f64],
    t_20: &[f64],
    y_euler_20: &[f64],
    y_euler_1000: &[f64],
    filename: &str,
) -> Result<(), Box<dyn Error>> {
    // Create CSV writer
    let mut wtr = csv::Writer::from_path(filename)?;
    // Write header row
    wtr.write_record(["t", "analytic", "euler_n20", "euler_n1000"])?;
    let mut idx_20 = 0;
    // Write each row of data
    for (i, &t) in t_1000.iter().enumerate() {
        let analytic = y_analytic_1000[i];
        let euler_1000 = y_euler_1000[i];
        // Find matching Euler n=20 value if t matches
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
    // Flush and finish
    wtr.flush()?;
    Ok(())
}

/// Write Euler method errors to a CSV file.
pub fn write_errors_csv(
    t_1000: &[f64],
    y_analytic_1000: &[f64],
    t_20: &[f64],
    y_euler_20: &[f64],
    y_euler_1000: &[f64],
    analytic_fn: &dyn Fn(f64) -> f64,
    filename: &str,
) -> Result<(), Box<dyn Error>> {
    // Create CSV writer
    let mut wtr = csv::Writer::from_path(filename)?;
    // Write header row
    wtr.write_record(["t", "error_n20", "error_n1000"])?;
    let mut idx_20 = 0;
    // Write each row of error data
    for (i, &t) in t_1000.iter().enumerate() {
        let analytic = y_analytic_1000[i];
        let error_1000 = y_euler_1000[i] - analytic;
        // Find matching error for Euler n=20 if t matches
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
    // Flush and finish
    wtr.flush()?;
    Ok(())
} 