//! Convenient imports for the declarative blueprint API.

pub use crate::contracts::blueprint::{neighbors, rng};
pub use crate::contracts::{
    AutomatonSpec, AxisTopology, BlueprintSpec, BuildError, CellState, Condition, CountComparison,
    Hyle, NamedNeighborhood, NeighborhoodFalloff, NeighborhoodShape, NeighborhoodSpec, Rule,
    RuleEffect, Semantics, TopologyDescriptor,
};
pub use crate::{Cell, Rng};
