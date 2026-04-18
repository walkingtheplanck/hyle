//! Umbrella runtime trait composed from behavior-focused runtime capabilities.

use super::{
    RuntimeAttributes, RuntimeCells, RuntimeGrid, RuntimeMetadata, RuntimeMetrics, RuntimeStepping,
};

/// A compact simulation runtime surface for consumers.
///
/// This umbrella trait keeps the common consumer-facing bound ergonomic while
/// the actual behavior is split into smaller capability traits:
///
/// - [`RuntimeMetadata`]: static schema descriptors and dimensions
/// - [`RuntimeCells`]: cell handles, coordinates, materials, and neighborhoods
/// - [`RuntimeAttributes`]: per-cell attribute reads and writes
/// - [`RuntimeGrid`]: bulk material-grid IO
/// - [`RuntimeStepping`]: step advancement and step counters
/// - [`RuntimeMetrics`]: latest-step and population metrics
pub trait CaRuntime:
    Send
    + RuntimeMetadata
    + RuntimeCells
    + RuntimeAttributes
    + RuntimeGrid
    + RuntimeStepping
    + RuntimeMetrics
{
}

impl<T> CaRuntime for T where
    T: Send
        + RuntimeMetadata
        + RuntimeCells
        + RuntimeAttributes
        + RuntimeGrid
        + RuntimeStepping
        + RuntimeMetrics
{
}
