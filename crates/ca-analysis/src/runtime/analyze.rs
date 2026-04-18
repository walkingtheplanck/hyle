//! Entry points for runtime report analysis.

use hyle_ca_interface::{CaRuntime, MaterialId};

use super::{
    AttributeView, CellReport, MaterialPopulation, MaterialView, NeighborhoodMaterialCount,
    NeighborhoodReport, RuntimeReport,
};

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

/// Analyze one selected cell position and derive a structured cell report.
pub fn analyze_cell<R: CaRuntime>(runtime: &R, position: [i32; 3]) -> Option<CellReport> {
    let cell = runtime.cell_at(position[0], position[1], position[2])?;
    let resolved_position = runtime.cell_position(cell).ok()?;
    let material_id = runtime.material(cell).ok()?;
    let material = runtime
        .material_defs()
        .iter()
        .find(|candidate| candidate.id == material_id)
        .map(|candidate| MaterialView {
            id: candidate.id,
            name: candidate.name,
        })
        .unwrap_or(MaterialView {
            id: material_id,
            name: "unknown",
        });
    let attributes = runtime
        .attributes(cell)
        .ok()?
        .into_iter()
        .map(|entry| {
            let attribute = runtime
                .attribute_defs()
                .iter()
                .find(|candidate| candidate.id == entry.attribute);
            AttributeView {
                id: entry.attribute,
                name: attribute.map_or("unknown", |attribute| attribute.name),
                value_type: attribute
                    .map_or(entry.value.value_type(), |attribute| attribute.value_type),
                value: entry.value,
            }
        })
        .collect();

    let neighborhoods = runtime
        .neighborhood_specs()
        .iter()
        .filter_map(|spec| {
            let neighbors = runtime.neighbors(cell, spec.id()).ok()?;
            let mut counts = std::collections::BTreeMap::<u16, u64>::new();

            for neighbor in &neighbors {
                let material = runtime.material(*neighbor).ok()?;
                *counts.entry(material.raw()).or_default() += 1;
            }

            Some(NeighborhoodReport {
                id: spec.id(),
                name: spec.name(),
                neighbor_count: neighbors.len(),
                materials: counts
                    .into_iter()
                    .map(|(raw, count)| {
                        let material = MaterialId::new(raw);
                        let name = runtime
                            .material_defs()
                            .iter()
                            .find(|candidate| candidate.id == material)
                            .map(|candidate| candidate.name)
                            .unwrap_or("unknown");
                        NeighborhoodMaterialCount {
                            material,
                            name,
                            count,
                        }
                    })
                    .collect(),
            })
        })
        .collect();

    Some(CellReport {
        requested_position: position,
        cell,
        resolved_position,
        material,
        attributes,
        neighborhoods,
    })
}
