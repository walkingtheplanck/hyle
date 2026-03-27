//! Per-voxel runtime state.

/// Default chunk size used for state storage.
/// Must match the chunk size used by the grid implementation.
pub const CHUNK_SIZE: usize = 32;

/// Total number of voxels in a single chunk.
pub const CHUNK_VOLUME: usize = CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE;

// -- VoxelState ---------------------------------------------------------------

/// Mutable simulation state for a single voxel.
#[derive(Debug, Clone, Copy)]
pub struct VoxelState {
    pub temperature: f32,
    pub saturation: f32,
    pub stress: f32,
}

impl Default for VoxelState {
    fn default() -> Self {
        Self {
            temperature: 20.0,
            saturation:  0.0,
            stress:      0.0,
        }
    }
}

// -- VoxelStateChunk ----------------------------------------------------------

/// Dense 32^3 array of [`VoxelState`], one per voxel in a chunk.
pub struct VoxelStateChunk(pub Box<[VoxelState; CHUNK_VOLUME]>);

impl VoxelStateChunk {
    pub fn new() -> Self {
        Self(Box::new([VoxelState::default(); CHUNK_VOLUME]))
    }

    #[inline]
    pub fn get(&self, x: usize, y: usize, z: usize) -> VoxelState {
        self.0[Self::index(x, y, z)]
    }

    #[inline]
    pub fn set(&mut self, x: usize, y: usize, z: usize, state: VoxelState) {
        self.0[Self::index(x, y, z)] = state;
    }

    #[inline]
    fn index(x: usize, y: usize, z: usize) -> usize {
        y * CHUNK_SIZE * CHUNK_SIZE + z * CHUNK_SIZE + x
    }
}

impl Default for VoxelStateChunk {
    fn default() -> Self {
        Self::new()
    }
}
