//! Enum-backed symbol traits used by schema authoring.

mod attribute;
mod error;
mod material;
mod neighborhood;

pub use attribute::AttributeSet;
pub use error::SetContractError;
pub use material::MaterialSet;
pub use neighborhood::NeighborhoodSet;
