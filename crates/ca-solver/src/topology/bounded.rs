use hyle_ca_interface::{AxisTopology, GridDims, Topology, TopologyDescriptor};

use super::index::linear_index;

/// Coordinates outside the grid are treated as out-of-bounds.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct BoundedTopology;

impl Topology for BoundedTopology {
    fn descriptor(&self) -> TopologyDescriptor {
        TopologyDescriptor::uniform(AxisTopology::Bounded)
    }

    fn resolve_index(&self, x: i32, y: i32, z: i32, dims: GridDims, guard_idx: usize) -> usize {
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
        let in_bounds = (dims.width() <= max_dim)
            & (dims.height() <= max_dim)
            & (dims.depth() <= max_dim)
            & (ux < dims.width())
            & (uy < dims.height())
            & (uz < dims.depth());

        if in_bounds {
            linear_index(ux, uy, uz, dims.width(), dims.height())
        } else {
            guard_idx
        }
    }
}
