/// The right-hand side of the ODE: y' = cos(t) - y
pub fn f(t: f64, y: f64) -> f64 {
    t.cos() - y
}

/// Analytical solution for the ODE: y(t) = (e^-t + cos(t) + sin(t)) / 2
pub fn y_analytic(t: f64) -> f64 {
    ((-t).exp() + t.cos() + t.sin()) / 2.0
}

/// Returns the step size for the Euler method given the interval and number of steps.
pub fn step_size(start: f64, end: f64, n: usize) -> f64 {
    (end - start) / n as f64
}

/// Solves the ODE y' = cos(t) - y using Euler's method.
///
/// Returns two vectors:
/// - t values
/// - y values from Euler's method
pub fn euler_solve(t0: f64, y0: f64, t_end: f64, n: usize) -> (Vec<f64>, Vec<f64>) {
    let h = step_size(t0, t_end, n);
    let mut t = t0;
    let mut y = y0;
    let mut t_values = Vec::new();
    let mut y_values = Vec::new();
    for _ in 0..=n {
        t_values.push(t);
        y_values.push(y);
        y = y + h * f(t, y);
        t = t + h;
    }
    (t_values, y_values)
}

/// Computes the analytic solution y(t) at each step.
pub fn analytic_solve(t0: f64, t_end: f64, n: usize) -> (Vec<f64>, Vec<f64>) {
    let h = step_size(t0, t_end, n);
    let mut t = t0;
    let mut t_values = Vec::new();
    let mut y_values = Vec::new();
    for _ in 0..=n {
        t_values.push(t);
        y_values.push(y_analytic(t));
        t = t + h;
    }
    (t_values, y_values)
}
