use hyle_runtime::{
    Cell, CellBatch, CellPosition, CellRead, CellWrite, EncodedCellIo, HyleValue, InputAccess,
    Instance, RuntimeError, SolverBackend, Step,
};
use hyle_sole::{decode_sole_json_bytes, SoleModule};

use crate::access::GpuAccess;

/// Placeholder GPU solver used to validate runtime wiring.
#[derive(Default)]
pub struct GpuSolver;

impl SolverBackend for GpuSolver {
    fn name(&self) -> &'static str {
        "gpu"
    }

    fn init(&self, sole: &[u8]) -> Result<Box<dyn Instance>, RuntimeError> {
        let module = decode_sole_json_bytes(sole)
            .map_err(|error| RuntimeError::ModuleLoad(error.to_string()))?;
        Ok(Box::new(GpuInstance {
            access: GpuAccess::default(),
            module,
            steps: 0,
        }))
    }
}

struct GpuInstance {
    access: GpuAccess,
    module: SoleModule,
    steps: u64,
}

impl Step for GpuInstance {
    fn step(&mut self) -> Result<(), RuntimeError> {
        let _ = &self.module;
        self.steps += 1;
        Ok(())
    }
}

impl InputAccess for GpuInstance {
    fn get_input(&self, name: &str) -> Result<HyleValue, RuntimeError> {
        self.access.get_input(name)
    }

    fn set_input(&mut self, name: &str, value: HyleValue) -> Result<(), RuntimeError> {
        self.access.set_input(name, value)
    }
}

impl CellRead for GpuInstance {
    fn cell_exists(&self, model: &str, position: &CellPosition) -> Result<bool, RuntimeError> {
        self.access.cell_exists(model, position)
    }

    fn read_cell(
        &self,
        model: &str,
        position: &CellPosition,
    ) -> Result<Option<Cell>, RuntimeError> {
        self.access.read_cell(model, position)
    }

    fn read_cells(
        &self,
        model: &str,
        positions: &[CellPosition],
    ) -> Result<Vec<Option<Cell>>, RuntimeError> {
        self.access.read_cells(model, positions)
    }

    fn read_batch(
        &self,
        model: &str,
        fields: &[&str],
        positions: &[CellPosition],
    ) -> Result<CellBatch, RuntimeError> {
        self.access.read_batch(model, fields, positions)
    }

    fn get_field(
        &self,
        model: &str,
        field: &str,
        position: &CellPosition,
    ) -> Result<Option<HyleValue>, RuntimeError> {
        self.access.get_field(model, field, position)
    }
}

impl CellWrite for GpuInstance {
    fn set_field(
        &mut self,
        model: &str,
        field: &str,
        position: &CellPosition,
        value: HyleValue,
    ) -> Result<(), RuntimeError> {
        self.access.set_field(model, field, position, value)
    }

    fn add_cells(&mut self, batch: CellBatch) -> Result<(), RuntimeError> {
        self.access.add_cells(batch)
    }

    fn update_cells(&mut self, batch: CellBatch) -> Result<(), RuntimeError> {
        self.access.update_cells(batch)
    }

    fn upsert_cells(&mut self, batch: CellBatch) -> Result<(), RuntimeError> {
        self.access.upsert_cells(batch)
    }

    fn remove_cells(
        &mut self,
        model: &str,
        positions: &[CellPosition],
    ) -> Result<(), RuntimeError> {
        self.access.remove_cells(model, positions)
    }
}

impl EncodedCellIo for GpuInstance {
    fn write_encoded_cells(&mut self, bytes: &[u8]) -> Result<(), RuntimeError> {
        self.access.write_encoded_cells(bytes)
    }

    fn read_encoded_cells(&self, request: &[u8], out: &mut [u8]) -> Result<usize, RuntimeError> {
        self.access.read_encoded_cells(request, out)
    }
}
