use crate::config::ast::ConfigAst;
use crate::diagnostics::{Diagnostic, DiagnosticReport};
use crate::source::SourceFile;

/// Parses a KDL config source into a placeholder AST.
///
/// The scaffold keeps the raw source and performs only minimal empty-input
/// checks. Real KDL parsing belongs here later.
pub fn parse_config(source: &SourceFile) -> Result<ConfigAst, DiagnosticReport> {
    if source.contents.trim().is_empty() {
        let mut report = DiagnosticReport::new();
        report.push(Diagnostic::error(
            Some(source.path.clone()),
            "config source is empty",
        ));
        return Err(report);
    }

    Ok(ConfigAst {
        source_path: source.path.clone(),
        raw: source.contents.clone(),
    })
}
