//! Type-erased symbol references used by schema authoring.

mod attribute;
mod material;
mod neighborhood;

pub use attribute::AttributeRef;
pub use material::MaterialRef;
pub use neighborhood::NeighborhoodRef;
