//! Semantic analysis and lowering from syntax AST to `.sole` IR.

pub mod hir;
pub mod lower;

pub use lower::lower_script;
