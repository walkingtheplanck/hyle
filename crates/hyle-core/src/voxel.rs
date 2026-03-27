//! A single voxel cell.
//!
//! Stored in a packed 32-bit word so a `Chunk` fits in <= 128 KiB.
//! `material_id == 0` is the canonical *air* (empty) voxel.

/// Stable identifier for a voxel type.  Assigned sequentially by
/// a material registry.
pub type VoxelId = u16;

/// Semantic alias — use this when the intent is "which material is this voxel".
///
/// Both `VoxelId` and `MaterialId` are `u16` under the hood; the alias just
/// makes call-sites more readable.
pub type MaterialId = VoxelId;

/// The air voxel ID — always `0`, pre-registered in every new
/// material registry.
pub const AIR_ID: VoxelId = 0;

/// A single voxel cell stored as two packed `u16` fields.
///
/// Total size: **4 bytes**.
///
/// | Field         | Bits  | Description                              |
/// |---------------|-------|------------------------------------------|
/// | `material_id` | 0-15  | 0 = air, 1-65535 = material ID           |
/// | `flags`       | 16-31 | Reserved for lighting, AO, etc.          |
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct Voxel {
    pub material_id: VoxelId,
    pub flags:       u16,
}

impl Voxel {
    /// The canonical empty/air voxel.
    pub const AIR: Self = Self { material_id: 0, flags: 0 };

    /// Create a solid voxel with the given material ID.
    pub const fn new(id: VoxelId) -> Self {
        Self { material_id: id, flags: 0 }
    }

    /// Return `true` if this is an air (transparent, empty) cell.
    #[inline]
    pub fn is_air(self) -> bool {
        self.material_id == 0
    }
}
