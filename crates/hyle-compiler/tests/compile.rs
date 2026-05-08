use hyle_compiler::{compile, CompileInput, CompileOptions, SourceFile};

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
fn compiles_placeholder_sources() {
    let output = compile(
        CompileInput {
            config: SourceFile::new("hyle.kdl", CONFIG),
            logic: vec![SourceFile::new("logic.hyle", "rule placeholder")],
            module_name: Some("life".to_owned()),
        },
        CompileOptions::default(),
    )
    .expect("compile should succeed");

    assert_eq!(output.module.name.as_str(), "life");
    assert_eq!(output.module.pipeline.stages.len(), 1);
}

#[test]
fn rejects_empty_config_source() {
    let result = compile(
        CompileInput {
            config: SourceFile::new("hyle.kdl", "   "),
            logic: vec![SourceFile::new("logic.hyle", "rule placeholder")],
            module_name: None,
        },
        CompileOptions::default(),
    );

    assert!(result.is_err());
}
