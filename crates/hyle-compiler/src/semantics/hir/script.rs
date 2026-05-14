use crate::semantics::hir::hir_ids::*;
use crate::semantics::hir::hir_rules::RuleHir;
use crate::semantics::hir::hir_values::*;
use std::collections::HashMap;
// ----------------
// Version
// ----------------

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VersionHir {
    pub major: u16,
    pub minor: u16,
}

// ----------------
// World
// ----------------

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CellShape {
    Line,
    Triangle,
    Square,
    Hexagon,
    Cube,
    Tetrahedron,
    TruncatedOctahedron,
    RhombicDodecahedron,
    Tesseract,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorldHir {
    pub dimensions: u8,
    pub cell: CellShape,
}

// ----------------
// Neighborhood
// ----------------

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DistanceMetric {
    Manhattan,
    Euclidean,
    Chebyshev,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NeighborhoodHir {
    pub id: NeighborhoodId,
    pub name: String,
    pub radius: NumberValue,
    pub center: bool,
    pub metric: DistanceMetric,
}

// ----------------
// Model
// ----------------

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FieldHir {
    pub id: FieldId,
    pub model: ModelId,
    pub name: String,
    pub ty: ScalarType,
    pub default: Option<ConstValue>,
    pub bounds: Option<BoundsHir>,
    pub precision: PrecisionHir,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ModelHir {
    pub id: ModelId,
    pub name: String,
    pub resolution: u32,
    pub default_range: Option<NeighborhoodId>,
    pub fields: Vec<FieldHir>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InputHir {
    pub id: InputId,
    pub name: String,
    pub ty: ScalarType,
    pub default: Option<ConstValue>,
    pub bounds: Option<BoundsHir>,
    pub precision: PrecisionHir,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ScriptHir {
    pub source_path: String,
    pub version: VersionHir,
    pub world: WorldHir,

    pub neighborhoods: Vec<NeighborhoodHir>,
    pub models: Vec<ModelHir>,
    pub inputs: Vec<InputHir>,
    pub rules: Vec<RuleHir>,

    pub index: HirIndex,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct HirIndex {
    pub models_by_name: HashMap<String, ModelId>,
    pub neighborhoods_by_name: HashMap<String, NeighborhoodId>,
    pub inputs_by_name: HashMap<String, InputId>,
    pub fields_by_model: HashMap<ModelId, HashMap<String, FieldId>>,
}
