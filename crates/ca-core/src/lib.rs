pub mod backend;
pub mod cell;
pub mod validated;

pub use backend::CaSolver;
pub use cell::{Action, Cell, GridReader, GridWriter, Neighborhood, Rng};
pub use validated::ValidatedSolver;

/// A rule function: given a neighborhood and RNG, return what happens to the center cell.
///
/// Radius 1 (default) gives 26 neighbors (Moore neighborhood).
/// Higher radii give `(2R+1)³ - 1` neighbors.
pub type Rule<C> = fn(&Neighborhood<C>, Rng) -> Action<C>;

/// A world pass: full grid access, runs as a separate stage after all per-cell rules.
/// Use for global operations like pressure solving, gravity fields, or conservation correction.
pub type WorldPass<C> = fn(&GridReader<C>, &mut GridWriter<C>);
