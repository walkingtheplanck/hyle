use hyle_ca_contracts::{AxisTopology, GridDims, Topology, TopologyDescriptor};

use super::index::linear_index;

/// Topology adapter backed by a [`TopologyDescriptor`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DescriptorTopology {
    descriptor: TopologyDescriptor,
}

impl DescriptorTopology {
    /// Construct a topology from a backend-neutral descriptor.
    pub const fn new(descriptor: TopologyDescriptor) -> Self {
        Self { descriptor }
    }
}

impl Default for DescriptorTopology {
    fn default() -> Self {
        Self::new(TopologyDescriptor::bounded())
    }
}

impl Topology for DescriptorTopology {
    fn descriptor(&self) -> TopologyDescriptor {
        self.descriptor
    }

    fn resolve_index(&self, x: i32, y: i32, z: i32, dims: GridDims, guard_idx: usize) -> usize {
        match (
            resolve_axis(x, dims.width, self.descriptor.x),
            resolve_axis(y, dims.height, self.descriptor.y),
            resolve_axis(z, dims.depth, self.descriptor.z),
        ) {
            (Some(x), Some(y), Some(z)) => linear_index(x, y, z, dims.width, dims.height),
            _ => guard_idx,
        }
    }
}

fn resolve_axis(coord: i32, size: u32, topology: AxisTopology) -> Option<u32> {
    match topology {
        AxisTopology::Bounded => {
            let coord = coord as u32;
            let max_dim = i32::MAX as u32;
            if size <= max_dim && coord < size {
                Some(coord)
            } else {
                None
            }
        }
        AxisTopology::Wrap => {
            if size == 0 {
                None
            } else {
                Some(i64::from(coord).rem_euclid(i64::from(size)) as u32)
            }
        }
    }
}
