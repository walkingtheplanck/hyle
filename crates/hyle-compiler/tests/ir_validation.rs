use hyle_compiler::ir::{
    validate_module, FieldIr, Identifier, ModelIr, ModuleIr, PipelineIr, RuleIr, RuleSourceIr,
    RuleStatementIr, StageIr, TypeIr,
};

fn identifier(value: &str) -> Identifier {
    Identifier::new(value).expect("identifier")
}

#[test]
fn validates_basic_module() {
    let module = ModuleIr {
        name: identifier("life"),
        models: vec![ModelIr {
            name: identifier("Cell"),
            resolution: 1,
            default_neighborhood: None,
            fields: vec![FieldIr {
                name: identifier("state"),
                ty: TypeIr::Bool,
                default: None,
                bounds: None,
            }],
        }],
        rules: vec![RuleIr {
            name: identifier("step"),
            sources: vec![RuleSourceIr {
                model: identifier("Cell"),
                sampling: None,
            }],
            output: identifier("Cell"),
            range: None,
            condition: None,
            statements: vec![RuleStatementIr::Next {
                model: identifier("Cell"),
                field: identifier("state"),
                expression: "true".to_owned(),
            }],
        }],
        pipeline: PipelineIr {
            stages: vec![StageIr {
                name: identifier("update"),
                rules: vec![identifier("step")],
            }],
        },
        ..ModuleIr::default()
    };

    assert!(validate_module(&module).is_ok());
}

#[test]
fn rejects_unknown_rule_reference() {
    let module = ModuleIr {
        name: identifier("life"),
        pipeline: PipelineIr {
            stages: vec![StageIr {
                name: identifier("update"),
                rules: vec![identifier("missing")],
            }],
        },
        ..ModuleIr::default()
    };

    assert!(validate_module(&module).is_err());
}
