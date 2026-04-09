//! Solver trait - the common interface all CA solvers implement.

use crate::{cell::Cell, topology::Topology};

/// The common interface shared by all CA solvers (CPU, GPU, etc.).
///
/// Rule registration is NOT part of this trait - it's solver-specific.
/// CPU solvers take Rust closures, GPU solvers take WGSL shader source.
///
/// Contracts (enforced by `ValidatedSolver` in debug builds):
/// - `get(x,y,z)` and `set(x,y,z,...)` follow `resolve_index(...)`.
/// - If `resolve_index(...)` returns `guard_index()`, `get(...)` returns
///   `C::default()` and `set(...)` is a silent no-op.
/// - If `resolve_index(...)` returns an index other than `guard_index()`,
///   `get(...)` and `set(...)` must behave as if they were applied to that
///   resolved in-bounds cell index.
/// - `step()` increments `step_count()` by exactly 1.
/// - `width()`, `height()`, `depth()` never change after construction.
pub trait CaSolver<C: Cell> {
    /// Topology policy used by this solver.
    type Topology: Topology;

    /// Grid width in cells.
    fn width(&self) -> u32;
    /// Grid height in cells.
    fn height(&self) -> u32;
    /// Grid depth in cells.
    fn depth(&self) -> u32;
    /// Topology policy used to resolve coordinates for reads, writes, and steps.
    fn topology(&self) -> &Self::Topology;

    /// One-past-the-end logical cell index used as the "no cell" sentinel.
    fn guard_index(&self) -> usize {
        let width = self.width() as usize;
        let height = self.height() as usize;
        let depth = self.depth() as usize;

        width
            .checked_mul(height)
            .and_then(|xy| xy.checked_mul(depth))
            .expect("grid cell count must fit in usize")
    }

    /// Resolve a possibly out-of-range coordinate to a linear cell index.
    ///
    /// The default implementation delegates to the solver's topology policy.
    fn resolve_index(&self, x: i32, y: i32, z: i32) -> usize {
        self.topology().resolve_index(
            x,
            y,
            z,
            self.width(),
            self.height(),
            self.depth(),
            self.guard_index(),
        )
    }

    /// Get the cell at (x, y, z) according to `resolve_index(...)`.
    fn get(&self, x: i32, y: i32, z: i32) -> C;

    /// Set the cell at (x, y, z) according to `resolve_index(...)`.
    fn set(&mut self, x: i32, y: i32, z: i32, cell: C);

    /// Advance the automaton by one step.
    fn step(&mut self);

    /// Number of steps completed so far.
    fn step_count(&self) -> u32;

    /// Iterate all cells as `(x, y, z, cell)`.
    ///
    /// For GPU backends, this may trigger a device-to-host download.
    /// The returned Vec is owned - no lifetime issues across backends.
    fn iter_cells(&self) -> Vec<(u32, u32, u32, C)>;
}
