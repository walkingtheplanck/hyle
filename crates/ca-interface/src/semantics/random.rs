//! Canonical rule-visible randomness derived from position, step, and stream.

use super::Rng;

/// Build the deterministic RNG value for one cell, step, and random stream.
pub fn cell_rng(pos: [i32; 3], step: u32, stream: u32) -> Rng {
    debug_assert!(pos[0] >= 0 && pos[1] >= 0 && pos[2] >= 0);
    Rng::with_stream(pos[0] as u32, pos[1] as u32, pos[2] as u32, step, stream)
}
