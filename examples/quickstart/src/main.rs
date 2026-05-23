use std::error::Error;

use hyle::prelude::*;
use hyle_cpu::CpuSolver;

fn main() -> Result<(), Box<dyn Error>> {
    let output = compile(
        CompileInput {
            source: SourceFile::new("examples/game.hyle", include_str!("../../game.hyle")),
            module_name: Some("game".to_owned()),
        },
        CompileOptions::default(),
    )?;

    let sole_json = output.module.to_json_string()?;
    let solver = solver(CpuSolver);
    let mut instance = solver.init(sole_json.as_bytes())?;

    let positions = vec![
        CellPosition {
            coordinates: vec![0, 0, 0],
        },
        CellPosition {
            coordinates: vec![1, 0, 0],
        },
    ];

    instance.add_cells(CellBatch {
        model: "Grass".to_owned(),
        positions: positions.clone(),
        fields: Vec::new(),
    })?;

    instance.step()?;
    instance.remove_cells("Grass", &positions)?;

    println!(
        "ran {} solver with {} cells",
        solver.name(),
        positions.len()
    );
    Ok(())
}
