use hyle_ca_interface::resolved::ResolvedTopology;
use hyle_ca_interface::{GridDims, Topology, TopologyDescriptor};

/// Topology adapter backed by a [`TopologyDescriptor`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DescriptorTopology {
    topology: ResolvedTopology,
}

impl DescriptorTopology {
    /// Construct a topology from a solver-neutral descriptor.
    pub const fn new(descriptor: TopologyDescriptor) -> Self {
        Self {
            topology: ResolvedTopology::from_descriptor(descriptor),
        }
    }
}

impl Default for DescriptorTopology {
    fn default() -> Self {
        Self::new(TopologyDescriptor::bounded())
    }
}

impl Topology for DescriptorTopology {
    fn descriptor(&self) -> TopologyDescriptor {
        self.topology.descriptor()
    }

    fn resolve_index(&self, x: i32, y: i32, z: i32, dims: GridDims, guard_idx: usize) -> usize {
        self.topology.resolve_index(x, y, z, dims, guard_idx)
    }
}
