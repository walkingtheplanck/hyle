use crate::RuntimeError;

/// Read access to backend field storage.
pub trait FieldReader {
    /// Reads the raw bytes for one logical field slot.
    fn read_field(&self, field: &str, index: u64) -> Result<Vec<u8>, RuntimeError>;
}

/// Write access to backend field storage.
pub trait FieldWriter {
    /// Writes raw bytes into one logical field slot.
    fn write_field(&mut self, field: &str, index: u64, bytes: &[u8]) -> Result<(), RuntimeError>;
}
