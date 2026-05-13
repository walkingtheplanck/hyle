use serde::{Deserialize, Serialize};

use crate::ir::Identifier;

/// Lattice metadata shared across compiler and runtime crates.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LatticeIr {
    /// Number of lattice dimensions.
    pub dimensions: u8,
    /// Declared world cell shape.
    pub cell: String,
}

impl Default for LatticeIr {
    fn default() -> Self {
        Self {
            dimensions: 3,
            cell: "Cube".to_owned(),
        }
    }
}

/// Named neighborhood declaration available to model and rule ranges.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct NeighborhoodIr {
    /// Stable neighborhood name.
    pub name: Identifier,
    /// Radius expression preserved from source.
    pub radius: String,
    /// Whether the center cell is included.
    pub center: bool,
    /// Distance metric label.
    pub metric: String,
}
