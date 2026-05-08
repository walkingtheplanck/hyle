use crate::config::ConfigAst;
use crate::diagnostics::DiagnosticReport;
use crate::dsl::DslAst;
use crate::source::SourceFile;

/// Resolved inputs ready for type checking and lowering.
#[derive(Clone, Debug, PartialEq)]
pub struct ResolvedModule {
    /// Placeholder module name used during lowering.
    pub module_name: String,
    /// Parsed config source.
    pub config: ConfigAst,
    /// Parsed DSL sources.
    pub logic: Vec<DslAst>,
}

/// Resolves top-level source naming into a lowering-friendly shape.
pub fn resolve_module(
    requested_name: Option<&str>,
    config: ConfigAst,
    logic: Vec<DslAst>,
    _config_source: &SourceFile,
) -> Result<ResolvedModule, DiagnosticReport> {
    let module_name = requested_name
        .map(str::trim)
        .filter(|name| !name.is_empty())
        .unwrap_or("unnamed");

    Ok(ResolvedModule {
        module_name: module_name.to_owned(),
        config,
        logic,
    })
}
