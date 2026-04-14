//! Scenario definitions and shared viewer cell states.

use hyle_ca_interface::{
    neighbors, rng, AxisTopology, Blueprint, CaRuntime, CellModel, CellSchema, Instance,
    NeighborhoodFalloff, NeighborhoodShape, NeighborhoodSpec, Rng, StateDef, TopologyDescriptor,
    Weight,
};

use super::world::{Material, Materials};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
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

const VIEWER_CELL_STATES: [StateDef; 11] = [
    StateDef::new("Dead"),
    StateDef::new("Alive"),
    StateDef::new("Bloom"),
    StateDef::new("Crystal"),
    StateDef::new("Hot"),
    StateDef::new("Grass"),
    StateDef::new("Fire"),
    StateDef::new("Ember"),
    StateDef::new("Ash"),
    StateDef::new("Stone"),
    StateDef::new("Wall"),
];

impl ViewerCell {
    pub const fn voxel(self) -> u16 {
        self as u16
    }

    pub const fn palette_len() -> usize {
        VIEWER_CELL_STATES.len()
    }
}

impl CellModel for ViewerCell {
    fn schema() -> CellSchema {
        CellSchema::enumeration("ViewerCell", &VIEWER_CELL_STATES)
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
        Instance::new(64, 64, 64).with_seed(match self {
            Scenario::Life4555 => 1,
            Scenario::WeightedBloom => 2,
            Scenario::CrystalForge => 3,
            Scenario::FireCycle => 4,
            Scenario::TubeGarden => 5,
        })
    }

    pub fn blueprint(self) -> Blueprint<ViewerCell> {
        match self {
            Scenario::Life4555 => life_blueprint(),
            Scenario::WeightedBloom => weighted_bloom_blueprint(),
            Scenario::CrystalForge => crystal_forge_blueprint(),
            Scenario::FireCycle => fire_cycle_blueprint(),
            Scenario::TubeGarden => tube_garden_blueprint(),
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

    pub fn seed(self, ca: &mut impl CaRuntime<ViewerCell>) {
        match self {
            Scenario::Life4555 => seed_life(ca),
            Scenario::WeightedBloom => seed_weighted_bloom(ca),
            Scenario::CrystalForge => seed_crystal_forge(ca),
            Scenario::FireCycle => seed_fire_cycle(ca),
            Scenario::TubeGarden => seed_tube_garden(ca),
        }
    }
}

fn life_blueprint() -> Blueprint<ViewerCell> {
    Blueprint::<ViewerCell>::builder()
        .topology(TopologyDescriptor::wrap())
        .rules(|rules| {
            rules
                .when(ViewerCell::Alive)
                .unless(neighbors(ViewerCell::Alive).count().in_range(4..=5))
                .becomes(ViewerCell::Dead);
            rules
                .when(ViewerCell::Dead)
                .require(neighbors(ViewerCell::Alive).count().eq(5))
                .becomes(ViewerCell::Alive);
        })
        .build()
        .expect("life blueprint should build")
}

fn weighted_bloom_blueprint() -> Blueprint<ViewerCell> {
    Blueprint::<ViewerCell>::builder()
        .neighborhood(
            "field",
            NeighborhoodSpec::new(
                NeighborhoodShape::Spherical,
                4,
                NeighborhoodFalloff::InverseSquare,
            ),
        )
        .rules(|rules| {
            rules
                .when(ViewerCell::Bloom)
                .using("field")
                .unless(
                    neighbors(ViewerCell::Bloom)
                        .weighted_sum()
                        .in_range(Weight::cells(2)..=Weight::cells(7)),
                )
                .becomes(ViewerCell::Dead);
            rules
                .when(ViewerCell::Dead)
                .using("field")
                .require(
                    neighbors(ViewerCell::Bloom)
                        .weighted_sum()
                        .in_range(Weight::cells(3)..=Weight::cells(5)),
                )
                .require(rng(0).one_in(3))
                .becomes(ViewerCell::Bloom);
            rules
                .when(ViewerCell::Dead)
                .require(neighbors(ViewerCell::Bloom).count().eq(4))
                .becomes(ViewerCell::Bloom);
        })
        .build()
        .expect("weighted bloom blueprint should build")
}

fn crystal_forge_blueprint() -> Blueprint<ViewerCell> {
    Blueprint::<ViewerCell>::builder()
        .neighborhood(
            "heat",
            NeighborhoodSpec::new(
                NeighborhoodShape::Spherical,
                3,
                NeighborhoodFalloff::InverseSquare,
            ),
        )
        .rules(|rules| {
            rules
                .when(ViewerCell::Hot)
                .using("heat")
                .unless(
                    neighbors(ViewerCell::Hot)
                        .weighted_sum()
                        .at_least(Weight::cells(2)),
                )
                .becomes(ViewerCell::Crystal);
            rules
                .when(ViewerCell::Crystal)
                .using("heat")
                .require(
                    neighbors(ViewerCell::Hot)
                        .weighted_sum()
                        .at_least(Weight::cells(2)),
                )
                .becomes(ViewerCell::Hot);
            rules
                .when(ViewerCell::Dead)
                .require(neighbors(ViewerCell::Crystal).count().in_range(2..=3))
                .require(rng(1).one_in(4))
                .becomes(ViewerCell::Crystal);
            rules
                .when(ViewerCell::Dead)
                .using("heat")
                .require(
                    neighbors(ViewerCell::Hot)
                        .weighted_sum()
                        .at_least(Weight::cells(3)),
                )
                .becomes(ViewerCell::Hot);
        })
        .build()
        .expect("crystal forge blueprint should build")
}

fn fire_cycle_blueprint() -> Blueprint<ViewerCell> {
    Blueprint::<ViewerCell>::builder()
        .neighborhood(
            "ember-field",
            NeighborhoodSpec::new(
                NeighborhoodShape::Spherical,
                2,
                NeighborhoodFalloff::InverseSquare,
            ),
        )
        .rules(|rules| {
            rules.when(ViewerCell::Stone).keep();
            rules.when(ViewerCell::Fire).becomes(ViewerCell::Ember);
            rules
                .when(ViewerCell::Ember)
                .require(rng(1).one_in(2))
                .becomes(ViewerCell::Ash);
            rules
                .when(ViewerCell::Grass)
                .require(neighbors(ViewerCell::Fire).any())
                .becomes(ViewerCell::Fire);
            rules
                .when(ViewerCell::Grass)
                .using("ember-field")
                .require(
                    neighbors(ViewerCell::Ember)
                        .weighted_sum()
                        .at_least(Weight::cells(1)),
                )
                .require(rng(2).one_in(2))
                .becomes(ViewerCell::Fire);
            rules
                .when(ViewerCell::Ash)
                .require(neighbors(ViewerCell::Grass).count().at_least(2))
                .require(rng(3).one_in(6))
                .becomes(ViewerCell::Grass);
            rules
                .when(ViewerCell::Dead)
                .require(neighbors(ViewerCell::Grass).count().at_least(3))
                .require(neighbors(ViewerCell::Fire).count().eq(0))
                .require(rng(4).one_in(8))
                .becomes(ViewerCell::Grass);
        })
        .build()
        .expect("fire cycle blueprint should build")
}

fn tube_garden_blueprint() -> Blueprint<ViewerCell> {
    Blueprint::<ViewerCell>::builder()
        .topology(TopologyDescriptor::by_axis(
            AxisTopology::Wrap,
            AxisTopology::Bounded,
            AxisTopology::Wrap,
        ))
        .neighborhood(
            "flow",
            NeighborhoodSpec::new(
                NeighborhoodShape::VonNeumann,
                1,
                NeighborhoodFalloff::Uniform,
            ),
        )
        .neighborhood("support", NeighborhoodSpec::adjacent())
        .default_neighborhood("flow")
        .rules(|rules| {
            rules.when(ViewerCell::Wall).keep();
            rules
                .when(ViewerCell::Alive)
                .using("support")
                .require(neighbors(ViewerCell::Wall).count().eq(0))
                .becomes(ViewerCell::Dead);
            rules
                .when(ViewerCell::Alive)
                .unless(neighbors(ViewerCell::Alive).count().in_range(1..=2))
                .becomes(ViewerCell::Dead);
            rules
                .when(ViewerCell::Dead)
                .using("support")
                .require(neighbors(ViewerCell::Wall).count().at_least(3))
                .require(rng(5).one_in(5))
                .becomes(ViewerCell::Alive);
            rules
                .when(ViewerCell::Dead)
                .require(neighbors(ViewerCell::Alive).count().eq(2))
                .becomes(ViewerCell::Alive);
        })
        .build()
        .expect("tube garden blueprint should build")
}

fn seed_life(ca: &mut impl CaRuntime<ViewerCell>) {
    seed_random_box(ca, 24..40, 24..40, 24..40, ViewerCell::Alive, 6, 11, 0);
}

fn seed_weighted_bloom(ca: &mut impl CaRuntime<ViewerCell>) {
    fill_sphere(ca, [20, 22, 20], 3, ViewerCell::Bloom);
    fill_sphere(ca, [42, 30, 40], 3, ViewerCell::Bloom);
    fill_sphere(ca, [30, 44, 28], 2, ViewerCell::Bloom);
    seed_random_box(ca, 16..48, 16..48, 16..48, ViewerCell::Bloom, 28, 19, 1);
}

fn seed_crystal_forge(ca: &mut impl CaRuntime<ViewerCell>) {
    fill_sphere(ca, [32, 32, 32], 3, ViewerCell::Crystal);
    fill_sphere(ca, [32, 32, 32], 1, ViewerCell::Hot);
    fill_sphere(ca, [20, 24, 20], 2, ViewerCell::Crystal);
    fill_sphere(ca, [44, 40, 44], 2, ViewerCell::Crystal);
    seed_random_box(ca, 18..46, 18..46, 18..46, ViewerCell::Crystal, 32, 23, 2);
}

fn seed_fire_cycle(ca: &mut impl CaRuntime<ViewerCell>) {
    fill_box(ca, 8..56, 6..30, 8..56, ViewerCell::Grass);
    fill_sphere(ca, [20, 16, 20], 4, ViewerCell::Stone);
    fill_sphere(ca, [42, 14, 38], 3, ViewerCell::Stone);
    fill_sphere(ca, [28, 18, 44], 3, ViewerCell::Stone);
    fill_sphere(ca, [16, 12, 16], 2, ViewerCell::Fire);
    fill_sphere(ca, [48, 14, 44], 2, ViewerCell::Fire);
    seed_random_box(ca, 10..54, 6..24, 10..54, ViewerCell::Grass, 18, 29, 3);
}

fn seed_tube_garden(ca: &mut impl CaRuntime<ViewerCell>) {
    for x in (8..56).step_by(16) {
        for z in (8..56).step_by(16) {
            for y in 6..58 {
                ca.set(x, y, z, ViewerCell::Wall);
            }
        }
    }

    seed_random_box(ca, 6..58, 10..54, 6..58, ViewerCell::Alive, 20, 31, 4);
}

fn fill_box(
    ca: &mut impl CaRuntime<ViewerCell>,
    x_range: std::ops::Range<i32>,
    y_range: std::ops::Range<i32>,
    z_range: std::ops::Range<i32>,
    cell: ViewerCell,
) {
    for z in z_range.clone() {
        for y in y_range.clone() {
            for x in x_range.clone() {
                ca.set(x, y, z, cell);
            }
        }
    }
}

fn fill_sphere(
    ca: &mut impl CaRuntime<ViewerCell>,
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
                    ca.set(x, y, z, cell);
                }
            }
        }
    }
}

fn seed_random_box(
    ca: &mut impl CaRuntime<ViewerCell>,
    x_range: std::ops::Range<i32>,
    y_range: std::ops::Range<i32>,
    z_range: std::ops::Range<i32>,
    cell: ViewerCell,
    chance: u32,
    seed: u64,
    stream: u32,
) {
    for z in z_range.clone() {
        for y in y_range.clone() {
            for x in x_range.clone() {
                let rng = Rng::with_stream_and_seed(x as u32, y as u32, z as u32, 0, stream, seed);
                if rng.chance(chance) {
                    ca.set(x, y, z, cell);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use hyle_ca_interface::{CaRuntime, CaSolverProvider};
    use hyle_ca_solver::CpuSolverProvider;

    use super::{Scenario, ViewerCell};

    #[test]
    fn scenarios_build_seed_and_step() {
        let provider = CpuSolverProvider::new();

        for scenario in Scenario::ALL {
            let blueprint = scenario.blueprint();
            let instance = scenario.instance();
            let mut runtime = provider.build(instance, &blueprint);

            scenario.seed(&mut runtime);
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
}
