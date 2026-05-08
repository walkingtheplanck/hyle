use hyle_compiler::config::parse_config;
use hyle_compiler::SourceFile;

const CONFIG: &str = r#"
hyle version="0.1"

world {
    dimensions 3
}

lattice "Grid" cell="Cube" {
    spacing 1.0 1.0 1.0
}

neighborhood "von_neumann_1" {
    radius 1
    center #false
    metric "Manhattan"
}

model "Fire" on="Grid" {
    field "intensity" type="f32" {
        default 0.5
        bounds 0.0 1.0
        storage "Dense"
    }
}

simulation "WildfireMvp" {
    use-models "Fire"

    input "wind_speed" type="f32" {
        default 0.01
    }

    pipeline {
        stage "local" {
            run "fire_local" {
                model "Fire"
                neighborhood "von_neumann_1"
            }
        }
    }
}
"#;

#[test]
fn parses_requested_hyle_kdl_shape() {
    let config = parse_config(&SourceFile::new("hyle.kdl", CONFIG)).expect("config should parse");

    assert_eq!(config.source_path, "hyle.kdl");
    assert_eq!(config.hyle.version, "0.1");
    assert_eq!(config.world.dimensions, 3);

    let lattice = &config.lattices[0];
    assert_eq!(lattice.name, "Grid");
    assert_eq!(lattice.cell, "Cube");
    assert_eq!(lattice.spacing.x, 1.0);
    assert_eq!(lattice.spacing.y, 1.0);
    assert_eq!(lattice.spacing.z, 1.0);

    let neighborhood = &config.neighborhoods[0];
    assert_eq!(neighborhood.name, "von_neumann_1");
    assert_eq!(neighborhood.radius, 1);
    assert!(!neighborhood.center);
    assert_eq!(neighborhood.metric, "Manhattan");

    let model = &config.models[0];
    assert_eq!(model.name, "Fire");
    assert_eq!(model.lattice, "Grid");
    assert_eq!(model.fields.len(), 1);

    let field = &model.fields[0];
    assert_eq!(field.name, "intensity");
    assert_eq!(field.field_type, "f32");
    assert_eq!(field.default, 0.5);
    assert_eq!(field.bounds.min, 0.0);
    assert_eq!(field.bounds.max, 1.0);
    assert_eq!(field.storage, "Dense");

    let simulation = &config.simulations[0];
    assert_eq!(simulation.name, "WildfireMvp");
    assert_eq!(simulation.use_models, vec!["Fire"]);
    assert_eq!(simulation.inputs.len(), 1);
    assert_eq!(simulation.inputs[0].name, "wind_speed");
    assert_eq!(simulation.inputs[0].input_type, "f32");
    assert_eq!(simulation.inputs[0].default, 0.01);

    let stage = &simulation.pipeline.stages[0];
    assert_eq!(stage.name, "local");
    assert_eq!(stage.runs.len(), 1);
    assert_eq!(stage.runs[0].name, "fire_local");
    assert_eq!(stage.runs[0].model, "Fire");
    assert_eq!(stage.runs[0].neighborhood, "von_neumann_1");
}
