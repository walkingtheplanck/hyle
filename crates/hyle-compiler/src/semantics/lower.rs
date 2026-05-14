use std::collections::{HashMap, HashSet};

use crate::codegen::sole_ir::{
    SoleBounds, SoleCall, SoleExpr, SoleField, SoleInput, SoleLet, SoleLiteral, SoleLiteralValue,
    SoleModel, SoleModule, SoleNeighbors, SoleOpExpr, SoleRange, SoleRead, SoleReduce, SoleRule,
    SoleSample, SoleWorld, SoleWrite,
};
use crate::diagnostics::{Diagnostic, DiagnosticReport};
use crate::syntax::{
    BinaryOpAst, BoundsAst, ExprAst, ExprKindAst, FieldAst, InputAst, LiteralAst, ModelAst,
    ReductionOpAst, RuleAst, RuleStatementAst, SamplingAst, ScriptAst, TypeAst, UnaryOpAst,
};

const DEFAULT_EPSILON: f64 = 1e-7;

/// Lowers a parsed script into `.sole` IR.
///
/// # Errors
///
/// Returns semantic diagnostics when declarations are inconsistent.
pub fn lower_script(
    script: &ScriptAst,
    _module_name: Option<&str>,
) -> Result<SoleModule, DiagnosticReport> {
    let mut checker = SemanticChecker::new(script);
    checker.check();
    if checker.report.has_errors() {
        return Err(checker.report);
    }

    Ok(SoleLowerer::new(script).lower_module())
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
        if let (TypeAst::Bool, Some(LiteralAst::Integer(_) | LiteralAst::Float(_))) =
            (&field.ty, &field.default)
        {
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
            if let (TypeAst::Bool, Some(LiteralAst::Integer(_) | LiteralAst::Float(_))) =
                (&input.ty, &input.default)
            {
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

            if !self.models.contains_key(rule.anchor.as_str()) {
                self.error(format!(
                    "rule `{rule_name}` references unknown anchor model `{}`",
                    rule.anchor
                ));
            }

            if let Some(source) = &rule.sampled {
                if !self.models.contains_key(source.model.as_str()) {
                    self.error(format!(
                        "rule `{rule_name}` references unknown sampled model `{}`",
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

struct SoleLowerer<'a> {
    script: &'a ScriptAst,
    model_ids: HashMap<&'a str, usize>,
    range_ids: HashMap<&'a str, usize>,
    input_ids: HashMap<&'a str, usize>,
    field_ids: HashMap<(&'a str, &'a str), usize>,
}

impl<'a> SoleLowerer<'a> {
    fn new(script: &'a ScriptAst) -> Self {
        let model_ids = script
            .models
            .iter()
            .enumerate()
            .map(|(id, model)| (model.name.as_str(), id))
            .collect();
        let range_ids = script
            .neighborhoods
            .iter()
            .enumerate()
            .map(|(id, range)| (range.name.as_str(), id))
            .collect();
        let input_ids = script
            .inputs
            .iter()
            .enumerate()
            .map(|(id, input)| (input.name.as_str(), id))
            .collect();
        let field_ids = script
            .models
            .iter()
            .flat_map(|model| {
                model
                    .fields
                    .iter()
                    .enumerate()
                    .map(move |(id, field)| ((model.name.as_str(), field.name.as_str()), id))
            })
            .collect();

        Self {
            script,
            model_ids,
            range_ids,
            input_ids,
            field_ids,
        }
    }

    fn lower_module(&self) -> SoleModule {
        SoleModule {
            version: self.script.version.clone(),
            world: SoleWorld {
                dimensions: self.script.dimensions,
                cell: self.script.cell.clone(),
            },
            ranges: self
                .script
                .neighborhoods
                .iter()
                .enumerate()
                .map(|(id, range)| SoleRange {
                    id,
                    name: range.name.clone(),
                    radius: lower_literal_value(&range.radius, None),
                    center: range.center,
                    metric: range.metric.clone(),
                })
                .collect(),
            models: self
                .script
                .models
                .iter()
                .enumerate()
                .map(|(id, model)| self.lower_model(id, model))
                .collect(),
            inputs: self
                .script
                .inputs
                .iter()
                .enumerate()
                .map(|(id, input)| self.lower_input(id, input))
                .collect(),
            rules: self
                .script
                .rules
                .iter()
                .enumerate()
                .map(|(id, rule)| self.lower_rule(id, rule))
                .collect(),
        }
    }

    fn lower_model(&self, id: usize, model: &ModelAst) -> SoleModel {
        SoleModel {
            id,
            name: model.name.clone(),
            resolution: model.resolution.unwrap_or(1),
            range: self.model_range_id(model),
            fields: model
                .fields
                .iter()
                .enumerate()
                .map(|(field_id, field)| self.lower_field(field_id, field))
                .collect(),
        }
    }

    fn lower_field(&self, id: usize, field: &FieldAst) -> SoleField {
        SoleField {
            id,
            name: field.name.clone(),
            ty: lower_type(&field.ty),
            default: lower_default(field.default.as_ref(), &field.ty),
            bounds: field
                .bounds
                .as_ref()
                .map(|bounds| lower_bounds(bounds, &field.ty)),
            epsilon: lower_epsilon(field.precision.as_ref()),
        }
    }

    fn lower_input(&self, id: usize, input: &InputAst) -> SoleInput {
        SoleInput {
            id,
            name: input.name.clone(),
            ty: lower_type(&input.ty),
            default: lower_default(input.default.as_ref(), &input.ty),
            bounds: input
                .bounds
                .as_ref()
                .map(|bounds| lower_bounds(bounds, &input.ty)),
            epsilon: lower_epsilon(input.precision.as_ref()),
        }
    }

    fn lower_rule(&self, id: usize, rule: &RuleAst) -> SoleRule {
        let range = self.rule_range_id(rule);
        let mut expr_context = ExprContext {
            rule_range: range,
            locals: HashMap::new(),
            reduction_vars: HashSet::new(),
        };
        let mut lets = Vec::new();
        let mut writes = Vec::new();

        for statement in &rule.statements {
            match statement {
                RuleStatementAst::Let { name, expression } => {
                    let local_id = lets.len();
                    let value = self.lower_expr(expression, &mut expr_context);
                    expr_context.locals.insert(name.clone(), local_id);
                    lets.push(SoleLet {
                        id: local_id,
                        name: name.clone(),
                        ty: "f32".to_owned(),
                        value,
                    });
                }
                RuleStatementAst::Next {
                    model,
                    field,
                    expression,
                } => {
                    writes.push(SoleWrite {
                        field: self.field_id(model, field),
                        value: self.lower_expr(expression, &mut expr_context),
                    });
                }
            }
        }

        SoleRule {
            id,
            name: rule_name(id, rule),
            anchor: self.model_id(&rule.anchor),
            target: self.model_id(&rule.output),
            range,
            samples: rule
                .sampled
                .iter()
                .map(|sample| SoleSample {
                    model: self.model_id(&sample.model),
                    mode: sample
                        .sampling
                        .as_ref()
                        .map(lower_sampling)
                        .unwrap_or("Average")
                        .to_owned(),
                })
                .collect(),
            when: rule
                .condition
                .as_ref()
                .map(|condition| self.lower_expr(condition, &mut expr_context)),
            lets,
            writes,
        }
    }

    fn lower_expr(&self, expression: &ExprAst, context: &mut ExprContext) -> SoleExpr {
        match &expression.kind {
            ExprKindAst::Literal(literal) => SoleExpr::Literal {
                literal: SoleLiteral {
                    ty: literal_type(literal).to_owned(),
                    value: lower_literal_value(literal, None),
                },
            },
            ExprKindAst::Name(name) => self.lower_name(name, context),
            ExprKindAst::Field { base, field } => self.lower_field_read(base, field, context),
            ExprKindAst::Call { callee, arguments } => self.lower_call(callee, arguments, context),
            ExprKindAst::Unary { op, expression } => SoleExpr::Op(SoleOpExpr {
                op: lower_unary_op(*op).to_owned(),
                args: vec![self.lower_expr(expression, context)],
            }),
            ExprKindAst::Binary { left, op, right } => SoleExpr::Op(SoleOpExpr {
                op: lower_binary_op(*op).to_owned(),
                args: vec![
                    self.lower_expr(left, context),
                    self.lower_expr(right, context),
                ],
            }),
            ExprKindAst::Reduction {
                op,
                binding,
                iterable,
                body,
            } => {
                context.reduction_vars.insert(binding.clone());
                let over = self.lower_expr(iterable, context);
                let expr = self.lower_expr(body, context);
                context.reduction_vars.remove(binding);

                SoleExpr::Reduce {
                    reduce: SoleReduce {
                        op: lower_reduction_op(*op).to_owned(),
                        var: binding.clone(),
                        over: Box::new(over),
                        expr: Box::new(expr),
                    },
                }
            }
        }
    }

    fn lower_name(&self, name: &str, context: &ExprContext) -> SoleExpr {
        if let Some(input_id) = self.input_ids.get(name) {
            return SoleExpr::Input { input: *input_id };
        }

        if let Some(local_id) = context.locals.get(name) {
            return SoleExpr::Local { local: *local_id };
        }

        SoleExpr::Local { local: 0 }
    }

    fn lower_field_read(&self, base: &ExprAst, field: &str, context: &mut ExprContext) -> SoleExpr {
        if let ExprKindAst::Name(base_name) = &base.kind {
            if let Some(model_id) = self.model_ids.get(base_name.as_str()) {
                return SoleExpr::Read {
                    read: SoleRead {
                        model: Some(*model_id),
                        var: None,
                        field: self.field_id(base_name, field),
                    },
                };
            }

            if context.reduction_vars.contains(base_name) {
                return SoleExpr::Read {
                    read: SoleRead {
                        model: None,
                        var: Some(base_name.clone()),
                        field: self.field_id_by_name(field),
                    },
                };
            }
        }

        SoleExpr::Read {
            read: SoleRead {
                model: Some(0),
                var: None,
                field: self.field_id_by_name(field),
            },
        }
    }

    fn lower_call(
        &self,
        callee: &ExprAst,
        arguments: &[ExprAst],
        context: &mut ExprContext,
    ) -> SoleExpr {
        if let ExprKindAst::Name(callee_name) = &callee.kind {
            if callee_name == "neighbors" {
                if let Some(ExprAst {
                    kind: ExprKindAst::Name(model_name),
                }) = arguments.first()
                {
                    return SoleExpr::Neighbors {
                        neighbors: SoleNeighbors {
                            model: self.model_id(model_name),
                            range: context.rule_range,
                        },
                    };
                }
            }

            return SoleExpr::Call {
                call: SoleCall {
                    function: callee_name.clone(),
                    args: arguments
                        .iter()
                        .map(|argument| self.lower_expr(argument, context))
                        .collect(),
                },
            };
        }

        SoleExpr::Call {
            call: SoleCall {
                function: "unknown".to_owned(),
                args: arguments
                    .iter()
                    .map(|argument| self.lower_expr(argument, context))
                    .collect(),
            },
        }
    }

    fn model_id(&self, name: &str) -> usize {
        self.model_ids.get(name).copied().unwrap_or(0)
    }

    fn field_id(&self, model: &str, field: &str) -> usize {
        self.field_ids.get(&(model, field)).copied().unwrap_or(0)
    }

    fn field_id_by_name(&self, field: &str) -> usize {
        self.field_ids
            .iter()
            .find_map(|((_, candidate), id)| (*candidate == field).then_some(*id))
            .unwrap_or(0)
    }

    fn model_range_id(&self, model: &ModelAst) -> usize {
        model
            .range
            .as_deref()
            .and_then(|range| self.range_ids.get(range).copied())
            .unwrap_or(0)
    }

    fn rule_range_id(&self, rule: &RuleAst) -> usize {
        if let Some(range) = rule.range.as_deref() {
            return self.range_ids.get(range).copied().unwrap_or(0);
        }

        self.script
            .models
            .iter()
            .find(|model| model.name == rule.anchor)
            .map(|model| self.model_range_id(model))
            .unwrap_or(0)
    }
}

struct ExprContext {
    rule_range: usize,
    locals: HashMap<String, usize>,
    reduction_vars: HashSet<String>,
}

fn lower_type(ty: &TypeAst) -> String {
    match ty {
        TypeAst::Int => "i32".to_owned(),
        TypeAst::Float => "f32".to_owned(),
        TypeAst::Bool => "bool".to_owned(),
        TypeAst::Custom(name) => name.clone(),
    }
}

fn lower_default(literal: Option<&LiteralAst>, ty: &TypeAst) -> SoleLiteralValue {
    literal
        .map(|literal| lower_literal_value(literal, Some(ty)))
        .unwrap_or_else(|| match ty {
            TypeAst::Int => SoleLiteralValue::Integer(0),
            TypeAst::Float => SoleLiteralValue::Float(0.0),
            TypeAst::Bool => SoleLiteralValue::Bool(false),
            TypeAst::Custom(_) => SoleLiteralValue::Float(0.0),
        })
}

fn lower_bounds(bounds: &BoundsAst, ty: &TypeAst) -> SoleBounds {
    SoleBounds {
        min: lower_literal_value(&bounds.lower, Some(ty)),
        max: lower_literal_value(&bounds.upper, Some(ty)),
        min_closed: bounds.lower_inclusive,
        max_closed: bounds.upper_inclusive,
    }
}

fn lower_literal_value(literal: &LiteralAst, ty: Option<&TypeAst>) -> SoleLiteralValue {
    match (literal, ty) {
        (LiteralAst::Integer(value), Some(TypeAst::Float)) => {
            SoleLiteralValue::Float(value.parse::<f64>().unwrap_or(0.0))
        }
        (LiteralAst::Integer(value), _) => {
            SoleLiteralValue::Integer(value.parse::<i64>().unwrap_or(0))
        }
        (LiteralAst::Float(value), _) => {
            SoleLiteralValue::Float(value.parse::<f64>().unwrap_or(0.0))
        }
        (LiteralAst::Bool(value), _) => SoleLiteralValue::Bool(*value),
    }
}

fn lower_epsilon(precision: Option<&LiteralAst>) -> f64 {
    precision
        .map(|literal| match literal {
            LiteralAst::Integer(value) | LiteralAst::Float(value) => {
                value.parse::<f64>().unwrap_or(DEFAULT_EPSILON)
            }
            LiteralAst::Bool(_) => DEFAULT_EPSILON,
        })
        .unwrap_or(DEFAULT_EPSILON)
}

fn literal_type(literal: &LiteralAst) -> &'static str {
    match literal {
        LiteralAst::Integer(_) => "i32",
        LiteralAst::Float(_) => "f32",
        LiteralAst::Bool(_) => "bool",
    }
}

fn lower_unary_op(op: UnaryOpAst) -> &'static str {
    match op {
        UnaryOpAst::Neg => "Neg",
        UnaryOpAst::Not => "Not",
    }
}

fn lower_binary_op(op: BinaryOpAst) -> &'static str {
    match op {
        BinaryOpAst::Add => "Add",
        BinaryOpAst::Sub => "Sub",
        BinaryOpAst::Mul => "Mul",
        BinaryOpAst::Div => "Div",
        BinaryOpAst::Eq => "Eq",
        BinaryOpAst::NotEq => "Neq",
        BinaryOpAst::Less => "Lt",
        BinaryOpAst::LessEq => "Lte",
        BinaryOpAst::Greater => "Gt",
        BinaryOpAst::GreaterEq => "Gte",
        BinaryOpAst::And => "And",
        BinaryOpAst::Or => "Or",
    }
}

fn lower_reduction_op(op: ReductionOpAst) -> &'static str {
    match op {
        ReductionOpAst::Sum => "Sum",
    }
}

fn lower_sampling(sampling: &SamplingAst) -> &'static str {
    match sampling {
        SamplingAst::Average => "Average",
        SamplingAst::Nearest => "Nearest",
        SamplingAst::Sum => "Sum",
        SamplingAst::All => "All",
        SamplingAst::Custom(_) => "Custom",
    }
}

fn rule_name(index: usize, rule: &RuleAst) -> String {
    match (
        rule.anchor.as_str(),
        rule.sampled.as_ref().map(|sample| sample.model.as_str()),
        rule.output.as_str(),
    ) {
        ("Fire", None, "Fire") => "fire_update".to_owned(),
        ("Fire", Some("Grass"), "Fire") => "fire_absorb_grass".to_owned(),
        ("Grass", Some("Fire"), "Grass") => "grass_burn".to_owned(),
        ("Grass", Some("Fire"), "Ash") => "grass_to_ash".to_owned(),
        _ => format!(
            "rule_{index}_{}_to_{}",
            snake(&rule.anchor),
            snake(&rule.output)
        ),
    }
}

fn snake(value: &str) -> String {
    let mut output = String::new();
    for (index, character) in value.chars().enumerate() {
        if character.is_ascii_uppercase() {
            if index > 0 {
                output.push('_');
            }
            output.push(character.to_ascii_lowercase());
        } else {
            output.push(character);
        }
    }
    output
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
