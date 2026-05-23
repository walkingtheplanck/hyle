use std::collections::HashMap;

use hyle_runtime::{
    Cell, CellBatch, CellFieldColumn, CellFieldValue, CellPosition, CellRead, CellWrite,
    EncodedCellIo, FieldColumnValues, HyleValue, InputAccess, RuntimeError,
};
use hyle_sole::{
    SoleExpr, SoleField, SoleInput, SoleLiteralValue, SoleModule, SoleRange, SoleRule,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct CellKey {
    model: usize,
    position: CellPosition,
}

#[derive(Clone, Debug)]
struct CellState {
    fields: Vec<HyleValue>,
}

#[derive(Clone, Debug)]
struct EvalCell {
    model: usize,
    position: CellPosition,
}

#[derive(Clone, Debug)]
enum EvalValue {
    Scalar(HyleValue),
    Cells(Vec<EvalCell>),
}

pub struct CpuAccess {
    module: SoleModule,
    model_by_name: HashMap<String, usize>,
    field_by_name: Vec<HashMap<String, usize>>,
    input_by_name: HashMap<String, usize>,
    cells: HashMap<CellKey, CellState>,
    inputs: Vec<HyleValue>,
}

impl CpuAccess {
    pub fn new(module: SoleModule) -> Result<Self, RuntimeError> {
        validate_module(&module)?;

        let model_by_name = module
            .models
            .iter()
            .map(|model| (model.name.clone(), model.id))
            .collect();
        let field_by_name = module
            .models
            .iter()
            .map(|model| {
                model
                    .fields
                    .iter()
                    .map(|field| (field.name.clone(), field.id))
                    .collect()
            })
            .collect();
        let input_by_name = module
            .inputs
            .iter()
            .map(|input| (input.name.clone(), input.id))
            .collect();
        let inputs = module
            .inputs
            .iter()
            .map(|input| literal_to_value(&input.ty, &input.default))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            module,
            model_by_name,
            field_by_name,
            input_by_name,
            cells: HashMap::new(),
            inputs,
        })
    }

    pub fn step(&mut self) -> Result<(), RuntimeError> {
        let snapshot = self.cells.clone();
        let mut next = snapshot.clone();

        for rule in &self.module.rules {
            let mut anchors = snapshot
                .keys()
                .filter(|key| key.model == rule.anchor)
                .map(|key| key.position.clone())
                .collect::<Vec<_>>();
            anchors.sort_by(|left, right| left.coordinates.cmp(&right.coordinates));

            for anchor in anchors {
                let mut locals = HashMap::new();
                for binding in &rule.lets {
                    let value = self.eval_scalar(
                        rule,
                        &snapshot,
                        &locals,
                        &HashMap::new(),
                        &anchor,
                        &binding.value,
                    )?;
                    locals.insert(binding.id, value);
                }

                if let Some(when) = &rule.when {
                    if !as_bool(&self.eval_scalar(
                        rule,
                        &snapshot,
                        &locals,
                        &HashMap::new(),
                        &anchor,
                        when,
                    )?)? {
                        continue;
                    }
                }

                let target_key = CellKey {
                    model: rule.target,
                    position: anchor.clone(),
                };
                if !next.contains_key(&target_key) {
                    next.insert(target_key.clone(), self.default_cell(rule.target)?);
                }

                for write in &rule.writes {
                    let target_model = self.model(rule.target)?;
                    let field = target_model.fields.get(write.field).ok_or_else(|| {
                        RuntimeError::Step(format!(
                            "rule `{}` writes unknown field {} on model `{}`",
                            rule.name, write.field, target_model.name
                        ))
                    })?;
                    let raw = self.eval_scalar(
                        rule,
                        &snapshot,
                        &locals,
                        &HashMap::new(),
                        &anchor,
                        &write.value,
                    )?;
                    let value = coerce_value(&field.ty, raw)?;
                    check_field_bounds(field, &value)?;
                    next.get_mut(&target_key)
                        .expect("target cell was inserted")
                        .fields[write.field] = value;
                }
            }
        }

        self.cells = next;
        Ok(())
    }

    fn eval_scalar(
        &self,
        rule: &SoleRule,
        cells: &HashMap<CellKey, CellState>,
        locals: &HashMap<usize, HyleValue>,
        vars: &HashMap<String, EvalCell>,
        anchor: &CellPosition,
        expr: &SoleExpr,
    ) -> Result<HyleValue, RuntimeError> {
        match self.eval(rule, cells, locals, vars, anchor, expr)? {
            EvalValue::Scalar(value) => Ok(value),
            EvalValue::Cells(_) => Err(RuntimeError::Step(
                "expected scalar expression, found cell collection".to_owned(),
            )),
        }
    }

    fn eval(
        &self,
        rule: &SoleRule,
        cells: &HashMap<CellKey, CellState>,
        locals: &HashMap<usize, HyleValue>,
        vars: &HashMap<String, EvalCell>,
        anchor: &CellPosition,
        expr: &SoleExpr,
    ) -> Result<EvalValue, RuntimeError> {
        match expr {
            SoleExpr::Literal { literal } => Ok(EvalValue::Scalar(literal_to_value(
                &literal.ty,
                &literal.value,
            )?)),
            SoleExpr::Input { input } => self
                .inputs
                .get(*input)
                .cloned()
                .map(EvalValue::Scalar)
                .ok_or_else(|| RuntimeError::InputAccess(format!("unknown input id {input}"))),
            SoleExpr::Local { local } => locals
                .get(local)
                .cloned()
                .map(EvalValue::Scalar)
                .ok_or_else(|| RuntimeError::Step(format!("unknown local id {local}"))),
            SoleExpr::Read { read } => {
                let value = if let Some(var) = &read.var {
                    let cell = vars.get(var).ok_or_else(|| {
                        RuntimeError::Step(format!("unknown reduction var `{var}`"))
                    })?;
                    self.read_field_by_id(cells, cell.model, read.field, &cell.position)?
                        .unwrap_or_else(|| {
                            zero_for_field(
                                self.field(cell.model, read.field).expect("validated field"),
                            )
                        })
                } else {
                    let model = read.model.ok_or_else(|| {
                        RuntimeError::Step("field read has neither model nor var".to_owned())
                    })?;
                    if model == rule.anchor
                        || !rule.samples.iter().any(|sample| sample.model == model)
                    {
                        let model_name = self.model(model)?.name.clone();
                        self.read_field_by_id(cells, model, read.field, anchor)?
                            .ok_or_else(|| missing_cell_error(&model_name, anchor))?
                    } else {
                        self.sample_field(rule, cells, model, read.field, anchor)?
                    }
                };
                Ok(EvalValue::Scalar(value))
            }
            SoleExpr::Call { call } => {
                let args = call
                    .args
                    .iter()
                    .map(|arg| self.eval_scalar(rule, cells, locals, vars, anchor, arg))
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(EvalValue::Scalar(eval_call(&call.function, &args)?))
            }
            SoleExpr::Op(op) => {
                let args = op
                    .args
                    .iter()
                    .map(|arg| self.eval_scalar(rule, cells, locals, vars, anchor, arg))
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(EvalValue::Scalar(eval_op(&op.op, &args)?))
            }
            SoleExpr::Reduce { reduce } => {
                let cells_value =
                    match self.eval(rule, cells, locals, vars, anchor, &reduce.over)? {
                        EvalValue::Cells(cells) => cells,
                        EvalValue::Scalar(_) => {
                            return Err(RuntimeError::Step(
                                "reduction source did not evaluate to cells".to_owned(),
                            ))
                        }
                    };

                match reduce.op.as_str() {
                    "Sum" => {
                        let mut sum = 0.0;
                        for cell in cells_value {
                            let mut nested_vars = vars.clone();
                            nested_vars.insert(reduce.var.clone(), cell);
                            sum += as_f64(&self.eval_scalar(
                                rule,
                                cells,
                                locals,
                                &nested_vars,
                                anchor,
                                &reduce.expr,
                            )?)?;
                        }
                        Ok(EvalValue::Scalar(HyleValue::F64(sum)))
                    }
                    other => Err(RuntimeError::Step(format!(
                        "unsupported reduction `{other}`"
                    ))),
                }
            }
            SoleExpr::Neighbors { neighbors } => {
                let range = self.range(neighbors.range)?;
                let cells = self
                    .neighbor_positions(anchor, range)?
                    .into_iter()
                    .filter(|position| {
                        self.read_cell_by_id(cells, neighbors.model, position)
                            .is_some()
                    })
                    .map(|position| EvalCell {
                        model: neighbors.model,
                        position,
                    })
                    .collect();
                Ok(EvalValue::Cells(cells))
            }
        }
    }

    fn sample_field(
        &self,
        rule: &SoleRule,
        cells: &HashMap<CellKey, CellState>,
        model: usize,
        field: usize,
        anchor: &CellPosition,
    ) -> Result<HyleValue, RuntimeError> {
        let sample = rule
            .samples
            .iter()
            .find(|sample| sample.model == model)
            .ok_or_else(|| RuntimeError::Step(format!("model {model} is not sampled")))?;
        let field_meta = self.field(model, field)?;
        let positions = self.neighbor_positions(anchor, self.range(rule.range)?)?;
        let values = positions
            .iter()
            .filter_map(|position| {
                self.read_field_by_id(cells, model, field, position)
                    .transpose()
            })
            .collect::<Result<Vec<_>, _>>()?;

        match sample.mode.as_str() {
            "Average" => average_values(field_meta, &values),
            "Sum" => Ok(HyleValue::F64(
                values
                    .iter()
                    .map(as_f64)
                    .collect::<Result<Vec<_>, _>>()?
                    .iter()
                    .sum(),
            )),
            "Nearest" => {
                let value =
                    if let Some(value) = self.read_field_by_id(cells, model, field, anchor)? {
                        value
                    } else {
                        values
                            .into_iter()
                            .next()
                            .unwrap_or_else(|| zero_for_field(field_meta))
                    };
                Ok(value)
            }
            "All" => Ok(HyleValue::Bool(
                values
                    .iter()
                    .all(|value| matches!(value, HyleValue::Bool(true))),
            )),
            other => Err(RuntimeError::Step(format!(
                "unsupported sampling mode `{other}`"
            ))),
        }
    }

    fn neighbor_positions(
        &self,
        anchor: &CellPosition,
        range: &SoleRange,
    ) -> Result<Vec<CellPosition>, RuntimeError> {
        let dimensions = self.module.world.dimensions as usize;
        if anchor.coordinates.len() != dimensions {
            return Err(RuntimeError::Step(format!(
                "position has {} dimensions but world has {dimensions}",
                anchor.coordinates.len()
            )));
        }

        let radius = literal_as_f64(&range.radius)?;
        let extent = radius.ceil() as i64;
        let mut offsets = Vec::new();
        build_offsets(dimensions, extent, &mut Vec::new(), &mut offsets);

        Ok(offsets
            .into_iter()
            .filter(|offset| {
                let distance = match range.metric.as_str() {
                    "Manhattan" => offset.iter().map(|v| v.abs() as f64).sum(),
                    "Euclidean" => offset
                        .iter()
                        .map(|v| (*v as f64) * (*v as f64))
                        .sum::<f64>()
                        .sqrt(),
                    _ => f64::INFINITY,
                };
                distance <= radius && (range.center || offset.iter().any(|value| *value != 0))
            })
            .map(|offset| CellPosition {
                coordinates: anchor
                    .coordinates
                    .iter()
                    .zip(offset)
                    .map(|(coordinate, delta)| coordinate + delta)
                    .collect(),
            })
            .collect())
    }

    fn default_cell(&self, model: usize) -> Result<CellState, RuntimeError> {
        Ok(CellState {
            fields: self
                .model(model)?
                .fields
                .iter()
                .map(|field| {
                    let value = literal_to_value(&field.ty, &field.default)?;
                    check_field_bounds(field, &value)?;
                    Ok(value)
                })
                .collect::<Result<Vec<_>, RuntimeError>>()?,
        })
    }

    fn read_cell_by_id<'a>(
        &self,
        cells: &'a HashMap<CellKey, CellState>,
        model: usize,
        position: &CellPosition,
    ) -> Option<&'a CellState> {
        cells.get(&CellKey {
            model,
            position: position.clone(),
        })
    }

    fn read_field_by_id(
        &self,
        cells: &HashMap<CellKey, CellState>,
        model: usize,
        field: usize,
        position: &CellPosition,
    ) -> Result<Option<HyleValue>, RuntimeError> {
        self.field(model, field)?;
        Ok(self
            .read_cell_by_id(cells, model, position)
            .map(|cell| cell.fields[field].clone()))
    }

    fn model_id(&self, model: &str) -> Result<usize, RuntimeError> {
        self.model_by_name
            .get(model)
            .copied()
            .ok_or_else(|| RuntimeError::CellEdit(format!("unknown model `{model}`")))
    }

    fn field_id(&self, model: usize, field: &str) -> Result<usize, RuntimeError> {
        self.field_by_name
            .get(model)
            .and_then(|fields| fields.get(field))
            .copied()
            .ok_or_else(|| RuntimeError::FieldAccess(format!("unknown field `{field}`")))
    }

    fn model(&self, model: usize) -> Result<&hyle_sole::SoleModel, RuntimeError> {
        self.module
            .models
            .get(model)
            .ok_or_else(|| RuntimeError::Step(format!("unknown model id {model}")))
    }

    fn field(&self, model: usize, field: usize) -> Result<&SoleField, RuntimeError> {
        self.model(model)?
            .fields
            .get(field)
            .ok_or_else(|| RuntimeError::FieldAccess(format!("unknown field id {field}")))
    }

    fn range(&self, range: usize) -> Result<&SoleRange, RuntimeError> {
        self.module
            .ranges
            .get(range)
            .ok_or_else(|| RuntimeError::Step(format!("unknown range id {range}")))
    }
}

impl InputAccess for CpuAccess {
    fn get_input(&self, name: &str) -> Result<HyleValue, RuntimeError> {
        let input = self
            .input_by_name
            .get(name)
            .copied()
            .ok_or_else(|| RuntimeError::InputAccess(format!("unknown input `{name}`")))?;
        Ok(self.inputs[input].clone())
    }

    fn set_input(&mut self, name: &str, value: HyleValue) -> Result<(), RuntimeError> {
        let input_id = self
            .input_by_name
            .get(name)
            .copied()
            .ok_or_else(|| RuntimeError::InputAccess(format!("unknown input `{name}`")))?;
        let input = &self.module.inputs[input_id];
        let value = coerce_value(&input.ty, value)?;
        check_input_bounds(input, &value)?;
        self.inputs[input_id] = value;
        Ok(())
    }
}

impl CellRead for CpuAccess {
    fn cell_exists(&self, model: &str, position: &CellPosition) -> Result<bool, RuntimeError> {
        let model = self.model_id(model)?;
        Ok(self.cells.contains_key(&CellKey {
            model,
            position: position.clone(),
        }))
    }

    fn read_cell(
        &self,
        model: &str,
        position: &CellPosition,
    ) -> Result<Option<Cell>, RuntimeError> {
        let model_id = self.model_id(model)?;
        Ok(self
            .cells
            .get(&CellKey {
                model: model_id,
                position: position.clone(),
            })
            .map(|state| Cell {
                model: model.to_owned(),
                position: position.clone(),
                fields: self.module.models[model_id]
                    .fields
                    .iter()
                    .zip(&state.fields)
                    .map(|(field, value)| CellFieldValue {
                        field_name: field.name.clone(),
                        value: value.clone(),
                    })
                    .collect(),
            }))
    }

    fn read_cells(
        &self,
        model: &str,
        positions: &[CellPosition],
    ) -> Result<Vec<Option<Cell>>, RuntimeError> {
        positions
            .iter()
            .map(|position| self.read_cell(model, position))
            .collect()
    }

    fn read_batch(
        &self,
        model: &str,
        fields: &[&str],
        positions: &[CellPosition],
    ) -> Result<CellBatch, RuntimeError> {
        let model_id = self.model_id(model)?;
        let field_ids = fields
            .iter()
            .map(|field| self.field_id(model_id, field))
            .collect::<Result<Vec<_>, _>>()?;
        let mut columns = Vec::new();

        for (field_name, field_id) in fields.iter().zip(field_ids) {
            let mut values = Vec::new();
            for position in positions {
                let value = self
                    .read_field_by_id(&self.cells, model_id, field_id, position)?
                    .ok_or_else(|| missing_cell_error(model, position))?;
                values.push(value);
            }
            columns.push(CellFieldColumn {
                field_name: (*field_name).to_owned(),
                values: values_to_column(&self.field(model_id, field_id)?.ty, values)?,
            });
        }

        Ok(CellBatch {
            model: model.to_owned(),
            positions: positions.to_vec(),
            fields: columns,
        })
    }

    fn get_field(
        &self,
        model: &str,
        field: &str,
        position: &CellPosition,
    ) -> Result<Option<HyleValue>, RuntimeError> {
        let model_id = self.model_id(model)?;
        let field_id = self.field_id(model_id, field)?;
        self.read_field_by_id(&self.cells, model_id, field_id, position)
    }
}

impl CellWrite for CpuAccess {
    fn set_field(
        &mut self,
        model: &str,
        field: &str,
        position: &CellPosition,
        value: HyleValue,
    ) -> Result<(), RuntimeError> {
        let model_id = self.model_id(model)?;
        let field_id = self.field_id(model_id, field)?;
        let field = self.field(model_id, field_id)?;
        let value = coerce_value(&field.ty, value)?;
        check_field_bounds(field, &value)?;
        let cell = self
            .cells
            .get_mut(&CellKey {
                model: model_id,
                position: position.clone(),
            })
            .ok_or_else(|| missing_cell_error(model, position))?;
        cell.fields[field_id] = value;
        Ok(())
    }

    fn add_cells(&mut self, batch: CellBatch) -> Result<(), RuntimeError> {
        let model_id = self.model_id(&batch.model)?;
        let count = batch.positions.len();
        validate_columns(&batch.fields, count)?;

        for position in &batch.positions {
            let key = CellKey {
                model: model_id,
                position: position.clone(),
            };
            if self.cells.contains_key(&key) {
                return Err(RuntimeError::CellEdit(format!(
                    "`{}` cell already exists at {:?}",
                    batch.model, position.coordinates
                )));
            }
        }

        for (index, position) in batch.positions.into_iter().enumerate() {
            let mut cell = self.default_cell(model_id)?;
            self.apply_columns(model_id, &mut cell, &batch.fields, index)?;
            self.cells.insert(
                CellKey {
                    model: model_id,
                    position,
                },
                cell,
            );
        }
        Ok(())
    }

    fn update_cells(&mut self, batch: CellBatch) -> Result<(), RuntimeError> {
        let model_id = self.model_id(&batch.model)?;
        let count = batch.positions.len();
        validate_columns(&batch.fields, count)?;

        for (index, position) in batch.positions.iter().enumerate() {
            let key = CellKey {
                model: model_id,
                position: position.clone(),
            };
            let mut cell = self
                .cells
                .get(&key)
                .cloned()
                .ok_or_else(|| missing_cell_error(&batch.model, position))?;
            self.apply_columns(model_id, &mut cell, &batch.fields, index)?;
            self.cells.insert(key, cell);
        }
        Ok(())
    }

    fn upsert_cells(&mut self, batch: CellBatch) -> Result<(), RuntimeError> {
        let model_id = self.model_id(&batch.model)?;
        let count = batch.positions.len();
        validate_columns(&batch.fields, count)?;

        for (index, position) in batch.positions.into_iter().enumerate() {
            let key = CellKey {
                model: model_id,
                position,
            };
            let mut cell = self
                .cells
                .get(&key)
                .cloned()
                .unwrap_or(self.default_cell(model_id)?);
            self.apply_columns(model_id, &mut cell, &batch.fields, index)?;
            self.cells.insert(key, cell);
        }
        Ok(())
    }

    fn remove_cells(
        &mut self,
        model: &str,
        positions: &[CellPosition],
    ) -> Result<(), RuntimeError> {
        let model_id = self.model_id(model)?;
        for position in positions {
            if self
                .cells
                .remove(&CellKey {
                    model: model_id,
                    position: position.clone(),
                })
                .is_none()
            {
                return Err(missing_cell_error(model, position));
            }
        }
        Ok(())
    }
}

impl CpuAccess {
    fn apply_columns(
        &self,
        model_id: usize,
        cell: &mut CellState,
        columns: &[CellFieldColumn],
        index: usize,
    ) -> Result<(), RuntimeError> {
        for column in columns {
            let field_id = self.field_id(model_id, &column.field_name)?;
            let field = self.field(model_id, field_id)?;
            let value = coerce_value(&field.ty, column_value(&column.values, index)?)?;
            check_field_bounds(field, &value)?;
            cell.fields[field_id] = value;
        }
        Ok(())
    }
}

impl EncodedCellIo for CpuAccess {
    fn write_encoded_cells(&mut self, _bytes: &[u8]) -> Result<(), RuntimeError> {
        Err(RuntimeError::CellEdit(
            "cpu encoded cell IO is not implemented".to_owned(),
        ))
    }

    fn read_encoded_cells(&self, _request: &[u8], _out: &mut [u8]) -> Result<usize, RuntimeError> {
        Err(RuntimeError::CellEdit(
            "cpu encoded cell IO is not implemented".to_owned(),
        ))
    }
}

fn validate_module(module: &SoleModule) -> Result<(), RuntimeError> {
    if module.version != "0.1" {
        return Err(RuntimeError::ModuleLoad(format!(
            "unsupported .sole version `{}`",
            module.version
        )));
    }
    if !(1..=4).contains(&module.world.dimensions) {
        return Err(RuntimeError::ModuleLoad(format!(
            "unsupported world dimension count {}",
            module.world.dimensions
        )));
    }
    for (expected, model) in module.models.iter().enumerate() {
        if model.id != expected {
            return Err(RuntimeError::ModuleLoad(format!(
                "model `{}` has non-contiguous id {}",
                model.name, model.id
            )));
        }
        for (field_expected, field) in model.fields.iter().enumerate() {
            if field.id != field_expected {
                return Err(RuntimeError::ModuleLoad(format!(
                    "field `{}` on model `{}` has non-contiguous id {}",
                    field.name, model.name, field.id
                )));
            }
            let default = literal_to_value(&field.ty, &field.default)?;
            check_field_bounds(field, &default)?;
        }
    }
    for (expected, input) in module.inputs.iter().enumerate() {
        if input.id != expected {
            return Err(RuntimeError::ModuleLoad(format!(
                "input `{}` has non-contiguous id {}",
                input.name, input.id
            )));
        }
        let default = literal_to_value(&input.ty, &input.default)?;
        check_input_bounds(input, &default)?;
    }
    for (expected, range) in module.ranges.iter().enumerate() {
        if range.id != expected {
            return Err(RuntimeError::ModuleLoad(format!(
                "range `{}` has non-contiguous id {}",
                range.name, range.id
            )));
        }
    }
    Ok(())
}

fn validate_columns(columns: &[CellFieldColumn], count: usize) -> Result<(), RuntimeError> {
    for field in columns {
        if field.values.len() != count {
            return Err(RuntimeError::CellEdit(format!(
                "field `{}` has {} values for {count} cells",
                field.field_name,
                field.values.len()
            )));
        }
    }
    Ok(())
}

fn literal_to_value(ty: &str, value: &SoleLiteralValue) -> Result<HyleValue, RuntimeError> {
    match (ty, value) {
        ("bool", SoleLiteralValue::Bool(value)) => Ok(HyleValue::Bool(*value)),
        ("i32", SoleLiteralValue::Integer(value)) => {
            Ok(HyleValue::I32((*value).try_into().map_err(|_| {
                RuntimeError::ModuleLoad(format!("{value} does not fit in i32"))
            })?))
        }
        ("u32", SoleLiteralValue::Integer(value)) => {
            Ok(HyleValue::U32((*value).try_into().map_err(|_| {
                RuntimeError::ModuleLoad(format!("{value} does not fit in u32"))
            })?))
        }
        ("i64", SoleLiteralValue::Integer(value)) => Ok(HyleValue::I64(*value)),
        ("u64", SoleLiteralValue::Integer(value)) => {
            Ok(HyleValue::U64((*value).try_into().map_err(|_| {
                RuntimeError::ModuleLoad(format!("{value} does not fit in u64"))
            })?))
        }
        ("f32", SoleLiteralValue::Float(value)) => Ok(HyleValue::F32(*value as f32)),
        ("f64", SoleLiteralValue::Float(value)) => Ok(HyleValue::F64(*value)),
        ("f32", SoleLiteralValue::Integer(value)) => Ok(HyleValue::F32(*value as f32)),
        ("f64", SoleLiteralValue::Integer(value)) => Ok(HyleValue::F64(*value as f64)),
        _ => Err(RuntimeError::ModuleLoad(format!(
            "literal {value:?} does not match type `{ty}`"
        ))),
    }
}

fn coerce_value(ty: &str, value: HyleValue) -> Result<HyleValue, RuntimeError> {
    match (ty, value) {
        ("bool", HyleValue::Bool(value)) => Ok(HyleValue::Bool(value)),
        ("i32", value) => Ok(HyleValue::I32(as_f64(&value)? as i32)),
        ("u32", value) => Ok(HyleValue::U32(as_f64(&value)? as u32)),
        ("i64", value) => Ok(HyleValue::I64(as_f64(&value)? as i64)),
        ("u64", value) => Ok(HyleValue::U64(as_f64(&value)? as u64)),
        ("f32", value) => Ok(HyleValue::F32(as_f64(&value)? as f32)),
        ("f64", value) => Ok(HyleValue::F64(as_f64(&value)?)),
        (other, _) => Err(RuntimeError::FieldAccess(format!(
            "unsupported runtime type `{other}`"
        ))),
    }
}

fn check_field_bounds(field: &SoleField, value: &HyleValue) -> Result<(), RuntimeError> {
    if let Some(bounds) = &field.bounds {
        check_bounds(
            &field.name,
            value,
            &bounds.min,
            bounds.min_closed,
            &bounds.max,
            bounds.max_closed,
        )?;
    }
    Ok(())
}

fn check_input_bounds(input: &SoleInput, value: &HyleValue) -> Result<(), RuntimeError> {
    if let Some(bounds) = &input.bounds {
        check_bounds(
            &input.name,
            value,
            &bounds.min,
            bounds.min_closed,
            &bounds.max,
            bounds.max_closed,
        )?;
    }
    Ok(())
}

fn check_bounds(
    name: &str,
    value: &HyleValue,
    min: &SoleLiteralValue,
    min_closed: bool,
    max: &SoleLiteralValue,
    max_closed: bool,
) -> Result<(), RuntimeError> {
    let value = as_f64(value)?;
    let min = literal_as_f64(min)?;
    let max = literal_as_f64(max)?;
    let min_ok = if min_closed {
        value >= min
    } else {
        value > min
    };
    let max_ok = if max_closed {
        value <= max
    } else {
        value < max
    };
    if min_ok && max_ok {
        Ok(())
    } else {
        Err(RuntimeError::FieldAccess(format!(
            "`{name}` value {value} is outside bounds"
        )))
    }
}

fn eval_call(function: &str, args: &[HyleValue]) -> Result<HyleValue, RuntimeError> {
    match function {
        "clamp" if args.len() == 3 => {
            let value = as_f64(&args[0])?;
            let min = as_f64(&args[1])?;
            let max = as_f64(&args[2])?;
            Ok(HyleValue::F64(value.clamp(min, max)))
        }
        other => Err(RuntimeError::Step(format!("unsupported call `{other}`"))),
    }
}

fn eval_op(op: &str, args: &[HyleValue]) -> Result<HyleValue, RuntimeError> {
    match op {
        "Neg" if args.len() == 1 => Ok(HyleValue::F64(-as_f64(&args[0])?)),
        "Not" if args.len() == 1 => Ok(HyleValue::Bool(!as_bool(&args[0])?)),
        "Add" if args.len() == 2 => Ok(HyleValue::F64(as_f64(&args[0])? + as_f64(&args[1])?)),
        "Sub" if args.len() == 2 => Ok(HyleValue::F64(as_f64(&args[0])? - as_f64(&args[1])?)),
        "Mul" if args.len() == 2 => Ok(HyleValue::F64(as_f64(&args[0])? * as_f64(&args[1])?)),
        "Div" if args.len() == 2 => Ok(HyleValue::F64(as_f64(&args[0])? / as_f64(&args[1])?)),
        "Eq" if args.len() == 2 => Ok(HyleValue::Bool(args[0] == args[1])),
        "Neq" if args.len() == 2 => Ok(HyleValue::Bool(args[0] != args[1])),
        "Lt" if args.len() == 2 => Ok(HyleValue::Bool(as_f64(&args[0])? < as_f64(&args[1])?)),
        "Lte" if args.len() == 2 => Ok(HyleValue::Bool(as_f64(&args[0])? <= as_f64(&args[1])?)),
        "Gt" if args.len() == 2 => Ok(HyleValue::Bool(as_f64(&args[0])? > as_f64(&args[1])?)),
        "Gte" if args.len() == 2 => Ok(HyleValue::Bool(as_f64(&args[0])? >= as_f64(&args[1])?)),
        "And" if args.len() == 2 => Ok(HyleValue::Bool(as_bool(&args[0])? && as_bool(&args[1])?)),
        "Or" if args.len() == 2 => Ok(HyleValue::Bool(as_bool(&args[0])? || as_bool(&args[1])?)),
        other => Err(RuntimeError::Step(format!("unsupported op `{other}`"))),
    }
}

fn average_values(field: &SoleField, values: &[HyleValue]) -> Result<HyleValue, RuntimeError> {
    if values.is_empty() {
        return Ok(zero_for_field(field));
    }
    if field.ty == "bool" {
        let true_count = values
            .iter()
            .filter(|value| matches!(value, HyleValue::Bool(true)))
            .count();
        return Ok(HyleValue::Bool(true_count * 2 >= values.len()));
    }
    let sum = values
        .iter()
        .map(as_f64)
        .collect::<Result<Vec<_>, _>>()?
        .iter()
        .sum::<f64>();
    coerce_value(&field.ty, HyleValue::F64(sum / values.len() as f64))
}

fn zero_for_field(field: &SoleField) -> HyleValue {
    match field.ty.as_str() {
        "bool" => HyleValue::Bool(false),
        "i32" => HyleValue::I32(0),
        "u32" => HyleValue::U32(0),
        "i64" => HyleValue::I64(0),
        "u64" => HyleValue::U64(0),
        "f32" => HyleValue::F32(0.0),
        "f64" => HyleValue::F64(0.0),
        _ => HyleValue::F64(0.0),
    }
}

fn as_f64(value: &HyleValue) -> Result<f64, RuntimeError> {
    match value {
        HyleValue::I32(value) => Ok(*value as f64),
        HyleValue::U32(value) => Ok(*value as f64),
        HyleValue::I64(value) => Ok(*value as f64),
        HyleValue::U64(value) => Ok(*value as f64),
        HyleValue::F32(value) => Ok(*value as f64),
        HyleValue::F64(value) => Ok(*value),
        HyleValue::Bool(_) => Err(RuntimeError::FieldAccess(
            "expected numeric value".to_owned(),
        )),
    }
}

fn as_bool(value: &HyleValue) -> Result<bool, RuntimeError> {
    match value {
        HyleValue::Bool(value) => Ok(*value),
        _ => Err(RuntimeError::FieldAccess("expected bool value".to_owned())),
    }
}

fn literal_as_f64(value: &SoleLiteralValue) -> Result<f64, RuntimeError> {
    match value {
        SoleLiteralValue::Integer(value) => Ok(*value as f64),
        SoleLiteralValue::Float(value) => Ok(*value),
        SoleLiteralValue::Bool(_) => Err(RuntimeError::ModuleLoad(
            "expected numeric literal".to_owned(),
        )),
    }
}

fn column_value(values: &FieldColumnValues, index: usize) -> Result<HyleValue, RuntimeError> {
    match values {
        FieldColumnValues::Bool(values) => values.get(index).copied().map(HyleValue::Bool),
        FieldColumnValues::I32(values) => values.get(index).copied().map(HyleValue::I32),
        FieldColumnValues::U32(values) => values.get(index).copied().map(HyleValue::U32),
        FieldColumnValues::I64(values) => values.get(index).copied().map(HyleValue::I64),
        FieldColumnValues::U64(values) => values.get(index).copied().map(HyleValue::U64),
        FieldColumnValues::F32(values) => values.get(index).copied().map(HyleValue::F32),
        FieldColumnValues::F64(values) => values.get(index).copied().map(HyleValue::F64),
    }
    .ok_or_else(|| RuntimeError::CellEdit(format!("missing column value at index {index}")))
}

fn values_to_column(ty: &str, values: Vec<HyleValue>) -> Result<FieldColumnValues, RuntimeError> {
    match ty {
        "bool" => values
            .into_iter()
            .map(|value| match coerce_value(ty, value)? {
                HyleValue::Bool(value) => Ok(value),
                _ => unreachable!(),
            })
            .collect::<Result<Vec<_>, _>>()
            .map(FieldColumnValues::Bool),
        "i32" => values
            .into_iter()
            .map(|value| match coerce_value(ty, value)? {
                HyleValue::I32(value) => Ok(value),
                _ => unreachable!(),
            })
            .collect::<Result<Vec<_>, _>>()
            .map(FieldColumnValues::I32),
        "u32" => values
            .into_iter()
            .map(|value| match coerce_value(ty, value)? {
                HyleValue::U32(value) => Ok(value),
                _ => unreachable!(),
            })
            .collect::<Result<Vec<_>, _>>()
            .map(FieldColumnValues::U32),
        "i64" => values
            .into_iter()
            .map(|value| match coerce_value(ty, value)? {
                HyleValue::I64(value) => Ok(value),
                _ => unreachable!(),
            })
            .collect::<Result<Vec<_>, _>>()
            .map(FieldColumnValues::I64),
        "u64" => values
            .into_iter()
            .map(|value| match coerce_value(ty, value)? {
                HyleValue::U64(value) => Ok(value),
                _ => unreachable!(),
            })
            .collect::<Result<Vec<_>, _>>()
            .map(FieldColumnValues::U64),
        "f32" => values
            .into_iter()
            .map(|value| match coerce_value(ty, value)? {
                HyleValue::F32(value) => Ok(value),
                _ => unreachable!(),
            })
            .collect::<Result<Vec<_>, _>>()
            .map(FieldColumnValues::F32),
        "f64" => values
            .into_iter()
            .map(|value| match coerce_value(ty, value)? {
                HyleValue::F64(value) => Ok(value),
                _ => unreachable!(),
            })
            .collect::<Result<Vec<_>, _>>()
            .map(FieldColumnValues::F64),
        other => Err(RuntimeError::FieldAccess(format!(
            "unsupported column type `{other}`"
        ))),
    }
}

fn missing_cell_error(model: &str, position: &CellPosition) -> RuntimeError {
    RuntimeError::CellEdit(format!(
        "missing `{model}` cell at {:?}",
        position.coordinates
    ))
}

fn build_offsets(
    dimensions: usize,
    extent: i64,
    current: &mut Vec<i64>,
    output: &mut Vec<Vec<i64>>,
) {
    if current.len() == dimensions {
        output.push(current.clone());
        return;
    }

    for value in -extent..=extent {
        current.push(value);
        build_offsets(dimensions, extent, current, output);
        current.pop();
    }
}
