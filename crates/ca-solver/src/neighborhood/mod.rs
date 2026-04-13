//! Runtime neighborhood buffers for the CPU solver.
//!
//! Neighborhood shape, offsets, and falloff are interpreted by
//! [`hyle-ca-semantics`](https://crates.io/crates/hyle-ca-semantics).
//! This module only stores sampled runtime values for the CPU solver.

mod buffer;
mod types;

pub use buffer::Neighborhood;
pub use types::Entry;
