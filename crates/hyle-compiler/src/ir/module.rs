use serde::{Deserialize, Serialize};

use crate::ir::{
    Identifier, InputIr, LatticeIr, ModelIr, NeighborhoodIr, PipelineIr, RuleIr, SchemaVersion,
};

/// The top-level IR artifact produced by the compiler.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModuleIr {
    /// Schema version for serialization and compatibility checks.
    pub schema_version: SchemaVersion,
    /// Module name.
    pub name: Identifier,
    /// Lattice declaration.
    pub lattice: LatticeIr,
    /// Named neighborhoods declared by the source.
    pub neighborhoods: Vec<NeighborhoodIr>,
    /// Models declared by the source.
    pub models: Vec<ModelIr>,
    /// External simulation inputs.
    pub inputs: Vec<InputIr>,
    /// Rules lowered from the DSL.
    pub rules: Vec<RuleIr>,
    /// Ordered pipeline stages.
    pub pipeline: PipelineIr,
}
