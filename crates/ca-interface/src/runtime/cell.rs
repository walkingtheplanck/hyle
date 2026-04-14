//! Solver-facing cell model trait.

use crate::contracts::CellState;

/// A runtime cell model used by solvers.
///
/// This extends [`CellState`] with solver-facing hooks used during
/// neighborhood evaluation and optional rule dispatch optimizations.
pub trait Cell: CellState {
    /// A compact solver-defined dispatch key for this cell.
    ///
    /// Some solvers use exact-state matching and ignore this value entirely;
    /// others may still use it to bucket rules or choose fast paths.
    fn rule_id(&self) -> u8;

    /// Whether this cell counts as "alive" for solver neighborhood helpers.
    fn is_alive(&self) -> bool;
}

/// Default runtime cell implementation for `u32`.
///
/// Low byte = `rule_id`, non-zero = alive.
impl Cell for u32 {
    #[inline]
    fn rule_id(&self) -> u8 {
        (*self & 0xFF) as u8
    }

    #[inline]
    fn is_alive(&self) -> bool {
        *self != 0
    }
}
