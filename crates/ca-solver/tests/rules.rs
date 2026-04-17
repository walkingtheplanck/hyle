//! Rule application tests using declarative blueprints.

use hyle_ca_interface::resolved::{cell_rng, interpret_blueprint};
use hyle_ca_interface::{
    attr, neighbors, rng, AttrAssign, AttributeSet, AttributeType, AttributeValue, Blueprint,
    CaSolver, Instance, MatAttr, MaterialSet, NeighborhoodFalloff, NeighborhoodRadius,
    NeighborhoodSet, NeighborhoodShape, NeighborhoodSpec, RuleSpec, Weight,
};
use hyle_ca_solver::Solver;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum M {
    #[default]
    Dead,
    Alive,
    Water,
    Ice,
}

impl MaterialSet for M {
    fn variants() -> &'static [Self] {
        &[M::Dead, M::Alive, M::Water, M::Ice]
    }

    fn label(self) -> &'static str {
        match self {
            M::Dead => "dead",
            M::Alive => "alive",
            M::Water => "water",
            M::Ice => "ice",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum A {
    Heat,
}

impl AttributeSet for A {
    fn variants() -> &'static [Self] {
        &[A::Heat]
    }

    fn label(self) -> &'static str {
        "heat"
    }

    fn value_type(self) -> AttributeType {
        AttributeType::U8
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum N {
    Adjacent,
    RadiusTwo,
}

impl NeighborhoodSet for N {
    fn variants() -> &'static [Self] {
        &[N::Adjacent, N::RadiusTwo]
    }

    fn label(self) -> &'static str {
        match self {
            N::Adjacent => "adjacent",
            N::RadiusTwo => "radius_two",
        }
    }
}

fn specs() -> [NeighborhoodSpec; 2] {
    [
        NeighborhoodSpec::new(
            N::Adjacent,
            NeighborhoodShape::Moore,
            NeighborhoodRadius::new(1),
            NeighborhoodFalloff::Uniform,
        ),
        NeighborhoodSpec::new(
            N::RadiusTwo,
            NeighborhoodShape::Moore,
            NeighborhoodRadius::new(2),
            NeighborhoodFalloff::Uniform,
        ),
    ]
}

fn kill_all_spec() -> Blueprint {
    Blueprint::builder()
        .materials::<M>()
        .neighborhoods::<N>()
        .neighborhood_specs(specs())
        .rules([RuleSpec::when(M::Alive).becomes(M::Dead)])
        .build()
        .expect("valid spec")
}

#[test]
fn rule_kill_all() {
    let mut solver = Solver::from_spec(4, 4, 4, &kill_all_spec());
    solver.set(2, 2, 2, M::Alive.id());
    solver.set(1, 1, 1, M::Alive.id());
    solver.step();
    assert_eq!(solver.get(2, 2, 2), M::Dead.id());
    assert_eq!(solver.get(1, 1, 1), M::Dead.id());
}

#[test]
fn solver_from_blueprint_matches_from_spec() {
    let spec = kill_all_spec();
    let blueprint = interpret_blueprint(&spec);

    let mut from_spec = Solver::from_spec(4, 4, 4, &spec);
    let mut from_blueprint = Solver::from_blueprint(4, 4, 4, &blueprint);
    from_spec.set(2, 2, 2, M::Alive.id());
    from_blueprint.set(2, 2, 2, M::Alive.id());
    from_spec.step();
    from_blueprint.step();
    assert_eq!(from_spec.readback().cells, from_blueprint.readback().cells);
}

#[test]
fn rule_spread_to_neighbors() {
    let spec = Blueprint::builder()
        .materials::<M>()
        .neighborhoods::<N>()
        .neighborhood_specs(specs())
        .rules([RuleSpec::when(M::Dead)
            .require(neighbors(M::Alive).count().at_least(1))
            .becomes(M::Alive)])
        .build()
        .expect("valid spec");

    let mut solver = Solver::from_spec(5, 5, 5, &spec);
    solver.set(2, 2, 2, M::Alive.id());
    solver.step();
    assert_eq!(solver.get(1, 1, 1), M::Alive.id());
}

#[test]
fn rule_type_interaction() {
    let spec = Blueprint::builder()
        .materials::<M>()
        .neighborhoods::<N>()
        .neighborhood_specs(specs())
        .rules([RuleSpec::when(M::Water)
            .require(neighbors(M::Ice).count().eq(26))
            .becomes(M::Ice)])
        .build()
        .expect("valid spec");

    let mut solver = Solver::from_spec(5, 5, 5, &spec);
    solver.set(2, 2, 2, M::Water.id());
    for dz in -1..=1 {
        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx != 0 || dy != 0 || dz != 0 {
                    solver.set(2 + dx, 2 + dy, 2 + dz, M::Ice.id());
                }
            }
        }
    }
    solver.step();
    assert_eq!(solver.get(2, 2, 2), M::Ice.id());
}

#[test]
fn random_chance_rules_follow_semantic_rng() {
    let spec = Blueprint::builder()
        .materials::<M>()
        .neighborhoods::<N>()
        .neighborhood_specs(specs())
        .rules([RuleSpec::when(M::Dead)
            .require(rng(3).one_in(5))
            .becomes(M::Alive)])
        .build()
        .expect("valid spec");

    let instance = Instance::new(2, 2, 2).with_seed(41);
    let mut solver = Solver::from_spec_instance(instance, &spec);
    let expected = if cell_rng([0, 0, 0], 0, 3, 41).chance(5) {
        M::Alive.id()
    } else {
        M::Dead.id()
    };

    solver.step();
    assert_eq!(solver.get(0, 0, 0), expected);
}

#[test]
fn attribute_updates_apply_on_keep_rules() {
    let spec = Blueprint::builder()
        .materials::<M>()
        .attributes::<A>()
        .material_attributes([
            MatAttr::new(M::Dead, []),
            MatAttr::new(M::Alive, [AttrAssign::new(A::Heat).default(1u8)]),
            MatAttr::new(M::Water, []),
            MatAttr::new(M::Ice, []),
        ])
        .neighborhoods::<N>()
        .neighborhood_specs(specs())
        .rules([RuleSpec::when(M::Alive)
            .require(attr(A::Heat).at_least(1u8))
            .set_attr(A::Heat, 3u8)
            .keep()])
        .build()
        .expect("valid spec");

    let mut solver = Solver::from_spec(3, 3, 3, &spec);
    solver.set(1, 1, 1, M::Alive.id());
    solver.set_attr(A::Heat.id(), 1, 1, 1, AttributeValue::U8(1)).unwrap();
    solver.step();
    assert_eq!(solver.get_attr(A::Heat.id(), 1, 1, 1), Ok(AttributeValue::U8(3)));
}

#[test]
fn material_changes_reset_attributes_to_destination_defaults() {
    let spec = Blueprint::builder()
        .materials::<M>()
        .attributes::<A>()
        .material_attributes([
            MatAttr::new(M::Dead, []),
            MatAttr::new(M::Alive, [AttrAssign::new(A::Heat).default(9u8)]),
            MatAttr::new(M::Water, [AttrAssign::new(A::Heat).default(2u8)]),
            MatAttr::new(M::Ice, []),
        ])
        .neighborhoods::<N>()
        .neighborhood_specs(specs())
        .rules([RuleSpec::when(M::Alive)
            .require(attr(A::Heat).at_least(1u8))
            .becomes(M::Water)])
        .build()
        .expect("valid spec");

    let mut solver = Solver::from_spec(3, 3, 3, &spec);
    solver.set(1, 1, 1, M::Alive.id());
    solver.set_attr(A::Heat.id(), 1, 1, 1, AttributeValue::U8(7)).unwrap();
    solver.step();
    assert_eq!(solver.get(1, 1, 1), M::Water.id());
    assert_eq!(solver.get_attr(A::Heat.id(), 1, 1, 1), Ok(AttributeValue::U8(2)));
}

#[test]
fn weighted_sum_rules_work() {
    let spec = Blueprint::builder()
        .materials::<M>()
        .neighborhoods::<N>()
        .neighborhood_specs(specs())
        .rules([RuleSpec::when(M::Dead)
            .require(neighbors(M::Alive).weighted_sum().at_least(Weight::cells(2)))
            .becomes(M::Alive)])
        .build()
        .expect("valid spec");

    let mut solver = Solver::from_spec(5, 5, 5, &spec);
    solver.set(2, 2, 1, M::Alive.id());
    solver.set(2, 2, 3, M::Alive.id());
    solver.step();
    assert_eq!(solver.get(2, 2, 2), M::Alive.id());
}

#[test]
fn step_metrics_track_populations_and_transitions() {
    let mut solver = Solver::from_spec(4, 4, 4, &kill_all_spec());
    solver.set(1, 1, 1, M::Alive.id());
    solver.set(2, 2, 2, M::Alive.id());

    solver.step();

    assert_eq!(solver.step_count(), 1);
    assert_eq!(solver.last_changed_cells(), 2);
    assert_eq!(solver.population(M::Alive.id()), 0);
    assert_eq!(solver.population(M::Dead.id()), 64);
    assert_eq!(solver.last_transitions().len(), 1);
    assert_eq!(solver.last_transitions()[0].from, M::Alive.id());
    assert_eq!(solver.last_transitions()[0].to, M::Dead.id());
    assert_eq!(solver.last_transitions()[0].count, 2);
}
