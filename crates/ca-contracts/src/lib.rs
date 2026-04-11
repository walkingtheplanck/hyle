#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

pub mod action;
pub mod automaton;
pub mod backend;
pub mod cell;
/// Declarative, backend-neutral data structures shared across solvers.
pub mod descriptors;
/// Common imports for the declarative automaton API.
pub mod prelude;
pub mod topology;
pub mod validated;

pub use action::Action;
pub use automaton::{
    neighbors, AutomatonBuilder, AutomatonSpec, BuildError, Condition, CountComparison, Hyle,
    HyleBuilder, NamedNeighborhood, NeighborCount, NeighborSelector, Rule, RuleBuilder, RuleEffect,
    RulesBuilder, Semantics,
};
pub use backend::CaSolver;
pub use cell::Cell;
pub use descriptors::{
    AxisTopology, GridDims, GridRegion, GridSnapshot, NeighborhoodShape, NeighborhoodSpec,
    NeighborhoodWeight, TopologyDescriptor,
};
pub use topology::Topology;
pub use validated::ValidatedSolver;
