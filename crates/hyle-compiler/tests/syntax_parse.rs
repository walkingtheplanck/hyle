use hyle_compiler::syntax::parse_script;
use hyle_compiler::SourceFile;

const GAME: &str = include_str!("../../../examples/game.hyle");

#[test]
fn parses_comments_before_hyle_directive() {
    let script = parse_script(&SourceFile::new("game.hyle", GAME)).expect("script should parse");

    assert_eq!(script.version, "0.1");
    assert_eq!(script.dimensions, 3);
    assert_eq!(script.cell, "Cube");
    assert_eq!(script.rules.len(), 4);
}

#[test]
fn rejects_missing_version_directive() {
    let result = parse_script(&SourceFile::new(
        "bad.hyle",
        "#dimensions 3\n#cell Cube\nmodel Fire { fields { intensity: Float } }",
    ));

    assert!(result.is_err());
}
