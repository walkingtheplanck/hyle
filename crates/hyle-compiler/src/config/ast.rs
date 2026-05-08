use serde::{Deserialize, Serialize};

/// Parsed top-level Hyle configuration document.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConfigAst {
    /// Source path used for diagnostics and later compiler stages.
    pub source_path: String,
    /// Declared Hyle language version.
    pub hyle: HyleDirective,
    /// Global world metadata.
    pub world: WorldConfig,
    /// Declared lattices available to models.
    pub lattices: Vec<LatticeConfig>,
    /// Named neighborhoods available to pipeline runs.
    pub neighborhoods: Vec<NeighborhoodConfig>,
    /// Declared models and their fields.
    pub models: Vec<ModelConfig>,
    /// Declared simulations and their pipelines.
    pub simulations: Vec<SimulationConfig>,
}

/// Declares the Hyle document version.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HyleDirective {
    /// Version string for the document schema.
    pub version: String,
}

/// Declares world-wide simulation metadata.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct WorldConfig {
    /// Number of lattice dimensions used by the world.
    pub dimensions: u32,
}

/// Declares a named lattice and its cell shape.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LatticeConfig {
    /// Stable lattice name.
    pub name: String,
    /// Declared cell shape label.
    pub cell: String,
    /// Per-axis spacing values.
    pub spacing: SpacingConfig,
}

/// Declares lattice spacing along each axis.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SpacingConfig {
    /// X axis spacing.
    pub x: f64,
    /// Y axis spacing.
    pub y: f64,
    /// Z axis spacing.
    pub z: f64,
}

/// Declares a reusable neighborhood.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NeighborhoodConfig {
    /// Stable neighborhood name.
    pub name: String,
    /// Neighborhood radius.
    pub radius: u32,
    /// Whether the center cell is included.
    pub center: bool,
    /// Distance metric label.
    pub metric: String,
}

/// Declares a named model bound to a lattice.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ModelConfig {
    /// Stable model name.
    pub name: String,
    /// Referenced lattice name.
    pub lattice: String,
    /// Declared model fields.
    pub fields: Vec<FieldConfig>,
}

/// Declares a model field.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct FieldConfig {
    /// Stable field name.
    pub name: String,
    /// Scalar type label from the config.
    #[serde(rename = "type")]
    pub field_type: String,
    /// Default field value.
    pub default: f64,
    /// Minimum and maximum allowed values.
    pub bounds: BoundsConfig,
    /// Requested storage layout label.
    pub storage: String,
}

/// Declares lower and upper bounds for a field.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BoundsConfig {
    /// Lower bound.
    pub min: f64,
    /// Upper bound.
    pub max: f64,
}

/// Declares a simulation entry point.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SimulationConfig {
    /// Stable simulation name.
    pub name: String,
    /// Models enabled for this simulation.
    pub use_models: Vec<String>,
    /// External simulation inputs.
    pub inputs: Vec<InputConfig>,
    /// Ordered execution pipeline.
    pub pipeline: PipelineConfig,
}

/// Declares a simulation input.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InputConfig {
    /// Stable input name.
    pub name: String,
    /// Scalar type label from the config.
    #[serde(rename = "type")]
    pub input_type: String,
    /// Default input value.
    pub default: f64,
}

/// Declares a simulation pipeline.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PipelineConfig {
    /// Ordered pipeline stages.
    pub stages: Vec<StageConfig>,
}

/// Declares one pipeline stage.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StageConfig {
    /// Stable stage name.
    pub name: String,
    /// Runs executed in the stage.
    pub runs: Vec<RunConfig>,
}

/// Declares one pipeline run.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RunConfig {
    /// Stable run name.
    pub name: String,
    /// Referenced model name.
    pub model: String,
    /// Referenced neighborhood name.
    pub neighborhood: String,
}
