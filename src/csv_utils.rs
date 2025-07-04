//! Provides functions to write analytic and Euler results to CSV files.

use std::error::Error;

/// Write results for n=20 to a CSV file (t, analytic, euler_n20)
pub fn write_n20_csv(
    t_20: &[f64],
    y_analytic_20: &[f64],
    y_euler_20: &[f64],
    filename: &str,
) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_path(filename)?;
    wtr.write_record(["t", "analytic", "euler_n20"])?;
    for i in 0..t_20.len() {
        wtr.write_record(&[
            t_20[i].to_string(),
            y_analytic_20[i].to_string(),
            y_euler_20[i].to_string(),
        ])?;
    }
    wtr.flush()?;
    Ok(())
}

/// Write results for n=1000 to a CSV file (t, analytic, euler_n1000)
pub fn write_n1000_csv(
    t_1000: &[f64],
    y_analytic_1000: &[f64],
    y_euler_1000: &[f64],
    filename: &str,
) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_path(filename)?;
    wtr.write_record(["t", "analytic", "euler_n1000"])?;
    for i in 0..t_1000.len() {
        wtr.write_record(&[
            t_1000[i].to_string(),
            y_analytic_1000[i].to_string(),
            y_euler_1000[i].to_string(),
        ])?;
    }
    wtr.flush()?;
    Ok(())
}
