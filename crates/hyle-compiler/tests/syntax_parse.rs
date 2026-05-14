use hyle_compiler::syntax::{
    lex, parse, parse_tokens, BinaryOpAst, ExprKindAst, LiteralAst, ReductionOpAst,
    RuleStatementAst, SamplingAst,
};

const GAME: &str = include_str!("../../../examples/game.hyle");

#[test]
fn parses_comments_before_hyle_directive() {
    let script = parse(GAME).expect("script should parse");

    assert_eq!(script.version, "0.1");
    assert_eq!(script.dimensions, 3);
    assert_eq!(script.cell, "Cube");
    assert_eq!(script.rules.len(), 4);
}

#[test]
fn parses_tokenized_source() {
    let tokens = lex(GAME).expect("source should lex");
    let script = parse_tokens(tokens).expect("tokens should parse");

    assert_eq!(script.version, "0.1");
    assert_eq!(script.rules.len(), 4);
}

#[test]
fn parses_rule_anchor_sampled_source_and_expressions() {
    let script = parse(GAME).expect("script should parse");

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

    assert_eq!(
        script.neighborhoods[0].radius,
        LiteralAst::Integer("1".to_owned())
    );
    let bounds = script.models[0].fields[0].bounds.as_ref().expect("bounds");
    assert_eq!(bounds.lower, LiteralAst::Float("0.0".to_owned()));
    assert_eq!(bounds.upper, LiteralAst::Float("1.0".to_owned()));
}

#[test]
fn rejects_missing_version_directive() {
    let result = parse("#dimensions 3\n#cell Cube\nmodel Fire { fields { intensity: Float } }");

    assert!(result.is_err());
}

#[test]
fn rejects_multiple_sampled_sources_for_now() {
    let result = parse(
        "#hyle 0.1
#dimensions 3
#cell Cube
model Fire { fields { intensity: Float } }
model Grass { fields { biomass: Float } }
model Ash { fields { amount: Float } }
Fire + (Average) Grass + (Average) Ash -> Fire {
    next Fire.intensity = Grass.biomass;
}",
    );

    assert!(result.is_err());
}

#[test]
fn rejects_non_numeric_bounds() {
    let result = parse(
        "#hyle 0.1
#dimensions 3
#cell Cube
model Fire {
    fields {
        active: Bool [false true]
    }
}",
    );

    assert!(result.is_err());
}

#[test]
fn rejects_malformed_numeric_literals() {
    let result = parse(
        "#hyle 0.1
#dimensions 3
#cell Cube
model Fire {
    fields {
        intensity: Float<1.2.3>
    }
}",
    );

    assert!(result.is_err());
}
