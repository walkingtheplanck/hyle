use std::any::TypeId;

use crate::schema::{
    AttrAssign, AttributeComparison, AttributeDef, AttributeRef, AttributeSet,
    MaterialAttributeBinding, MaterialDef, MaterialRef, MaterialSet,
};
use crate::AttributeValue;

use super::BuildError;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct MaterialRegistry {
    pub(super) owner: TypeId,
    pub(super) default_material: crate::MaterialId,
    pub(super) materials: Vec<MaterialDef>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct AttributeRegistry {
    pub(super) owner: TypeId,
    pub(super) attributes: Vec<AttributeDef>,
}

/// One material-to-attribute attachment entry in the schema schema.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MatAttr {
    pub(super) material: MaterialRef,
    pub(super) attributes: Vec<AttrAssign>,
}

impl MatAttr {
    /// Construct a new material attribute attachment entry.
    ///
    /// This keeps the authoring API typed until the builder later erases the
    /// reference into schema-owned ids.
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

pub(super) fn register_materials<M: MaterialSet>() -> Result<MaterialRegistry, BuildError> {
    Ok(MaterialRegistry {
        owner: TypeId::of::<M>(),
        default_material: M::default_material().map_err(BuildError::from)?,
        materials: M::variants()
            .iter()
            .copied()
            .map(|material| {
                Ok(MaterialDef::new(
                    material.id().map_err(BuildError::from)?,
                    material.label(),
                    Vec::new(),
                ))
            })
            .collect::<Result<Vec<_>, BuildError>>()?,
    })
}

/// Erase one enum-backed attribute set into schema-owned definitions.
pub(super) fn register_attributes<A: AttributeSet>() -> Result<AttributeRegistry, BuildError> {
    Ok(AttributeRegistry {
        owner: TypeId::of::<A>(),
        attributes: A::variants()
            .iter()
            .copied()
            .map(|attribute| {
                Ok(AttributeDef::new(
                    attribute.id().map_err(BuildError::from)?,
                    attribute.label(),
                    attribute.value_type(),
                ))
            })
            .collect::<Result<Vec<_>, BuildError>>()?,
    })
}

/// Reject material sets whose declarative labels collide.
pub(super) fn validate_unique_material_labels(materials: &[MaterialDef]) -> Result<(), BuildError> {
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

/// Reject attribute sets whose declarative labels collide.
pub(super) fn validate_unique_attribute_labels(
    attributes: &[AttributeDef],
) -> Result<(), BuildError> {
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

/// Attach declared attributes and defaults to the materials that carry them.
///
/// This is where the builder turns typed `material_attributes(...)` input into
/// the material-local bindings that solvers later use for reset defaults.
pub(super) fn apply_material_attributes(
    materials: &mut MaterialRegistry,
    attributes: Option<&AttributeRegistry>,
    assignments: &[MatAttr],
) -> Result<(), BuildError> {
    let mut seen_materials = vec![false; materials.materials.len()];

    for assignment in assignments {
        if assignment.material.owner() != materials.owner {
            return Err(BuildError::MismatchedMaterial(assignment.material.label()));
        }

        let material_index = assignment.material.id().map_err(BuildError::from)?.index();
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
                attribute.attribute.id().map_err(BuildError::from)?,
                attribute.default,
            ));
        }
    }

    Ok(())
}

/// Confirm that a material actually carries the attribute a rule wants to use.
pub(super) fn ensure_material_has_attribute(
    materials: &MaterialRegistry,
    material: crate::MaterialId,
    attribute: AttributeRef,
) -> Result<(), BuildError> {
    let attribute_id = attribute.id().map_err(BuildError::from)?;
    if materials.materials[material.index()]
        .attributes
        .iter()
        .any(|binding| binding.attribute == attribute_id)
    {
        Ok(())
    } else {
        Err(BuildError::MissingMaterialAttribute {
            material: materials.materials[material.index()].name,
            attribute: attribute.label(),
        })
    }
}

/// Validate that a rule comparison matches the attribute's declared scalar type.
pub(super) fn validate_attribute_comparison(
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
