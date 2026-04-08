//! Solver trait — the common interface all CA solvers implement.

use crate::cell::Cell;

/// The common interface shared by all CA solvers (CPU, GPU, etc.).
///
/// Rule registration is NOT part of this trait — it's solver-specific.
/// CPU solvers take Rust closures, GPU solvers take WGSL shader source.
///
/// Contracts (enforced by `ValidatedSolver` in debug builds):
/// - `get(x,y,z)` returns `C::default()` for out-of-bounds coordinates.
/// - `set(x,y,z,c)` followed by `get(x,y,z)` returns `c` (if in-bounds).
/// - `set` on out-of-bounds coordinates is a silent no-op.
/// - `step()` increments `step_count()` by exactly 1.
/// - `width()`, `height()`, `depth()` never change after construction.
pub trait CaSolver<C: Cell> {
    /// Grid width in cells.
    fn width(&self) -> u32;
    /// Grid height in cells.
    fn height(&self) -> u32;
    /// Grid depth in cells.
    fn depth(&self) -> u32;

    /// Get the cell at (x, y, z). Returns `C::default()` for out-of-bounds.
    fn get(&self, x: i32, y: i32, z: i32) -> C;

    /// Set the cell at (x, y, z). No-op for out-of-bounds.
    fn set(&mut self, x: i32, y: i32, z: i32, cell: C);

    /// Advance the automaton by one step.
    fn step(&mut self);

    /// Number of steps completed so far.
    fn step_count(&self) -> u32;

    /// Iterate all cells as `(x, y, z, cell)`.
    ///
    /// For GPU backends, this may trigger a device→host download.
    /// The returned Vec is owned — no lifetime issues across backends.
    fn iter_cells(&self) -> Vec<(u32, u32, u32, C)>;
}
