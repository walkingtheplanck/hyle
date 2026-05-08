use crate::diagnostics::DiagnosticReport;
use crate::resolve::ResolvedModule;

/// Type checks a resolved module.
///
/// The scaffold currently performs no semantic checks beyond preserving the
/// pipeline shape.
pub fn typecheck_module(_module: &ResolvedModule) -> Result<(), DiagnosticReport> {
    Ok(())
}
