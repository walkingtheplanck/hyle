//! Builder types for authoring portable blueprints.

use std::error::Error;
use std::fmt::{Display, Formatter};

use crate::{
    AttributeDef, AttributeType, AttributeValue, CellModel, NeighborhoodSpec, TopologyDescriptor,
};

use super::{Blueprint, Condition, NamedNeighborhood, Rule, RuleEffect, Semantics};

const ADJACENT_NEIGHBORHOOD: &str = "adjacent";

/// Errors raised while building a [`Blueprint`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BuildError {
    /// An attribute name was empty.
    EmptyAttributeName,
    /// An attribute name was registered more than once.
    DuplicateAttribute(String),
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
            validate_condition(rule.condition.as_ref())?;
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

fn validate_condition<C: CellModel>(condition: Option<&Condition<C>>) -> Result<(), BuildError> {
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
        Condition::And(conditions) | Condition::Or(conditions) => {
            for condition in conditions {
                validate_condition(Some(condition))?;
            }
            Ok(())
        }
        Condition::Not(condition) => validate_condition(Some(condition)),
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct PendingRule<C: CellModel> {
    when: C,
    neighborhood: Option<String>,
    condition: Option<Condition<C>>,
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
            effect,
        });
        self.rules
    }
}
