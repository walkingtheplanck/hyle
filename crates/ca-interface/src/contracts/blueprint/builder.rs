//! Builder types for authoring portable blueprints.

use std::error::Error;
use std::fmt::{Display, Formatter};

use crate::{
    AttributeComparison, AttributeDef, AttributeType, AttributeValue, CellModel, NeighborhoodSpec,
    TopologyDescriptor,
};

use super::{
    AttributeAssignment, Blueprint, Condition, NamedNeighborhood, Rule, RuleEffect, Semantics,
};

const ADJACENT_NEIGHBORHOOD: &str = "adjacent";

/// Errors raised while building a [`Blueprint`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BuildError {
    /// An attribute name was empty.
    EmptyAttributeName,
    /// An attribute name was registered more than once.
    DuplicateAttribute(String),
    /// A rule referenced an empty attribute name in a condition.
    EmptyConditionAttribute,
    /// A rule referenced an unknown attribute name in a condition.
    UnknownConditionAttribute(String),
    /// A rule attempted to update an empty attribute name.
    EmptyRuleAttributeUpdate,
    /// A rule attempted to update an unknown attribute name.
    UnknownRuleAttributeUpdate(String),
    /// A rule attempted to update the same attribute more than once.
    DuplicateRuleAttributeUpdate(String),
    /// A rule used an attribute value whose scalar type does not match the declaration.
    AttributeTypeMismatch {
        /// Referenced attribute name.
        attribute: String,
        /// Declared scalar type.
        expected: AttributeType,
        /// Actual scalar type provided by the rule.
        actual: AttributeType,
    },
    /// A rule used an unsupported comparison for the attribute's scalar type.
    UnsupportedAttributeComparison {
        /// Referenced attribute name.
        attribute: String,
        /// Comparison kind used by the rule.
        comparison: &'static str,
        /// Declared scalar type.
        value_type: AttributeType,
    },
    /// A neighborhood name was empty.
    EmptyNeighborhoodName,
    /// The default neighborhood name was empty.
    EmptyDefaultNeighborhood,
    /// A rule referenced an empty neighborhood name.
    EmptyRuleNeighborhood,
    /// A neighborhood name was registered more than once.
    DuplicateNeighborhood(String),
    /// The default neighborhood name does not exist.
    UnknownDefaultNeighborhood(String),
    /// A rule references a neighborhood name that does not exist.
    UnknownRuleNeighborhood(String),
    /// A random condition requested an invalid denominator.
    InvalidRandomChance {
        /// Random stream identifier used by the invalid condition.
        stream: u32,
        /// Requested `1 / n` denominator.
        one_in: u32,
    },
}

impl Display for BuildError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BuildError::EmptyAttributeName => {
                write!(f, "attribute name must not be empty")
            }
            BuildError::DuplicateAttribute(name) => {
                write!(f, "duplicate attribute name: {name}")
            }
            BuildError::EmptyConditionAttribute => {
                write!(f, "rule condition attribute name must not be empty")
            }
            BuildError::UnknownConditionAttribute(name) => {
                write!(f, "rule condition references unknown attribute: {name}")
            }
            BuildError::EmptyRuleAttributeUpdate => {
                write!(f, "rule attribute update name must not be empty")
            }
            BuildError::UnknownRuleAttributeUpdate(name) => {
                write!(f, "rule updates unknown attribute: {name}")
            }
            BuildError::DuplicateRuleAttributeUpdate(name) => {
                write!(f, "rule updates attribute more than once: {name}")
            }
            BuildError::AttributeTypeMismatch {
                attribute,
                expected,
                actual,
            } => write!(
                f,
                "attribute '{attribute}' expects value type {:?}, got {:?}",
                expected, actual
            ),
            BuildError::UnsupportedAttributeComparison {
                attribute,
                comparison,
                value_type,
            } => write!(
                f,
                "attribute '{attribute}' does not support comparison '{comparison}' for {:?}",
                value_type
            ),
            BuildError::EmptyNeighborhoodName => {
                write!(f, "neighborhood name must not be empty")
            }
            BuildError::EmptyDefaultNeighborhood => {
                write!(f, "default neighborhood name must not be empty")
            }
            BuildError::EmptyRuleNeighborhood => {
                write!(f, "rule neighborhood name must not be empty")
            }
            BuildError::DuplicateNeighborhood(name) => {
                write!(f, "duplicate neighborhood name: {name}")
            }
            BuildError::UnknownDefaultNeighborhood(name) => {
                write!(f, "unknown default neighborhood: {name}")
            }
            BuildError::UnknownRuleNeighborhood(name) => {
                write!(f, "rule references unknown neighborhood: {name}")
            }
            BuildError::InvalidRandomChance { stream, one_in } => write!(
                f,
                "random condition on stream {stream} must use a non-zero denominator, got {one_in}"
            ),
        }
    }
}

impl Error for BuildError {}

/// Typed blueprint builder.
pub struct BlueprintBuilder<C: CellModel> {
    semantics: Semantics,
    topology: TopologyDescriptor,
    attributes: Vec<AttributeDef>,
    neighborhoods: Vec<NamedNeighborhood>,
    default_neighborhood: String,
    rules: Vec<PendingRule<C>>,
}

impl<C: CellModel> BlueprintBuilder<C> {
    pub(crate) fn new() -> Self {
        Self {
            semantics: Semantics::V1,
            topology: TopologyDescriptor::bounded(),
            attributes: Vec::new(),
            neighborhoods: vec![NamedNeighborhood::new(
                ADJACENT_NEIGHBORHOOD,
                NeighborhoodSpec::adjacent(),
            )],
            default_neighborhood: ADJACENT_NEIGHBORHOOD.to_string(),
            rules: Vec::new(),
        }
    }

    /// Override the topology descriptor used by this blueprint.
    pub fn topology(mut self, topology: TopologyDescriptor) -> Self {
        self.topology = topology;
        self
    }

    /// Register a reusable attached per-cell attribute with a zero default.
    pub fn attribute(mut self, name: impl Into<String>, value_type: AttributeType) -> Self {
        self.attributes.push(AttributeDef::new(name, value_type));
        self
    }

    /// Register a reusable attached per-cell attribute with an explicit default.
    pub fn attribute_with_default(
        mut self,
        name: impl Into<String>,
        default: AttributeValue,
    ) -> Self {
        self.attributes
            .push(AttributeDef::with_default(name, default));
        self
    }

    /// Register a reusable attached per-cell attribute descriptor.
    pub fn attribute_def(mut self, attribute: AttributeDef) -> Self {
        self.attributes.push(attribute);
        self
    }

    /// Register a reusable named neighborhood.
    pub fn neighborhood(mut self, name: impl Into<String>, spec: NeighborhoodSpec) -> Self {
        self.neighborhoods.push(NamedNeighborhood::new(name, spec));
        self
    }

    /// Set the default neighborhood used by rules that do not override it.
    pub fn default_neighborhood(mut self, name: impl Into<String>) -> Self {
        self.default_neighborhood = name.into();
        self
    }

    /// Add rules through the DSL-shaped rule builder.
    pub fn rules(mut self, build: impl FnOnce(&mut RulesBuilder<C>)) -> Self {
        let mut rules = RulesBuilder::new();
        build(&mut rules);
        self.rules.extend(rules.finish());
        self
    }

    /// Validate and build the portable blueprint.
    pub fn build(self) -> Result<Blueprint<C>, BuildError> {
        if self.default_neighborhood.is_empty() {
            return Err(BuildError::EmptyDefaultNeighborhood);
        }

        let mut attribute_names = Vec::with_capacity(self.attributes.len());
        for attribute in &self.attributes {
            if attribute.name.is_empty() {
                return Err(BuildError::EmptyAttributeName);
            }
            if attribute_names
                .iter()
                .any(|name: &String| name == &attribute.name)
            {
                return Err(BuildError::DuplicateAttribute(attribute.name.clone()));
            }
            attribute_names.push(attribute.name.clone());
        }

        let mut names = Vec::with_capacity(self.neighborhoods.len());
        for neighborhood in &self.neighborhoods {
            if neighborhood.name.is_empty() {
                return Err(BuildError::EmptyNeighborhoodName);
            }
            if names.iter().any(|name: &String| name == &neighborhood.name) {
                return Err(BuildError::DuplicateNeighborhood(neighborhood.name.clone()));
            }
            names.push(neighborhood.name.clone());
        }

        let default_neighborhood = names
            .iter()
            .position(|name| name == &self.default_neighborhood)
            .ok_or_else(|| {
                BuildError::UnknownDefaultNeighborhood(self.default_neighborhood.clone())
            })?;

        let mut rules = Vec::with_capacity(self.rules.len());
        for rule in self.rules {
            validate_condition(rule.condition.as_ref(), &self.attributes)?;
            validate_attribute_updates(&rule.attribute_updates, &self.attributes)?;
            let neighborhood_name = rule
                .neighborhood
                .unwrap_or_else(|| self.default_neighborhood.clone());
            if neighborhood_name.is_empty() {
                return Err(BuildError::EmptyRuleNeighborhood);
            }
            let neighborhood = names
                .iter()
                .position(|name| name == &neighborhood_name)
                .ok_or_else(|| BuildError::UnknownRuleNeighborhood(neighborhood_name.clone()))?;
            rules.push(Rule {
                when: rule.when,
                neighborhood,
                condition: rule.condition,
                attribute_updates: rule.attribute_updates,
                effect: rule.effect,
            });
        }

        Ok(Blueprint::new(
            C::schema(),
            self.semantics,
            self.topology,
            self.attributes,
            self.neighborhoods,
            default_neighborhood,
            rules,
        ))
    }
}

fn validate_condition<C: CellModel>(
    condition: Option<&Condition<C>>,
    attributes: &[AttributeDef],
) -> Result<(), BuildError> {
    let Some(condition) = condition else {
        return Ok(());
    };

    match condition {
        Condition::NeighborCount { .. } | Condition::NeighborWeightedSum { .. } => Ok(()),
        Condition::RandomChance { stream, one_in } => {
            if *one_in == 0 {
                Err(BuildError::InvalidRandomChance {
                    stream: *stream,
                    one_in: *one_in,
                })
            } else {
                Ok(())
            }
        }
        Condition::Attribute {
            attribute,
            comparison,
        } => validate_attribute_condition(attribute, *comparison, attributes),
        Condition::And(conditions) | Condition::Or(conditions) => {
            for condition in conditions {
                validate_condition(Some(condition), attributes)?;
            }
            Ok(())
        }
        Condition::Not(condition) => validate_condition(Some(condition), attributes),
    }
}

fn validate_attribute_condition(
    attribute: &str,
    comparison: AttributeComparison,
    attributes: &[AttributeDef],
) -> Result<(), BuildError> {
    if attribute.is_empty() {
        return Err(BuildError::EmptyConditionAttribute);
    }

    let attribute_def = attributes
        .iter()
        .find(|candidate| candidate.name == attribute)
        .ok_or_else(|| BuildError::UnknownConditionAttribute(attribute.to_string()))?;

    match comparison {
        AttributeComparison::Eq(value) => {
            validate_attribute_value(attribute, attribute_def.value_type, value)?;
        }
        AttributeComparison::InRange { min, max }
        | AttributeComparison::NotInRange { min, max } => {
            validate_attribute_ordered_value(attribute, attribute_def.value_type, min, "in_range")?;
            validate_attribute_ordered_value(attribute, attribute_def.value_type, max, "in_range")?;
        }
        AttributeComparison::AtLeast(value) => {
            validate_attribute_ordered_value(
                attribute,
                attribute_def.value_type,
                value,
                "at_least",
            )?;
        }
        AttributeComparison::AtMost(value) => {
            validate_attribute_ordered_value(
                attribute,
                attribute_def.value_type,
                value,
                "at_most",
            )?;
        }
    }

    Ok(())
}

fn validate_attribute_updates(
    updates: &[AttributeAssignment],
    attributes: &[AttributeDef],
) -> Result<(), BuildError> {
    let mut seen = Vec::with_capacity(updates.len());
    for update in updates {
        if update.attribute.is_empty() {
            return Err(BuildError::EmptyRuleAttributeUpdate);
        }
        if seen.iter().any(|name: &String| name == &update.attribute) {
            return Err(BuildError::DuplicateRuleAttributeUpdate(
                update.attribute.clone(),
            ));
        }

        let attribute_def = attributes
            .iter()
            .find(|candidate| candidate.name == update.attribute)
            .ok_or_else(|| BuildError::UnknownRuleAttributeUpdate(update.attribute.clone()))?;

        validate_attribute_value(&update.attribute, attribute_def.value_type, update.value)?;
        seen.push(update.attribute.clone());
    }
    Ok(())
}

fn validate_attribute_value(
    attribute: &str,
    expected: AttributeType,
    value: AttributeValue,
) -> Result<(), BuildError> {
    let actual = value.value_type();
    if actual != expected {
        Err(BuildError::AttributeTypeMismatch {
            attribute: attribute.to_string(),
            expected,
            actual,
        })
    } else {
        Ok(())
    }
}

fn validate_attribute_ordered_value(
    attribute: &str,
    expected: AttributeType,
    value: AttributeValue,
    comparison: &'static str,
) -> Result<(), BuildError> {
    validate_attribute_value(attribute, expected, value)?;
    if expected.is_boolean() {
        Err(BuildError::UnsupportedAttributeComparison {
            attribute: attribute.to_string(),
            comparison,
            value_type: expected,
        })
    } else {
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct PendingRule<C: CellModel> {
    when: C,
    neighborhood: Option<String>,
    condition: Option<Condition<C>>,
    attribute_updates: Vec<AttributeAssignment>,
    effect: RuleEffect<C>,
}

/// Builder for an ordered list of rules.
pub struct RulesBuilder<C: CellModel> {
    rules: Vec<PendingRule<C>>,
}

impl<C: CellModel> RulesBuilder<C> {
    fn new() -> Self {
        Self { rules: Vec::new() }
    }

    /// Start a new rule that applies to cells exactly equal to `state`.
    pub fn when(&mut self, state: C) -> RuleBuilder<'_, C> {
        RuleBuilder {
            rules: self,
            when: state,
            neighborhood: None,
            condition: None,
            attribute_updates: Vec::new(),
        }
    }

    fn finish(self) -> Vec<PendingRule<C>> {
        self.rules
    }
}

/// Builder for one rule clause.
pub struct RuleBuilder<'a, C: CellModel> {
    rules: &'a mut RulesBuilder<C>,
    when: C,
    neighborhood: Option<String>,
    condition: Option<Condition<C>>,
    attribute_updates: Vec<AttributeAssignment>,
}

impl<'a, C: CellModel> RuleBuilder<'a, C> {
    /// Override the default neighborhood for this rule.
    pub fn using(mut self, name: impl Into<String>) -> Self {
        self.neighborhood = Some(name.into());
        self
    }

    /// Add a required condition to this rule.
    pub fn require(mut self, condition: Condition<C>) -> Self {
        self.condition = Some(match self.condition.take() {
            Some(existing) => existing.and(condition),
            None => condition,
        });
        self
    }

    /// Add a negated condition to this rule.
    pub fn unless(self, condition: Condition<C>) -> Self {
        self.require(condition.negate())
    }

    /// Overwrite an attached attribute when this rule matches.
    pub fn set_attr(
        mut self,
        attribute: impl Into<String>,
        value: impl Into<AttributeValue>,
    ) -> Self {
        self.attribute_updates
            .push(AttributeAssignment::new(attribute, value));
        self
    }

    /// Keep the center cell unchanged when this rule matches.
    pub fn keep(self) -> &'a mut RulesBuilder<C> {
        self.finish(RuleEffect::Keep)
    }

    /// Replace the center cell with `state` when this rule matches.
    pub fn becomes(self, state: C) -> &'a mut RulesBuilder<C> {
        self.finish(RuleEffect::Become(state))
    }

    fn finish(self, effect: RuleEffect<C>) -> &'a mut RulesBuilder<C> {
        self.rules.rules.push(PendingRule {
            when: self.when,
            neighborhood: self.neighborhood,
            condition: self.condition,
            attribute_updates: self.attribute_updates,
            effect,
        });
        self.rules
    }
}
