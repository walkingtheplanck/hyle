//! Declarative cell-state constraints used by blueprint contracts.

/// A portable cell value that can appear in a blueprint contract.
///
/// This trait captures the data-level requirements shared by
/// blueprint specs, descriptors, and analysis. Solver-facing behavior
/// belongs to [`crate::Cell`], which extends this trait.
pub trait CellState: Copy + Default + Eq + Send + Sync + 'static {}

impl<T> CellState for T where T: Copy + Default + Eq + Send + Sync + 'static {}
