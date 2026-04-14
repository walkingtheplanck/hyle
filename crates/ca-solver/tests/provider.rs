use hyle_ca_interface::{
    neighbors, CaRuntime, CaSolverProvider, Cell, CellModel, CellSchema, Hyle, Instance,
};
use hyle_ca_solver::CpuSolverProvider;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct TestCell(u32);

impl Cell for TestCell {
    fn rule_id(&self) -> u8 {
        self.0 as u8
    }

    fn is_alive(&self) -> bool {
        self.0 != 0
    }
}

impl CellModel for TestCell {
    fn schema() -> CellSchema {
        CellSchema::opaque("TestCell")
    }
}

#[test]
fn cpu_provider_builds_runtime_trait_object() {
    let spec = Hyle::builder()
        .cells::<TestCell>()
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
    let mut runtime: Box<dyn CaRuntime<TestCell>> =
        provider.build(Instance::new(4, 4, 4).with_seed(7), &spec);

    runtime.set(1, 1, 1, TestCell(1));
    runtime.step();

    let snapshot = runtime.readback();
    assert_eq!(snapshot.dims.width, 4);
    assert_eq!(snapshot.dims.height, 4);
    assert_eq!(snapshot.dims.depth, 4);
    assert_eq!(runtime.step_count(), 1);
}
