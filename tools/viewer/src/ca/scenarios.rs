//! Viewer scenario registry and shared scenario types.

mod crystal_forge;
mod fire_cycle;
mod life_4555;
mod shared;
mod tube_garden;
mod weighted_bloom;

pub use shared::Scenario;
pub(crate) use shared::ViewerCell;

#[cfg(test)]
mod tests {
    use hyle_ca_interface::{CaSolverProvider, MaterialSet, RuntimeGrid, RuntimeStepping};
    use hyle_ca_solver::CpuSolverProvider;

    use super::{Scenario, ViewerCell};

    #[test]
    fn scenarios_build_seed_and_step() {
        let provider = CpuSolverProvider::new();

        for scenario in Scenario::ALL {
            let blueprint = scenario
                .blueprint()
                .expect("viewer scenarios should build successfully");
            let instance = scenario.instance();
            let mut runtime = provider.build(instance, &blueprint);

            scenario
                .seed(&mut runtime)
                .expect("viewer scenarios should seed successfully");
            runtime.step();

            let snapshot = runtime.readback();
            assert_eq!(snapshot.dims(), instance.dims());
            assert_eq!(snapshot.cells.len(), instance.dims().cell_count());
        }
    }

    #[test]
    fn palette_covers_every_viewer_cell() {
        let materials = Scenario::Life4555.materials();
        assert_eq!(materials.defs.len(), ViewerCell::palette_len());
    }

    #[test]
    fn alive_materials_resolve_ids() {
        for scenario in Scenario::ALL {
            let alive_materials = scenario
                .alive_materials()
                .iter()
                .map(|material| material.id())
                .collect::<Result<Vec<_>, _>>();

            assert!(
                alive_materials.is_ok(),
                "alive materials should resolve for {}",
                scenario.label()
            );
        }
    }
}
