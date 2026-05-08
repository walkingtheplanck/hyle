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
