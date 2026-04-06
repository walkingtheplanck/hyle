//! Solver trait — the common interface all CA solvers implement.

use crate::cell::Cell;

/// The simulation interface shared by all solvers (CPU, GPU, etc.).
///
/// Rule registration is NOT part of this trait — it's solver-specific.
/// CPU solvers take Rust `fn()`, GPU solvers take WGSL shader source.
///
/// Contracts (enforced by `ValidatedSolver` in debug builds):
/// - `get(x,y,z)` returns `C::default()` for out-of-bounds coordinates.
/// - `set(x,y,z,c)` followed by `get(x,y,z)` returns `c` (if in-bounds).
/// - `set` on out-of-bounds coordinates is a silent no-op.
/// - `step()` increments `step_count()` by exactly 1.
/// - `width()`, `height()`, `depth()` never change after construction.
pub trait CaSolver<C: Cell> {
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn depth(&self) -> u32;

    /// Get the cell at (x, y, z). Returns `C::default()` for out-of-bounds.
    fn get(&self, x: i32, y: i32, z: i32) -> C;

    /// Set the cell at (x, y, z). No-op for out-of-bounds.
    fn set(&mut self, x: i32, y: i32, z: i32, cell: C);

    /// Advance the simulation by one step.
    fn step(&mut self);

    /// Number of steps completed so far.
    fn step_count(&self) -> u32;
}
