//! Builder types for authoring portable blueprint specifications.

use std::error::Error;
use std::fmt::{Display, Formatter};

use crate::{CellModel, NeighborhoodSpec, TopologyDescriptor};

use super::{BlueprintSpec, Condition, NamedNeighborhood, Rule, RuleEffect, Semantics};

const ADJACENT_NEIGHBORHOOD: &str = "adjacent";

/// Branded entrypoint for blueprint authoring.
pub struct Hyle;

impl Hyle {
    /// Start building a solver-agnostic blueprint specification.
    pub fn builder() -> HyleBuilder {
        HyleBuilder
    }
}

/// Untyped entry builder returned by [`Hyle::builder`].
pub struct HyleBuilder;

impl HyleBuilder {
    /// Select the cell type used by the blueprint.
    pub fn cells<C: CellModel>(self) -> BlueprintBuilder<C> {
        BlueprintBuilder::new()
    }
}

/// Errors raised while building a [`BlueprintSpec`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BuildError {
    /// A neighborhood name was registered more than once.
    DuplicateNeighborhood(String),
    /// The default neighborhood name does not exist.
    UnknownDefaultNeighborhood(String),
    /// A rule references a neighborhood name that does not exist.
    UnknownRuleNeighborhood(String),
}

impl Display for BuildError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BuildError::DuplicateNeighborhood(name) => {
                write!(f, "duplicate neighborhood name: {name}")
            }
            BuildError::UnknownDefaultNeighborhood(name) => {
                write!(f, "unknown default neighborhood: {name}")
            }
            BuildError::UnknownRuleNeighborhood(name) => {
                write!(f, "rule references unknown neighborhood: {name}")
            }
        }
    }
}

impl Error for BuildError {}

/// Typed blueprint builder.
pub struct BlueprintBuilder<C: CellModel> {
    semantics: Semantics,
    topology: TopologyDescriptor,
    neighborhoods: Vec<NamedNeighborhood>,
    default_neighborhood: String,
    rules: Vec<PendingRule<C>>,
}

impl<C: CellModel> BlueprintBuilder<C> {
    fn new() -> Self {
        Self {
            semantics: Semantics::V1,
            topology: TopologyDescriptor::bounded(),
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

    /// Validate and build the portable blueprint specification.
    pub fn build(self) -> Result<BlueprintSpec<C>, BuildError> {
        let mut names = Vec::with_capacity(self.neighborhoods.len());
        for neighborhood in &self.neighborhoods {
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
            let neighborhood_name = rule
                .neighborhood
                .unwrap_or_else(|| self.default_neighborhood.clone());
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

        Ok(BlueprintSpec::new(
            C::schema(),
            self.semantics,
            self.topology,
            self.neighborhoods,
            default_neighborhood,
            rules,
        ))
    }
}

/// Backward-compatible alias for the blueprint builder type.
pub type AutomatonBuilder<C> = BlueprintBuilder<C>;

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

    fn push(&mut self, rule: PendingRule<C>) {
        self.rules.push(rule);
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
        self.rules.push(PendingRule {
            when: self.when,
            neighborhood: self.neighborhood,
            condition: self.condition,
            effect,
        });
        self.rules
    }
}
