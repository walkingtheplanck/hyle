#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

mod grid;
mod rule_set;
mod rules;
mod solver;

pub use rule_set::RuleSet;
pub use solver::Solver;
