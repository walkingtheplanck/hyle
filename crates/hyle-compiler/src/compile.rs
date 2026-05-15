use crate::codegen::sole_ir::SoleModule;
use crate::diagnostics::{Diagnostic, DiagnosticReport};
use crate::semantics::lower_script;
use crate::source::SourceFile;
use crate::syntax::{parse, SyntaxError};

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
