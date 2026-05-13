use std::collections::HashSet;

use thiserror::Error;

use crate::ir::{ModuleIr, RuleStatementIr};

/// Light validation errors for the shared IR scaffold.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum HyleIrError {
    /// The module name was blank after validation-friendly trimming.
    #[error("module name must not be empty")]
    EmptyModuleName,
    /// Only two- and three-dimensional lattices are currently accepted.
    #[error("unsupported lattice dimension count: {0}")]
    UnsupportedDimensions(u8),
    /// Cell shape is not supported for the declared dimension count.
    #[error("cell shape `{cell}` is not supported for {dimensions} dimensions")]
    UnsupportedCellForDimensions { dimensions: u8, cell: String },
    /// Model names must be unique.
    #[error("duplicate model name: {0}")]
    DuplicateModel(String),
    /// Neighborhood names must be unique.
    #[error("duplicate neighborhood name: {0}")]
    DuplicateNeighborhood(String),
    /// Input names must be unique.
    #[error("duplicate input name: {0}")]
    DuplicateInput(String),
    /// Field names must be unique within a module model.
    #[error("duplicate field name `{field}` in model `{model}`")]
    DuplicateField { model: String, field: String },
    /// A model referenced an unknown default neighborhood.
    #[error("model `{model}` references unknown neighborhood `{neighborhood}`")]
    UnknownModelNeighborhood { model: String, neighborhood: String },
    /// Rule names must be unique.
    #[error("duplicate rule name: {0}")]
    DuplicateRule(String),
    /// Rule source model is not defined.
    #[error("rule `{rule}` references unknown model `{model}`")]
    UnknownRuleModel { rule: String, model: String },
    /// Rule range is not defined.
    #[error("rule `{rule}` references unknown neighborhood `{neighborhood}`")]
    UnknownRuleNeighborhood { rule: String, neighborhood: String },
    /// Rule writes a field that does not exist on the destination model.
    #[error("rule `{rule}` writes unknown field `{model}.{field}`")]
    UnknownRuleField {
        /// Rule name.
        rule: String,
        /// Model name.
        model: String,
        /// Field name.
        field: String,
    },
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

    if !(1..=4).contains(&module.lattice.dimensions) {
        return Err(HyleIrError::UnsupportedDimensions(
            module.lattice.dimensions,
        ));
    }

    if !cell_allowed(module.lattice.dimensions, &module.lattice.cell) {
        return Err(HyleIrError::UnsupportedCellForDimensions {
            dimensions: module.lattice.dimensions,
            cell: module.lattice.cell.clone(),
        });
    }

    let mut neighborhood_names = HashSet::new();
    for neighborhood in &module.neighborhoods {
        let inserted = neighborhood_names.insert(neighborhood.name.as_str());
        if !inserted {
            return Err(HyleIrError::DuplicateNeighborhood(
                neighborhood.name.as_str().to_owned(),
            ));
        }
    }

    let mut model_names = HashSet::new();
    for model in &module.models {
        let inserted = model_names.insert(model.name.as_str());
        if !inserted {
            return Err(HyleIrError::DuplicateModel(model.name.as_str().to_owned()));
        }

        if let Some(default_neighborhood) = &model.default_neighborhood {
            if !neighborhood_names.contains(default_neighborhood.as_str()) {
                return Err(HyleIrError::UnknownModelNeighborhood {
                    model: model.name.as_str().to_owned(),
                    neighborhood: default_neighborhood.as_str().to_owned(),
                });
            }
        }

        let mut field_names = HashSet::new();
        for field in &model.fields {
            let inserted = field_names.insert(field.name.as_str());
            if !inserted {
                return Err(HyleIrError::DuplicateField {
                    model: model.name.as_str().to_owned(),
                    field: field.name.as_str().to_owned(),
                });
            }
        }
    }

    let mut input_names = HashSet::new();
    for input in &module.inputs {
        let inserted = input_names.insert(input.name.as_str());
        if !inserted {
            return Err(HyleIrError::DuplicateInput(input.name.as_str().to_owned()));
        }
    }

    let mut rule_names = HashSet::new();
    for rule in &module.rules {
        let inserted = rule_names.insert(rule.name.as_str());
        if !inserted {
            return Err(HyleIrError::DuplicateRule(rule.name.as_str().to_owned()));
        }

        if !model_names.contains(rule.output.as_str()) {
            return Err(HyleIrError::UnknownRuleModel {
                rule: rule.name.as_str().to_owned(),
                model: rule.output.as_str().to_owned(),
            });
        }

        for source in &rule.sources {
            if !model_names.contains(source.model.as_str()) {
                return Err(HyleIrError::UnknownRuleModel {
                    rule: rule.name.as_str().to_owned(),
                    model: source.model.as_str().to_owned(),
                });
            }
        }

        if let Some(range) = &rule.range {
            if !neighborhood_names.contains(range.as_str()) {
                return Err(HyleIrError::UnknownRuleNeighborhood {
                    rule: rule.name.as_str().to_owned(),
                    neighborhood: range.as_str().to_owned(),
                });
            }
        }

        for statement in &rule.statements {
            if let RuleStatementIr::Next { model, field, .. } = statement {
                let Some(model_ir) = module
                    .models
                    .iter()
                    .find(|candidate| candidate.name.as_str() == model.as_str())
                else {
                    return Err(HyleIrError::UnknownRuleModel {
                        rule: rule.name.as_str().to_owned(),
                        model: model.as_str().to_owned(),
                    });
                };

                let has_field = model_ir
                    .fields
                    .iter()
                    .any(|candidate| candidate.name.as_str() == field.as_str());
                if !has_field {
                    return Err(HyleIrError::UnknownRuleField {
                        rule: rule.name.as_str().to_owned(),
                        model: model.as_str().to_owned(),
                        field: field.as_str().to_owned(),
                    });
                }
            }
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

fn cell_allowed(dimensions: u8, cell: &str) -> bool {
    matches!(
        (dimensions, cell),
        (1, "Line")
            | (2, "Triangle" | "Square" | "Hexagon")
            | (
                3,
                "Cube" | "Tetrahedron" | "TruncatedOctahedron" | "RhombicDodecahedron"
            )
            | (4, "Tesseract")
    )
}
