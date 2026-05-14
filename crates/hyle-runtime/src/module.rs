use std::sync::Arc;

use hyle_compiler::SoleModule;

use crate::DispatchTarget;

/// A backend-loaded module handle.
#[derive(Clone, Debug)]
pub struct LoadedModule {
    /// Shared `.sole` module used to create runtime instances.
    pub module: Arc<SoleModule>,
    /// Backend selected for this loaded module.
    pub target: DispatchTarget,
}

impl LoadedModule {
    /// Creates a loaded module handle from an owned `.sole` module.
    pub fn new(module: SoleModule, target: DispatchTarget) -> Self {
        Self {
            module: Arc::new(module),
            target,
        }
    }
}
