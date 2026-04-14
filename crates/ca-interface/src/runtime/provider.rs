//! Centralized solver-construction interface for apps and tools.

use crate::{BlueprintSpec, Cell, CellModel, Instance};

use super::ca_runtime::CaRuntime;

/// A factory that builds a runtime from a blueprint specification.
///
/// Consumers such as viewers should depend on this trait instead of concrete
/// solver types so backend selection stays localized to construction.
pub trait CaSolverProvider<C: Cell + CellModel>: Send + Sync {
    /// Concrete runtime produced by this provider.
    type Runtime: CaRuntime<C>;

    /// Build a new runtime for the given instance and blueprint spec.
    fn build(&self, instance: Instance, spec: &BlueprintSpec<C>) -> Self::Runtime;
}
