//! Cheap deterministic per-cell RNG for probabilistic rules.

/// Returns a pseudo-random u32 from position and step counter.
/// Use `rng % N == 0` for 1-in-N probability, etc.
#[inline]
pub fn cell_rng(x: u32, y: u32, z: u32, step: u32) -> u32 {
    let mut h = x.wrapping_mul(2654435761)
        ^ y.wrapping_mul(2246822519)
        ^ z.wrapping_mul(3266489917)
        ^ step.wrapping_mul(668265263);
    h = h.wrapping_add(0x9e3779b9);
    h ^= h >> 16;
    h = h.wrapping_mul(0x45d9f3b);
    h ^= h >> 16;
    h
}
