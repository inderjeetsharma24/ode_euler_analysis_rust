//! Utility functions for plotting ODE solutions and errors using the plotters library.
//!
//! This module provides functions to plot the analytic solution and Euler method approximations
//! for different step counts, as well as the errors between the numerical and analytic solutions.

use plotters::prelude::*;

/// Plot analytic and Euler solutions for n=20 and n=1000.
pub fn plot_solutions(
    t_analytic: &[f64],
    y_analytic: &[f64],
    t_20: &[f64],
    y_20: &[f64],
    t_1000: &[f64],
    y_1000: &[f64],
    filename: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Create drawing area for PNG output
    let root = BitMapBackend::new(filename, (800, 600)).into_drawing_area();
    // Fill background with white
    root.fill(&WHITE)?;
    // Find minimum y for axis
    let y_min = y_analytic.iter().cloned().fold(f64::INFINITY, f64::min);
    // Find maximum y for axis
    let y_max = y_analytic.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    // Set up chart with title, margin, label areas, and axis ranges
    let mut chart = ChartBuilder::on(&root)
        .caption("Analytic vs Euler Solutions", ("sans-serif", 30))
        .margin(40)
        .x_label_area_size(40)
        .y_label_area_size(60)
        .build_cartesian_2d(t_analytic[0]..*t_analytic.last().unwrap(), y_min..y_max)?;
    // Draw axes and grid
    chart.configure_mesh().draw()?;
    // Plot analytic solution (blue)
    chart
        .draw_series(LineSeries::new(
            t_analytic.iter().cloned().zip(y_analytic.iter().cloned()),
            &RGBColor(0, 0, 255),
        ))?
        .label("Analytic")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RGBColor(0, 0, 255)));
    // Plot Euler n=20 (red)
    chart
        .draw_series(LineSeries::new(
            t_20.iter().cloned().zip(y_20.iter().cloned()),
            &RGBColor(220, 20, 60),
        ))?
        .label("Euler n=20")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RGBColor(220, 20, 60)));
    // Plot Euler n=1000 (green)
    chart
        .draw_series(LineSeries::new(
            t_1000.iter().cloned().zip(y_1000.iter().cloned()),
            &RGBColor(34, 139, 34),
        ))?
        .label("Euler n=1000")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RGBColor(34, 139, 34)));
    // Draw legend
    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;
    // Success
    Ok(())
}

/// Plot errors (Euler - analytic) for n=20 and n=1000.
pub fn plot_errors(
    t_20: &[f64],
    y_20: &[f64],
    y_analytic_20: &[f64],
    t_1000: &[f64],
    y_1000: &[f64],
    y_analytic_1000: &[f64],
    filename: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Create drawing area for PNG output
    let root = BitMapBackend::new(filename, (800, 600)).into_drawing_area();
    // Fill background with white
    root.fill(&WHITE)?;
    // Compute errors for n=20 and n=1000
    let errors_20: Vec<f64> = y_20
        .iter()
        .zip(y_analytic_20)
        .map(|(y, ya)| y - ya)
        .collect();
    let errors_1000: Vec<f64> = y_1000
        .iter()
        .zip(y_analytic_1000)
        .map(|(y, ya)| y - ya)
        .collect();
    // Set up chart with title, margin, label areas, and axis ranges
    let mut chart = ChartBuilder::on(&root)
        .caption("Euler Method Errors", ("sans-serif", 30))
        .margin(40)
        .x_label_area_size(40)
        .y_label_area_size(60)
        .build_cartesian_2d(t_20[0]..*t_1000.last().unwrap(), -0.5..0.5)?;
    // Draw axes and grid
    chart.configure_mesh().draw()?;
    // Plot error for n=20 (red)
    chart
        .draw_series(LineSeries::new(
            t_20.iter().cloned().zip(errors_20),
            &RGBColor(220, 20, 60),
        ))?
        .label("Error n=20")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RGBColor(220, 20, 60)));
    // Plot error for n=1000 (green)
    chart
        .draw_series(LineSeries::new(
            t_1000.iter().cloned().zip(errors_1000),
            &RGBColor(34, 139, 34),
        ))?
        .label("Error n=1000")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RGBColor(34, 139, 34)));
    // Draw legend
    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;
    // Success
    Ok(())
}
