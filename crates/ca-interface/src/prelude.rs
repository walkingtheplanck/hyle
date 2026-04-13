//! Convenient imports for the declarative automaton API.

pub use crate::contracts::automaton::{neighbors, rng};
pub use crate::contracts::{
    AutomatonSpec, AxisTopology, BlueprintSpec, BuildError, CellState, Condition, CountComparison,
    Hyle, NamedNeighborhood, NeighborhoodFalloff, NeighborhoodShape, NeighborhoodSpec, Rule,
    RuleEffect, Semantics, TopologyDescriptor,
};
pub use crate::{Cell, Rng};
