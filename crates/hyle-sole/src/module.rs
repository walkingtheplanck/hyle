use std::fmt;

use serde::{Deserialize, Serialize};

use crate::{SoleInput, SoleModel, SoleRange, SoleRule, SoleWorld};

/// `.sole` JSON module produced by code generation.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SoleModule {
    /// `.sole` schema version.
    pub version: String,
    /// World lattice declaration.
    pub world: SoleWorld,
    /// Named neighborhood ranges.
    pub ranges: Vec<SoleRange>,
    /// Runtime models.
    pub models: Vec<SoleModel>,
    /// External inputs.
    pub inputs: Vec<SoleInput>,
    /// Executable update/transform rules.
    pub rules: Vec<SoleRule>,
}

impl SoleModule {
    /// Serializes this module to pretty `.sole.json`.
    ///
    /// # Errors
    ///
    /// Returns a JSON serialization error if any contained value cannot be
    /// represented as JSON.
    pub fn to_json_string(&self) -> Result<String, serde_json::Error> {
        crate::encode_sole_json(self)
    }

    /// Parses a `.sole.json` string into a module.
    ///
    /// # Errors
    ///
    /// Returns a JSON deserialization error if `source` is not valid
    /// `.sole.json`.
    pub fn from_json_str(source: &str) -> Result<Self, serde_json::Error> {
        crate::decode_sole_json(source)
    }
}

impl fmt::Display for SoleModule {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let json = self.to_json_string().map_err(|_| fmt::Error)?;
        formatter.write_str(&json)
    }
}
