mod common;

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

    common::dump_json("compile::compiles_single_hyle_script", GAME, &output.module);

    assert_eq!(output.module.version, "0.1");
    assert_eq!(output.module.world.dimensions, 3);
    assert_eq!(output.module.world.cell, "Cube");
    assert_eq!(output.module.ranges.len(), 1);
    assert_eq!(output.module.models.len(), 3);
    assert_eq!(output.module.inputs.len(), 1);
    assert_eq!(output.module.rules.len(), 4);

    let grass = output
        .module
        .models
        .iter()
        .find(|model| model.name == "Grass")
        .expect("grass model");
    let humidity = grass
        .fields
        .iter()
        .find(|field| field.name == "humidity")
        .expect("humidity field");
    let biomass = grass
        .fields
        .iter()
        .find(|field| field.name == "biomass")
        .expect("biomass field");
    assert_eq!(humidity.epsilon, 0.001);
    assert_eq!(biomass.epsilon, 1e-7);

    let wind_speed = &output.module.inputs[0];
    let bounds = wind_speed.bounds.as_ref().expect("wind speed bounds");
    assert_eq!(wind_speed.epsilon, 1e-7);
    assert_eq!(format!("{:?}", bounds.min), "Float(0.0)");
    assert_eq!(format!("{:?}", bounds.max), "Float(250.0)");
}

#[test]
fn rejects_empty_source() {
    let source = "   ";
    let result = compile(
        CompileInput {
            source: SourceFile::new("empty.hyle", source),
            module_name: None,
        },
        CompileOptions::default(),
    );

    common::dump_debug("compile::rejects_empty_source", source, &result);

    assert!(result.is_err());
}
