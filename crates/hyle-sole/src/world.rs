use serde::{Deserialize, Serialize};

use crate::SoleLiteralValue;

/// World lattice declaration.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SoleWorld {
    /// Number of dimensions.
    pub dimensions: u8,
    /// Cell shape name.
    pub cell: String,
}

/// Named neighborhood range.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SoleRange {
    /// Range id.
    pub id: usize,
    /// Source range name.
    pub name: String,
    /// Numeric radius.
    pub radius: SoleLiteralValue,
    /// Whether the center cell is included.
    pub center: bool,
    /// Distance metric name.
    pub metric: String,
}
