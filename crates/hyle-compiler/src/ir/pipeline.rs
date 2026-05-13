use serde::{Deserialize, Serialize};

use crate::Identifier;

/// A named execution stage for the module pipeline.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StageIr {
    /// Stable stage name.
    pub name: Identifier,
    /// Rules executed in this stage.
    pub rules: Vec<Identifier>,
}

/// Ordered execution stages for a compiled module.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct PipelineIr {
    /// Stages executed by a backend for one logical step.
    pub stages: Vec<StageIr>,
}
