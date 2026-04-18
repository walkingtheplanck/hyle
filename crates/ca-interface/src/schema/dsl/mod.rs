//! DSL-shaped condition builders for portable rules.

mod attr_assign;
mod attribute;
mod condition;
mod neighbors;
mod random;
mod weight;

pub use attr_assign::AttrAssign;
pub use attribute::{attr, AttributeSelector};
pub use condition::{AttributeComparison, Condition, CountComparison, WeightComparison};
pub use neighbors::{neighbors, NeighborCount, NeighborSelector, NeighborWeightedSum};
pub use random::{rng, RandomSource};
pub use weight::Weight;
