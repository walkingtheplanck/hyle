use hyle_compiler::{SoleModule, SoleWorld};
use hyle_runtime::{DispatchTarget, Instance, LoadedModule};

#[test]
fn instance_advances() {
    let mut instance = Instance::new("life");
    instance.advance();

    assert_eq!(instance.steps, 1);
}

#[test]
fn loaded_module_wraps_ir() {
    let module = SoleModule {
        version: "0.1".to_owned(),
        world: SoleWorld {
            dimensions: 2,
            cell: "Square".to_owned(),
        },
        ranges: Vec::new(),
        models: Vec::new(),
        inputs: Vec::new(),
        rules: Vec::new(),
    };
    let loaded = LoadedModule::new(module, DispatchTarget::Cpu);

    assert_eq!(loaded.module.version, "0.1");
}
