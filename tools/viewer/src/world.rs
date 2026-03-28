//! A simple in-memory voxel world for the viewer.
//!
//! Flat `HashMap<(i32,i32,i32), Voxel>` — no chunking, no fancy storage.
//! Implements `VoxelAccess`, `MaterialAccess`, and `VoxelStateAccess` so
//! the CA rules and raytracer can operate on it directly.

use std::collections::HashMap;

use hyle_core::props::{MaterialDef, PhaseProps, PhaseState, VisualProps};
use hyle_core::state::VoxelState;
use hyle_core::voxel::{MaterialId, Voxel};
use hyle_core::{MaterialAccess, VoxelAccess, VoxelStateAccess};

/// Sparse voxel grid backed by a `HashMap`.
pub struct SimpleWorld {
    voxels: HashMap<(i32, i32, i32), Voxel>,
}

impl SimpleWorld {
    pub fn new() -> Self {
        Self {
            voxels: HashMap::new(),
        }
    }

    pub fn place(&mut self, x: i32, y: i32, z: i32, voxel: Voxel) {
        if voxel.is_air() {
            self.voxels.remove(&(x, y, z));
        } else {
            self.voxels.insert((x, y, z), voxel);
        }
    }
}

impl VoxelAccess for SimpleWorld {
    fn get_voxel(&self, x: i32, y: i32, z: i32) -> Voxel {
        self.voxels
            .get(&(x, y, z))
            .copied()
            .unwrap_or(Voxel::AIR)
    }

    fn set_voxel(&mut self, x: i32, y: i32, z: i32, voxel: Voxel) {
        self.place(x, y, z, voxel);
    }

    fn set_voxel_or_create(&mut self, x: i32, y: i32, z: i32, voxel: Voxel) {
        self.place(x, y, z, voxel);
    }

    fn is_valid(&self, _x: i32, _y: i32, _z: i32) -> bool {
        true
    }

    fn iter_voxels(&self) -> Vec<(i32, i32, i32, Voxel)> {
        self.voxels
            .iter()
            .map(|(&(x, y, z), &v)| (x, y, z, v))
            .collect()
    }
}

impl VoxelStateAccess for SimpleWorld {
    fn get_state(&self, _x: i32, _y: i32, _z: i32) -> VoxelState {
        VoxelState::default()
    }
}

// -- Material catalogue -------------------------------------------------------

/// Trivial material registry: a flat Vec indexed by `MaterialId`.
pub struct Materials {
    defs: Vec<MaterialDef>,
    bedrock: i32,
}

impl Materials {
    pub fn new(bedrock: i32) -> Self {
        // id 0 = air
        let air = MaterialDef {
            name: "air".into(),
            ..Default::default()
        };
        Self {
            defs: vec![air],
            bedrock,
        }
    }

    /// Register a material and return its id.
    pub fn register(&mut self, def: MaterialDef) -> MaterialId {
        let id = self.defs.len() as MaterialId;
        self.defs.push(def);
        id
    }
}

impl MaterialAccess for Materials {
    fn get_material(&self, id: MaterialId) -> &MaterialDef {
        &self.defs[id as usize]
    }

    fn bedrock_y(&self) -> i32 {
        self.bedrock
    }
}

// -- Scene builders -----------------------------------------------------------

/// Build a small demo scene: stone floor + sand pile.
pub fn demo_scene() -> (SimpleWorld, Materials) {
    let mut mats = Materials::new(-1);

    // 1 = stone (gray, solid)
    let stone_id = mats.register(MaterialDef {
        name: "stone".into(),
        visual: VisualProps {
            color: [0.45, 0.44, 0.42, 1.0],
            roughness: 0.9,
            ..Default::default()
        },
        ..Default::default()
    });

    // 2 = sand (tan, granular — affected by gravity)
    let sand_id = mats.register(MaterialDef {
        name: "sand".into(),
        visual: VisualProps {
            color: [0.86, 0.78, 0.55, 1.0],
            roughness: 1.0,
            ..Default::default()
        },
        phase: PhaseProps {
            state: PhaseState::Granular,
            repose_angle: 35.0,
            ..Default::default()
        },
        ..Default::default()
    });

    // 3 = water (blue, liquid)
    let water_id = mats.register(MaterialDef {
        name: "water".into(),
        visual: VisualProps {
            color: [0.2, 0.4, 0.8, 0.6],
            roughness: 0.1,
            transmittance: 0.5,
            ior: 1.33,
            ..Default::default()
        },
        phase: PhaseProps {
            state: PhaseState::Liquid,
            viscosity: 0.01,
            ..Default::default()
        },
        ..Default::default()
    });

    // 4 = grass (green, solid)
    let grass_id = mats.register(MaterialDef {
        name: "grass".into(),
        visual: VisualProps {
            color: [0.3, 0.6, 0.2, 1.0],
            roughness: 0.95,
            ..Default::default()
        },
        ..Default::default()
    });

    let mut world = SimpleWorld::new();

    // Stone floor: 32x32 at y=0
    for x in -16..16 {
        for z in -16..16 {
            world.place(x, 0, z, Voxel::new(stone_id));
        }
    }

    // Grass layer at y=1, except a pool area
    for x in -16..16 {
        for z in -16..16 {
            let in_pool = x >= 4 && x < 12 && z >= -8 && z < 0;
            if !in_pool {
                world.place(x, 1, z, Voxel::new(grass_id));
            }
        }
    }

    // Water pool at y=1
    for x in 4..12 {
        for z in -8..0 {
            world.place(x, 1, z, Voxel::new(water_id));
        }
    }

    // Sand pile (pyramid) centred at (-6, 2, -4)
    let cx = -6;
    let cz = -4;
    for layer in 0..4 {
        let y = 2 + layer;
        let radius = 4 - layer;
        for dx in -radius..=radius {
            for dz in -radius..=radius {
                world.place(cx + dx, y, cz + dz, Voxel::new(sand_id));
            }
        }
    }

    // Small stone wall
    for x in -14..-10 {
        for y in 1..5 {
            world.place(x, y, 10, Voxel::new(stone_id));
        }
    }

    (world, mats)
}
