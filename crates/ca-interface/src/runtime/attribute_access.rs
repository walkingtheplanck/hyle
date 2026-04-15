//! Shared runtime attribute-access error types.

/// Errors raised by runtime attribute reads and writes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AttributeAccessError {
    /// The named attribute is not declared on the active blueprint.
    UnknownAttribute(String),
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
