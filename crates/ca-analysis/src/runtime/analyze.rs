//! Entry points for runtime report analysis.

use hyle_ca_interface::{CaRuntime, MaterialId};

use super::{MaterialPopulation, RuntimeReport};

/// Analyze the latest completed runtime step and derive higher-level counters.
pub fn analyze_runtime<R: CaRuntime>(runtime: &R, alive_materials: &[MaterialId]) -> RuntimeReport {
    let step = runtime.step_count();
    let changed_cells = runtime.last_changed_cells();
    let populations = runtime.populations();
    let total_cells: u64 = populations.iter().sum();
    let transitions = runtime.last_transitions().to_vec();
    let mut born_cells = 0u64;
    let mut died_cells = 0u64;

    for transition in &transitions {
        let from_alive = alive_materials.contains(&transition.from);
        let to_alive = alive_materials.contains(&transition.to);

        match (from_alive, to_alive) {
            (false, true) => born_cells += transition.count,
            (true, false) => died_cells += transition.count,
            _ => {}
        }
    }

    RuntimeReport {
        step,
        total_cells,
        changed_cells,
        stable_cells: total_cells - changed_cells,
        living_cells: alive_materials
            .iter()
            .map(|material| runtime.population(*material))
            .sum(),
        born_cells,
        died_cells,
        populations: populations
            .iter()
            .enumerate()
            .filter_map(|(index, count)| {
                if *count == 0 {
                    None
                } else {
                    Some(MaterialPopulation {
                        material: MaterialId::new(index as u16),
                        count: *count,
                    })
                }
            })
            .collect(),
        transitions,
    }
}
