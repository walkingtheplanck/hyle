//! Runtime attribute query capabilities.

use crate::{
    AttributeAccessError, AttributeId, AttributeValue, CellAttributeValue, CellId, CellQueryError,
};

/// Attribute-oriented queries exposed by a live runtime.
pub trait RuntimeAttributes {
    /// Read one attached attribute from a resolved cell handle.
    ///
    /// # Errors
    ///
    /// Returns [`CellQueryError`] when the cell handle is invalid or the
    /// attribute is not declared on the active schema/runtime.
    fn attribute(
        &self,
        cell: CellId,
        attribute: AttributeId,
    ) -> Result<AttributeValue, CellQueryError>;

    /// Read all declared attached attributes from a resolved cell handle.
    ///
    /// The returned list is ordered like [`RuntimeMetadata::attribute_defs`],
    /// which makes it stable for UIs and analysis tooling.
    fn attributes(&self, cell: CellId) -> Result<Vec<CellAttributeValue>, CellQueryError>;

    /// Read one attached attribute by id from the resolved cell coordinate.
    ///
    /// # Errors
    ///
    /// Returns [`AttributeAccessError`] when the coordinate is invalid for the
    /// current topology or the attribute does not exist at that location.
    fn get_attr(
        &self,
        attribute: AttributeId,
        x: i32,
        y: i32,
        z: i32,
    ) -> Result<AttributeValue, AttributeAccessError>;

    /// Overwrite one attached attribute by id at the resolved cell coordinate.
    ///
    /// # Errors
    ///
    /// Returns [`AttributeAccessError`] when the coordinate is invalid, the
    /// attribute is unavailable, or the provided value uses the wrong scalar
    /// type for that attribute channel.
    fn set_attr(
        &mut self,
        attribute: AttributeId,
        x: i32,
        y: i32,
        z: i32,
        value: AttributeValue,
    ) -> Result<(), AttributeAccessError>;
}
