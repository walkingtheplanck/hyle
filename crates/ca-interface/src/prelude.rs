//! Common framework imports for typical blueprint and runtime setup.

pub use crate::contracts::blueprint::{attr, neighbors, rng};
pub use crate::contracts::{
    AttributeDef, AttributeType, AttributeValue, AxisTopology, Blueprint, BuildError, CellModel,
    CellSchema, NeighborhoodFalloff, NeighborhoodShape, NeighborhoodSpec, StateDef,
    TopologyDescriptor, Weight,
};
pub use crate::{CaRuntime, CaSolverProvider, Cell, Instance, Rng};
