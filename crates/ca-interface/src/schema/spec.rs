//! Canonical schema contract types.

use crate::{AttributeId, MaterialId, NeighborhoodId};
use crate::schema::{
    AttributeComparison, AttributeDef, AttributeValue, CountComparison, MaterialDef,
    NeighborhoodSpec, TopologyDescriptor, WeightComparison,
};

use super::BlueprintBuilder;

/// Portable semantics version for a schema contract.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Semantics {
    /// Version 1 semantics: deterministic local rules with first-match wins.
    #[default]
    V1,
}

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
        stream: u32,
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

/// One deterministic rule in a [`Blueprint`].
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

/// Immutable, solver-agnostic schema contract.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Blueprint {
    semantics: Semantics,
    topology: TopologyDescriptor,
    default_material: MaterialId,
    materials: Vec<MaterialDef>,
    attributes: Vec<AttributeDef>,
    neighborhoods: Vec<NeighborhoodSpec>,
    default_neighborhood: NeighborhoodId,
    rules: Vec<Rule>,
}

impl Blueprint {
    /// Start building a solver-agnostic schema.
    pub fn builder() -> BlueprintBuilder {
        BlueprintBuilder::new()
    }

    pub(crate) fn new(
        semantics: Semantics,
        topology: TopologyDescriptor,
        default_material: MaterialId,
        materials: Vec<MaterialDef>,
        attributes: Vec<AttributeDef>,
        neighborhoods: Vec<NeighborhoodSpec>,
        default_neighborhood: NeighborhoodId,
        rules: Vec<Rule>,
    ) -> Self {
        Self {
            semantics,
            topology,
            default_material,
            materials,
            attributes,
            neighborhoods,
            default_neighborhood,
            rules,
        }
    }

    /// The declared semantics version.
    pub fn semantics(&self) -> Semantics {
        self.semantics
    }

    /// The topology descriptor shared across solver implementations.
    pub fn topology(&self) -> TopologyDescriptor {
        self.topology
    }

    /// Material used to initialize empty runtime grids and guard reads.
    pub fn default_material(&self) -> MaterialId {
        self.default_material
    }

    /// Declared material universe with attached attributes.
    pub fn materials(&self) -> &[MaterialDef] {
        &self.materials
    }

    /// Declared attached per-cell attributes.
    pub fn attributes(&self) -> &[AttributeDef] {
        &self.attributes
    }

    /// Reusable named neighborhoods referenced by rules.
    pub fn neighborhoods(&self) -> &[NeighborhoodSpec] {
        &self.neighborhoods
    }

    /// Default neighborhood used by rules that do not override it.
    pub fn default_neighborhood(&self) -> NeighborhoodId {
        self.default_neighborhood
    }

    /// Ordered rules evaluated with first-match-wins semantics.
    pub fn rules(&self) -> &[Rule] {
        &self.rules
    }
}
