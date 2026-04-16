//! Common framework imports for typical blueprint and runtime setup.

pub use crate::contracts::blueprint::{attr, neighbors, rng};
pub use crate::contracts::{
    AttrAssign, AttributeDef, AttributeId, AttributeSet, AttributeType, AttributeValue,
    AxisTopology, Blueprint, BuildError, MatAttr, MaterialId, MaterialSet, NeighborhoodFalloff,
    NeighborhoodRadius, NeighborhoodSet, NeighborhoodShape, NeighborhoodSpec, RuleSpec,
    TopologyDescriptor, Weight,
};
pub use crate::{
    AttributeAccessError, CaRuntime, CaSolverProvider, CellId, CellQueryError, Instance, Rng,
};
