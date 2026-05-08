use hyle_gpu::GpuSolver;
use hyle_ir::{Identifier, ModuleIr};
use hyle_runtime::Solver;

#[test]
fn gpu_solver_advances_instance() {
    let mut solver = GpuSolver;
    let module = ModuleIr {
        name: Identifier::new("life").expect("identifier"),
        ..ModuleIr::default()
    };
    let loaded = solver.load_module(module).expect("load");
    let mut instance = solver.create_instance(&loaded).expect("instance");

    solver.step(&mut instance).expect("step");

    assert_eq!(instance.steps, 1);
}
