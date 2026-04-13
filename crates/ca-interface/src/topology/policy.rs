use crate::{GridDims, TopologyDescriptor};

/// Boundary behavior for solver coordinate access.
pub trait Topology {
    /// Uploadable description of this topology policy.
    fn descriptor(&self) -> TopologyDescriptor;

    /// Resolve a 3D coordinate to a linear cell index.
    ///
    /// The returned index must either be a valid in-bounds cell index or the
    /// supplied `guard_idx`, which represents "no cell" for bounded access.
    fn resolve_index(&self, x: i32, y: i32, z: i32, dims: GridDims, guard_idx: usize) -> usize;
}
