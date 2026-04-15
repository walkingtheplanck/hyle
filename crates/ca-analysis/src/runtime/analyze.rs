//! Entry points for runtime report analysis.

use hyle_ca_interface::{MaterialId, StepReport};

use super::{MaterialPopulation, RuntimeReport};

/// Analyze one low-level step report and derive higher-level runtime counters.
pub fn analyze_step_report(step_report: &StepReport, alive_materials: &[MaterialId]) -> RuntimeReport {
    let mut born_cells = 0u64;
    let mut died_cells = 0u64;

    for transition in &step_report.transitions {
        let from_alive = alive_materials.contains(&transition.from);
        let to_alive = alive_materials.contains(&transition.to);

        match (from_alive, to_alive) {
            (false, true) => born_cells += transition.count,
            (true, false) => died_cells += transition.count,
            _ => {}
        }
    }

    RuntimeReport {
        step: step_report.step,
        total_cells: step_report.total_cells(),
        changed_cells: step_report.changed_cells,
        stable_cells: step_report.total_cells() - step_report.changed_cells,
        living_cells: alive_materials
            .iter()
            .map(|material| step_report.population(*material))
            .sum(),
        born_cells,
        died_cells,
        populations: step_report
            .populations
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
        transitions: step_report.transitions.clone(),
    }
}
