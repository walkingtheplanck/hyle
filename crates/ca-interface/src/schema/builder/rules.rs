use crate::schema::{
    AttributeAssignment, AttributeRef, AttributeSet, Condition, MaterialRef, MaterialSet,
    NeighborhoodRef, NeighborhoodSet, ResolvedCondition, Rule, RuleEffect,
};
use crate::AttributeValue;
use crate::NeighborhoodId;

use super::errors::BuildError;
use super::materials::{
    ensure_material_has_attribute, validate_attribute_comparison, AttributeRegistry,
    MaterialRegistry,
};
use super::neighborhoods::NeighborhoodRegistry;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct PendingAttributeUpdate {
    attribute: AttributeRef,
    value: AttributeValue,
}

/// Author-time rule item used by [`crate::schema::BlueprintBuilder::rules`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RuleSpec {
    when: MaterialRef,
    neighborhood: Option<NeighborhoodRef>,
    condition: Option<Condition>,
    attribute_updates: Vec<PendingAttributeUpdate>,
    effect: RuleEffect,
}

impl RuleSpec {
    /// Start a new rule for the given source material.
    ///
    /// Rules are authored against the material currently occupying the center
    /// cell, then later compiled into schema ids during `build()`.
    pub fn when<M: MaterialSet>(material: M) -> Self {
        Self {
            when: material.material(),
            neighborhood: None,
            condition: None,
            attribute_updates: Vec::new(),
            effect: RuleEffect::Keep,
        }
    }

    /// Override the neighborhood used by this rule.
    ///
    /// Rules without this call inherit the schema-level default neighborhood.
    pub fn using<N: NeighborhoodSet>(mut self, neighborhood: N) -> Self {
        self.neighborhood = Some(neighborhood.neighborhood());
        self
    }

    /// Conjoin an additional condition.
    ///
    /// Repeated calls intentionally flatten into one `And(...)` tree so the
    /// authored rule shape stays compact and predictable.
    pub fn require(mut self, condition: Condition) -> Self {
        self.condition = Some(match self.condition.take() {
            Some(existing) => existing.and(condition),
            None => condition,
        });
        self
    }

    /// Make the rule keep the current material.
    ///
    /// This is still meaningful when combined with attribute writes.
    pub fn keep(mut self) -> Self {
        self.effect = RuleEffect::Keep;
        self
    }

    /// Make the rule become a different material.
    ///
    /// Material changes also cause attribute defaults for the destination
    /// material to be re-applied by the runtime.
    pub fn becomes<M: MaterialSet>(mut self, material: M) -> Self {
        self.effect = RuleEffect::Become(material.id());
        self
    }

    /// Write one attribute when the rule matches.
    ///
    /// Updates are validated against the destination material, not just the
    /// source material, so `becomes(...)` and `set_attr(...)` stay coherent.
    pub fn set_attr<A: AttributeSet>(
        mut self,
        attribute: A,
        value: impl Into<AttributeValue>,
    ) -> Self {
        self.attribute_updates.push(PendingAttributeUpdate {
            attribute: attribute.attribute(),
            value: value.into(),
        });
        self
    }

    /// Validate one authored rule and erase it into schema ids.
    ///
    /// # Errors
    ///
    /// Returns [`BuildError`] when the rule references the wrong enum family,
    /// uses undeclared attributes, duplicates writes, or contains invalid
    /// attribute/random predicates.
    pub(super) fn build(
        self,
        materials: &MaterialRegistry,
        attributes: Option<&AttributeRegistry>,
        neighborhoods: &NeighborhoodRegistry,
        default_neighborhood: NeighborhoodId,
    ) -> Result<Rule, BuildError> {
        if self.when.owner() != materials.owner {
            return Err(BuildError::MismatchedMaterial(self.when.label()));
        }

        let target_material = match self.effect {
            RuleEffect::Keep => self.when.id(),
            RuleEffect::Become(target) => target,
        };

        let neighborhood = match self.neighborhood {
            Some(reference) => {
                if reference.owner() != neighborhoods.owner {
                    return Err(BuildError::UnknownRuleNeighborhood(reference.label()));
                }
                reference.id()
            }
            None => default_neighborhood,
        };

        let condition = self
            .condition
            .as_ref()
            .map(|condition| validate_condition(condition, self.when, materials, attributes))
            .transpose()?;

        let mut seen_updates = Vec::new();
        let mut attribute_updates = Vec::with_capacity(self.attribute_updates.len());
        for update in &self.attribute_updates {
            let registry = attributes.ok_or(BuildError::MissingAttributes)?;
            if update.attribute.owner() != registry.owner {
                return Err(BuildError::MismatchedAttribute(update.attribute.label()));
            }
            if seen_updates
                .iter()
                .any(|attribute: &AttributeRef| attribute.id() == update.attribute.id())
            {
                return Err(BuildError::DuplicateMaterialAttribute {
                    material: materials.materials[target_material.index()].name,
                    attribute: update.attribute.label(),
                });
            }
            seen_updates.push(update.attribute);

            if update.value.value_type() != update.attribute.value_type() {
                return Err(BuildError::AttributeTypeMismatch {
                    attribute: update.attribute.label(),
                    expected: update.attribute.value_type(),
                    actual: update.value.value_type(),
                });
            }
            ensure_material_has_attribute(materials, target_material, update.attribute)?;
            attribute_updates.push(AttributeAssignment::new(
                update.attribute.id(),
                update.value,
            ));
        }

        Ok(Rule {
            when: self.when.id(),
            neighborhood,
            condition,
            attribute_updates,
            effect: self.effect,
        })
    }
}

/// Resolve and validate one declarative condition tree.
fn validate_condition(
    condition: &Condition,
    when: MaterialRef,
    materials: &MaterialRegistry,
    attributes: Option<&AttributeRegistry>,
) -> Result<ResolvedCondition, BuildError> {
    match condition {
        Condition::NeighborCount {
            material,
            comparison,
        } => {
            if material.owner() != materials.owner {
                return Err(BuildError::MismatchedMaterial(material.label()));
            }
            Ok(ResolvedCondition::NeighborCount {
                material: material.id(),
                comparison: *comparison,
            })
        }
        Condition::NeighborWeightedSum {
            material,
            comparison,
        } => {
            if material.owner() != materials.owner {
                return Err(BuildError::MismatchedMaterial(material.label()));
            }
            Ok(ResolvedCondition::NeighborWeightedSum {
                material: material.id(),
                comparison: *comparison,
            })
        }
        Condition::RandomChance { stream, one_in } => {
            if *one_in == 0 {
                return Err(BuildError::InvalidRandomChance {
                    stream: *stream,
                    one_in: *one_in,
                });
            }
            Ok(ResolvedCondition::RandomChance {
                stream: *stream,
                one_in: *one_in,
            })
        }
        Condition::Attribute {
            attribute,
            comparison,
        } => {
            let registry = attributes.ok_or(BuildError::MissingAttributes)?;
            if attribute.owner() != registry.owner {
                return Err(BuildError::MismatchedAttribute(attribute.label()));
            }

            ensure_material_has_attribute(materials, when.id(), *attribute)?;
            validate_attribute_comparison(*attribute, *comparison)?;

            Ok(ResolvedCondition::Attribute {
                attribute: attribute.id(),
                comparison: *comparison,
            })
        }
        Condition::And(conditions) => Ok(ResolvedCondition::And(
            conditions
                .iter()
                .map(|condition| validate_condition(condition, when, materials, attributes))
                .collect::<Result<Vec<_>, _>>()?,
        )),
        Condition::Or(conditions) => Ok(ResolvedCondition::Or(
            conditions
                .iter()
                .map(|condition| validate_condition(condition, when, materials, attributes))
                .collect::<Result<Vec<_>, _>>()?,
        )),
        Condition::Not(condition) => Ok(ResolvedCondition::Not(Box::new(validate_condition(
            condition, when, materials, attributes,
        )?))),
    }
}
