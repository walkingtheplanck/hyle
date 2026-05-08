/// Placeholder AST for a parsed Hyle config source.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConfigAst {
    /// Source path used for diagnostics and lowering defaults.
    pub source_path: String,
    /// Raw source preserved until real parsing lands.
    pub raw: String,
}
