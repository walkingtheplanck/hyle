use crate::diagnostics::{Diagnostic, DiagnosticReport};
use crate::semantics::lower_script;
use crate::syntax::{parse, SyntaxError};
use hyle_sole::SoleModule;

/// An input source file consumed by the compiler scaffold.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SourceFile {
    /// Human-readable source path.
    pub path: String,
    /// Full file contents.
    pub contents: String,
}

impl SourceFile {
    /// Builds a source file from owned strings.
    pub fn new(path: impl Into<String>, contents: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            contents: contents.into(),
        }
    }
}

/// Source input expected by the compiler.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CompileInput {
    /// Single `.hyle` script.
    pub source: SourceFile,
    /// Optional explicit module name override.
    pub module_name: Option<String>,
}

/// Compiler options for the scaffold pipeline.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CompileOptions {}

/// Successful compiler output.
#[derive(Clone, Debug, PartialEq)]
pub struct CompileOutput {
    /// Lowered `.sole` IR.
    pub module: SoleModule,
}

/// Compiles a single `.hyle` script into `.sole` IR.
///
/// The code generation stage is scaffolded but intentionally not implemented
/// yet, so this function currently produces the `.sole` JSON data model only.
pub fn compile(
    input: CompileInput,
    _options: CompileOptions,
) -> Result<CompileOutput, DiagnosticReport> {
    let mut syntax =
        parse(&input.source.contents).map_err(|error| syntax_report(&input.source, error))?;
    syntax.source_path = input.source.path.clone();
    let module = lower_script(&syntax, input.module_name.as_deref())?;

    Ok(CompileOutput { module })
}

fn syntax_report(source: &SourceFile, error: SyntaxError) -> DiagnosticReport {
    let mut report = DiagnosticReport::new();
    report.push(Diagnostic::error(
        Some(source.path.clone()),
        error.to_string(),
    ));
    report
}
