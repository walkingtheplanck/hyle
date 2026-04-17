//! Solver attribute query capabilities.

use crate::{
    CellAttributeValue, CellId, CellQueryError, AttributeId, AttributeValue,
};

use super::{SolverExecution, SolverMetadata, SolverCells};

/// Attribute-oriented queries derived from core solver execution.
pub trait SolverAttributes: SolverExecution + SolverMetadata + SolverCells {
    /// Read one attached attribute from a resolved cell handle.
    fn attribute(
        &self,
        cell: CellId,
        attribute: AttributeId,
    ) -> Result<AttributeValue, CellQueryError> {
        let [x, y, z] = self.cell_position(cell)?;
        self.get_attr(attribute, x as i32, y as i32, z as i32)
            .map_err(CellQueryError::from)
    }

    /// Read all declared attached attributes from a resolved cell handle.
    fn attributes(&self, cell: CellId) -> Result<Vec<CellAttributeValue>, CellQueryError> {
        let mut values = Vec::with_capacity(self.attribute_defs().len());
        for attribute in self.attribute_defs() {
            values.push(CellAttributeValue::new(
                attribute.id,
                self.attribute(cell, attribute.id)?,
            ));
        }
        Ok(values)
    }
}
