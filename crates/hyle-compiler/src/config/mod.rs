//! KDL config scaffold.

pub mod ast;
pub mod parse;

pub use ast::{
    BoundsConfig, ConfigAst, FieldConfig, HyleDirective, InputConfig, LatticeConfig, ModelConfig,
    NeighborhoodConfig, PipelineConfig, RunConfig, SimulationConfig, SpacingConfig, StageConfig,
    WorldConfig,
};
pub use parse::parse_config;
