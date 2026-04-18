//! Canonical rule-visible randomness derived from position, step, and stream.

use crate::RngStreamId;

use super::Rng;

/// Build the deterministic RNG value for one cell, step, and random stream.
///
/// The schema-level notion of randomness is pure and reproducible: the same
/// `(position, step, stream, seed)` tuple always yields the same generator.
pub fn cell_rng(pos: [i32; 3], step: u32, stream: impl Into<RngStreamId>, seed: u64) -> Rng {
    let stream = stream.into();
    // Runtime positions are expected to be validated logical coordinates before
    // rule-visible randomness is requested. If a malformed caller still passes
    // negative coordinates, clamp them to the origin instead of panicking.
    let [x, y, z] = pos.map(|coord| coord.max(0) as u32);
    Rng::with_stream_and_seed(x, y, z, step, stream, seed)
}
