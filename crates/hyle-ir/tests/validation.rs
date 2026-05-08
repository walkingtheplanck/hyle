use hyle_ir::{
    validate_module, FieldIr, Identifier, ModelIr, ModuleIr, PipelineIr, RuleIr, StageIr, TypeIr,
};

fn identifier(value: &str) -> Identifier {
    Identifier::new(value).expect("identifier")
}

#[test]
fn validates_basic_module() {
    let module = ModuleIr {
        name: identifier("life"),
        model: ModelIr {
            fields: vec![FieldIr {
                name: identifier("state"),
                ty: TypeIr::Bool,
            }],
        },
        rules: vec![RuleIr {
            name: identifier("step"),
            expression: "placeholder".to_owned(),
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
