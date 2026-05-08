/// Backend categories supported by the runtime scaffold.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DispatchTarget {
    /// CPU proof-of-concept backend.
    Cpu,
    /// GPU proof-of-concept backend.
    Gpu,
    /// Viewer-only consumer.
    Viewer,
    /// Custom backend label.
    Other(String),
}
