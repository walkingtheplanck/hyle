mod common;

use hyle_compiler::{compile, CompileInput, CompileOptions, SourceFile};
use hyle_sole::{SoleExpr, SoleLiteralValue};

const GAME: &str = include_str!("../../../examples/game.hyle");
const GAME_SOLE_JSON: &str = include_str!("../../../examples/game.sole.json");

#[test]
fn compiles_game_to_expected_sole_json_shape() {
    let output = compile(
        CompileInput {
            source: SourceFile::new("game.hyle", GAME),
            module_name: None,
        },
        CompileOptions::default(),
    )
    .expect("compile should succeed");

    let actual = serde_json::to_value(&output.module).expect("sole json");
    let expected = serde_json::from_str::<serde_json::Value>(GAME_SOLE_JSON).expect("fixture json");
    let json = output.module.to_json_string().expect("json string");

    common::dump_sections(
        "sole_ir::compiles_game_to_expected_sole_json_shape",
        &[
            ("input", GAME.to_owned()),
            (
                "actual output",
                serde_json::to_string_pretty(&actual).expect("actual json"),
            ),
            ("expected output", GAME_SOLE_JSON.to_owned()),
        ],
    );

    assert_eq!(actual, expected);
    assert_eq!(json, GAME_SOLE_JSON.trim_end());
    assert_eq!(output.module.to_string(), json);
}

#[test]
fn lowers_sole_ids_and_precision() {
    let output = compile(
        CompileInput {
            source: SourceFile::new("game.hyle", GAME),
            module_name: None,
        },
        CompileOptions::default(),
    )
    .expect("compile should succeed");

    common::dump_json(
        "sole_ir::lowers_sole_ids_and_precision",
        GAME,
        &output.module,
    );

    let grass = &output.module.models[1];
    let humidity = &grass.fields[0];
    let biomass = &grass.fields[1];
    let wind_speed = &output.module.inputs[0];

    assert_eq!(grass.name, "Grass");
    assert_eq!(humidity.epsilon, 0.001);
    assert_eq!(biomass.epsilon, 1e-7);
    assert_eq!(wind_speed.epsilon, 1e-7);
    assert_eq!(
        wind_speed.bounds.as_ref().expect("bounds").max,
        SoleLiteralValue::Float(250.0)
    );
}

#[test]
fn lowers_structured_rule_expressions() {
    let output = compile(
        CompileInput {
            source: SourceFile::new("game.hyle", GAME),
            module_name: None,
        },
        CompileOptions::default(),
    )
    .expect("compile should succeed");

    common::dump_json(
        "sole_ir::lowers_structured_rule_expressions",
        GAME,
        &output.module.rules,
    );

    let first_rule = &output.module.rules[0];
    assert_eq!(first_rule.name, "fire_update");
    assert_eq!(first_rule.anchor, 0);
    assert_eq!(first_rule.target, 0);
    assert_eq!(first_rule.range, 0);

    let SoleExpr::Reduce { reduce } = &first_rule.lets[0].value else {
        panic!("expected reduce expression");
    };
    assert_eq!(reduce.op, "Sum");
    assert_eq!(reduce.var, "n");

    let SoleExpr::Call { call } = &first_rule.writes[0].value else {
        panic!("expected clamp call");
    };
    assert_eq!(call.function, "clamp");
    assert_eq!(call.args.len(), 3);
}
