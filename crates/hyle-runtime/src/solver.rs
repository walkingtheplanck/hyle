use hyle_compiler::SoleModule;

use crate::{DispatchTarget, Instance, LoadedModule, RuntimeError};

/// Minimal solver contract shared by proof-of-concept backends.
pub trait Solver {
    /// Returns the backend category implemented by this solver.
    fn target(&self) -> DispatchTarget;

    /// Loads a compiled module into backend-specific state.
    fn load_module(&mut self, module: SoleModule) -> Result<LoadedModule, RuntimeError>;

    /// Creates an instance from a loaded module.
    fn create_instance(&mut self, module: &LoadedModule) -> Result<Instance, RuntimeError>;

    /// Advances an instance by one logical step.
    fn step(&mut self, instance: &mut Instance) -> Result<(), RuntimeError>;
}
