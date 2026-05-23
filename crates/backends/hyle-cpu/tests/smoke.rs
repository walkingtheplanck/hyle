use hyle_cpu::CpuSolver;
use hyle_runtime::{
    CellBatch, CellFieldColumn, CellPosition, FieldColumnValues, HyleValue, SolverBackend,
};
use hyle_sole::{
    SoleBounds, SoleCall, SoleExpr, SoleField, SoleInput, SoleLiteral, SoleLiteralValue, SoleModel,
    SoleModule, SoleNeighbors, SoleOpExpr, SoleRange, SoleRead, SoleReduce, SoleRule, SoleSample,
    SoleWorld, SoleWrite,
};

#[test]
fn stores_cells_defaults_and_fields() {
    let solver = CpuSolver;
    let mut instance = solver
        .init(
            module_with_models(Vec::new())
                .to_json_string()
                .unwrap()
                .as_bytes(),
        )
        .expect("init");
    let position = pos(&[0, 0]);

    instance
        .add_cells(CellBatch {
            model: "Grass".to_owned(),
            positions: vec![position.clone()],
            fields: vec![CellFieldColumn {
                field_name: "biomass".to_owned(),
                values: FieldColumnValues::F32(vec![2.0]),
            }],
        })
        .expect("add");

    assert_eq!(
        instance.get_field("Grass", "humidity", &position).unwrap(),
        Some(HyleValue::F32(0.5))
    );
    assert_eq!(
        instance.get_field("Grass", "biomass", &position).unwrap(),
        Some(HyleValue::F32(2.0))
    );

    instance
        .set_field("Grass", "humidity", &position, HyleValue::F32(0.25))
        .expect("set field");
    assert_eq!(
        instance.get_field("Grass", "humidity", &position).unwrap(),
        Some(HyleValue::F32(0.25))
    );

    let batch = instance
        .read_batch("Grass", &["humidity", "biomass"], &[position])
        .expect("read batch");
    assert_eq!(batch.fields.len(), 2);
    assert_eq!(batch.fields[0].values, FieldColumnValues::F32(vec![0.25]));
    assert_eq!(batch.fields[1].values, FieldColumnValues::F32(vec![2.0]));
}

#[test]
fn manages_inputs_and_checks_bounds() {
    let solver = CpuSolver;
    let mut instance = solver
        .init(
            module_with_models(Vec::new())
                .to_json_string()
                .unwrap()
                .as_bytes(),
        )
        .expect("init");

    assert_eq!(
        instance.get_input("wind_speed").unwrap(),
        HyleValue::F32(0.01)
    );
    instance
        .set_input("wind_speed", HyleValue::F32(2.5))
        .expect("set input");
    assert_eq!(
        instance.get_input("wind_speed").unwrap(),
        HyleValue::F32(2.5)
    );

    assert!(instance
        .set_input("wind_speed", HyleValue::F32(251.0))
        .is_err());
}

#[test]
fn executes_direct_rule_writes() {
    let solver = CpuSolver;
    let module = module_with_models(vec![SoleRule {
        id: 0,
        name: "heat".to_owned(),
        anchor: 1,
        target: 1,
        range: 0,
        samples: Vec::new(),
        when: None,
        lets: Vec::new(),
        writes: vec![SoleWrite {
            field: 0,
            value: clamp(add(read(1, 0), input(0)), f32_lit(0.0), f32_lit(1.0)),
        }],
    }]);
    let mut instance = solver
        .init(module.to_json_string().unwrap().as_bytes())
        .expect("init");
    let position = pos(&[0, 0]);

    instance
        .add_cells(CellBatch {
            model: "Fire".to_owned(),
            positions: vec![position.clone()],
            fields: vec![CellFieldColumn {
                field_name: "intensity".to_owned(),
                values: FieldColumnValues::F32(vec![0.5]),
            }],
        })
        .expect("add");
    instance
        .set_input("wind_speed", HyleValue::F32(0.2))
        .expect("input");
    instance.step().expect("step");

    assert_eq!(
        instance.get_field("Fire", "intensity", &position).unwrap(),
        Some(HyleValue::F32(0.7))
    );
}

#[test]
fn executes_neighbor_reductions_from_previous_state() {
    let solver = CpuSolver;
    let module = module_with_models(vec![SoleRule {
        id: 0,
        name: "spread".to_owned(),
        anchor: 1,
        target: 1,
        range: 0,
        samples: Vec::new(),
        when: None,
        lets: Vec::new(),
        writes: vec![SoleWrite {
            field: 0,
            value: clamp(
                add(
                    read(1, 0),
                    SoleExpr::Reduce {
                        reduce: SoleReduce {
                            op: "Sum".to_owned(),
                            var: "n".to_owned(),
                            over: Box::new(SoleExpr::Neighbors {
                                neighbors: SoleNeighbors { model: 1, range: 0 },
                            }),
                            expr: Box::new(SoleExpr::Read {
                                read: SoleRead {
                                    model: None,
                                    var: Some("n".to_owned()),
                                    field: 0,
                                },
                            }),
                        },
                    },
                ),
                f32_lit(0.0),
                f32_lit(1.0),
            ),
        }],
    }]);
    let mut instance = solver
        .init(module.to_json_string().unwrap().as_bytes())
        .expect("init");
    let left = pos(&[0, 0]);
    let right = pos(&[1, 0]);

    instance
        .add_cells(CellBatch {
            model: "Fire".to_owned(),
            positions: vec![left.clone(), right.clone()],
            fields: vec![CellFieldColumn {
                field_name: "intensity".to_owned(),
                values: FieldColumnValues::F32(vec![0.5, 0.25]),
            }],
        })
        .expect("add");
    instance.step().expect("step");

    assert_eq!(
        instance.get_field("Fire", "intensity", &left).unwrap(),
        Some(HyleValue::F32(0.75))
    );
    assert_eq!(
        instance.get_field("Fire", "intensity", &right).unwrap(),
        Some(HyleValue::F32(0.75))
    );
}

#[test]
fn executes_sampled_transform_rules() {
    let solver = CpuSolver;
    let module = module_with_models(vec![SoleRule {
        id: 0,
        name: "ash".to_owned(),
        anchor: 0,
        target: 2,
        range: 0,
        samples: vec![SoleSample {
            model: 1,
            mode: "Average".to_owned(),
        }],
        when: Some(gt(read(1, 0), f32_lit(0.3))),
        lets: Vec::new(),
        writes: vec![SoleWrite {
            field: 0,
            value: read(0, 1),
        }],
    }]);
    let mut instance = solver
        .init(module.to_json_string().unwrap().as_bytes())
        .expect("init");
    let grass = pos(&[0, 0]);
    let fire = pos(&[1, 0]);

    instance
        .add_cells(CellBatch {
            model: "Grass".to_owned(),
            positions: vec![grass.clone()],
            fields: vec![CellFieldColumn {
                field_name: "biomass".to_owned(),
                values: FieldColumnValues::F32(vec![3.0]),
            }],
        })
        .expect("grass");
    instance
        .add_cells(CellBatch {
            model: "Fire".to_owned(),
            positions: vec![fire],
            fields: vec![CellFieldColumn {
                field_name: "intensity".to_owned(),
                values: FieldColumnValues::F32(vec![0.5]),
            }],
        })
        .expect("fire");

    instance.step().expect("step");

    assert!(instance.cell_exists("Ash", &grass).unwrap());
    assert_eq!(
        instance.get_field("Ash", "amount", &grass).unwrap(),
        Some(HyleValue::F32(3.0))
    );
}

fn module_with_models(rules: Vec<SoleRule>) -> SoleModule {
    SoleModule {
        version: "0.1".to_owned(),
        world: SoleWorld {
            dimensions: 2,
            cell: "Square".to_owned(),
        },
        ranges: vec![SoleRange {
            id: 0,
            name: "VonNeumann1".to_owned(),
            radius: SoleLiteralValue::Integer(1),
            center: false,
            metric: "Manhattan".to_owned(),
        }],
        models: vec![
            SoleModel {
                id: 0,
                name: "Grass".to_owned(),
                resolution: 1,
                range: 0,
                fields: vec![
                    field(0, "humidity", "f32", 0.5, 0.0, 1.0, false),
                    field(1, "biomass", "f32", 0.5, 0.0, 10.0, true),
                ],
            },
            SoleModel {
                id: 1,
                name: "Fire".to_owned(),
                resolution: 1,
                range: 0,
                fields: vec![field(0, "intensity", "f32", 0.5, 0.0, 1.0, false)],
            },
            SoleModel {
                id: 2,
                name: "Ash".to_owned(),
                resolution: 1,
                range: 0,
                fields: vec![field(0, "amount", "f32", 0.0, 0.0, 10.0, true)],
            },
        ],
        inputs: vec![SoleInput {
            id: 0,
            name: "wind_speed".to_owned(),
            ty: "f32".to_owned(),
            default: SoleLiteralValue::Float(0.01),
            bounds: Some(SoleBounds {
                min: SoleLiteralValue::Float(0.0),
                max: SoleLiteralValue::Float(250.0),
                min_closed: true,
                max_closed: true,
            }),
            epsilon: 1e-7,
        }],
        rules,
    }
}

fn field(
    id: usize,
    name: &str,
    ty: &str,
    default: f64,
    min: f64,
    max: f64,
    max_closed: bool,
) -> SoleField {
    SoleField {
        id,
        name: name.to_owned(),
        ty: ty.to_owned(),
        default: SoleLiteralValue::Float(default),
        bounds: Some(SoleBounds {
            min: SoleLiteralValue::Float(min),
            max: SoleLiteralValue::Float(max),
            min_closed: true,
            max_closed,
        }),
        epsilon: 1e-7,
    }
}

fn pos(coordinates: &[i64]) -> CellPosition {
    CellPosition {
        coordinates: coordinates.to_vec(),
    }
}

fn f32_lit(value: f64) -> SoleExpr {
    SoleExpr::Literal {
        literal: SoleLiteral {
            ty: "f32".to_owned(),
            value: SoleLiteralValue::Float(value),
        },
    }
}

fn input(input: usize) -> SoleExpr {
    SoleExpr::Input { input }
}

fn read(model: usize, field: usize) -> SoleExpr {
    SoleExpr::Read {
        read: SoleRead {
            model: Some(model),
            var: None,
            field,
        },
    }
}

fn add(left: SoleExpr, right: SoleExpr) -> SoleExpr {
    op("Add", left, right)
}

fn gt(left: SoleExpr, right: SoleExpr) -> SoleExpr {
    op("Gt", left, right)
}

fn op(name: &str, left: SoleExpr, right: SoleExpr) -> SoleExpr {
    SoleExpr::Op(SoleOpExpr {
        op: name.to_owned(),
        args: vec![left, right],
    })
}

fn clamp(value: SoleExpr, min: SoleExpr, max: SoleExpr) -> SoleExpr {
    SoleExpr::Call {
        call: SoleCall {
            function: "clamp".to_owned(),
            args: vec![value, min, max],
        },
    }
}
