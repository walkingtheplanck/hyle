use crate::RuntimeError;

pub trait SolverBackend: Send + Sync {
    fn name(&self) -> &'static str;

    fn init(&self, sole: &[u8]) -> Result<Box<dyn Instance>, RuntimeError>;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CellId(pub u64);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CellPosition {
    pub coordinates: Vec<i64>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Cell {
    pub model: String,
    pub position: CellPosition,
    pub fields: Vec<CellFieldValue>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CellFieldValue {
    pub field_name: String,
    pub value: HyleValue,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CellBatch {
    pub model: String,
    pub positions: Vec<CellPosition>,
    pub fields: Vec<CellFieldColumn>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CellFieldColumn {
    pub field_name: String,
    pub values: FieldColumnValues,
}

#[derive(Clone, Debug, PartialEq)]
pub enum FieldColumnValues {
    Bool(Vec<bool>),
    I32(Vec<i32>),
    U32(Vec<u32>),
    I64(Vec<i64>),
    U64(Vec<u64>),
    F32(Vec<f32>),
    F64(Vec<f64>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum HyleValue {
    Bool(bool),
    I32(i32),
    U32(u32),
    I64(i64),
    U64(u64),
    F32(f32),
    F64(f64),
}

impl FieldColumnValues {
    pub fn len(&self) -> usize {
        match self {
            Self::Bool(v) => v.len(),
            Self::I32(v) => v.len(),
            Self::U32(v) => v.len(),
            Self::I64(v) => v.len(),
            Self::U64(v) => v.len(),
            Self::F32(v) => v.len(),
            Self::F64(v) => v.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

pub trait Instance: Step + InputAccess + CellRead + CellWrite + EncodedCellIo + Send {}

impl<T> Instance for T where T: Step + InputAccess + CellRead + CellWrite + EncodedCellIo + Send {}
pub trait Step {
    fn step(&mut self) -> Result<(), RuntimeError>;
}

pub trait InputAccess {
    fn get_input(&self, name: &str) -> Result<HyleValue, RuntimeError>;
    fn set_input(&mut self, name: &str, value: HyleValue) -> Result<(), RuntimeError>;
}

pub trait CellRead {
    fn cell_exists(&self, model: &str, position: &CellPosition) -> Result<bool, RuntimeError>;

    fn read_cell(&self, model: &str, position: &CellPosition)
        -> Result<Option<Cell>, RuntimeError>;

    fn read_cells(
        &self,
        model: &str,
        positions: &[CellPosition],
    ) -> Result<Vec<Option<Cell>>, RuntimeError>;

    fn read_batch(
        &self,
        model: &str,
        fields: &[&str],
        positions: &[CellPosition],
    ) -> Result<CellBatch, RuntimeError>;

    fn get_field(
        &self,
        model: &str,
        field: &str,
        position: &CellPosition,
    ) -> Result<Option<HyleValue>, RuntimeError>;
}

pub trait CellWrite {
    fn set_field(
        &mut self,
        model: &str,
        field: &str,
        position: &CellPosition,
        value: HyleValue,
    ) -> Result<(), RuntimeError>;

    fn add_cells(&mut self, batch: CellBatch) -> Result<(), RuntimeError>;
    fn update_cells(&mut self, batch: CellBatch) -> Result<(), RuntimeError>;
    fn upsert_cells(&mut self, batch: CellBatch) -> Result<(), RuntimeError>;

    fn remove_cells(&mut self, model: &str, positions: &[CellPosition])
        -> Result<(), RuntimeError>;
}

pub trait EncodedCellIo {
    fn write_encoded_cells(&mut self, bytes: &[u8]) -> Result<(), RuntimeError>;

    fn read_encoded_cells(&self, request: &[u8], out: &mut [u8]) -> Result<usize, RuntimeError>;
}
