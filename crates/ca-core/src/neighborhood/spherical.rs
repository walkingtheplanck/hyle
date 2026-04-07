//! Spherical neighborhood: Euclidean distance ≤ R.

use super::{impl_neighborhood, Neighborhood, NeighborhoodData};
use crate::Cell;

/// Spherical neighborhood: cells within Euclidean distance R.
///
/// R=1 → 6 neighbors, R=2 → 32, R=3 → 122.
pub struct SphericalNeighborhood<C: Cell>(NeighborhoodData<C>);

impl_neighborhood!(
    SphericalNeighborhood,
    |dx: i32, dy: i32, dz: i32, r: u32| { ((dx * dx + dy * dy + dz * dz) as u32) <= r * r }
);
