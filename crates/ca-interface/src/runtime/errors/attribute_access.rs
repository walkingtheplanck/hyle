//! Shared runtime attribute-access error types.

use crate::AttributeId;

/// Errors raised by runtime attribute reads and writes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AttributeAccessError {
    /// The requested attribute id is not declared on the active schema.
    UnknownAttribute(AttributeId),
    /// The requested coordinate resolves outside the simulation bounds.
    OutOfBounds {
        /// X coordinate requested by the caller.
        x: i32,
        /// Y coordinate requested by the caller.
        y: i32,
        /// Z coordinate requested by the caller.
        z: i32,
    },
}
