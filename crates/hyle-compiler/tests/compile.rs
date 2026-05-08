use hyle_compiler::{compile, CompileInput, CompileOptions, SourceFile};

#[test]
fn compiles_placeholder_sources() {
    let output = compile(
        CompileInput {
            config: SourceFile::new("hyle.kdl", "module \"life\""),
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
