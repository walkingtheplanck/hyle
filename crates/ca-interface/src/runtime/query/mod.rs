//! Cell-oriented runtime query types and errors.

mod attribute_access;
mod cell;

pub use attribute_access::AttributeAccessError;
pub use cell::{CellAttributeValue, CellQueryError};
