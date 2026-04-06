pub mod cell;
pub mod backend;
pub mod validated;

pub use cell::{cell_rng, Action, Cell, Direction, GridReader, GridWriter, Neighborhood, Region};
pub use backend::CaSolver;
pub use validated::ValidatedSolver;

/// Tier 1: local rule — radius-1 neighborhood, cheapest, GPU-perfect.
pub type Rule<C> = fn(Neighborhood<C>, u32) -> Action<C>;

/// Tier 2: regional rule — configurable radius, pre-fetched region.
/// More expensive: fetches `(2R+1)³ - 1` cells per evaluation.
pub type RegionalRule<C> = fn(&Region<C>, u32) -> Action<C>;

/// Tier 3: world pass — full grid access, runs as a separate stage
/// after all per-cell rules. Use for global operations like pressure
/// solving, gravity fields, or conservation correction.
pub type WorldPass<C> = fn(&GridReader<C>, &mut GridWriter<C>);
