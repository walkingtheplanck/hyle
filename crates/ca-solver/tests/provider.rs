use hyle_ca_interface::{neighbors, CaRuntime, CaSolverProvider, Hyle, Instance};
use hyle_ca_solver::CpuSolverProvider;

#[test]
fn cpu_provider_builds_runtime_trait_object() {
    let spec = Hyle::builder()
        .cells::<u32>()
        .rules(|rules| {
            rules.when(0).require(neighbors(1).count().eq(3)).becomes(1);
            rules
                .when(1)
                .unless(neighbors(1).count().in_range(2..=3))
                .becomes(0);
        })
        .build()
        .expect("test blueprint should build");

    let provider = CpuSolverProvider::new();
    let mut runtime: Box<dyn CaRuntime<u32>> =
        provider.build(Instance::new(4, 4, 4).with_seed(7), &spec);

    runtime.set(1, 1, 1, 1);
    runtime.step();

    let snapshot = runtime.readback();
    assert_eq!(snapshot.dims.width, 4);
    assert_eq!(snapshot.dims.height, 4);
    assert_eq!(snapshot.dims.depth, 4);
    assert_eq!(runtime.step_count(), 1);
}
