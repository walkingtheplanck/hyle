//! Centralized solver-construction interface for apps and tools.

use crate::{Blueprint, CaRuntime, Instance};

/// A factory that builds a concrete runtime from a schema.
///
/// Consumers such as viewers can depend on this trait instead of naming a
/// concrete solver type directly, which keeps backend selection localized to
/// construction while preserving static dispatch.
pub trait CaSolverProvider: Send + Sync {
    /// Concrete runtime produced by this provider.
    type Runtime: CaRuntime;

    /// Build a new runtime for the given instance and schema.
    fn build(&self, instance: Instance, blueprint: &Blueprint) -> Self::Runtime;
}
