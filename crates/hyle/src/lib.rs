//! `hyle` — headless voxel simulation library.
//!
//! Hyle provides material physics, cellular automata rules, and
//! simulation algorithms that operate on abstract grid traits.
//! No rendering, no ECS, no game logic.
//!
//! # Crate structure
//!
//! - [`hyle_core`] — types, traits, material model
//! - [`hyle_ca`] — cellular automata rules (gravity, gas, thermal, acoustic)
//!
//! # Usage
//!
//! ```rust,ignore
//! use hyle::prelude::*;
//!
//! // Implement VoxelAccess for your grid type, then:
//! gravity_step(&mut my_world, &my_materials, &mut my_dirty_set);
//! ```

pub use hyle_core;
pub use hyle_ca;

/// Convenience prelude — import everything commonly needed.
pub mod prelude {
    pub use hyle_core::{
        Voxel, VoxelId, MaterialId, AIR_ID,
        VoxelAccess, DirtyTracker, MaterialAccess, VoxelStateAccess,
        MaterialDef, VoxelState,
    };
    pub use hyle_ca::{
        gravity_step, settle, gas_flow_step,
        phase_transition_step, AcousticField, acoustic_propagation_step,
    };
}
