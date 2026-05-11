use hyle_compiler::config::{
    parse_config, BoundsConfig, ConfigAst, FieldConfig, HyleDirective, InputConfig, LatticeConfig,
    ModelConfig, NeighborhoodConfig, PipelineConfig, RunConfig, SimulationConfig, SpacingConfig,
    StageConfig, WorldConfig,
};
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

    assert_eq!(config, expected_config());
}

#[test]
fn parsed_config_preserves_all_fields_through_serde() {
    let config = parse_config(&SourceFile::new("hyle.kdl", CONFIG)).expect("config should parse");
    let value = serde_json::to_value(&config).expect("config should serialize");
    let round_tripped: ConfigAst =
        serde_json::from_value(value).expect("config should deserialize");

    assert_eq!(round_tripped, expected_config());
}

fn expected_config() -> ConfigAst {
    ConfigAst {
        source_path: "hyle.kdl".to_owned(),
        hyle: HyleDirective {
            version: "0.1".to_owned(),
        },
        world: WorldConfig { dimensions: 3 },
        lattices: vec![LatticeConfig {
            name: "Grid".to_owned(),
            cell: "Cube".to_owned(),
            spacing: SpacingConfig {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
        }],
        neighborhoods: vec![NeighborhoodConfig {
            name: "von_neumann_1".to_owned(),
            radius: 1,
            center: false,
            metric: "Manhattan".to_owned(),
        }],
        models: vec![ModelConfig {
            name: "Fire".to_owned(),
            lattice: "Grid".to_owned(),
            fields: vec![FieldConfig {
                name: "intensity".to_owned(),
                field_type: "f32".to_owned(),
                default: 0.5,
                bounds: BoundsConfig { min: 0.0, max: 1.0 },
                storage: "Dense".to_owned(),
            }],
        }],
        simulations: vec![SimulationConfig {
            name: "WildfireMvp".to_owned(),
            use_models: vec!["Fire".to_owned()],
            inputs: vec![InputConfig {
                name: "wind_speed".to_owned(),
                input_type: "f32".to_owned(),
                default: 0.01,
            }],
            pipeline: PipelineConfig {
                stages: vec![StageConfig {
                    name: "local".to_owned(),
                    runs: vec![RunConfig {
                        name: "fire_local".to_owned(),
                        model: "Fire".to_owned(),
                        neighborhood: "von_neumann_1".to_owned(),
                    }],
                }],
            },
        }],
    }
}
