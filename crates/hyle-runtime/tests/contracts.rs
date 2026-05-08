use hyle_ir::{Identifier, ModuleIr};
use hyle_runtime::{DispatchTarget, Instance, LoadedModule};

#[test]
fn instance_advances() {
    let mut instance = Instance::new("life");
    instance.advance();

    assert_eq!(instance.steps, 1);
}

#[test]
fn loaded_module_wraps_ir() {
    let module = ModuleIr {
        name: Identifier::new("life").expect("identifier"),
        ..ModuleIr::default()
    };
    let loaded = LoadedModule::new(module, DispatchTarget::Cpu);

    assert_eq!(loaded.ir.name.as_str(), "life");
}
