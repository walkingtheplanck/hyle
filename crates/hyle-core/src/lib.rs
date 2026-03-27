//! `hyle-core` — fundamental simulation types, traits, and material model.
//!
//! This crate defines the vocabulary of Hyle: what a voxel is, what
//! materials can do, how interactions are described, and the abstract
//! traits that grid implementations must satisfy.

pub mod voxel;
pub mod traits;
pub mod props;
pub mod interaction;
pub mod bond;
pub mod state;

// -- Re-exports ---------------------------------------------------------------

pub use voxel::{Voxel, VoxelId, MaterialId, AIR_ID};
pub use traits::{VoxelAccess, DirtyTracker, MaterialAccess, VoxelStateAccess};
pub use props::*;
pub use interaction::*;
pub use bond::BondDef;
pub use state::{VoxelState, VoxelStateChunk};
