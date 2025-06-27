//! Utility functions for plotting ODE solutions and errors using the plotters crate.
//!
//! This module provides functions to plot the analytic solution and Euler method approximations
//! for different step counts, as well as the errors between the numerical and analytic solutions.

use plotters::prelude::*;

/// Plots the analytic solution, Euler n=20, and Euler n=1000 solutions on a single PNG file.
///
/// # Arguments
/// * `t_analytic` - Time points for the analytic solution (typically a fine grid, e.g., n=1000)
/// * `y_analytic` - Analytic solution values at `t_analytic`
/// * `t_20` - Time points for Euler n=20
/// * `y_20` - Euler method values for n=20
/// * `t_1000` - Time points for Euler n=1000 (should match `t_analytic`)
/// * `y_1000` - Euler method values for n=1000
/// * `filename` - Output PNG file name
///
/// # Output
/// Saves a PNG file with the plotted solutions.
pub fn plot_solutions(
    t_analytic: &[f64],
    y_analytic: &[f64],
    t_20: &[f64],
    y_20: &[f64],
    t_1000: &[f64],
    y_1000: &[f64],
    filename: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(filename, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("Analytic vs Euler Solutions (n=20, n=1000)", ("sans-serif", 30))
        .margin(40)
        .x_label_area_size(40)
        .y_label_area_size(60)
        .build_cartesian_2d(
            t_analytic[0]..*t_analytic.last().unwrap(),
            y_analytic.iter().cloned().fold(f64::INFINITY, f64::min)
                ..y_analytic.iter().cloned().fold(f64::NEG_INFINITY, f64::max),
        )?;
    chart.configure_mesh().draw()?;
    chart.draw_series(LineSeries::new(
        t_analytic.iter().cloned().zip(y_analytic.iter().cloned()),
        &RGBColor(0, 0, 255),
    ))?.label("Analytic").legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RGBColor(0, 0, 255)));
    chart.draw_series(LineSeries::new(
        t_20.iter().cloned().zip(y_20.iter().cloned()),
        &RGBColor(220, 20, 60),
    ))?.label("Euler n=20").legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RGBColor(220, 20, 60)));
    chart.draw_series(LineSeries::new(
        t_1000.iter().cloned().zip(y_1000.iter().cloned()),
        &RGBColor(34, 139, 34),
    ))?.label("Euler n=1000").legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RGBColor(34, 139, 34)));
    chart.configure_series_labels().background_style(&WHITE.mix(0.8)).border_style(&BLACK).draw()?;
    Ok(())
}

/// Plots the errors (Euler - analytic) for n=20 and n=1000 on a single PNG file.
///
/// # Arguments
/// * `t_20` - Time points for Euler n=20
/// * `y_20` - Euler method values for n=20
/// * `y_analytic_20` - Analytic solution values at `t_20`
/// * `t_1000` - Time points for Euler n=1000
/// * `y_1000` - Euler method values for n=1000
/// * `y_analytic_1000` - Analytic solution values at `t_1000`
/// * `filename` - Output PNG file name
///
/// # Output
/// Saves a PNG file with the plotted errors.
pub fn plot_errors(
    t_20: &[f64],
    y_20: &[f64],
    y_analytic_20: &[f64],
    t_1000: &[f64],
    y_1000: &[f64],
    y_analytic_1000: &[f64],
    filename: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(filename, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("Euler Method Errors (n=20, n=1000)", ("sans-serif", 30))
        .margin(40)
        .x_label_area_size(40)
        .y_label_area_size(60)
        .build_cartesian_2d(
            t_20[0]..*t_1000.last().unwrap(),
            -0.5..0.5,
        )?;
    chart.configure_mesh().draw()?;
    let errors_20: Vec<f64> = y_20.iter().zip(y_analytic_20.iter()).map(|(y, ya)| y - ya).collect();
    let errors_1000: Vec<f64> = y_1000.iter().zip(y_analytic_1000.iter()).map(|(y, ya)| y - ya).collect();
    chart.draw_series(LineSeries::new(
        t_20.iter().cloned().zip(errors_20),
        &RGBColor(220, 20, 60),
    ))?.label("Error n=20").legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RGBColor(220, 20, 60)));
    chart.draw_series(LineSeries::new(
        t_1000.iter().cloned().zip(errors_1000),
        &RGBColor(34, 139, 34),
    ))?.label("Error n=1000").legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RGBColor(34, 139, 34)));
    chart.configure_series_labels().background_style(&WHITE.mix(0.8)).border_style(&BLACK).draw()?;
    Ok(())
}
