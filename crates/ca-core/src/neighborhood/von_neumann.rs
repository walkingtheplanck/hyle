//! Von Neumann neighborhood: diamond, Manhattan distance ≤ R.

use super::{impl_neighborhood, Neighborhood, NeighborhoodData};
use crate::Cell;

/// Von Neumann neighborhood: cells within Manhattan distance R.
///
/// R=1 → 6 neighbors (face-adjacent), R=2 → 24, R=3 → 62.
pub struct VonNeumannNeighborhood<C: Cell>(NeighborhoodData<C>);

impl_neighborhood!(
    VonNeumannNeighborhood,
    |dx: i32, dy: i32, dz: i32, r: u32| {
        (dx.unsigned_abs() + dy.unsigned_abs() + dz.unsigned_abs()) <= r
    }
);
