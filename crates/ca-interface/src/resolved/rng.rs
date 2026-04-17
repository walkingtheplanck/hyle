//! Canonical deterministic RNG used by rule semantics.

use crate::RngStreamId;

/// Deterministic per-cell random number generator.
///
/// Produced from the cell's position, step count, and stream identifier.
/// The same inputs always produce the same value on every backend.
#[derive(Clone, Copy, Debug)]
pub struct Rng(u64);

impl Rng {
    /// Create from position and step count.
    #[inline]
    pub fn new(x: u32, y: u32, z: u32, step: u32) -> Self {
        Self::with_stream_and_seed(x, y, z, step, RngStreamId::new(0), 0)
    }

    /// Create from position, step count, and an independent stream identifier.
    #[inline]
    pub fn with_stream(
        x: u32,
        y: u32,
        z: u32,
        step: u32,
        stream: impl Into<RngStreamId>,
    ) -> Self {
        Self::with_stream_and_seed(x, y, z, step, stream, 0)
    }

    /// Create from position, step count, and a deterministic run seed.
    #[inline]
    pub fn with_seed(x: u32, y: u32, z: u32, step: u32, seed: u64) -> Self {
        Self::with_stream_and_seed(x, y, z, step, RngStreamId::new(0), seed)
    }

    /// Create from position, step count, stream, and deterministic run seed.
    #[inline]
    pub fn with_stream_and_seed(
        x: u32,
        y: u32,
        z: u32,
        step: u32,
        stream: impl Into<RngStreamId>,
        seed: u64,
    ) -> Self {
        let stream = stream.into();
        let mut h = seed
            ^ (x as u64).wrapping_mul(0x9E37_79B1_85EB_CA87)
            ^ (y as u64).wrapping_mul(0xC2B2_AE3D_27D4_EB4F)
            ^ (z as u64).wrapping_mul(0x1656_67B1_9E37_79F9)
            ^ (step as u64).wrapping_mul(0x85EB_CA77_C2B2_AE63)
            ^ (stream.raw() as u64).wrapping_mul(0x27D4_EB2F_1656_67C5);
        h ^= h >> 33;
        h = h.wrapping_mul(0xff51_afd7_ed55_8ccd);
        h ^= h >> 33;
        h = h.wrapping_mul(0xc4ce_b9fe_1a85_ec53);
        h ^= h >> 33;
        Rng(h)
    }

    /// Returns `true` with probability `1/n`.
    #[inline]
    pub fn chance(&self, n: u32) -> bool {
        self.0.is_multiple_of(n as u64)
    }

    /// Returns a value in `0..n`.
    #[inline]
    pub fn range(&self, n: u32) -> u32 {
        (self.0 % n as u64) as u32
    }

    /// Raw hash value for custom use.
    #[inline]
    pub fn raw(&self) -> u64 {
        self.0
    }
}
