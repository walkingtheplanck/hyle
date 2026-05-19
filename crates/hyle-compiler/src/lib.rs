//! Compiler for Hyle source inputs.

pub mod codegen;
pub mod compile;
pub mod diagnostics;
pub mod semantics;
pub mod source;
pub mod syntax;

pub use compile::{compile, CompileInput, CompileOptions, CompileOutput};
pub use diagnostics::{Diagnostic, DiagnosticReport, DiagnosticSeverity};
pub use source::SourceFile;
pub use syntax::parse;
