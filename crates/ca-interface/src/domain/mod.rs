//! Shared CA domain value types used by both schema authoring and runtime APIs.
//!
//! These are neutral concepts such as grid geometry and scalar attribute values.
//! They do not belong to the declarative schema layer or to the live runtime
//! layer specifically.

mod attribute;
mod grid;
mod neighborhood;

pub use attribute::{AttributeType, AttributeValue};
pub use grid::{GridDataError, GridDims, GridRegion, GridShapeError, GridSnapshot};
pub use neighborhood::{NeighborhoodRadius, WEIGHT_SCALE};
