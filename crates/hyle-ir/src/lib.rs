//! Shared typed IR for Hyle.

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
pub use lattice::LatticeIr;
pub use model::{FieldIr, ModelIr};
pub use module::ModuleIr;
pub use pipeline::{PipelineIr, StageIr};
pub use rule::RuleIr;
pub use types::TypeIr;
pub use validate::{validate_module, HyleIrError};
pub use version::SchemaVersion;
