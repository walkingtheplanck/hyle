//! Runtime attribute query capabilities.

use crate::{
    AttributeAccessError, AttributeId, AttributeValue, CellAttributeValue, CellId, CellQueryError,
};

/// Attribute-oriented queries exposed by a live runtime.
pub trait RuntimeAttributes {
    /// Read one attached attribute from a resolved cell handle.
    fn attribute(
        &self,
        cell: CellId,
        attribute: AttributeId,
    ) -> Result<AttributeValue, CellQueryError>;

    /// Read all declared attached attributes from a resolved cell handle.
    fn attributes(&self, cell: CellId) -> Result<Vec<CellAttributeValue>, CellQueryError>;

    /// Read one attached attribute by id from the resolved cell coordinate.
    fn get_attr(
        &self,
        attribute: AttributeId,
        x: i32,
        y: i32,
        z: i32,
    ) -> Result<AttributeValue, AttributeAccessError>;

    /// Overwrite one attached attribute by id at the resolved cell coordinate.
    fn set_attr(
        &mut self,
        attribute: AttributeId,
        x: i32,
        y: i32,
        z: i32,
        value: AttributeValue,
    ) -> Result<(), AttributeAccessError>;
}
