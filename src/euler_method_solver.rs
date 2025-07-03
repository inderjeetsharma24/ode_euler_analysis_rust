/// The right-hand side of the ODE: y' = cos(t) - y
pub fn f(t: f64, y: f64) -> f64 {
    t.cos() - y
}

/// Analytical solution for the ODE: y(t) = (e^-t + cos(t) + sin(t)) / 2
pub fn y_analytic(t: f64) -> f64 {
    ((-t).exp() + t.cos() + t.sin()) / 2.0
}

/// Computes the step size for the Euler method.
///
/// # Arguments
/// * `start` - Start of the interval
/// * `end` - End of the interval
/// * `n` - Number of steps
///
/// # Returns
/// Step size as f64
pub fn step_size(start: f64, end: f64, n: usize) -> f64 {
    (end - start) / n as f64
}

/// Computes the Euler and analytical solutions for the ODE y' = cos(t) - y.
/// Returns (t_values, y_euler_values, y_analytic_values)
pub fn euler_vs_analytic(t0: f64, y0: f64, t_end: f64, n: usize) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
    let h = step_size(t0, t_end, n);
    let mut t = t0;
    let mut y = y0;
    let mut t_values = Vec::new();
    let mut y_euler_values = Vec::new();
    let mut y_analytic_values = Vec::new();
    for _ in 0..=n {
        t_values.push(t);
        y_euler_values.push(y);
        y_analytic_values.push(y_analytic(t));
        y = y + h * f(t, y);
        t = t + h;
    }
    (t_values, y_euler_values, y_analytic_values)
}
