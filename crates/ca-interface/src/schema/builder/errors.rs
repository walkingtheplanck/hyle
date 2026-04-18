use std::error::Error;
use std::fmt::{Display, Formatter};

use crate::{AttributeType, RngStreamId, SetContractError};

/// Errors raised while building a [`crate::schema::Blueprint`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BuildError {
    /// One of the registered schema sets violated its own trait contract.
    InvalidSetContract(SetContractError),
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
        stream: RngStreamId,
        /// Requested `1 / n` denominator.
        one_in: u32,
    },
}

impl Display for BuildError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BuildError::InvalidSetContract(error) => {
                write!(f, "{error}")
            }
            BuildError::MissingMaterials => {
                write!(f, "materials::<M>() must be called before build")
            }
            BuildError::DuplicateMaterialLabel(label) => {
                write!(f, "duplicate material label in material set: {label}")
            }
            BuildError::MismatchedMaterial(label) => {
                write!(f, "material '{label}' belongs to a different material set")
            }
            BuildError::DuplicateMaterialAssignment(label) => {
                write!(
                    f,
                    "material attributes were assigned more than once for '{label}'"
                )
            }
            BuildError::MissingAttributes => {
                write!(
                    f,
                    "attributes::<A>() must be called before using attributes"
                )
            }
            BuildError::DuplicateAttributeLabel(label) => {
                write!(f, "duplicate attribute label in attribute set: {label}")
            }
            BuildError::MismatchedAttribute(label) => {
                write!(
                    f,
                    "attribute '{label}' belongs to a different attribute set"
                )
            }
            BuildError::DuplicateMaterialAttribute {
                material,
                attribute,
            } => write!(
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
            BuildError::MissingMaterialAttribute {
                material,
                attribute,
            } => write!(
                f,
                "material '{material}' does not carry attribute '{attribute}'"
            ),
            BuildError::MissingNeighborhoods => {
                write!(f, "neighborhoods::<N>() must be called before build")
            }
            BuildError::DuplicateNeighborhoodLabel(label) => {
                write!(
                    f,
                    "duplicate neighborhood label in neighborhood set: {label}"
                )
            }
            BuildError::MismatchedNeighborhood(label) => {
                write!(
                    f,
                    "neighborhood '{label}' belongs to a different neighborhood set"
                )
            }
            BuildError::DuplicateNeighborhoodSpec(label) => {
                write!(f, "neighborhood '{label}' was configured more than once")
            }
            BuildError::MissingNeighborhoodSpec(label) => {
                write!(f, "neighborhood '{label}' is missing a specification")
            }
            BuildError::UnknownRuleNeighborhood(label) => {
                write!(
                    f,
                    "rule references neighborhood '{label}' from a different set"
                )
            }
            BuildError::InvalidRandomChance { stream, one_in } => write!(
                f,
                "random stream {stream} requires a positive denominator, got {one_in}"
            ),
        }
    }
}

impl Error for BuildError {}

impl From<SetContractError> for BuildError {
    fn from(value: SetContractError) -> Self {
        Self::InvalidSetContract(value)
    }
}
