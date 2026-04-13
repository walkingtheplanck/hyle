//! DSL-shaped condition builders for portable rules.

use std::ops::RangeInclusive;

use crate::CellState;

/// A deterministic rule condition.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Condition<C: CellState> {
    /// Compare the count of matching neighbors against a predicate.
    NeighborCount {
        /// State that neighbors must equal to be counted.
        state: C,
        /// Count comparison to apply.
        comparison: CountComparison,
    },
    /// Deterministic per-cell random gate derived from the step and position.
    RandomChance {
        /// Independent random stream identifier.
        stream: u32,
        /// True when the derived RNG hits a `1 / n` chance.
        one_in: u32,
    },
    /// Logical conjunction.
    And(Vec<Condition<C>>),
    /// Logical disjunction.
    Or(Vec<Condition<C>>),
    /// Logical negation.
    Not(Box<Condition<C>>),
}

impl<C: CellState> Condition<C> {
    /// Combine two conditions with logical AND.
    #[must_use]
    pub fn and(self, other: Self) -> Self {
        match (self, other) {
            (Condition::And(mut left), Condition::And(right)) => {
                left.extend(right);
                Condition::And(left)
            }
            (Condition::And(mut left), right) => {
                left.push(right);
                Condition::And(left)
            }
            (left, Condition::And(mut right)) => {
                let mut all = vec![left];
                all.append(&mut right);
                Condition::And(all)
            }
            (left, right) => Condition::And(vec![left, right]),
        }
    }

    /// Combine two conditions with logical OR.
    #[must_use]
    pub fn or(self, other: Self) -> Self {
        match (self, other) {
            (Condition::Or(mut left), Condition::Or(right)) => {
                left.extend(right);
                Condition::Or(left)
            }
            (Condition::Or(mut left), right) => {
                left.push(right);
                Condition::Or(left)
            }
            (left, Condition::Or(mut right)) => {
                let mut any = vec![left];
                any.append(&mut right);
                Condition::Or(any)
            }
            (left, right) => Condition::Or(vec![left, right]),
        }
    }

    /// Negate a condition.
    #[must_use]
    pub fn negate(self) -> Self {
        Condition::Not(Box::new(self))
    }

    /// The matching state used by this condition if it is a direct neighbor count.
    pub fn state(&self) -> Option<&C> {
        match self {
            Condition::NeighborCount { state, .. } => Some(state),
            Condition::RandomChance { .. }
            | Condition::And(_)
            | Condition::Or(_)
            | Condition::Not(_) => None,
        }
    }

    /// The count comparison used by this condition if it is a direct neighbor count.
    pub fn comparison(&self) -> Option<CountComparison> {
        match self {
            Condition::NeighborCount { comparison, .. } => Some(*comparison),
            Condition::RandomChance { .. }
            | Condition::And(_)
            | Condition::Or(_)
            | Condition::Not(_) => None,
        }
    }
}

/// Numeric comparison used for neighbor counts.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CountComparison {
    /// Equal to an exact count.
    Eq(u32),
    /// Within an inclusive range.
    InRange {
        /// Inclusive lower bound.
        min: u32,
        /// Inclusive upper bound.
        max: u32,
    },
    /// Outside an inclusive range.
    NotInRange {
        /// Inclusive lower bound.
        min: u32,
        /// Inclusive upper bound.
        max: u32,
    },
    /// Greater than or equal to a count.
    AtLeast(u32),
    /// Less than or equal to a count.
    AtMost(u32),
}

/// Select neighbors equal to a specific cell state.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NeighborSelector<C: CellState> {
    state: C,
}

impl<C: CellState> NeighborSelector<C> {
    /// Start a count comparison for the selected state.
    pub fn count(self) -> NeighborCount<C> {
        NeighborCount { state: self.state }
    }

    /// Require at least one matching neighbor.
    pub fn any(self) -> Condition<C> {
        self.count().at_least(1)
    }

    /// Require no matching neighbors.
    pub fn none(self) -> Condition<C> {
        self.count().eq(0)
    }
}

/// A pending numeric comparison on the count of matching neighbors.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NeighborCount<C: CellState> {
    state: C,
}

impl<C: CellState> NeighborCount<C> {
    /// Require an exact neighbor count.
    pub fn eq(self, count: u32) -> Condition<C> {
        Condition::NeighborCount {
            state: self.state,
            comparison: CountComparison::Eq(count),
        }
    }

    /// Require the neighbor count to fall inside an inclusive range.
    pub fn in_range(self, range: RangeInclusive<u32>) -> Condition<C> {
        Condition::NeighborCount {
            state: self.state,
            comparison: CountComparison::InRange {
                min: *range.start(),
                max: *range.end(),
            },
        }
    }

    /// Require the neighbor count to fall outside an inclusive range.
    pub fn not_in(self, range: RangeInclusive<u32>) -> Condition<C> {
        Condition::NeighborCount {
            state: self.state,
            comparison: CountComparison::NotInRange {
                min: *range.start(),
                max: *range.end(),
            },
        }
    }

    /// Require the neighbor count to be at least the given value.
    pub fn at_least(self, count: u32) -> Condition<C> {
        Condition::NeighborCount {
            state: self.state,
            comparison: CountComparison::AtLeast(count),
        }
    }

    /// Require the neighbor count to be at most the given value.
    pub fn at_most(self, count: u32) -> Condition<C> {
        Condition::NeighborCount {
            state: self.state,
            comparison: CountComparison::AtMost(count),
        }
    }
}

/// Select neighbors equal to `state`.
pub fn neighbors<C: CellState>(state: C) -> NeighborSelector<C> {
    NeighborSelector { state }
}

/// Select a deterministic random stream for a rule condition.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RandomSource {
    stream: u32,
}

impl RandomSource {
    /// Require a deterministic `1 / n` random hit for this cell, step, and stream.
    pub fn one_in<C: CellState>(self, n: u32) -> Condition<C> {
        Condition::RandomChance {
            stream: self.stream,
            one_in: n,
        }
    }
}

/// Select a deterministic random stream for rule-visible randomness.
pub fn rng(stream: u32) -> RandomSource {
    RandomSource { stream }
}
