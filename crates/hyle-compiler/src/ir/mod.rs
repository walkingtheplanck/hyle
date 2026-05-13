//! Typed intermediate representation produced by the Hyle compiler.

pub mod ids;
pub mod lattice;
pub mod model;
pub mod module;
pub mod pipeline;
pub mod rule;
pub mod types;
pub mod validate;
pub mod version;

pub use ids::Identifier;
pub use lattice::{LatticeIr, NeighborhoodIr};
pub use model::{BoundsIr, FieldIr, InputIr, LiteralIr, ModelIr};
pub use module::ModuleIr;
pub use pipeline::{PipelineIr, StageIr};
pub use rule::{RuleIr, RuleSourceIr, RuleStatementIr, SamplingIr};
pub use types::TypeIr;
pub use validate::{validate_module, HyleIrError};
pub use version::SchemaVersion;
