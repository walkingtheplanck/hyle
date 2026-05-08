use hyle_ir::ModuleIr;

/// Placeholder viewer-facing state holder.
#[derive(Default)]
pub struct ViewerScaffold {
    module: Option<ModuleIr>,
}

impl ViewerScaffold {
    /// Attaches a module for future visualization work.
    pub fn attach_module(&mut self, module: ModuleIr) {
        self.module = Some(module);
    }

    /// Returns true when a module has been attached.
    pub fn has_module(&self) -> bool {
        self.module.is_some()
    }
}
