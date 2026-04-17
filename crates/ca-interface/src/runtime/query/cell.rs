//! Runtime cell-query payload types and errors.

use crate::{AttributeAccessError, AttributeId, AttributeValue, CellId, NeighborhoodId};

/// One current attached attribute value for a queried cell.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CellAttributeValue {
    /// Queried attribute identifier.
    pub attribute: AttributeId,
    /// Current value stored on the cell.
    pub value: AttributeValue,
}

impl CellAttributeValue {
    /// Construct a new attribute/value pair.
    pub const fn new(attribute: AttributeId, value: AttributeValue) -> Self {
        Self { attribute, value }
    }
}

/// Errors raised while querying runtime cell data.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CellQueryError {
    /// The requested cell handle does not belong to the active runtime.
    UnknownCell(CellId),
    /// The requested neighborhood id is not declared on the active schema.
    UnknownNeighborhood(NeighborhoodId),
    /// The underlying attribute lookup failed.
    Attribute(AttributeAccessError),
}

impl From<AttributeAccessError> for CellQueryError {
    fn from(value: AttributeAccessError) -> Self {
        Self::Attribute(value)
    }
}
