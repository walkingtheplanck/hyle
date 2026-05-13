use hyle_ir::ModuleIr;
use hyle_runtime::{DispatchTarget, Instance, LoadedModule, RuntimeError, Solver};

/// Placeholder CPU solver used to validate runtime wiring.
#[derive(Default)]
pub struct CpuSolver;

impl Solver for CpuSolver {
    fn target(&self) -> DispatchTarget {
        DispatchTarget::Cpu
    }

    fn load_module(&mut self, module: ModuleIr) -> Result<LoadedModule, RuntimeError> {
        Ok(LoadedModule::new(module, self.target()))
    }

    fn create_instance(&mut self, module: &LoadedModule) -> Result<Instance, RuntimeError> {
        Ok(Instance::new(module.ir.name.as_str()))
    }

    fn step(&mut self, instance: &mut Instance) -> Result<(), RuntimeError> {
        instance.advance();
        Ok(())
    }
}
