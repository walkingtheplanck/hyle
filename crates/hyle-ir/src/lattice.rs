use serde::{Deserialize, Serialize};

/// Lattice metadata shared across compiler and runtime crates.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct LatticeIr {
    /// Number of lattice dimensions.
    pub dimensions: u8,
    /// Free-form topology label until the config schema is finalized.
    pub topology: String,
}

impl Default for LatticeIr {
    fn default() -> Self {
        Self {
            dimensions: 2,
            topology: "rect".to_owned(),
        }
    }
}
