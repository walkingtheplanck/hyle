use hyle_runtime::SolverBackend;

pub use hyle_runtime::{
    Cell, CellBatch, CellFieldColumn, CellFieldValue, CellId, CellPosition, CellRead, CellWrite,
    EncodedCellIo, FieldColumnValues, HyleValue, InputAccess, Instance, RuntimeError, Step,
};

pub struct Solver {
    inner: Box<dyn SolverBackend>,
}

impl Solver {
    pub fn name(&self) -> &'static str {
        self.inner.name()
    }

    pub fn init(&self, sole: &[u8]) -> Result<Box<dyn Instance>, RuntimeError> {
        self.inner.init(sole)
    }
}

pub fn solver<B>(backend: B) -> Solver
where
    B: SolverBackend + 'static,
{
    Solver {
        inner: Box::new(backend),
    }
}
