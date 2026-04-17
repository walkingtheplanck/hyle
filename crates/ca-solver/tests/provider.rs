use hyle_ca_interface::{
    neighbors, AttrAssign, AttributeSet, AttributeType, AttributeValue, Blueprint, CaRuntime,
    CaSolver,
    CaSolverProvider, GridRegion, Instance, MatAttr, MaterialSet,
    NeighborhoodFalloff, NeighborhoodRadius, NeighborhoodSet, NeighborhoodShape, NeighborhoodSpec,
    RuleSpec,
};
use hyle_ca_solver::CpuSolverProvider;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum M {
    #[default]
    Dead,
    Alive,
    Other,
}

impl MaterialSet for M {
    fn variants() -> &'static [Self] {
        &[M::Dead, M::Alive, M::Other]
    }

    fn label(self) -> &'static str {
        match self {
            M::Dead => "dead",
            M::Alive => "alive",
            M::Other => "other",
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
}

impl NeighborhoodSet for N {
    fn variants() -> &'static [Self] {
        &[N::Adjacent]
    }

    fn label(self) -> &'static str {
        "adjacent"
    }
}

#[test]
fn cpu_provider_builds_runtime() {
    let spec = Blueprint::builder()
        .materials::<M>()
        .attributes::<A>()
        .material_attributes([
            MatAttr::new(M::Dead, []),
            MatAttr::new(M::Alive, [AttrAssign::new(A::Heat).default(0u8)]),
            MatAttr::new(M::Other, []),
        ])
        .neighborhoods::<N>()
        .neighborhood_specs([NeighborhoodSpec::new(
            N::Adjacent,
            NeighborhoodShape::Moore,
            NeighborhoodRadius::new(1),
            NeighborhoodFalloff::Uniform,
        )])
        .rules([RuleSpec::when(M::Dead)
            .require(neighbors(M::Alive).count().eq(3))
            .becomes(M::Alive)])
        .build()
        .expect("test schema should build");

    let provider = CpuSolverProvider::new();
    let mut runtime = provider.build(Instance::new(4, 4, 4).with_seed(7), &spec);
    assert_eq!(runtime.solver().dims().width, 4);

    runtime.set(1, 1, 1, M::Alive.id());
    runtime.write_region(
        GridRegion::new([0, 0, 0], [2, 1, 1]),
        &[M::Other.id(), M::Alive.id()],
    );
    runtime
        .set_attr(A::Heat.id(), 1, 1, 1, AttributeValue::U8(4))
        .expect("runtime attribute writes should succeed");
    runtime.step();

    let snapshot = runtime.readback();
    assert_eq!(runtime.dims().width, 4);
    assert_eq!(snapshot.get([1, 0, 0]), Some(&M::Alive.id()));
    assert_eq!(
        runtime.read_region(GridRegion::new([0, 0, 0], [2, 1, 1])),
        vec![M::Other.id(), M::Alive.id()]
    );
    assert_eq!(runtime.get_attr(A::Heat.id(), 1, 1, 1), Ok(AttributeValue::U8(4)));
    assert_eq!(runtime.step_count(), 1);
}
