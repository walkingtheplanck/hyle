//! Abstract traits for grid access.
//!
//! Consumers (game engines, tools) implement these for their specific
//! storage layout. Hyle's simulation algorithms operate on these traits.

use glam::IVec3;
use crate::voxel::{Voxel, MaterialId};
use crate::props::MaterialDef;
use crate::state::VoxelState;

/// Abstract read/write access to a voxel grid.
pub trait VoxelAccess {
    /// Get the voxel at a world position. Returns AIR for out-of-bounds.
    fn get_voxel(&self, x: i32, y: i32, z: i32) -> Voxel;

    /// Set the voxel at a world position. No-op for out-of-bounds.
    fn set_voxel(&mut self, x: i32, y: i32, z: i32, voxel: Voxel);

    /// Set a voxel, auto-creating the chunk if needed.
    fn set_voxel_or_create(&mut self, x: i32, y: i32, z: i32, voxel: Voxel);

    /// Whether this position is within loaded/valid space.
    fn is_valid(&self, x: i32, y: i32, z: i32) -> bool;

    /// Collect all non-air voxels as `(x, y, z, Voxel)` tuples.
    fn iter_voxels(&self) -> Vec<(i32, i32, i32, Voxel)>;
}

/// Track which regions were modified by a simulation step.
pub trait DirtyTracker {
    /// Mark a chunk-coordinate as dirty (needs remeshing/processing).
    fn mark_dirty(&mut self, chunk_pos: IVec3);
}

impl DirtyTracker for std::collections::HashSet<IVec3> {
    fn mark_dirty(&mut self, chunk_pos: IVec3) {
        self.insert(chunk_pos);
    }
}

/// Read-only access to material definitions.
pub trait MaterialAccess {
    fn get_material(&self, id: MaterialId) -> &MaterialDef;

    /// Granular and liquid voxels cannot fall below this world-space Y coordinate.
    fn bedrock_y(&self) -> i32;
}

/// Abstract read/write access to per-voxel runtime state (temperature, etc.).
pub trait VoxelStateAccess {
    fn get_state(&self, x: i32, y: i32, z: i32) -> VoxelState;
}
