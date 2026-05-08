use hyle_ir::{ModuleIr, SchemaVersion};

use crate::config::parse_config;
use crate::diagnostics::DiagnosticReport;
use crate::dsl::parse_dsl;
use crate::lower::lower_module;
use crate::resolve::resolve_module;
use crate::source::SourceFile;
use crate::typecheck::typecheck_module;

/// The full source set expected by the compiler.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CompileInput {
    /// Primary KDL config source, expected to become `hyle.kdl`.
    pub config: SourceFile,
    /// One or more DSL sources, expected to include files such as `logic.hyle`.
    pub logic: Vec<SourceFile>,
    /// Optional explicit module name override.
    pub module_name: Option<String>,
}

/// Compiler options for the scaffold pipeline.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CompileOptions {
    /// Schema version to stamp onto the lowered IR.
    pub schema_version: SchemaVersion,
}

/// Successful compiler output.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CompileOutput {
    /// Lowered module IR.
    pub module: ModuleIr,
}

/// Compiles config and DSL sources into Hyle IR.
///
/// The config side performs real KDL decoding for the current scaffolded
/// document shape. DSL parsing and lowering remain placeholder stages.
pub fn compile(
    input: CompileInput,
    options: CompileOptions,
) -> Result<CompileOutput, DiagnosticReport> {
    let config = parse_config(&input.config)?;

    let logic = input
        .logic
        .iter()
        .map(parse_dsl)
        .collect::<Result<Vec<_>, _>>()?;

    let resolved = resolve_module(input.module_name.as_deref(), config, logic, &input.config)?;

    typecheck_module(&resolved)?;

    let module = lower_module(&resolved, options.schema_version).map_err(|error| {
        let mut report = DiagnosticReport::new();
        report.push(crate::diagnostics::Diagnostic::error(
            None::<String>,
            error.to_string(),
        ));
        report
    })?;

    Ok(CompileOutput { module })
}
