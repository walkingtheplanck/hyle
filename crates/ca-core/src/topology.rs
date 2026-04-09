//! Coordinate-topology policies for solver access.

/// Boundary behavior for solver coordinate access.
pub trait Topology {
    /// Resolve a 3D coordinate to a linear cell index.
    ///
    /// The returned index must either be a valid in-bounds cell index or the
    /// supplied `guard_idx`, which represents "no cell" for bounded access.
    #[allow(clippy::too_many_arguments)]
    fn resolve_index(
        &self,
        x: i32,
        y: i32,
        z: i32,
        width: u32,
        height: u32,
        depth: u32,
        guard_idx: usize,
    ) -> usize;
}

/// Coordinates outside the grid are treated as out-of-bounds.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct BoundedTopology;

impl Topology for BoundedTopology {
    fn resolve_index(
        &self,
        x: i32,
        y: i32,
        z: i32,
        width: u32,
        height: u32,
        depth: u32,
        guard_idx: usize,
    ) -> usize {
        // Motivation: for the common bounded case we want a simple cast-and-
        // compare path instead of a signed conversion branch on each axis.
        //
        // This is correct because we also gate on `size <= i32::MAX`: when the
        // axis size fits in `i32`, any negative `i32` becomes a `u32` value
        // >= 2^31, which is then necessarily >= every valid in-bounds index and
        // rejected by the bounds check. If an implementation reports a larger
        // axis than that, we conservatively return `guard_idx` instead of
        // relying on the cast trick, which keeps bounded behavior safe in both
        // debug and release builds.
        let ux = x as u32;
        let uy = y as u32;
        let uz = z as u32;
        let max_dim = i32::MAX as u32;
        let in_bounds = (width <= max_dim)
            & (height <= max_dim)
            & (depth <= max_dim)
            & (ux < width)
            & (uy < height)
            & (uz < depth);

        if in_bounds {
            linear_index(ux, uy, uz, width, height)
        } else {
            guard_idx
        }
    }
}

/// Coordinates wrap around each axis independently.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TorusTopology;

impl Topology for TorusTopology {
    fn resolve_index(
        &self,
        x: i32,
        y: i32,
        z: i32,
        width: u32,
        height: u32,
        depth: u32,
        guard_idx: usize,
    ) -> usize {
        if width == 0 || height == 0 || depth == 0 {
            return guard_idx;
        }

        let x = wrap_axis(x, width);
        let y = wrap_axis(y, height);
        let z = wrap_axis(z, depth);
        linear_index(x, y, z, width, height)
    }
}

#[inline]
fn wrap_axis(coord: i32, size: u32) -> u32 {
    let size = i64::from(size);
    i64::from(coord).rem_euclid(size) as u32
}

#[inline]
fn linear_index(x: u32, y: u32, z: u32, width: u32, height: u32) -> usize {
    (x as usize)
        + (y as usize) * (width as usize)
        + (z as usize) * (width as usize) * (height as usize)
}
