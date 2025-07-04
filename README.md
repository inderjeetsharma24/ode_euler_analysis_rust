# ODE Euler Analysis

This project demonstrates solving an ordinary differential equation (ODE) using the Euler method and compares it to the analytic solution. It generates plots and CSV files to visualize and analyze the results.

## Features
- Solves a simple ODE using the Euler method for different step sizes (n=20 and n=1000)
- Computes the analytic (exact) solution for comparison
- Plots both the solutions and the errors using the `plotters` crate
- Exports results to separate CSV files for each step size

## Requirements
- Rust (latest stable version recommended)
- [plotters](https://crates.io/crates/plotters) crate (installed automatically by Cargo)
- [csv](https://crates.io/crates/csv) crate (installed automatically by Cargo)

## How to Build and Run
1. **Clone the repository:**
   ```sh
   git clone <repo-url>
   cd ode_euler_analysis
   ```
2. **Build the project:**
   ```sh
   cargo build --release
   ```
3. **Run the project:**
   ```sh
   cargo run --release
   ```

## Outputs
- `results_n20.csv`: CSV file with time, analytic solution, and Euler n=20 results (coarse grid)
- `results_n1000.csv`: CSV file with time, analytic solution, and Euler n=1000 results (fine grid)
- `solutions.png`: Plot comparing analytic and Euler solutions for both step sizes
- `errors.png`: Plot showing the error (Euler - analytic) for both step sizes

## File Structure
- `src/main.rs`: Main program logic
- `src/euler_method_solver.rs`: ODE solvers (Euler and analytic)
- `src/plot_utils.rs`: Plotting utilities
- `src/csv_utils.rs`: CSV export utilities

## Notes
- All output files are generated in the project root directory after running the program.
