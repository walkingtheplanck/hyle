//! Moore neighborhood: full cube, Chebyshev distance ≤ R.

use super::{impl_neighborhood, Neighborhood, NeighborhoodData};
use crate::Cell;

/// Moore neighborhood: all cells within Chebyshev distance R.
///
/// R=1 → 26 neighbors, R=2 → 124, R=3 → 342. Formula: `(2R+1)³ - 1`.
pub struct MooreNeighborhood<C: Cell>(NeighborhoodData<C>);

impl_neighborhood!(
    MooreNeighborhood,
    |_dx: i32, _dy: i32, _dz: i32, _r: u32| {
        true // all offsets within the cube are included
    }
);
