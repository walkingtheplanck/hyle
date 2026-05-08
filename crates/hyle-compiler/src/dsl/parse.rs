use crate::diagnostics::{Diagnostic, DiagnosticReport};
use crate::dsl::ast::DslAst;
use crate::source::SourceFile;

/// Parses a Hyle DSL source into a placeholder AST.
///
/// The scaffold accepts any non-empty file and defers real syntax handling to a
/// future parser implementation.
pub fn parse_dsl(source: &SourceFile) -> Result<DslAst, DiagnosticReport> {
    if source.contents.trim().is_empty() {
        let mut report = DiagnosticReport::new();
        report.push(Diagnostic::error(
            Some(source.path.clone()),
            "dsl source is empty",
        ));
        return Err(report);
    }

    Ok(DslAst {
        source_path: source.path.clone(),
        raw: source.contents.clone(),
    })
}
