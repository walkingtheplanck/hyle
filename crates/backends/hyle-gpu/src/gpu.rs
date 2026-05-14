use hyle_compiler::SoleModule;
use hyle_runtime::{DispatchTarget, Instance, LoadedModule, RuntimeError, Solver};

/// Placeholder GPU solver used to validate runtime wiring.
#[derive(Default)]
pub struct GpuSolver;

impl Solver for GpuSolver {
    fn target(&self) -> DispatchTarget {
        DispatchTarget::Gpu
    }

    fn load_module(&mut self, module: SoleModule) -> Result<LoadedModule, RuntimeError> {
        Ok(LoadedModule::new(module, self.target()))
    }

    fn create_instance(&mut self, module: &LoadedModule) -> Result<Instance, RuntimeError> {
        Ok(Instance::new(&module.module.version))
    }

    fn step(&mut self, instance: &mut Instance) -> Result<(), RuntimeError> {
        instance.advance();
        Ok(())
    }
}
