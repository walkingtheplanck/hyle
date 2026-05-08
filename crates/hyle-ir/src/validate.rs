use std::collections::HashSet;

use thiserror::Error;

use crate::ModuleIr;

/// Light validation errors for the shared IR scaffold.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum HyleIrError {
    /// The module name was blank after validation-friendly trimming.
    #[error("module name must not be empty")]
    EmptyModuleName,
    /// Only two- and three-dimensional lattices are currently accepted.
    #[error("unsupported lattice dimension count: {0}")]
    UnsupportedDimensions(u8),
    /// Field names must be unique within a module model.
    #[error("duplicate field name: {0}")]
    DuplicateField(String),
    /// Stage names must be unique within a module pipeline.
    #[error("duplicate stage name: {0}")]
    DuplicateStage(String),
    /// A pipeline stage referenced a rule name that is not defined.
    #[error("stage `{stage}` references unknown rule `{rule}`")]
    UnknownRuleReference { stage: String, rule: String },
}

/// Validates a module using intentionally light scaffold rules.
///
/// This keeps the shared IR honest without pulling compiler or backend-specific
/// semantics into the crate.
pub fn validate_module(module: &ModuleIr) -> Result<(), HyleIrError> {
    if module.name.as_str().trim().is_empty() {
        return Err(HyleIrError::EmptyModuleName);
    }

    if !matches!(module.lattice.dimensions, 2 | 3) {
        return Err(HyleIrError::UnsupportedDimensions(
            module.lattice.dimensions,
        ));
    }

    let mut field_names = HashSet::new();
    for field in &module.model.fields {
        let inserted = field_names.insert(field.name.as_str());
        if !inserted {
            return Err(HyleIrError::DuplicateField(field.name.as_str().to_owned()));
        }
    }

    let known_rules = module
        .rules
        .iter()
        .map(|rule| rule.name.as_str())
        .collect::<HashSet<_>>();

    let mut stage_names = HashSet::new();
    for stage in &module.pipeline.stages {
        let inserted = stage_names.insert(stage.name.as_str());
        if !inserted {
            return Err(HyleIrError::DuplicateStage(stage.name.as_str().to_owned()));
        }

        for rule_name in &stage.rules {
            if !known_rules.contains(rule_name.as_str()) {
                return Err(HyleIrError::UnknownRuleReference {
                    stage: stage.name.as_str().to_owned(),
                    rule: rule_name.as_str().to_owned(),
                });
            }
        }
    }

    Ok(())
}
