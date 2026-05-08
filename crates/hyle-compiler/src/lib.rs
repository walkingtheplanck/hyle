//! Compiler scaffold for Hyle source inputs.

pub mod compile;
pub mod config;
pub mod diagnostics;
pub mod dsl;
pub mod lower;
pub mod resolve;
pub mod source;
pub mod typecheck;

pub use compile::{compile, CompileInput, CompileOptions, CompileOutput};
pub use diagnostics::{Diagnostic, DiagnosticReport, DiagnosticSeverity};
pub use source::SourceFile;
