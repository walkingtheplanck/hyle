//! `hyle-ca` — cellular automata simulation rules.
//!
//! Pure simulation functions operating on trait-abstracted grids.
//! Each function takes `&mut impl VoxelAccess` — the consumer provides
//! the concrete grid implementation.

pub mod gravity;
pub mod gas;
pub mod phase;
pub mod acoustic;

pub use gravity::{gravity_step, settle};
pub use gas::gas_flow_step;
pub use phase::phase_transition_step;
pub use acoustic::{AcousticField, acoustic_propagation_step};
