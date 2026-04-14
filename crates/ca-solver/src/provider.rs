//! Provider adapters for constructing runtimes from blueprints.

use hyle_ca_interface::{Blueprint, CaSolverProvider, Cell, CellModel, Instance};

use crate::{DescriptorTopology, Solver};

/// Default provider that builds runtimes using the CPU solver.
#[derive(Clone, Copy, Debug, Default)]
pub struct CpuSolverProvider;

impl CpuSolverProvider {
    /// Construct a new CPU solver provider.
    pub const fn new() -> Self {
        Self
    }
}

impl<C> CaSolverProvider<C> for CpuSolverProvider
where
    C: Cell + CellModel + Clone + Eq + Send + 'static,
{
    type Runtime = Solver<C, DescriptorTopology>;

    fn build(&self, instance: Instance, blueprint: &Blueprint<C>) -> Self::Runtime {
        Solver::from_spec_instance(instance, blueprint)
    }
}
