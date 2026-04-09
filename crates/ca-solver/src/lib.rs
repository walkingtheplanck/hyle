#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

mod grid;
pub mod neighborhood;
mod rule_set;
mod rules;
mod solver;
mod world;

pub use neighborhood::{
    inverse_square, moore, spherical, unweighted, von_neumann, Entry, Neighborhood, Offset,
    ShapeFn, WeightFn,
};
pub use rule_set::RuleSet;
pub use solver::Solver;
pub use world::{GridReader, GridWriter, Rule, WorldPass};
