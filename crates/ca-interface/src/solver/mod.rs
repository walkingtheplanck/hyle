//! Shared solver-facing runtime interfaces.

mod ca_solver;
mod validated;

pub use ca_solver::CaSolver;
pub use validated::ValidatedSolver;
