//! Common framework imports for typical blueprint and runtime setup.

pub use crate::contracts::blueprint::{neighbors, rng};
pub use crate::contracts::{
    AxisTopology, BlueprintSpec, BuildError, CellModel, CellSchema, NeighborhoodFalloff,
    NeighborhoodShape, NeighborhoodSpec, StateDef, TopologyDescriptor, Weight,
};
pub use crate::{CaRuntime, CaSolverProvider, Cell, Instance, Rng};
