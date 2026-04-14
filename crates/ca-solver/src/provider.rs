//! Provider adapters for constructing runtimes from blueprint specs.

use hyle_ca_interface::{BlueprintSpec, CaRuntime, CaSolverProvider, Cell, CellModel, Instance};

use crate::Solver;

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
    fn build(&self, instance: Instance, spec: &BlueprintSpec<C>) -> Box<dyn CaRuntime<C>> {
        Box::new(Solver::from_spec_instance(instance, spec))
    }
}
