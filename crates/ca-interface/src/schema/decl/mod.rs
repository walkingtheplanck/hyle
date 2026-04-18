//! Declarative schema records stored inside blueprints.

mod attribute;
mod material;
mod neighborhood;
mod topology;

pub use attribute::{AttributeDef, MaterialAttributeBinding};
pub use material::MaterialDef;
pub use neighborhood::{
    NeighborhoodFalloff, NeighborhoodShape, NeighborhoodSpec,
};
pub use topology::{AxisTopology, TopologyDescriptor};
