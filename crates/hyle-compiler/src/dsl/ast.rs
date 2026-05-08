/// Placeholder AST for a parsed Hyle DSL source.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DslAst {
    /// Source path used for diagnostics and placeholder rule names.
    pub source_path: String,
    /// Raw source preserved until real parsing lands.
    pub raw: String,
}
