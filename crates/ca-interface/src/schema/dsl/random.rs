use crate::RngStreamId;

use super::Condition;

/// Deterministic random source selector.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RandomSource {
    stream: RngStreamId,
}

impl RandomSource {
    /// Require a `1 / n` random gate.
    ///
    /// The actual random bit is deterministic for a given cell position, step,
    /// stream, and simulation seed.
    pub fn one_in(self, n: u32) -> Condition {
        Condition::RandomChance {
            stream: self.stream,
            one_in: n,
        }
    }
}

/// Start a deterministic random condition with the given stream id.
///
/// Stream ids let unrelated rules draw independent deterministic randomness
/// without introducing global mutable RNG state.
pub fn rng(stream: impl Into<RngStreamId>) -> RandomSource {
    RandomSource {
        stream: stream.into(),
    }
}
