//! Runtime neighborhood buffers for the CPU solver.
//!
//! Shape and weight are configured via function pointers at construction.
//! Built-in shape functions: [`moore`], [`von_neumann`], [`spherical`].
//! Built-in weight functions: [`unweighted`], [`inverse_square`].

mod buffer;
mod shapes;
mod types;
mod weights;

pub use buffer::Neighborhood;
pub use shapes::{moore, spherical, von_neumann};
pub use types::{Entry, Offset, ShapeFn, WeightFn};
pub use weights::{inverse_square, unweighted};
