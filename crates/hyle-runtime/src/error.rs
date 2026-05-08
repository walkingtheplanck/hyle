use thiserror::Error;

/// Errors emitted by runtime-facing scaffolds.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum RuntimeError {
    /// The backend rejected the supplied module.
    #[error("module load failed: {0}")]
    ModuleLoad(String),
    /// The backend rejected instance creation.
    #[error("instance creation failed: {0}")]
    InstanceCreate(String),
    /// The backend cannot advance the instance.
    #[error("step failed: {0}")]
    Step(String),
    /// Generic field access failure.
    #[error("field access failed: {0}")]
    FieldAccess(String),
}
