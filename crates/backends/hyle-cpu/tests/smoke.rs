use hyle_compiler::ir::{Identifier, ModuleIr};
use hyle_cpu::CpuSolver;
use hyle_runtime::Solver;

#[test]
fn cpu_solver_advances_instance() {
    let mut solver = CpuSolver;
    let module = ModuleIr {
        name: Identifier::new("life").expect("identifier"),
        ..ModuleIr::default()
    };
    let loaded = solver.load_module(module).expect("load");
    let mut instance = solver.create_instance(&loaded).expect("instance");

    solver.step(&mut instance).expect("step");

    assert_eq!(instance.steps, 1);
}
