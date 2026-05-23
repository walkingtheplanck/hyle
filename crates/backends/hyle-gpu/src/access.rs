use std::collections::{HashMap, HashSet};

use hyle_runtime::{
    Cell, CellBatch, CellFieldColumn, CellPosition, CellRead, CellWrite, EncodedCellIo, HyleValue,
    InputAccess, RuntimeError,
};

#[derive(Default)]
pub struct GpuAccess {
    active_cells: HashSet<(String, CellPosition)>,
    inputs: HashMap<String, HyleValue>,
}

impl InputAccess for GpuAccess {
    fn get_input(&self, name: &str) -> Result<HyleValue, RuntimeError> {
        self.inputs
            .get(name)
            .cloned()
            .ok_or_else(|| RuntimeError::InputAccess(format!("unknown input `{name}`")))
    }

    fn set_input(&mut self, name: &str, value: HyleValue) -> Result<(), RuntimeError> {
        self.inputs.insert(name.to_owned(), value);
        Ok(())
    }
}

impl CellRead for GpuAccess {
    fn cell_exists(&self, model: &str, position: &CellPosition) -> Result<bool, RuntimeError> {
        Ok(self
            .active_cells
            .contains(&(model.to_owned(), position.clone())))
    }

    fn read_cell(
        &self,
        model: &str,
        position: &CellPosition,
    ) -> Result<Option<Cell>, RuntimeError> {
        if self.cell_exists(model, position)? {
            Ok(Some(Cell {
                model: model.to_owned(),
                position: position.clone(),
                fields: Vec::new(),
            }))
        } else {
            Ok(None)
        }
    }

    fn read_cells(
        &self,
        model: &str,
        positions: &[CellPosition],
    ) -> Result<Vec<Option<Cell>>, RuntimeError> {
        positions
            .iter()
            .map(|position| self.read_cell(model, position))
            .collect()
    }

    fn read_batch(
        &self,
        model: &str,
        fields: &[&str],
        positions: &[CellPosition],
    ) -> Result<CellBatch, RuntimeError> {
        for position in positions {
            if !self.cell_exists(model, position)? {
                return Err(RuntimeError::FieldAccess(format!(
                    "missing `{model}` cell at {:?}",
                    position.coordinates
                )));
            }
        }

        Ok(CellBatch {
            model: model.to_owned(),
            positions: positions.to_vec(),
            fields: fields
                .iter()
                .map(|field| CellFieldColumn {
                    field_name: (*field).to_owned(),
                    values: hyle_runtime::FieldColumnValues::F64(Vec::new()),
                })
                .collect(),
        })
    }

    fn get_field(
        &self,
        model: &str,
        field: &str,
        position: &CellPosition,
    ) -> Result<Option<HyleValue>, RuntimeError> {
        if self.cell_exists(model, position)? {
            Err(RuntimeError::FieldAccess(format!(
                "gpu field storage is not implemented for `{model}.{field}`"
            )))
        } else {
            Ok(None)
        }
    }
}

impl CellWrite for GpuAccess {
    fn set_field(
        &mut self,
        model: &str,
        field: &str,
        position: &CellPosition,
        _value: HyleValue,
    ) -> Result<(), RuntimeError> {
        if self.cell_exists(model, position)? {
            Err(RuntimeError::FieldAccess(format!(
                "gpu field storage is not implemented for `{model}.{field}`"
            )))
        } else {
            Err(RuntimeError::CellEdit(format!(
                "missing `{model}` cell at {:?}",
                position.coordinates
            )))
        }
    }

    fn add_cells(&mut self, batch: CellBatch) -> Result<(), RuntimeError> {
        let count = batch.positions.len();
        for field in &batch.fields {
            if field.values.len() != count {
                return Err(RuntimeError::CellEdit(format!(
                    "field `{}` has {} values for {count} cells",
                    field.field_name,
                    field.values.len()
                )));
            }
        }

        for position in batch.positions {
            self.active_cells.insert((batch.model.clone(), position));
        }

        Ok(())
    }

    fn update_cells(&mut self, batch: CellBatch) -> Result<(), RuntimeError> {
        for position in &batch.positions {
            if !self.cell_exists(&batch.model, position)? {
                return Err(RuntimeError::CellEdit(format!(
                    "missing `{}` cell at {:?}",
                    batch.model, position.coordinates
                )));
            }
        }

        self.add_cells(batch)
    }

    fn upsert_cells(&mut self, batch: CellBatch) -> Result<(), RuntimeError> {
        self.add_cells(batch)
    }

    fn remove_cells(
        &mut self,
        model: &str,
        positions: &[CellPosition],
    ) -> Result<(), RuntimeError> {
        for position in positions {
            if !self
                .active_cells
                .remove(&(model.to_owned(), position.clone()))
            {
                return Err(RuntimeError::CellEdit(format!(
                    "missing `{model}` cell at {:?}",
                    position.coordinates
                )));
            }
        }

        Ok(())
    }
}

impl EncodedCellIo for GpuAccess {
    fn write_encoded_cells(&mut self, _bytes: &[u8]) -> Result<(), RuntimeError> {
        Err(RuntimeError::CellEdit(
            "gpu encoded cell IO is not implemented".to_owned(),
        ))
    }

    fn read_encoded_cells(&self, _request: &[u8], _out: &mut [u8]) -> Result<usize, RuntimeError> {
        Err(RuntimeError::CellEdit(
            "gpu encoded cell IO is not implemented".to_owned(),
        ))
    }
}
