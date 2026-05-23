use hyle_gpu::GpuSolver;
use hyle_runtime::{CellBatch, CellPosition, SolverBackend};
use hyle_sole::{SoleModule, SoleWorld};

#[test]
fn gpu_solver_advances_instance() {
    let solver = GpuSolver;
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
    let mut instance = solver.init(sole.as_bytes()).expect("init");
    let positions = vec![
        CellPosition {
            coordinates: vec![0, 0],
        },
        CellPosition {
            coordinates: vec![1, 0],
        },
    ];
    instance
        .add_cells(CellBatch {
            model: "Grass".to_owned(),
            positions: positions.clone(),
            fields: Vec::new(),
        })
        .expect("add cells");

    instance.step().expect("step");
    instance
        .remove_cells("Grass", &positions)
        .expect("remove cells");
}
