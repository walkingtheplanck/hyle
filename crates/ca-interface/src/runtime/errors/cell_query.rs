//! Runtime cell-query error types.

use crate::{AttributeAccessError, CellId, NeighborhoodId};

/// Errors raised while querying runtime cell data.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CellQueryError {
    /// The requested cell handle does not belong to the active runtime.
    UnknownCell(CellId),
    /// The requested neighborhood id is not declared on the active schema.
    UnknownNeighborhood(NeighborhoodId),
    /// Neighborhood queries require schema metadata that is not available.
    SchemaUnavailable,
    /// The underlying attribute lookup failed.
    Attribute(AttributeAccessError),
}

impl From<AttributeAccessError> for CellQueryError {
    fn from(value: AttributeAccessError) -> Self {
        Self::Attribute(value)
    }
}
