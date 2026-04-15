//! Builder types for authoring portable blueprints.

use std::any::TypeId;
use std::error::Error;
use std::fmt::{Display, Formatter};

use crate::contracts::{
    AttrAssign, AttributeAssignment, AttributeComparison, AttributeDef, AttributeRef, AttributeSet,
    AttributeType, AttributeValue, Blueprint, Condition, MaterialAttributeBinding, MaterialDef,
    MaterialRef, MaterialSet, NeighborhoodId, NeighborhoodRef, NeighborhoodSet, NeighborhoodSpec,
    ResolvedCondition, Rule, RuleEffect, Semantics, TopologyDescriptor,
};

/// Errors raised while building a [`Blueprint`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BuildError {
    /// No material set was registered.
    MissingMaterials,
    /// A material label was duplicated inside one material set.
    DuplicateMaterialLabel(&'static str),
    /// A material-scoped assignment references a different material set.
    MismatchedMaterial(&'static str),
    /// Two `MatAttr` entries referenced the same material.
    DuplicateMaterialAssignment(&'static str),
    /// No attribute set was registered before using attributes.
    MissingAttributes,
    /// An attribute label was duplicated inside one attribute set.
    DuplicateAttributeLabel(&'static str),
    /// A rule or assignment references a different attribute set.
    MismatchedAttribute(&'static str),
    /// A material attempted to attach the same attribute more than once.
    DuplicateMaterialAttribute {
        /// Material name.
        material: &'static str,
        /// Attribute name.
        attribute: &'static str,
    },
    /// A provided default does not match the declared attribute type.
    AttributeTypeMismatch {
        /// Attribute name.
        attribute: &'static str,
        /// Declared scalar type.
        expected: AttributeType,
        /// Provided scalar type.
        actual: AttributeType,
    },
    /// An attribute comparison is not valid for the declared attribute type.
    UnsupportedAttributeComparison {
        /// Attribute name.
        attribute: &'static str,
        /// Comparison kind.
        comparison: &'static str,
        /// Declared scalar type.
        value_type: AttributeType,
    },
    /// A rule references an attribute not attached to its source material.
    MissingMaterialAttribute {
        /// Material name.
        material: &'static str,
        /// Attribute name.
        attribute: &'static str,
    },
    /// No neighborhood set was registered before using neighborhoods.
    MissingNeighborhoods,
    /// A neighborhood label was duplicated inside one neighborhood set.
    DuplicateNeighborhoodLabel(&'static str),
    /// A neighborhood definition references a different neighborhood set.
    MismatchedNeighborhood(&'static str),
    /// Two neighborhood specs referenced the same neighborhood.
    DuplicateNeighborhoodSpec(&'static str),
    /// One registered neighborhood did not receive a specification.
    MissingNeighborhoodSpec(&'static str),
    /// A rule referenced a neighborhood from a different neighborhood set.
    UnknownRuleNeighborhood(&'static str),
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
            BuildError::MissingMaterials => write!(f, "materials::<M>() must be called before build"),
            BuildError::DuplicateMaterialLabel(label) => {
                write!(f, "duplicate material label in material set: {label}")
            }
            BuildError::MismatchedMaterial(label) => {
                write!(f, "material '{label}' belongs to a different material set")
            }
            BuildError::DuplicateMaterialAssignment(label) => {
                write!(f, "material attributes were assigned more than once for '{label}'")
            }
            BuildError::MissingAttributes => {
                write!(f, "attributes::<A>() must be called before using attributes")
            }
            BuildError::DuplicateAttributeLabel(label) => {
                write!(f, "duplicate attribute label in attribute set: {label}")
            }
            BuildError::MismatchedAttribute(label) => {
                write!(f, "attribute '{label}' belongs to a different attribute set")
            }
            BuildError::DuplicateMaterialAttribute { material, attribute } => write!(
                f,
                "material '{material}' attaches attribute '{attribute}' more than once"
            ),
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
            BuildError::MissingMaterialAttribute { material, attribute } => write!(
                f,
                "material '{material}' does not carry attribute '{attribute}'"
            ),
            BuildError::MissingNeighborhoods => {
                write!(f, "neighborhoods::<N>() must be called before build")
            }
            BuildError::DuplicateNeighborhoodLabel(label) => {
                write!(f, "duplicate neighborhood label in neighborhood set: {label}")
            }
            BuildError::MismatchedNeighborhood(label) => {
                write!(f, "neighborhood '{label}' belongs to a different neighborhood set")
            }
            BuildError::DuplicateNeighborhoodSpec(label) => {
                write!(f, "neighborhood '{label}' was configured more than once")
            }
            BuildError::MissingNeighborhoodSpec(label) => {
                write!(f, "neighborhood '{label}' is missing a specification")
            }
            BuildError::UnknownRuleNeighborhood(label) => {
                write!(f, "rule references neighborhood '{label}' from a different set")
            }
            BuildError::InvalidRandomChance { stream, one_in } => write!(
                f,
                "random stream {stream} requires a positive denominator, got {one_in}"
            ),
        }
    }
}

impl Error for BuildError {}

#[derive(Clone, Debug, PartialEq, Eq)]
struct MaterialRegistry {
    owner: TypeId,
    default_material: crate::contracts::MaterialId,
    materials: Vec<MaterialDef>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct AttributeRegistry {
    owner: TypeId,
    attributes: Vec<AttributeDef>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct NeighborhoodRegistry {
    owner: TypeId,
    default_neighborhood: NeighborhoodId,
    expected_names: Vec<&'static str>,
}

/// One material-to-attribute attachment entry in the blueprint schema.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MatAttr {
    material: MaterialRef,
    attributes: Vec<AttrAssign>,
}

impl MatAttr {
    /// Construct a new material attribute attachment entry.
    pub fn new<M, I>(material: M, attributes: I) -> Self
    where
        M: MaterialSet,
        I: IntoIterator<Item = AttrAssign>,
    {
        Self {
            material: material.material(),
            attributes: attributes.into_iter().collect(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct PendingAttributeUpdate {
    attribute: AttributeRef,
    value: AttributeValue,
}

/// Author-time rule item used by [`BlueprintBuilder::rules`].
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
    pub fn using<N: NeighborhoodSet>(mut self, neighborhood: N) -> Self {
        self.neighborhood = Some(neighborhood.neighborhood());
        self
    }

    /// Conjoin an additional condition.
    pub fn require(mut self, condition: Condition) -> Self {
        self.condition = Some(match self.condition.take() {
            Some(existing) => existing.and(condition),
            None => condition,
        });
        self
    }

    /// Make the rule keep the current material.
    pub fn keep(mut self) -> Self {
        self.effect = RuleEffect::Keep;
        self
    }

    /// Make the rule become a different material.
    pub fn becomes<M: MaterialSet>(mut self, material: M) -> Self {
        self.effect = RuleEffect::Become(material.id());
        self
    }

    /// Write one attribute when the rule matches.
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
}

/// Typed blueprint builder.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BlueprintBuilder {
    semantics: Semantics,
    topology: TopologyDescriptor,
    materials: Option<MaterialRegistry>,
    attributes: Option<AttributeRegistry>,
    material_attributes: Vec<MatAttr>,
    neighborhoods: Option<NeighborhoodRegistry>,
    neighborhood_specs: Vec<NeighborhoodSpec>,
    default_neighborhood: Option<NeighborhoodRef>,
    rules: Vec<RuleSpec>,
}

impl BlueprintBuilder {
    pub(crate) fn new() -> Self {
        Self {
            semantics: Semantics::V1,
            topology: TopologyDescriptor::bounded(),
            materials: None,
            attributes: None,
            material_attributes: Vec::new(),
            neighborhoods: None,
            neighborhood_specs: Vec::new(),
            default_neighborhood: None,
            rules: Vec::new(),
        }
    }

    /// Override the topology descriptor used by this blueprint.
    pub fn topology(mut self, topology: TopologyDescriptor) -> Self {
        self.topology = topology;
        self
    }

    /// Register the enum-backed material universe for this blueprint.
    pub fn materials<M: MaterialSet>(mut self) -> Self {
        self.materials = Some(MaterialRegistry {
            owner: TypeId::of::<M>(),
            default_material: M::default_material(),
            materials: M::variants()
                .iter()
                .copied()
                .map(|material| MaterialDef::new(material.id(), material.label(), Vec::new()))
                .collect(),
        });
        self
    }

    /// Register the enum-backed attribute universe for this blueprint.
    pub fn attributes<A: AttributeSet>(mut self) -> Self {
        self.attributes = Some(AttributeRegistry {
            owner: TypeId::of::<A>(),
            attributes: A::variants()
                .iter()
                .copied()
                .map(|attribute| {
                    AttributeDef::new(attribute.id(), attribute.label(), attribute.value_type())
                })
                .collect(),
        });
        self
    }

    /// Attach attributes to materials with material-specific defaults.
    pub fn material_attributes<I>(mut self, assignments: I) -> Self
    where
        I: IntoIterator<Item = MatAttr>,
    {
        self.material_attributes = assignments.into_iter().collect();
        self
    }

    /// Register the enum-backed neighborhood universe for this blueprint.
    pub fn neighborhoods<N: NeighborhoodSet>(mut self) -> Self {
        self.neighborhoods = Some(NeighborhoodRegistry {
            owner: TypeId::of::<N>(),
            default_neighborhood: N::default_neighborhood(),
            expected_names: N::variants().iter().map(|value| value.label()).collect(),
        });
        self
    }

    /// Override the default neighborhood used by rules without `using(...)`.
    pub fn default_neighborhood<N: NeighborhoodSet>(mut self, neighborhood: N) -> Self {
        self.default_neighborhood = Some(neighborhood.neighborhood());
        self
    }

    /// Provide one spec for each declared neighborhood.
    pub fn neighborhood_specs<I>(mut self, neighborhoods: I) -> Self
    where
        I: IntoIterator<Item = NeighborhoodSpec>,
    {
        self.neighborhood_specs = neighborhoods.into_iter().collect();
        self
    }

    /// Provide the ordered rules for this blueprint.
    pub fn rules<I>(mut self, rules: I) -> Self
    where
        I: IntoIterator<Item = RuleSpec>,
    {
        self.rules = rules.into_iter().collect();
        self
    }

    /// Validate and build the portable blueprint.
    pub fn build(self) -> Result<Blueprint, BuildError> {
        let mut materials = self.materials.ok_or(BuildError::MissingMaterials)?;
        validate_unique_material_labels(&materials.materials)?;

        let attributes = match self.attributes {
            Some(attributes) => {
                validate_unique_attribute_labels(&attributes.attributes)?;
                Some(attributes)
            }
            None => None,
        };

        apply_material_attributes(&mut materials, attributes.as_ref(), &self.material_attributes)?;

        let neighborhoods = self.neighborhoods.ok_or(BuildError::MissingNeighborhoods)?;
        let (neighborhood_specs, default_neighborhood) =
            validate_neighborhoods(&neighborhoods, &self.neighborhood_specs, self.default_neighborhood)?;

        let rules = self
            .rules
            .into_iter()
            .map(|rule| {
                build_rule(
                    rule,
                    &materials,
                    attributes.as_ref(),
                    &neighborhoods,
                    default_neighborhood,
                )
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Blueprint::new(
            self.semantics,
            self.topology,
            materials.default_material,
            materials.materials,
            attributes.map_or_else(Vec::new, |registry| registry.attributes),
            neighborhood_specs,
            default_neighborhood,
            rules,
        ))
    }
}

fn validate_unique_material_labels(materials: &[MaterialDef]) -> Result<(), BuildError> {
    for (index, material) in materials.iter().enumerate() {
        if materials[index + 1..]
            .iter()
            .any(|candidate| candidate.name == material.name)
        {
            return Err(BuildError::DuplicateMaterialLabel(material.name));
        }
    }
    Ok(())
}

fn validate_unique_attribute_labels(attributes: &[AttributeDef]) -> Result<(), BuildError> {
    for (index, attribute) in attributes.iter().enumerate() {
        if attributes[index + 1..]
            .iter()
            .any(|candidate| candidate.name == attribute.name)
        {
            return Err(BuildError::DuplicateAttributeLabel(attribute.name));
        }
    }
    Ok(())
}

fn apply_material_attributes(
    materials: &mut MaterialRegistry,
    attributes: Option<&AttributeRegistry>,
    assignments: &[MatAttr],
) -> Result<(), BuildError> {
    let mut seen_materials = vec![false; materials.materials.len()];

    for assignment in assignments {
        if assignment.material.owner() != materials.owner {
            return Err(BuildError::MismatchedMaterial(assignment.material.label()));
        }

        let material_index = assignment.material.id().index();
        if seen_materials[material_index] {
            return Err(BuildError::DuplicateMaterialAssignment(
                assignment.material.label(),
            ));
        }
        seen_materials[material_index] = true;

        let material = &mut materials.materials[material_index];
        let mut seen_attributes = Vec::new();

        for attribute in &assignment.attributes {
            let registry = attributes.ok_or(BuildError::MissingAttributes)?;
            if attribute.attribute.owner() != registry.owner {
                return Err(BuildError::MismatchedAttribute(attribute.attribute.label()));
            }

            if seen_attributes
                .iter()
                .any(|candidate: &AttributeRef| candidate.id() == attribute.attribute.id())
            {
                return Err(BuildError::DuplicateMaterialAttribute {
                    material: material.name,
                    attribute: attribute.attribute.label(),
                });
            }
            seen_attributes.push(attribute.attribute);

            if attribute.default.value_type() != attribute.attribute.value_type() {
                return Err(BuildError::AttributeTypeMismatch {
                    attribute: attribute.attribute.label(),
                    expected: attribute.attribute.value_type(),
                    actual: attribute.default.value_type(),
                });
            }

            material.attributes.push(MaterialAttributeBinding::new(
                attribute.attribute.id(),
                attribute.default,
            ));
        }
    }

    Ok(())
}

fn validate_neighborhoods(
    registry: &NeighborhoodRegistry,
    specs: &[NeighborhoodSpec],
    default_override: Option<NeighborhoodRef>,
) -> Result<(Vec<NeighborhoodSpec>, NeighborhoodId), BuildError> {
    let mut resolved = vec![None; registry.expected_names.len()];

    for spec in specs {
        if spec.id().index() >= resolved.len() {
            return Err(BuildError::MismatchedNeighborhood(spec.name()));
        }

        let expected_name = registry.expected_names[spec.id().index()];
        if expected_name != spec.name() {
            return Err(BuildError::MismatchedNeighborhood(spec.name()));
        }
        if resolved[spec.id().index()].is_some() {
            return Err(BuildError::DuplicateNeighborhoodSpec(spec.name()));
        }
        resolved[spec.id().index()] = Some(*spec);
    }

    let mut neighborhoods = Vec::with_capacity(resolved.len());
    for (index, spec) in resolved.into_iter().enumerate() {
        let spec = spec.ok_or(BuildError::MissingNeighborhoodSpec(
            registry.expected_names[index],
        ))?;
        neighborhoods.push(spec);
    }

    let default_neighborhood = match default_override {
        Some(reference) => {
            if reference.owner() != registry.owner {
                return Err(BuildError::UnknownRuleNeighborhood(reference.label()));
            }
            reference.id()
        }
        None => registry.default_neighborhood,
    };

    Ok((neighborhoods, default_neighborhood))
}

fn build_rule(
    rule: RuleSpec,
    materials: &MaterialRegistry,
    attributes: Option<&AttributeRegistry>,
    neighborhoods: &NeighborhoodRegistry,
    default_neighborhood: NeighborhoodId,
) -> Result<Rule, BuildError> {
    if rule.when.owner() != materials.owner {
        return Err(BuildError::MismatchedMaterial(rule.when.label()));
    }

    let target_material = match rule.effect {
        RuleEffect::Keep => rule.when.id(),
        RuleEffect::Become(target) => target,
    };

    let neighborhood = match rule.neighborhood {
        Some(reference) => {
            if reference.owner() != neighborhoods.owner {
                return Err(BuildError::UnknownRuleNeighborhood(reference.label()));
            }
            reference.id()
        }
        None => default_neighborhood,
    };

    let condition = rule
        .condition
        .as_ref()
        .map(|condition| validate_condition(condition, rule.when, materials, attributes))
        .transpose()?;

    let mut seen_updates = Vec::new();
    let mut attribute_updates = Vec::with_capacity(rule.attribute_updates.len());
    for update in &rule.attribute_updates {
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
        attribute_updates.push(AttributeAssignment::new(update.attribute.id(), update.value));
    }

    Ok(Rule {
        when: rule.when.id(),
        neighborhood,
        condition,
        attribute_updates,
        effect: rule.effect,
    })
}

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

fn ensure_material_has_attribute(
    materials: &MaterialRegistry,
    material: crate::contracts::MaterialId,
    attribute: AttributeRef,
) -> Result<(), BuildError> {
    if materials.materials[material.index()]
        .attributes
        .iter()
        .any(|binding| binding.attribute == attribute.id())
    {
        Ok(())
    } else {
        Err(BuildError::MissingMaterialAttribute {
            material: materials.materials[material.index()].name,
            attribute: attribute.label(),
        })
    }
}

fn validate_attribute_comparison(
    attribute: AttributeRef,
    comparison: AttributeComparison,
) -> Result<(), BuildError> {
    let expected = attribute.value_type();
    let validate_value = |value: AttributeValue| -> Result<(), BuildError> {
        if value.value_type() != expected {
            Err(BuildError::AttributeTypeMismatch {
                attribute: attribute.label(),
                expected,
                actual: value.value_type(),
            })
        } else {
            Ok(())
        }
    };

    match comparison {
        AttributeComparison::Eq(value)
        | AttributeComparison::AtLeast(value)
        | AttributeComparison::AtMost(value) => {
            validate_value(value)?;
        }
        AttributeComparison::InRange { min, max }
        | AttributeComparison::NotInRange { min, max } => {
            validate_value(min)?;
            validate_value(max)?;
        }
    }

    if expected.is_boolean() {
        match comparison {
            AttributeComparison::Eq(_) => Ok(()),
            AttributeComparison::InRange { .. } => Ok(()),
            AttributeComparison::NotInRange { .. } => Ok(()),
            AttributeComparison::AtLeast(_) => Err(BuildError::UnsupportedAttributeComparison {
                attribute: attribute.label(),
                comparison: "at_least",
                value_type: expected,
            }),
            AttributeComparison::AtMost(_) => Err(BuildError::UnsupportedAttributeComparison {
                attribute: attribute.label(),
                comparison: "at_most",
                value_type: expected,
            }),
        }
    } else {
        Ok(())
    }
}
