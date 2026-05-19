//! Code generation scaffold for future `.sole` output.

use thiserror::Error;

use hyle_sole::SoleModule;

/// Generated `.sole` artifact.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SoleArtifact {
    /// Suggested output file extension.
    pub extension: &'static str,
    /// Generated source text.
    pub contents: String,
}

/// Code generation failure.
#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum CodegenError {
    /// `.sole` generation has not been implemented yet.
    #[error(".sole code generation is not implemented yet")]
    NotImplemented,
}

/// Generates `.sole` code from a `.sole` module.
///
/// # Errors
///
/// Always returns [`CodegenError::NotImplemented`] until the code generator is
/// implemented.
pub fn generate_sole(_module: &SoleModule) -> Result<SoleArtifact, CodegenError> {
    Err(CodegenError::NotImplemented)
}
