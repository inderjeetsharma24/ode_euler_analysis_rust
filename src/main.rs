mod euler_method_solver;
mod plot_utils;
mod csv_utils;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let t0 = 0.0;
    let y0 = 1.0;
    let t_end = 5.0;

    
    // --- 20 steps ---
    let (t_20, y_euler_20) = euler_method_solver::euler_solve(t0, y0, t_end, 20);
    let (_, y_analytic_20) = euler_method_solver::analytic_solve(t0, t_end, 20);

    
    // --- 1000 steps ---
    let (t_1000, y_euler_1000) = euler_method_solver::euler_solve(t0, y0, t_end, 1000);
    let (_, y_analytic_1000) = euler_method_solver::analytic_solve(t0, t_end, 1000);

    // Plot analytic, Euler n=20, and Euler n=1000 solutions
    plot_utils::plot_solutions(
        &t_1000, // Use t_1000 for analytic (more points)
        &y_analytic_1000,
        &t_20,
        &y_euler_20,
        &t_1000,
        &y_euler_1000,
        "solutions.png",
    )?;

    // Plot errors for n=20 and n=1000
    plot_utils::plot_errors(
        &t_20,
        &y_euler_20,
        &y_analytic_20,
        &t_1000,
        &y_euler_1000,
        &y_analytic_1000,
        "errors.png",
    )?;

    // Write CSVs using the modularized csv_utils
    csv_utils::write_solutions_csv(&t_1000, &y_analytic_1000, &t_20, &y_euler_20, &y_euler_1000, "solutions.csv")?;
    csv_utils::write_errors_csv(&t_1000, &y_analytic_1000, &t_20, &y_euler_20, &y_euler_1000, &euler_method_solver::y_analytic, "errors.csv")?;

    Ok(())
}
