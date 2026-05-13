use hyle_compiler::{compile, CompileInput, CompileOptions, SourceFile};

const GAME: &str = include_str!("../../../examples/game.hyle");

#[test]
fn compiles_single_hyle_script() {
    let output = compile(
        CompileInput {
            source: SourceFile::new("game.hyle", GAME),
            module_name: Some("wildfire".to_owned()),
        },
        CompileOptions::default(),
    )
    .expect("compile should succeed");

    assert_eq!(output.module.name.as_str(), "wildfire");
    assert_eq!(output.module.lattice.dimensions, 3);
    assert_eq!(output.module.lattice.cell, "Cube");
    assert_eq!(output.module.neighborhoods.len(), 1);
    assert_eq!(output.module.models.len(), 3);
    assert_eq!(output.module.inputs.len(), 1);
    assert_eq!(output.module.rules.len(), 4);
    assert_eq!(output.module.pipeline.stages.len(), 1);
}

#[test]
fn rejects_empty_source() {
    let result = compile(
        CompileInput {
            source: SourceFile::new("empty.hyle", "   "),
            module_name: None,
        },
        CompileOptions::default(),
    );

    assert!(result.is_err());
}
