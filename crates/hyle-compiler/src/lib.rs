//! Compiler for Hyle source inputs.

pub mod codegen;
pub mod compile;
pub mod diagnostics;
pub mod ir;
pub mod semantics;
pub mod source;
pub mod syntax;

pub use compile::{compile, CompileInput, CompileOptions, CompileOutput};
pub use diagnostics::{Diagnostic, DiagnosticReport, DiagnosticSeverity};
pub use ir::{
    validate_module, BoundsIr, FieldIr, HyleIrError, Identifier, InputIr, LatticeIr, LiteralIr,
    ModelIr, ModuleIr, NeighborhoodIr, PipelineIr, RuleIr, RuleSourceIr, RuleStatementIr,
    SamplingIr, SchemaVersion, StageIr, TypeIr,
};
pub use source::SourceFile;
pub use syntax::parse;
