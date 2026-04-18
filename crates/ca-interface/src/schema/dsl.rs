//! DSL-shaped condition builders for portable rules.

use std::ops::RangeInclusive;

use crate::schema::{AttributeRef, AttributeSet, MaterialRef, MaterialSet};
use crate::WEIGHT_SCALE;
use crate::{AttributeType, AttributeValue, RngStreamId};

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

/// Fixed-point portable weight used by weighted neighborhood predicates.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Weight(u64);

impl Weight {
    /// Construct a weight from raw fixed-point units.
    ///
    /// This exists for advanced callers that already speak the portable
    /// fixed-point representation used by weighted neighborhoods.
    pub const fn raw(units: u64) -> Self {
        Self(units)
    }

    /// Construct the weight corresponding to `cells` uniform neighbors.
    ///
    /// This is usually the ergonomic entry point when authoring rules against
    /// uniform neighborhoods.
    pub const fn cells(cells: u32) -> Self {
        Self(cells as u64 * WEIGHT_SCALE as u64)
    }

    /// Return the raw fixed-point value.
    pub const fn units(self) -> u64 {
        self.0
    }
}

/// One material-scoped attribute assignment used by `material_attributes`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AttrAssign {
    /// Attribute being attached to a material.
    pub attribute: AttributeRef,
    /// Default value for that material.
    pub default: AttributeValue,
}

impl AttrAssign {
    /// Start building a material-scoped attribute assignment.
    ///
    /// The typed first step keeps the authored API ergonomic while the builder
    /// later validates that the default value matches the attribute's type.
    pub fn new<A: AttributeSet>(attribute: A) -> PendingAttrAssign {
        PendingAttrAssign {
            attribute: attribute.attribute(),
        }
    }

    /// Construct a material-scoped attribute assignment with a default value.
    ///
    /// This is a compact form of `AttrAssign::new(attribute).default(value)`.
    pub fn with_default<A: AttributeSet>(attribute: A, default: impl Into<AttributeValue>) -> Self {
        Self {
            attribute: attribute.attribute(),
            default: default.into(),
        }
    }
}

/// Pending material-scoped attribute assignment awaiting its default value.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PendingAttrAssign {
    attribute: AttributeRef,
}

impl PendingAttrAssign {
    /// Finalize the assignment with a material default value.
    ///
    /// The builder later checks that this value matches the declared attribute
    /// scalar type before it reaches the schema.
    pub fn default(self, value: impl Into<AttributeValue>) -> AttrAssign {
        AttrAssign {
            attribute: self.attribute,
            default: value.into(),
        }
    }
}

/// Select neighbors equal to a specific material.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NeighborSelector {
    material: MaterialRef,
}

impl NeighborSelector {
    /// Start a count comparison for the selected material.
    ///
    /// This counts matching neighbor cells, not weighted samples.
    pub fn count(self) -> NeighborCount {
        NeighborCount {
            material: self.material,
        }
    }

    /// Start a weighted-sum comparison for the selected material.
    ///
    /// This uses the neighborhood's interpreted weights rather than plain
    /// neighbor cardinality.
    pub fn weighted_sum(self) -> NeighborWeightedSum {
        NeighborWeightedSum {
            material: self.material,
        }
    }

    /// Require at least one matching neighbor.
    ///
    /// This is shorthand for `neighbors(material).count().at_least(1)`.
    pub fn any(self) -> Condition {
        self.count().at_least(1)
    }

    /// Require no matching neighbors.
    ///
    /// This is shorthand for `neighbors(material).count().eq(0)`.
    pub fn none(self) -> Condition {
        self.count().eq(0)
    }
}

/// A pending numeric comparison on the count of matching neighbors.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NeighborCount {
    material: MaterialRef,
}

impl NeighborCount {
    /// Require an exact neighbor count.
    pub fn eq(self, count: u32) -> Condition {
        Condition::NeighborCount {
            material: self.material,
            comparison: CountComparison::Eq(count),
        }
    }

    /// Require the neighbor count to fall inside an inclusive range.
    pub fn in_range(self, range: RangeInclusive<u32>) -> Condition {
        Condition::NeighborCount {
            material: self.material,
            comparison: CountComparison::InRange {
                min: *range.start(),
                max: *range.end(),
            },
        }
    }

    /// Require the neighbor count to fall outside an inclusive range.
    pub fn not_in(self, range: RangeInclusive<u32>) -> Condition {
        Condition::NeighborCount {
            material: self.material,
            comparison: CountComparison::NotInRange {
                min: *range.start(),
                max: *range.end(),
            },
        }
    }

    /// Require the neighbor count to be at least the given value.
    pub fn at_least(self, count: u32) -> Condition {
        Condition::NeighborCount {
            material: self.material,
            comparison: CountComparison::AtLeast(count),
        }
    }

    /// Require the neighbor count to be at most the given value.
    pub fn at_most(self, count: u32) -> Condition {
        Condition::NeighborCount {
            material: self.material,
            comparison: CountComparison::AtMost(count),
        }
    }
}

/// A pending weighted comparison on matching neighbors.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NeighborWeightedSum {
    material: MaterialRef,
}

impl NeighborWeightedSum {
    /// Require an exact weighted sum.
    pub fn eq(self, weight: Weight) -> Condition {
        Condition::NeighborWeightedSum {
            material: self.material,
            comparison: WeightComparison::Eq(weight),
        }
    }

    /// Require the weighted sum to fall inside an inclusive range.
    pub fn in_range(self, range: RangeInclusive<Weight>) -> Condition {
        Condition::NeighborWeightedSum {
            material: self.material,
            comparison: WeightComparison::InRange {
                min: *range.start(),
                max: *range.end(),
            },
        }
    }

    /// Require the weighted sum to fall outside an inclusive range.
    pub fn not_in(self, range: RangeInclusive<Weight>) -> Condition {
        Condition::NeighborWeightedSum {
            material: self.material,
            comparison: WeightComparison::NotInRange {
                min: *range.start(),
                max: *range.end(),
            },
        }
    }

    /// Require the weighted sum to be at least the given value.
    pub fn at_least(self, weight: Weight) -> Condition {
        Condition::NeighborWeightedSum {
            material: self.material,
            comparison: WeightComparison::AtLeast(weight),
        }
    }

    /// Require the weighted sum to be at most the given value.
    pub fn at_most(self, weight: Weight) -> Condition {
        Condition::NeighborWeightedSum {
            material: self.material,
            comparison: WeightComparison::AtMost(weight),
        }
    }
}

/// Select neighbors equal to a specific material.
///
/// The returned selector is still declarative; no neighborhood is chosen until
/// the surrounding rule either supplies one or falls back to the schema default.
pub fn neighbors<M: MaterialSet>(material: M) -> NeighborSelector {
    NeighborSelector {
        material: material.material(),
    }
}

/// Center-cell attribute selector.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AttributeSelector {
    attribute: AttributeRef,
}

impl AttributeSelector {
    /// Require the attribute to equal an exact value.
    ///
    /// Type compatibility is checked later against the declared attribute type.
    pub fn eq(self, value: impl Into<AttributeValue>) -> Condition {
        Condition::Attribute {
            attribute: self.attribute,
            comparison: AttributeComparison::Eq(value.into()),
        }
    }

    /// Require the attribute to lie inside an inclusive range.
    pub fn in_range<T>(self, range: RangeInclusive<T>) -> Condition
    where
        T: Into<AttributeValue> + Copy,
    {
        Condition::Attribute {
            attribute: self.attribute,
            comparison: AttributeComparison::InRange {
                min: (*range.start()).into(),
                max: (*range.end()).into(),
            },
        }
    }

    /// Require the attribute to lie outside an inclusive range.
    pub fn not_in<T>(self, range: RangeInclusive<T>) -> Condition
    where
        T: Into<AttributeValue> + Copy,
    {
        Condition::Attribute {
            attribute: self.attribute,
            comparison: AttributeComparison::NotInRange {
                min: (*range.start()).into(),
                max: (*range.end()).into(),
            },
        }
    }

    /// Require the attribute to be at least the given value.
    pub fn at_least(self, value: impl Into<AttributeValue>) -> Condition {
        Condition::Attribute {
            attribute: self.attribute,
            comparison: AttributeComparison::AtLeast(value.into()),
        }
    }

    /// Require the attribute to be at most the given value.
    pub fn at_most(self, value: impl Into<AttributeValue>) -> Condition {
        Condition::Attribute {
            attribute: self.attribute,
            comparison: AttributeComparison::AtMost(value.into()),
        }
    }

    /// Return the declared scalar type for this attribute.
    ///
    /// This is mostly useful for helper code that wants to branch while still
    /// staying in the typed DSL world.
    pub const fn value_type(self) -> AttributeType {
        self.attribute.value_type()
    }
}

/// Select the center cell's attached attribute.
///
/// The returned selector targets the rule's center cell, not neighboring cells.
pub fn attr<A: AttributeSet>(attribute: A) -> AttributeSelector {
    AttributeSelector {
        attribute: attribute.attribute(),
    }
}

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
