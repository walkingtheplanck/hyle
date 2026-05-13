use std::collections::{HashMap, HashSet};

use crate::diagnostics::{Diagnostic, DiagnosticReport};
use crate::ir::{
    validate_module, BoundsIr, FieldIr, Identifier, InputIr, LatticeIr, LiteralIr, ModelIr,
    ModuleIr, NeighborhoodIr, PipelineIr, RuleIr, RuleSourceIr, RuleStatementIr, SamplingIr,
    SchemaVersion, StageIr, TypeIr,
};
use crate::syntax::{
    BoundsAst, FieldAst, InputAst, LiteralAst, ModelAst, RuleAst, RuleSourceAst, RuleStatementAst,
    SamplingAst, ScriptAst, TypeAst,
};

/// Lowers a parsed script into compiler IR.
///
/// # Errors
///
/// Returns semantic diagnostics when declarations are inconsistent.
pub fn lower_script(
    script: &ScriptAst,
    module_name: Option<&str>,
    schema_version: SchemaVersion,
) -> Result<ModuleIr, DiagnosticReport> {
    let mut checker = SemanticChecker::new(script);
    checker.check();
    if checker.report.has_errors() {
        return Err(checker.report);
    }

    let rules = script
        .rules
        .iter()
        .enumerate()
        .map(|(index, rule)| lower_rule(index, rule))
        .collect::<Vec<_>>();
    let stage_rules = rules
        .iter()
        .map(|rule| rule.name.clone())
        .collect::<Vec<_>>();

    let module = ModuleIr {
        schema_version,
        name: Identifier::new(
            module_name
                .map(str::trim)
                .filter(|name| !name.is_empty())
                .unwrap_or("main"),
        )
        .unwrap_or_default(),
        lattice: LatticeIr {
            dimensions: script.dimensions,
            cell: script.cell.clone(),
        },
        neighborhoods: script
            .neighborhoods
            .iter()
            .map(|neighborhood| NeighborhoodIr {
                name: ident(&neighborhood.name),
                radius: neighborhood.radius.clone(),
                center: neighborhood.center,
                metric: neighborhood.metric.clone(),
            })
            .collect(),
        models: script.models.iter().map(lower_model).collect(),
        inputs: script.inputs.iter().map(lower_input).collect(),
        rules,
        pipeline: PipelineIr {
            stages: vec![StageIr {
                name: ident("main"),
                rules: stage_rules,
            }],
        },
    };

    validate_module(&module).map_err(|error| {
        let mut report = DiagnosticReport::new();
        report.push(Diagnostic::error(
            Some(script.source_path.clone()),
            error.to_string(),
        ));
        report
    })?;

    Ok(module)
}

struct SemanticChecker<'a> {
    script: &'a ScriptAst,
    report: DiagnosticReport,
    neighborhoods: HashSet<&'a str>,
    models: HashMap<&'a str, &'a ModelAst>,
}

impl<'a> SemanticChecker<'a> {
    fn new(script: &'a ScriptAst) -> Self {
        Self {
            script,
            report: DiagnosticReport::new(),
            neighborhoods: HashSet::new(),
            models: HashMap::new(),
        }
    }

    fn check(&mut self) {
        self.check_header();
        self.collect_neighborhoods();
        self.collect_models();
        self.check_inputs();
        self.check_rules();
    }

    fn check_header(&mut self) {
        if self.script.version != "0.1" {
            self.error(format!(
                "unsupported Hyle version `{}`; expected `0.1`",
                self.script.version
            ));
        }

        if !(1..=4).contains(&self.script.dimensions) {
            self.error(format!(
                "unsupported dimension count `{}`; expected 1 through 4",
                self.script.dimensions
            ));
        }

        if !cell_allowed(self.script.dimensions, &self.script.cell) {
            self.error(format!(
                "cell shape `{}` is not valid for {} dimensions",
                self.script.cell, self.script.dimensions
            ));
        }
    }

    fn collect_neighborhoods(&mut self) {
        for neighborhood in &self.script.neighborhoods {
            if !self.neighborhoods.insert(&neighborhood.name) {
                self.error(format!("duplicate neighborhood `{}`", neighborhood.name));
            }
        }
    }

    fn collect_models(&mut self) {
        for model in &self.script.models {
            if self.models.insert(&model.name, model).is_some() {
                self.error(format!("duplicate model `{}`", model.name));
            }

            if let Some(range) = &model.range {
                if !self.neighborhoods.contains(range.as_str()) {
                    self.error(format!(
                        "model `{}` references unknown neighborhood `{range}`",
                        model.name
                    ));
                }
            }

            let mut fields = HashSet::new();
            for field in &model.fields {
                if !fields.insert(field.name.as_str()) {
                    self.error(format!(
                        "duplicate field `{}` in model `{}`",
                        field.name, model.name
                    ));
                }
                self.check_default_matches_type(&model.name, field);
            }
        }
    }

    fn check_default_matches_type(&mut self, model_name: &str, field: &FieldAst) {
        if let (TypeAst::Bool, Some(LiteralAst::Number(_))) = (&field.ty, &field.default) {
            self.error(format!(
                "field `{}.{}` has numeric default for Bool field",
                model_name, field.name
            ));
        }
    }

    fn check_inputs(&mut self) {
        let mut inputs = HashSet::new();
        for input in &self.script.inputs {
            if !inputs.insert(input.name.as_str()) {
                self.error(format!("duplicate input `{}`", input.name));
            }
            if let (TypeAst::Bool, Some(LiteralAst::Number(_))) = (&input.ty, &input.default) {
                self.error(format!(
                    "input `{}` has numeric default for Bool input",
                    input.name
                ));
            }
        }
    }

    fn check_rules(&mut self) {
        for (index, rule) in self.script.rules.iter().enumerate() {
            let rule_name = rule_name(index, rule);
            if !self.models.contains_key(rule.output.as_str()) {
                self.error(format!(
                    "rule `{rule_name}` writes unknown output model `{}`",
                    rule.output
                ));
            }

            for source in &rule.sources {
                if !self.models.contains_key(source.model.as_str()) {
                    self.error(format!(
                        "rule `{rule_name}` references unknown source model `{}`",
                        source.model
                    ));
                }
            }

            if let Some(range) = &rule.range {
                if !self.neighborhoods.contains(range.as_str()) {
                    self.error(format!(
                        "rule `{rule_name}` references unknown neighborhood `{range}`"
                    ));
                }
            }

            for statement in &rule.statements {
                if let RuleStatementAst::Next { model, field, .. } = statement {
                    self.check_next_statement(&rule_name, model, field);
                }
            }
        }
    }

    fn check_next_statement(&mut self, rule_name: &str, model: &str, field: &str) {
        let Some(model_ast) = self.models.get(model) else {
            self.error(format!("rule `{rule_name}` writes unknown model `{model}`"));
            return;
        };

        let has_field = model_ast
            .fields
            .iter()
            .any(|candidate| candidate.name == field);
        if !has_field {
            self.error(format!(
                "rule `{rule_name}` writes unknown field `{model}.{field}`"
            ));
        }
    }

    fn error(&mut self, message: impl Into<String>) {
        self.report.push(Diagnostic::error(
            Some(self.script.source_path.clone()),
            message.into(),
        ));
    }
}

fn lower_model(model: &ModelAst) -> ModelIr {
    ModelIr {
        name: ident(&model.name),
        resolution: model.resolution.unwrap_or(1),
        default_neighborhood: model.range.as_deref().map(ident),
        fields: model.fields.iter().map(lower_field).collect(),
    }
}

fn lower_field(field: &FieldAst) -> FieldIr {
    FieldIr {
        name: ident(&field.name),
        ty: lower_type(&field.ty),
        default: field.default.as_ref().map(lower_literal),
        bounds: field.bounds.as_ref().map(lower_bounds),
    }
}

fn lower_input(input: &InputAst) -> InputIr {
    InputIr {
        name: ident(&input.name),
        ty: lower_type(&input.ty),
        default: input.default.as_ref().map(lower_literal),
    }
}

fn lower_rule(index: usize, rule: &RuleAst) -> RuleIr {
    RuleIr {
        name: ident(rule_name(index, rule)),
        sources: rule.sources.iter().map(lower_rule_source).collect(),
        output: ident(&rule.output),
        range: rule.range.as_deref().map(ident),
        condition: rule
            .condition
            .as_ref()
            .map(|condition| condition.text.clone()),
        statements: rule.statements.iter().map(lower_statement).collect(),
    }
}

fn lower_rule_source(source: &RuleSourceAst) -> RuleSourceIr {
    RuleSourceIr {
        model: ident(&source.model),
        sampling: source.sampling.as_ref().map(lower_sampling),
    }
}

fn lower_statement(statement: &RuleStatementAst) -> RuleStatementIr {
    match statement {
        RuleStatementAst::Let { name, expression } => RuleStatementIr::Let {
            name: ident(name),
            expression: expression.text.clone(),
        },
        RuleStatementAst::Next {
            model,
            field,
            expression,
        } => RuleStatementIr::Next {
            model: ident(model),
            field: ident(field),
            expression: expression.text.clone(),
        },
    }
}

fn lower_type(ty: &TypeAst) -> TypeIr {
    match ty {
        TypeAst::Int => TypeIr::Int,
        TypeAst::Float => TypeIr::Float,
        TypeAst::Bool => TypeIr::Bool,
        TypeAst::Custom(name) => TypeIr::Custom(ident(name)),
    }
}

fn lower_literal(literal: &LiteralAst) -> LiteralIr {
    match literal {
        LiteralAst::Number(value) => LiteralIr::Number(value.clone()),
        LiteralAst::Bool(value) => LiteralIr::Bool(*value),
    }
}

fn lower_bounds(bounds: &BoundsAst) -> BoundsIr {
    BoundsIr {
        lower: bounds.lower.clone(),
        lower_inclusive: bounds.lower_inclusive,
        upper: bounds.upper.clone(),
        upper_inclusive: bounds.upper_inclusive,
    }
}

fn lower_sampling(sampling: &SamplingAst) -> SamplingIr {
    match sampling {
        SamplingAst::Average => SamplingIr::Average,
        SamplingAst::Nearest => SamplingIr::Nearest,
        SamplingAst::Sum => SamplingIr::Sum,
        SamplingAst::All => SamplingIr::All,
        SamplingAst::Custom(name) => SamplingIr::Custom(ident(name)),
    }
}

fn ident(value: impl AsRef<str>) -> Identifier {
    Identifier::new(value.as_ref()).unwrap_or_default()
}

fn rule_name(index: usize, rule: &RuleAst) -> String {
    let sources = rule
        .sources
        .iter()
        .map(|source| source.model.as_str())
        .collect::<Vec<_>>()
        .join("_");
    format!("rule_{index}_{sources}_to_{}", rule.output)
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
