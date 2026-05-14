mod common;

use hyle_compiler::syntax::{
    lex, parse, parse_tokens, BinaryOpAst, ExprKindAst, LiteralAst, ReductionOpAst,
    RuleStatementAst, SamplingAst,
};

const GAME: &str = include_str!("../../../examples/game.hyle");

#[test]
fn parses_comments_before_hyle_directive() {
    let script = parse(GAME).expect("script should parse");

    common::dump_debug(
        "syntax_parse::parses_comments_before_hyle_directive",
        GAME,
        &script,
    );

    assert_eq!(script.version, "0.1");
    assert_eq!(script.dimensions, 3);
    assert_eq!(script.cell, "Cube");
    assert_eq!(script.rules.len(), 4);
}

#[test]
fn parses_tokenized_source() {
    let tokens = lex(GAME).expect("source should lex");
    let script = parse_tokens(tokens).expect("tokens should parse");

    common::dump_debug("syntax_parse::parses_tokenized_source", GAME, &script);

    assert_eq!(script.version, "0.1");
    assert_eq!(script.rules.len(), 4);
}

#[test]
fn parses_rule_anchor_sampled_source_and_expressions() {
    let script = parse(GAME).expect("script should parse");

    common::dump_debug(
        "syntax_parse::parses_rule_anchor_sampled_source_and_expressions",
        GAME,
        &script.rules,
    );

    let sampled_rule = &script.rules[1];
    assert_eq!(sampled_rule.anchor, "Fire");
    let sampled = sampled_rule.sampled.as_ref().expect("sampled source");
    assert_eq!(sampled.model, "Grass");
    assert_eq!(sampled.sampling, Some(SamplingAst::Average));

    let RuleStatementAst::Let { expression, .. } = &script.rules[0].statements[0] else {
        panic!("expected let statement");
    };
    let ExprKindAst::Reduction { op, binding, .. } = &expression.kind else {
        panic!("expected reduction expression");
    };
    assert_eq!(*op, ReductionOpAst::Sum);
    assert_eq!(binding, "n");

    let condition = script.rules[3].condition.as_ref().expect("condition");
    let ExprKindAst::Binary { op, .. } = &condition.kind else {
        panic!("expected conjunction condition");
    };
    assert_eq!(*op, BinaryOpAst::And);
}

#[test]
fn parses_numeric_radius_and_bounds_as_literals() {
    let script = parse(GAME).expect("script should parse");

    common::dump_debug(
        "syntax_parse::parses_numeric_radius_and_bounds_as_literals",
        GAME,
        &script.neighborhoods,
    );

    assert_eq!(
        script.neighborhoods[0].radius,
        LiteralAst::Integer("1".to_owned())
    );
    let bounds = script.models[0].fields[0].bounds.as_ref().expect("bounds");
    assert_eq!(bounds.lower, LiteralAst::Float("0.0".to_owned()));
    assert_eq!(bounds.upper, LiteralAst::Float("1.0".to_owned()));
}

#[test]
fn parses_float_literal_forms() {
    let source = "#hyle 0.1
#dimensions 3
#cell Cube
model Numbers {
    fields {
        a: Float<123.45>
        b: Float<123.>
        c: Float<.45>
        d: Float<1e10>
        e: Float<1.0e-10>
        f: Float<9.21E-43>
    }
}";
    let script = parse(source).expect("script should parse");

    let defaults = script.models[0]
        .fields
        .iter()
        .map(|field| field.default.as_ref().expect("default"))
        .collect::<Vec<_>>();

    common::dump_debug(
        "syntax_parse::parses_float_literal_forms",
        source,
        &defaults,
    );

    assert_eq!(defaults[0], &LiteralAst::Float("123.45".to_owned()));
    assert_eq!(defaults[1], &LiteralAst::Float("123.".to_owned()));
    assert_eq!(defaults[2], &LiteralAst::Float(".45".to_owned()));
    assert_eq!(defaults[3], &LiteralAst::Float("1e10".to_owned()));
    assert_eq!(defaults[4], &LiteralAst::Float("1.0e-10".to_owned()));
    assert_eq!(defaults[5], &LiteralAst::Float("9.21E-43".to_owned()));
}

#[test]
fn parses_optional_field_precision() {
    let script = parse(GAME).expect("script should parse");
    let grass = script
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

    common::dump_debug(
        "syntax_parse::parses_optional_field_precision",
        GAME,
        &grass.fields,
    );

    assert_eq!(
        humidity.precision,
        Some(LiteralAst::Float("1e-3".to_owned()))
    );
    assert_eq!(biomass.precision, None);
}

#[test]
fn parses_optional_input_bounds_and_precision() {
    let source = "#hyle 0.1
#dimensions 3
#cell Cube
in humidity: Float [0.0 1.0) ~1e-3;";
    let script = parse(source).expect("script should parse");

    let input = &script.inputs[0];
    let bounds = input.bounds.as_ref().expect("bounds");

    common::dump_debug(
        "syntax_parse::parses_optional_input_bounds_and_precision",
        source,
        input,
    );

    assert_eq!(bounds.lower, LiteralAst::Float("0.0".to_owned()));
    assert_eq!(bounds.upper, LiteralAst::Float("1.0".to_owned()));
    assert!(bounds.lower_inclusive);
    assert!(!bounds.upper_inclusive);
    assert_eq!(input.precision, Some(LiteralAst::Float("1e-3".to_owned())));
}

#[test]
fn rejects_missing_version_directive() {
    let source = "#dimensions 3\n#cell Cube\nmodel Fire { fields { intensity: Float } }";
    let result = parse(source);

    common::dump_debug(
        "syntax_parse::rejects_missing_version_directive",
        source,
        &result,
    );

    assert!(result.is_err());
}

#[test]
fn rejects_multiple_sampled_sources_for_now() {
    let source = "#hyle 0.1
#dimensions 3
#cell Cube
model Fire { fields { intensity: Float } }
model Grass { fields { biomass: Float } }
model Ash { fields { amount: Float } }
Fire + (Average) Grass + (Average) Ash -> Fire {
    next Fire.intensity = Grass.biomass;
}";
    let result = parse(source);

    common::dump_debug(
        "syntax_parse::rejects_multiple_sampled_sources_for_now",
        source,
        &result,
    );

    assert!(result.is_err());
}

#[test]
fn rejects_non_numeric_bounds() {
    let source = "#hyle 0.1
#dimensions 3
#cell Cube
model Fire {
    fields {
        active: Bool [false true]
    }
}";
    let result = parse(source);

    common::dump_debug("syntax_parse::rejects_non_numeric_bounds", source, &result);

    assert!(result.is_err());
}

#[test]
fn rejects_malformed_numeric_literals() {
    let source = "#hyle 0.1
#dimensions 3
#cell Cube
model Fire {
    fields {
        intensity: Float<1.2.3>
    }
}";
    let result = parse(source);

    common::dump_debug(
        "syntax_parse::rejects_malformed_numeric_literals",
        source,
        &result,
    );

    assert!(result.is_err());
}
