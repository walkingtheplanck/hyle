//! Umbrella solver trait composed from behavior-focused solver capabilities.

use super::{
    SolverAttributes, SolverCells, SolverExecution, SolverGrid, SolverMetadata, SolverMetrics,
};

/// The common interface shared by all CA solvers (CPU, GPU, etc.).
///
/// Solver functionality is split into behavior-focused capability traits:
///
/// - [`SolverExecution`]: low-level execution, topology, and coordinate resolution
/// - [`SolverMetadata`]: static schema descriptors
/// - [`SolverCells`]: cell handles, coordinates, materials, and neighborhoods
/// - [`SolverAttributes`]: per-cell attribute reads
/// - [`SolverGrid`]: bulk material-grid IO
/// - [`SolverMetrics`]: latest-step and population metrics
///
/// Solver backends are expected to satisfy these contracts:
/// - `get(x,y,z)` and `set(x,y,z,...)` follow `resolve_index(...)`.
/// - If `resolve_index(...)` returns `guard_index()`, `get(...)` returns
///   `MaterialId::default()` and `set(...)` is a silent no-op.
/// - If `resolve_index(...)` returns an index other than `guard_index()`,
///   `get(...)` and `set(...)` must behave as if they were applied to that
///   resolved in-bounds cell index.
/// - `step()` increments `step_count()` by exactly 1.
/// - `width()`, `height()`, `depth()` never change after construction.
/// - After `set(...)`, `write_region(...)`, `replace_cells(...)`, or `step()`
///   returns, subsequent reads through `get(...)`, `iter_materials()`,
///   `read_region(...)`, and `readback()` must observe the latest state.
///   GPU solvers may block internally to satisfy this contract.
pub trait CaSolver:
    SolverExecution + SolverMetadata + SolverCells + SolverAttributes + SolverGrid + SolverMetrics
{
}

impl<T> CaSolver for T where
    T: SolverExecution
        + SolverMetadata
        + SolverCells
        + SolverAttributes
        + SolverGrid
        + SolverMetrics
{
}
