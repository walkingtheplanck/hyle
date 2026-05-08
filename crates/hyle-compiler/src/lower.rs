use hyle_ir::{
    validate_module, HyleIrError, Identifier, ModuleIr, PipelineIr, RuleIr, SchemaVersion, StageIr,
};

use crate::resolve::ResolvedModule;

/// Lowers resolved compiler state into the shared Hyle IR.
pub fn lower_module(
    module: &ResolvedModule,
    schema_version: SchemaVersion,
) -> Result<ModuleIr, HyleIrError> {
    let rules = module
        .logic
        .iter()
        .enumerate()
        .map(|(index, source)| RuleIr {
            name: Identifier::new(format!("logic_{index}")).unwrap_or_default(),
            expression: format!("placeholder lowering for {}", source.source_path),
        })
        .collect::<Vec<_>>();

    let stage_rules = rules
        .iter()
        .map(|rule| rule.name.clone())
        .collect::<Vec<_>>();

    let lowered = ModuleIr {
        schema_version,
        name: Identifier::new(module.module_name.clone()).unwrap_or_default(),
        rules,
        pipeline: PipelineIr {
            stages: vec![StageIr {
                name: Identifier::new("main").unwrap_or_default(),
                rules: stage_rules,
            }],
        },
        ..ModuleIr::default()
    };

    validate_module(&lowered)?;

    Ok(lowered)
}
