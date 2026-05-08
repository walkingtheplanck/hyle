use serde::{Deserialize, Serialize};

use crate::{Identifier, LatticeIr, ModelIr, PipelineIr, RuleIr, SchemaVersion};

/// The top-level IR artifact produced by the compiler.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModuleIr {
    /// Schema version for serialization and compatibility checks.
    pub schema_version: SchemaVersion,
    /// Module name.
    pub name: Identifier,
    /// Lattice declaration.
    pub lattice: LatticeIr,
    /// Model fields referenced by the module.
    pub model: ModelIr,
    /// Rules lowered from the DSL.
    pub rules: Vec<RuleIr>,
    /// Ordered pipeline stages.
    pub pipeline: PipelineIr,
}
