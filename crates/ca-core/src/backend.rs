//! Solver trait - the common interface all CA solvers implement.

use crate::cell::Cell;

/// The common interface shared by all CA solvers (CPU, GPU, etc.).
///
/// Rule registration is NOT part of this trait - it's solver-specific.
/// CPU solvers take Rust closures, GPU solvers take WGSL shader source.
///
/// Contracts (enforced by `ValidatedSolver` in debug builds):
/// - The default bounded `resolve_coord(...)` implementation assumes
///   `width()`, `height()`, and `depth()` are each `<= i32::MAX`.
/// - `get(x,y,z)` and `set(x,y,z,...)` follow `resolve_coord(...)`.
/// - If `resolve_coord(...)` returns `None`, `get(...)` returns `C::default()`
///   and `set(...)` is a silent no-op.
/// - If `resolve_coord(...)` returns `Some((ix,iy,iz))`, `get(...)` and `set(...)`
///   must behave as if they were applied to that resolved in-bounds coordinate.
/// - `step()` increments `step_count()` by exactly 1.
/// - `width()`, `height()`, `depth()` never change after construction.
pub trait CaSolver<C: Cell> {
    /// Grid width in cells.
    fn width(&self) -> u32;
    /// Grid height in cells.
    fn height(&self) -> u32;
    /// Grid depth in cells.
    fn depth(&self) -> u32;

    /// Resolve a possibly out-of-range coordinate to an in-bounds cell.
    ///
    /// The default implementation treats the solver as bounded.
    fn resolve_coord(&self, x: i32, y: i32, z: i32) -> Option<(u32, u32, u32)> {
        let width = self.width();
        let height = self.height();
        let depth = self.depth();

        debug_assert!(width <= i32::MAX as u32);
        debug_assert!(height <= i32::MAX as u32);
        debug_assert!(depth <= i32::MAX as u32);

        // Motivation: for the common bounded case we want a simple cast-and-
        // compare path instead of a signed conversion branch on each axis.
        //
        // This is correct because of the `<= i32::MAX` dimension invariant:
        // any negative `i32` becomes a `u32` value >= 2^31, which is then
        // necessarily >= every valid dimension and rejected by the bounds check.
        let x = x as u32;
        let y = y as u32;
        let z = z as u32;

        ((x < width) & (y < height) & (z < depth)).then_some((x, y, z))
    }

    /// Get the cell at (x, y, z) according to `resolve_coord(...)`.
    fn get(&self, x: i32, y: i32, z: i32) -> C;

    /// Set the cell at (x, y, z) according to `resolve_coord(...)`.
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
