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
