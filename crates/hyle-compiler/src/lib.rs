//! Compiler for Hyle source inputs.

pub mod codegen;
pub mod compile;
pub mod diagnostics;
pub mod semantics;
pub mod source;
pub mod syntax;

pub use codegen::sole_ir::{
    SoleBounds, SoleCall, SoleExpr, SoleField, SoleInput, SoleLet, SoleLiteral, SoleLiteralValue,
    SoleModel, SoleModule, SoleNeighbors, SoleOpExpr, SoleRange, SoleRead, SoleReduce, SoleRule,
    SoleSample, SoleWorld, SoleWrite,
};
pub use compile::{compile, CompileInput, CompileOptions, CompileOutput};
pub use diagnostics::{Diagnostic, DiagnosticReport, DiagnosticSeverity};
pub use source::SourceFile;
pub use syntax::parse;
