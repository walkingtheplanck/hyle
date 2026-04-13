//! Deterministic per-cell RNG for probabilistic rules.

/// Deterministic per-cell random number generator.
///
/// Produced by the solver from the cell's position and step count.
/// The same (x, y, z, step) always produces the same value —
/// results are reproducible.
#[derive(Clone, Copy, Debug)]
pub struct Rng(u32);

impl Rng {
    /// Create from position and step count.
    #[inline]
    pub fn new(x: u32, y: u32, z: u32, step: u32) -> Self {
        Self::with_stream(x, y, z, step, 0)
    }

    /// Create from position, step count, and an independent stream identifier.
    #[inline]
    pub fn with_stream(x: u32, y: u32, z: u32, step: u32, stream: u32) -> Self {
        let mut h = x.wrapping_mul(2654435761)
            ^ y.wrapping_mul(2246822519)
            ^ z.wrapping_mul(3266489917)
            ^ step.wrapping_mul(668265263)
            ^ stream.wrapping_mul(374761393);
        h = h.wrapping_add(0x9e3779b9);
        h ^= h >> 16;
        h = h.wrapping_mul(0x45d9f3b);
        h ^= h >> 16;
        Rng(h)
    }

    /// Returns `true` with probability `1/n`.
    /// `rng.chance(7)` is true ~14% of the time.
    #[inline]
    pub fn chance(&self, n: u32) -> bool {
        self.0.is_multiple_of(n)
    }

    /// Returns a value in `0..n`.
    #[inline]
    pub fn range(&self, n: u32) -> u32 {
        self.0 % n
    }

    /// Raw hash value for custom use.
    #[inline]
    pub fn raw(&self) -> u32 {
        self.0
    }
}
