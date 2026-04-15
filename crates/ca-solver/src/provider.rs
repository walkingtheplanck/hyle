//! Provider adapters for constructing runtimes from blueprints.

use hyle_ca_interface::{Blueprint, CaSolverProvider, Instance};

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

impl CaSolverProvider for CpuSolverProvider {
    type Runtime = Solver<DescriptorTopology>;

    fn build(&self, instance: Instance, blueprint: &Blueprint) -> Self::Runtime {
        Solver::from_spec_instance(instance, blueprint)
    }
}
