//! Runtime payload for one queried cell attribute.

use crate::{AttributeId, AttributeValue};

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
