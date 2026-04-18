use crate::schema::{AttributeComparison, CountComparison, WeightComparison};
use crate::{AttributeId, AttributeValue, MaterialId, NeighborhoodId, RngStreamId};

/// A deterministic rule condition resolved to stable numeric identifiers.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ResolvedCondition {
    /// Compare the count of matching neighboring materials.
    NeighborCount {
        /// Material that neighbors must equal to be counted.
        material: MaterialId,
        /// Count comparison to apply.
        comparison: CountComparison,
    },
    /// Compare the weighted sum of matching neighboring materials.
    NeighborWeightedSum {
        /// Material that neighbors must equal to be included in the sum.
        material: MaterialId,
        /// Weighted comparison to apply.
        comparison: WeightComparison,
    },
    /// Deterministic per-cell random gate derived from step and position.
    RandomChance {
        /// Independent random stream identifier.
        stream: RngStreamId,
        /// True when the derived RNG hits a `1 / n` chance.
        one_in: u32,
    },
    /// Compare a center-cell attached attribute.
    Attribute {
        /// Attached attribute channel to read.
        attribute: AttributeId,
        /// Attribute comparison to apply.
        comparison: AttributeComparison,
    },
    /// Logical conjunction.
    And(Vec<ResolvedCondition>),
    /// Logical disjunction.
    Or(Vec<ResolvedCondition>),
    /// Logical negation.
    Not(Box<ResolvedCondition>),
}

/// One attribute write applied when a rule matches.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AttributeAssignment {
    /// Attached attribute channel to overwrite.
    pub attribute: AttributeId,
    /// Replacement value written to the next state.
    pub value: AttributeValue,
}

impl AttributeAssignment {
    /// Construct a new resolved attribute assignment.
    pub const fn new(attribute: AttributeId, value: AttributeValue) -> Self {
        Self { attribute, value }
    }
}

/// The effect produced by a matching rule.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RuleEffect {
    /// Leave the center material unchanged and stop evaluating later rules.
    Keep,
    /// Replace the center material with a new value and stop evaluating later rules.
    Become(MaterialId),
}

/// One deterministic rule in a [`crate::schema::Blueprint`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Rule {
    /// Exact center material that this rule applies to.
    pub when: MaterialId,
    /// Neighborhood used to sample neighbors for this rule.
    pub neighborhood: NeighborhoodId,
    /// Optional condition that must evaluate to `true`.
    pub condition: Option<ResolvedCondition>,
    /// Attached attribute writes applied when the rule matches.
    pub attribute_updates: Vec<AttributeAssignment>,
    /// Effect applied when the rule matches.
    pub effect: RuleEffect,
}
