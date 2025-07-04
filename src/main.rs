mod csv_utils;
mod euler_method_solver;
mod plot_utils;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // This program solves an ODE using the Euler method and compares it to the analytic solution.
    // It plots both solutions and their errors, and writes the results to CSV files.

    // --- Parameter Setup ---
    let t0 = 0.0; // Initial time
    let y0 = 1.0; // Initial value
    let t_end = 5.0; // End time

    //------------------------------------ Solution Computation ------------------------------------
    // Compute Euler and analytic solutions for n=20 (coarse) and n=1000 (fine)
    let (t_20, y_euler_20) = euler_method_solver::euler_solve(t0, y0, t_end, 20);
    let (_, y_analytic_20) = euler_method_solver::analytic_solve(t0, t_end, 20);
    let (t_1000, y_euler_1000) = euler_method_solver::euler_solve(t0, y0, t_end, 1000);
    let (_, y_analytic_1000) = euler_method_solver::analytic_solve(t0, t_end, 1000);

    //--------------------------------------- Plotting Section ------------------------------------
    // Plot the analytic solution and Euler method approximations (n=20 and n=1000) to a PNG file
    plot_utils::plot_solutions(
        &t_1000,
        &y_analytic_1000,
        &t_20,
        &y_euler_20,
        &t_1000,
        &y_euler_1000,
        "solutions.png",
    )?;
    // Plot the errors (Euler - analytic) for n=20 and n=1000 to a PNG file
    plot_utils::plot_errors(
        &t_20,
        &y_euler_20,
        &y_analytic_20,
        &t_1000,
        &y_euler_1000,
        &y_analytic_1000,
        "errors.png",
    )?;

    // -------------------------------------- CSV Writing Section -----------------------------
    // Write the n=20 results to a CSV file
    csv_utils::write_n20_csv(&t_20, &y_analytic_20, &y_euler_20, "results_n20.csv")?;
    // Write the n=1000 results to a CSV file
    csv_utils::write_n1000_csv(
        &t_1000,
        &y_analytic_1000,
        &y_euler_1000,
        "results_n1000.csv",
    )?;

    Ok(())
}
