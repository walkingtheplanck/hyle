//! Shared viewer material definitions, palette, and seeding helpers.

use hyle_ca_interface::{
    Blueprint, CaRuntime, GridDims, Instance, MaterialId, MaterialSet, Rng, RngStreamId,
};

use crate::ca::world::{Material, Materials};

use super::{crystal_forge, fire_cycle, life_4555, tube_garden, weighted_bloom};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, MaterialSet)]
#[repr(u16)]
pub(crate) enum ViewerCell {
    #[default]
    Dead = 0,
    Alive = 1,
    Bloom = 2,
    Crystal = 3,
    Hot = 4,
    Grass = 5,
    Fire = 6,
    Ember = 7,
    Ash = 8,
    Stone = 9,
    Wall = 10,
}

impl ViewerCell {
    pub const fn voxel(self) -> u16 {
        self as u16
    }

    pub fn from_material_id(material: MaterialId) -> Option<Self> {
        <Self as MaterialSet>::variants()
            .get(material.index())
            .copied()
    }

    pub fn palette_len() -> usize {
        <Self as MaterialSet>::variants().len()
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Scenario {
    #[default]
    Life4555,
    WeightedBloom,
    CrystalForge,
    FireCycle,
    TubeGarden,
}

impl Scenario {
    pub const ALL: [Scenario; 5] = [
        Scenario::Life4555,
        Scenario::WeightedBloom,
        Scenario::CrystalForge,
        Scenario::FireCycle,
        Scenario::TubeGarden,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Scenario::Life4555 => "Life 4555",
            Scenario::WeightedBloom => "Weighted Bloom",
            Scenario::CrystalForge => "Crystal Forge",
            Scenario::FireCycle => "Fire Cycle",
            Scenario::TubeGarden => "Tube Garden",
        }
    }

    pub const fn description(self) -> &'static str {
        match self {
            Scenario::Life4555 => "Classic wrapped 3D life with a Moore neighborhood.",
            Scenario::WeightedBloom => {
                "Inverse-square weighted growth driven by a soft spherical field."
            }
            Scenario::CrystalForge => {
                "Two-phase crystal growth with hot cores and weighted heat spread."
            }
            Scenario::FireCycle => "A stochastic grass, fire, ember, ash, and stone ecology.",
            Scenario::TubeGarden => {
                "Mixed-axis topology with von Neumann flow around wall scaffolds."
            }
        }
    }

    pub const fn step_interval_ms(self) -> f64 {
        match self {
            Scenario::Life4555 => 180.0,
            Scenario::WeightedBloom => 220.0,
            Scenario::CrystalForge => 200.0,
            Scenario::FireCycle => 140.0,
            Scenario::TubeGarden => 180.0,
        }
    }

    pub const fn instance(self) -> Instance {
        Instance::from_dims(GridDims::from_validated(64, 64, 64, 64 * 64 * 64)).with_seed(
            match self {
                Scenario::Life4555 => 1,
                Scenario::WeightedBloom => 2,
                Scenario::CrystalForge => 3,
                Scenario::FireCycle => 4,
                Scenario::TubeGarden => 5,
            },
        )
    }

    pub fn blueprint(self) -> Blueprint {
        match self {
            Scenario::Life4555 => life_4555::blueprint(),
            Scenario::WeightedBloom => weighted_bloom::blueprint(),
            Scenario::CrystalForge => crystal_forge::blueprint(),
            Scenario::FireCycle => fire_cycle::blueprint(),
            Scenario::TubeGarden => tube_garden::blueprint(),
        }
    }

    pub fn materials(self) -> Materials {
        let mut materials = Materials::blank(ViewerCell::palette_len());
        materials.set(
            ViewerCell::Alive.voxel(),
            Material::glow([1.0, 0.62, 0.18, 1.0], [1.0, 0.45, 0.1], 0.28),
        );
        materials.set(
            ViewerCell::Bloom.voxel(),
            Material::glow([0.38, 0.95, 0.65, 1.0], [0.2, 0.9, 0.55], 0.18),
        );
        materials.set(
            ViewerCell::Crystal.voxel(),
            Material::solid([0.62, 0.88, 1.0, 1.0]),
        );
        materials.set(
            ViewerCell::Hot.voxel(),
            Material::glow([1.0, 0.82, 0.32, 1.0], [1.0, 0.72, 0.18], 0.5),
        );
        materials.set(
            ViewerCell::Grass.voxel(),
            Material::solid([0.28, 0.72, 0.24, 1.0]),
        );
        materials.set(
            ViewerCell::Fire.voxel(),
            Material::glow([1.0, 0.45, 0.08, 1.0], [1.0, 0.3, 0.02], 0.75),
        );
        materials.set(
            ViewerCell::Ember.voxel(),
            Material::glow([0.92, 0.2, 0.08, 1.0], [1.0, 0.12, 0.04], 0.35),
        );
        materials.set(
            ViewerCell::Ash.voxel(),
            Material::solid([0.48, 0.48, 0.5, 1.0]),
        );
        materials.set(
            ViewerCell::Stone.voxel(),
            Material::solid([0.32, 0.34, 0.4, 1.0]),
        );
        materials.set(
            ViewerCell::Wall.voxel(),
            Material::solid([0.22, 0.45, 0.82, 1.0]),
        );
        materials
    }

    pub fn alive_materials(self) -> &'static [ViewerCell] {
        match self {
            Scenario::Life4555 => &[ViewerCell::Alive],
            Scenario::WeightedBloom => &[ViewerCell::Bloom],
            Scenario::CrystalForge => &[ViewerCell::Crystal, ViewerCell::Hot],
            Scenario::FireCycle => &[ViewerCell::Grass, ViewerCell::Fire, ViewerCell::Ember],
            Scenario::TubeGarden => &[ViewerCell::Alive],
        }
    }

    pub fn seed(self, ca: &mut impl CaRuntime) {
        match self {
            Scenario::Life4555 => life_4555::seed(ca),
            Scenario::WeightedBloom => weighted_bloom::seed(ca),
            Scenario::CrystalForge => crystal_forge::seed(ca),
            Scenario::FireCycle => fire_cycle::seed(ca),
            Scenario::TubeGarden => tube_garden::seed(ca),
        }
    }
}

pub(super) fn fill_box(
    ca: &mut impl CaRuntime,
    x_range: std::ops::Range<i32>,
    y_range: std::ops::Range<i32>,
    z_range: std::ops::Range<i32>,
    cell: ViewerCell,
) {
    for z in z_range.clone() {
        for y in y_range.clone() {
            for x in x_range.clone() {
                ca.set(x, y, z, cell.id());
            }
        }
    }
}

pub(super) fn fill_sphere(
    ca: &mut impl CaRuntime,
    center: [i32; 3],
    radius: i32,
    cell: ViewerCell,
) {
    let [cx, cy, cz] = center;
    let radius_sq = radius * radius;
    for z in (cz - radius)..=(cz + radius) {
        for y in (cy - radius)..=(cy + radius) {
            for x in (cx - radius)..=(cx + radius) {
                let dx = x - cx;
                let dy = y - cy;
                let dz = z - cz;
                if dx * dx + dy * dy + dz * dz <= radius_sq {
                    ca.set(x, y, z, cell.id());
                }
            }
        }
    }
}

pub(super) fn seed_random_box(
    ca: &mut impl CaRuntime,
    x_range: std::ops::Range<i32>,
    y_range: std::ops::Range<i32>,
    z_range: std::ops::Range<i32>,
    cell: ViewerCell,
    chance: u32,
    seed: u64,
    stream: impl Into<RngStreamId>,
) {
    let stream = stream.into();
    for z in z_range.clone() {
        for y in y_range.clone() {
            for x in x_range.clone() {
                let rng = Rng::with_stream_and_seed(x as u32, y as u32, z as u32, 0, stream, seed);
                if rng.chance(chance) {
                    ca.set(x, y, z, cell.id());
                }
            }
        }
    }
}
