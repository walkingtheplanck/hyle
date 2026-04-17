//! Common framework imports for typical schema and runtime setup.

pub use crate::schema::{
    attr, neighbors, rng, AttrAssign, AttributeDef, AttributeId, AttributeSet, AttributeType,
    AttributeValue, AxisTopology, Blueprint, BuildError, MatAttr, MaterialId, MaterialSet,
    NeighborhoodFalloff, NeighborhoodRadius, NeighborhoodSet, NeighborhoodShape, NeighborhoodSpec,
    RuleSpec, TopologyDescriptor, Weight,
};
pub use crate::{
    AttributeAccessError, CaRuntime, CaSolverProvider, CellId, CellQueryError, Instance, Rng,
};
