//! Enum-backed symbol traits used by schema authoring.

mod attribute;
mod material;
mod neighborhood;

pub use attribute::{AttributeRef, AttributeSet};
pub use material::{MaterialRef, MaterialSet};
pub use neighborhood::{NeighborhoodRef, NeighborhoodSet};
