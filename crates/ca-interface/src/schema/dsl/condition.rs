use crate::schema::{AttributeRef, MaterialRef};
use crate::{AttributeValue, RngStreamId};

use super::Weight;

/// A deterministic rule condition.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Condition {
    /// Compare the count of matching neighbors against a predicate.
    NeighborCount {
        /// Material that neighbors must equal to be counted.
        material: MaterialRef,
        /// Count comparison to apply.
        comparison: CountComparison,
    },
    /// Compare the weighted sum of matching neighbors against a predicate.
    NeighborWeightedSum {
        /// Material that neighbors must equal to be included in the sum.
        material: MaterialRef,
        /// Weighted comparison to apply.
        comparison: WeightComparison,
    },
    /// Deterministic per-cell random gate derived from the step and position.
    RandomChance {
        /// Independent random stream identifier.
        stream: RngStreamId,
        /// True when the derived RNG hits a `1 / n` chance.
        one_in: u32,
    },
    /// Compare the center cell's attached attribute against a predicate.
    Attribute {
        /// Attribute channel read from the center cell.
        attribute: AttributeRef,
        /// Attribute comparison to apply.
        comparison: AttributeComparison,
    },
    /// Logical conjunction.
    And(Vec<Condition>),
    /// Logical disjunction.
    Or(Vec<Condition>),
    /// Logical negation.
    Not(Box<Condition>),
}

impl Condition {
    /// Combine two conditions with logical AND.
    ///
    /// Existing conjunctions are flattened so repeated `.require(...)` calls do
    /// not build deeply nested binary trees for a simple authored rule.
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
    ///
    /// Like [`Condition::and`], this keeps authored disjunctions flat and easy
    /// to validate later.
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
    ///
    /// Negation remains structural here; semantic validation still happens in
    /// the builder once the surrounding rule is known.
    #[must_use]
    pub fn negate(self) -> Self {
        Condition::Not(Box::new(self))
    }
}

/// Comparison used for center-cell attached attributes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AttributeComparison {
    /// Equal to an exact attribute value.
    Eq(AttributeValue),
    /// Within an inclusive range.
    InRange {
        /// Inclusive lower bound.
        min: AttributeValue,
        /// Inclusive upper bound.
        max: AttributeValue,
    },
    /// Outside an inclusive range.
    NotInRange {
        /// Inclusive lower bound.
        min: AttributeValue,
        /// Inclusive upper bound.
        max: AttributeValue,
    },
    /// Greater than or equal to a value.
    AtLeast(AttributeValue),
    /// Less than or equal to a value.
    AtMost(AttributeValue),
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

/// Weighted comparison used for weighted neighbor sums.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WeightComparison {
    /// Equal to an exact weight.
    Eq(Weight),
    /// Within an inclusive range.
    InRange {
        /// Inclusive lower bound.
        min: Weight,
        /// Inclusive upper bound.
        max: Weight,
    },
    /// Outside an inclusive range.
    NotInRange {
        /// Inclusive lower bound.
        min: Weight,
        /// Inclusive upper bound.
        max: Weight,
    },
    /// Greater than or equal to a weight.
    AtLeast(Weight),
    /// Less than or equal to a weight.
    AtMost(Weight),
}
