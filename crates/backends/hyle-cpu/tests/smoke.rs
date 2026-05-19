use hyle_cpu::CpuSolver;
use hyle_runtime::{LoadOptions, Solver};
use hyle_sole::{SoleModule, SoleWorld};

#[test]
fn cpu_solver_advances_instance() {
    let solver = CpuSolver;
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
    let sole = module.to_json_string().expect("json");
    let mut instance = solver
        .load(sole.as_bytes(), LoadOptions::default())
        .expect("load");

    instance.step().expect("step");
}
