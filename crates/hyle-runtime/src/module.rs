use std::sync::Arc;

use hyle_ir::ModuleIr;

use crate::DispatchTarget;

/// A backend-loaded module handle.
#[derive(Clone, Debug)]
pub struct LoadedModule {
    /// Shared IR used to create runtime instances.
    pub ir: Arc<ModuleIr>,
    /// Backend selected for this loaded module.
    pub target: DispatchTarget,
}

impl LoadedModule {
    /// Creates a loaded module handle from owned IR.
    pub fn new(ir: ModuleIr, target: DispatchTarget) -> Self {
        Self {
            ir: Arc::new(ir),
            target,
        }
    }
}
