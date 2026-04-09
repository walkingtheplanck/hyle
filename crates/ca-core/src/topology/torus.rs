use super::{index::linear_index, Topology};
use crate::{AxisTopology, GridDims, TopologyDescriptor};

/// Coordinates wrap around each axis independently.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TorusTopology;

impl Topology for TorusTopology {
    fn descriptor(&self) -> TopologyDescriptor {
        TopologyDescriptor::uniform(AxisTopology::Wrap)
    }

    fn resolve_index(&self, x: i32, y: i32, z: i32, dims: GridDims, guard_idx: usize) -> usize {
        if dims.width == 0 || dims.height == 0 || dims.depth == 0 {
            return guard_idx;
        }

        let x = wrap_axis(x, dims.width);
        let y = wrap_axis(y, dims.height);
        let z = wrap_axis(z, dims.depth);
        linear_index(x, y, z, dims.width, dims.height)
    }
}

#[inline]
fn wrap_axis(coord: i32, size: u32) -> u32 {
    let size = i64::from(size);
    i64::from(coord).rem_euclid(size) as u32
}
