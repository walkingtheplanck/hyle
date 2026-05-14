use hyle_compiler::{SoleModule, SoleWorld};
use hyle_gpu::GpuSolver;
use hyle_runtime::Solver;

#[test]
fn gpu_solver_advances_instance() {
    let mut solver = GpuSolver;
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
    let loaded = solver.load_module(module).expect("load");
    let mut instance = solver.create_instance(&loaded).expect("instance");

    solver.step(&mut instance).expect("step");

    assert_eq!(instance.steps, 1);
}
