//! A simple in-memory voxel world for the viewer.
//!
//! Dense 3D array covering a fixed AABB — O(1) voxel lookups with zero
//! hashing overhead.  Implements `VoxelAccess` so the CA rules and
//! raytracer can operate on it directly.

use hyle_core::props::{MaterialDef, PhaseProps, PhaseState, VisualProps};
use hyle_core::state::VoxelState;
use hyle_core::voxel::{MaterialId, Voxel};
use hyle_core::{MaterialAccess, VoxelAccess, VoxelStateAccess};

// -- AABB ---------------------------------------------------------------------

/// Axis-aligned bounding box (inclusive min, exclusive max).
#[derive(Clone, Copy)]
pub struct Aabb {
    pub min_x: i32,
    pub min_y: i32,
    pub min_z: i32,
    pub max_x: i32,
    pub max_y: i32,
    pub max_z: i32,
}

impl Aabb {
    #[inline]
    pub fn contains(&self, x: i32, y: i32, z: i32) -> bool {
        x >= self.min_x
            && x < self.max_x
            && y >= self.min_y
            && y < self.max_y
            && z >= self.min_z
            && z < self.max_z
    }

    #[inline]
    pub fn size_x(&self) -> usize {
        (self.max_x - self.min_x) as usize
    }

    #[inline]
    pub fn size_y(&self) -> usize {
        (self.max_y - self.min_y) as usize
    }

    #[inline]
    pub fn size_z(&self) -> usize {
        (self.max_z - self.min_z) as usize
    }

    #[inline]
    #[allow(dead_code)]
    pub fn volume(&self) -> usize {
        self.size_x() * self.size_y() * self.size_z()
    }

    /// Return min/max as f32 tuples for ray-AABB intersection.
    #[inline]
    pub fn min_f(&self) -> (f32, f32, f32) {
        (self.min_x as f32, self.min_y as f32, self.min_z as f32)
    }

    #[inline]
    pub fn max_f(&self) -> (f32, f32, f32) {
        (self.max_x as f32, self.max_y as f32, self.max_z as f32)
    }
}

// -- SimpleWorld (dense array) ------------------------------------------------

/// Dense voxel grid backed by a flat `Vec<Voxel>`.
pub struct SimpleWorld {
    voxels: Vec<Voxel>,
    pub bounds: Aabb,
    stride_y: usize,
    stride_z: usize,
}

impl SimpleWorld {
    pub fn new(bounds: Aabb) -> Self {
        let sx = bounds.size_x();
        let sy = bounds.size_y();
        let sz = bounds.size_z();
        Self {
            voxels: vec![Voxel::AIR; sx * sy * sz],
            bounds,
            stride_z: sx,
            stride_y: sx * sz,
        }
    }

    #[inline]
    fn index(&self, x: i32, y: i32, z: i32) -> usize {
        let lx = (x - self.bounds.min_x) as usize;
        let ly = (y - self.bounds.min_y) as usize;
        let lz = (z - self.bounds.min_z) as usize;
        ly * self.stride_y + lz * self.stride_z + lx
    }

    pub fn place(&mut self, x: i32, y: i32, z: i32, voxel: Voxel) {
        if self.bounds.contains(x, y, z) {
            let idx = self.index(x, y, z);
            self.voxels[idx] = voxel;
        }
    }
}

impl VoxelAccess for SimpleWorld {
    #[inline]
    fn get_voxel(&self, x: i32, y: i32, z: i32) -> Voxel {
        if self.bounds.contains(x, y, z) {
            unsafe { *self.voxels.get_unchecked(self.index(x, y, z)) }
        } else {
            Voxel::AIR
        }
    }

    fn set_voxel(&mut self, x: i32, y: i32, z: i32, voxel: Voxel) {
        self.place(x, y, z, voxel);
    }

    fn set_voxel_or_create(&mut self, x: i32, y: i32, z: i32, voxel: Voxel) {
        self.place(x, y, z, voxel);
    }

    fn is_valid(&self, x: i32, y: i32, z: i32) -> bool {
        self.bounds.contains(x, y, z)
    }

    fn iter_voxels(&self) -> Vec<(i32, i32, i32, Voxel)> {
        let mut out = Vec::new();
        for y in self.bounds.min_y..self.bounds.max_y {
            for z in self.bounds.min_z..self.bounds.max_z {
                for x in self.bounds.min_x..self.bounds.max_x {
                    let v = self.get_voxel(x, y, z);
                    if !v.is_air() {
                        out.push((x, y, z, v));
                    }
                }
            }
        }
        out
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
        let air = MaterialDef {
            name: "air".into(),
            ..Default::default()
        };
        Self {
            defs: vec![air],
            bedrock,
        }
    }

    pub fn register(&mut self, def: MaterialDef) -> MaterialId {
        let id = self.defs.len() as MaterialId;
        self.defs.push(def);
        id
    }

    pub fn count(&self) -> usize {
        self.defs.len()
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

pub fn demo_scene() -> (SimpleWorld, Materials) {
    let mut mats = Materials::new(-1);

    let stone_id = mats.register(MaterialDef {
        name: "stone".into(),
        visual: VisualProps {
            color: [0.45, 0.44, 0.42, 1.0],
            roughness: 0.9,
            ..Default::default()
        },
        ..Default::default()
    });

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

    let grass_id = mats.register(MaterialDef {
        name: "grass".into(),
        visual: VisualProps {
            color: [0.3, 0.6, 0.2, 1.0],
            roughness: 0.95,
            ..Default::default()
        },
        ..Default::default()
    });

    let bounds = Aabb {
        min_x: -20,
        min_y: -2,
        min_z: -20,
        max_x: 20,
        max_y: 16,
        max_z: 20,
    };
    let mut world = SimpleWorld::new(bounds);

    // Stone floor: 32x32 at y=0
    for x in -16..16 {
        for z in -16..16 {
            world.place(x, 0, z, Voxel::new(stone_id));
        }
    }

    // Grass layer at y=1, except pool area
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

    // Sand pyramid at (-6, 2, -4)
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

    // Stone wall
    for x in -14..-10 {
        for y in 1..5 {
            world.place(x, y, 10, Voxel::new(stone_id));
        }
    }

    (world, mats)
}
