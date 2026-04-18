use std::error::Error;
use std::fmt::{Display, Formatter};

/// Error raised when a manual schema set implementation breaks its contract.
///
/// Derived set implementations keep these invariants automatically. Manual impls
/// can trigger this when `variants()` omits the current enum value or, for
/// neighborhood sets, when no default neighborhood can be chosen.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SetContractError {
    /// The current material was not present in `variants()`.
    MissingMaterialVariant {
        /// Concrete set type name.
        set_type: &'static str,
        /// Human-readable label for the missing material.
        label: &'static str,
    },
    /// The current attribute was not present in `variants()`.
    MissingAttributeVariant {
        /// Concrete set type name.
        set_type: &'static str,
        /// Human-readable label for the missing attribute.
        label: &'static str,
    },
    /// The current neighborhood was not present in `variants()`.
    MissingNeighborhoodVariant {
        /// Concrete set type name.
        set_type: &'static str,
        /// Human-readable label for the missing neighborhood.
        label: &'static str,
    },
    /// The neighborhood set declared no variants at all.
    EmptyNeighborhoodSet {
        /// Concrete set type name.
        set_type: &'static str,
    },
}

impl Display for SetContractError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SetContractError::MissingMaterialVariant { set_type, label } => write!(
                f,
                "material set '{set_type}' omitted variant '{label}' from variants()"
            ),
            SetContractError::MissingAttributeVariant { set_type, label } => write!(
                f,
                "attribute set '{set_type}' omitted variant '{label}' from variants()"
            ),
            SetContractError::MissingNeighborhoodVariant { set_type, label } => write!(
                f,
                "neighborhood set '{set_type}' omitted variant '{label}' from variants()"
            ),
            SetContractError::EmptyNeighborhoodSet { set_type } => write!(
                f,
                "neighborhood set '{set_type}' must declare at least one variant"
            ),
        }
    }
}

impl Error for SetContractError {}
