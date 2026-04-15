use hyle_ca_interface::{
    neighbors, AttributeType, AttributeValue, Blueprint, CaRuntime, CaSolverProvider, CellModel,
    CellSchema, GridRegion, Instance,
};
use hyle_ca_solver::CpuSolverProvider;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct TestCell(u32);

impl CellModel for TestCell {
    fn schema() -> CellSchema {
        CellSchema::opaque("TestCell")
    }
}

#[test]
fn cpu_provider_builds_runtime() {
    let spec = Blueprint::<TestCell>::builder()
        .attribute("heat", AttributeType::U8)
        .rules(|rules| {
            rules
                .when(TestCell(0))
                .require(neighbors(TestCell(1)).count().eq(3))
                .becomes(TestCell(1));
            rules
                .when(TestCell(1))
                .unless(neighbors(TestCell(1)).count().in_range(2..=3))
                .becomes(TestCell(0));
        })
        .build()
        .expect("test blueprint should build");

    let provider = CpuSolverProvider::new();
    let mut runtime = provider.build(Instance::new(4, 4, 4).with_seed(7), &spec);

    runtime.set(1, 1, 1, TestCell(1));
    runtime.write_region(
        GridRegion::new([0, 0, 0], [2, 1, 1]),
        &[TestCell(2), TestCell(3)],
    );
    runtime
        .set_attr("heat", 1, 1, 1, AttributeValue::U8(4))
        .expect("runtime attribute writes should succeed");
    runtime.step();

    let snapshot = runtime.readback();
    assert_eq!(runtime.dims().width, 4);
    assert_eq!(snapshot.get([1, 0, 0]), Some(&TestCell(3)));
    assert_eq!(
        runtime.read_region(GridRegion::new([0, 0, 0], [2, 1, 1])),
        vec![TestCell(2), TestCell(3)]
    );
    assert_eq!(runtime.get_attr("heat", 1, 1, 1), Ok(AttributeValue::U8(4)));
    assert_eq!(runtime.step_count(), 1);
}
