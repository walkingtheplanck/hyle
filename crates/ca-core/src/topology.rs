//! Coordinate-topology policies for solver access.

/// Boundary behavior for solver coordinate access.
pub trait Topology {
    /// Resolve a single coordinate according to this topology.
    ///
    /// Returns `None` when the coordinate cannot be mapped onto the axis.
    fn map_coord(&self, coord: i32, size: u32) -> Option<u32>;

    /// Resolve a 3D coordinate according to this topology.
    fn resolve_coord(
        &self,
        x: i32,
        y: i32,
        z: i32,
        width: u32,
        height: u32,
        depth: u32,
    ) -> Option<(u32, u32, u32)> {
        Some((
            self.map_coord(x, width)?,
            self.map_coord(y, height)?,
            self.map_coord(z, depth)?,
        ))
    }
}

/// Coordinates outside the grid are treated as out-of-bounds.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct BoundedTopology;

impl Topology for BoundedTopology {
    fn map_coord(&self, coord: i32, size: u32) -> Option<u32> {
        // Motivation: for the common bounded case we want a simple cast-and-
        // compare path instead of a signed conversion branch on each axis.
        //
        // This is correct because we also gate on `size <= i32::MAX`: when the
        // axis size fits in `i32`, any negative `i32` becomes a `u32` value
        // >= 2^31, which is then necessarily >= every valid in-bounds index and
        // rejected by the bounds check. If an implementation reports a larger
        // axis than that, we conservatively return `None` instead of relying on
        // the cast trick, which keeps the default bounded behavior safe in both
        // debug and release builds.
        let coord = coord as u32;
        ((size <= i32::MAX as u32) & (coord < size)).then_some(coord)
    }
}

/// Coordinates wrap around each axis independently.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TorusTopology;

impl Topology for TorusTopology {
    fn map_coord(&self, coord: i32, size: u32) -> Option<u32> {
        if size == 0 {
            return None;
        }

        let size = i64::from(size);
        let wrapped = i64::from(coord).rem_euclid(size);
        Some(wrapped as u32)
    }
}
