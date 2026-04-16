//! Runtime cell handles and cell-query payload types.

use crate::{AttributeAccessError, AttributeId, AttributeValue, NeighborhoodId};

/// Opaque handle to one logical cell in the active runtime grid.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Cell(u32);

impl Cell {
    /// Construct a cell handle from its raw runtime value.
    pub const fn new(raw: u32) -> Self {
        Self(raw)
    }

    /// Return the raw cell handle value.
    pub const fn raw(self) -> u32 {
        self.0
    }

    pub(crate) const fn index(self) -> usize {
        self.0 as usize
    }
}

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
    UnknownCell(Cell),
    /// The requested neighborhood id is not declared on the active blueprint.
    UnknownNeighborhood(NeighborhoodId),
    /// The underlying attribute lookup failed.
    Attribute(AttributeAccessError),
}

impl From<AttributeAccessError> for CellQueryError {
    fn from(value: AttributeAccessError) -> Self {
        Self::Attribute(value)
    }
}
