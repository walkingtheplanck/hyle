//! Minimal voxel world and material palette support for the viewer.

// -- Voxel --------------------------------------------------------------------

/// A voxel is just a material ID. 0 = air.
pub type Voxel = u16;

pub const AIR: Voxel = 0;

// -- Material -----------------------------------------------------------------

#[derive(Clone, Copy, Debug)]
pub struct Material {
    pub base_color: [f32; 4],     // linear RGBA
    pub emission_color: [f32; 3], // linear RGB
    pub emission_intensity: f32,
}

impl Material {
    pub const fn air() -> Self {
        Self {
            base_color: [0.0, 0.0, 0.0, 0.0],
            emission_color: [0.0, 0.0, 0.0],
            emission_intensity: 0.0,
        }
    }

    pub const fn solid(base_color: [f32; 4]) -> Self {
        Self {
            base_color,
            emission_color: [0.0, 0.0, 0.0],
            emission_intensity: 0.0,
        }
    }

    pub const fn glow(
        base_color: [f32; 4],
        emission_color: [f32; 3],
        emission_intensity: f32,
    ) -> Self {
        Self {
            base_color,
            emission_color,
            emission_intensity,
        }
    }
}

/// Material palette. Index by voxel value.
pub struct Materials {
    pub defs: Vec<Material>,
}

impl Materials {
    pub fn blank(count: usize) -> Self {
        Self {
            defs: vec![Material::air(); count.max(1)],
        }
    }

    pub fn set(&mut self, voxel: Voxel, material: Material) {
        let index = voxel as usize;
        if index >= self.defs.len() {
            self.defs.resize(index + 1, Material::air());
        }
        self.defs[index] = material;
    }

    /// Export as flat [f32; 8] palette for GPU buffer.
    /// Each entry: [r, g, b, a, emit_r, emit_g, emit_b, emit_intensity].
    pub fn export_palette(&self) -> Vec<[f32; 8]> {
        self.defs
            .iter()
            .map(|d| {
                [
                    d.base_color[0],
                    d.base_color[1],
                    d.base_color[2],
                    d.base_color[3],
                    d.emission_color[0] * d.emission_intensity,
                    d.emission_color[1] * d.emission_intensity,
                    d.emission_color[2] * d.emission_intensity,
                    d.emission_intensity,
                ]
            })
            .collect()
    }
}

// -- Aabb ---------------------------------------------------------------------

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
}

// -- SimpleWorld --------------------------------------------------------------

pub struct SimpleWorld {
    voxels: Vec<Voxel>,
    pub bounds: Aabb,
    stride_z: usize,
    stride_y: usize,
}

impl SimpleWorld {
    pub fn new(bounds: Aabb) -> Self {
        let sx = bounds.size_x();
        let sz = bounds.size_z();
        let total = sx * bounds.size_y() * sz;
        Self {
            voxels: vec![AIR; total],
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

    pub fn set(&mut self, x: i32, y: i32, z: i32, v: Voxel) {
        if self.bounds.contains(x, y, z) {
            let i = self.index(x, y, z);
            self.voxels[i] = v;
        }
    }

    /// Export material IDs as flat u16 array for GPU texture upload.
    pub fn export_material_ids(&self) -> Vec<u16> {
        self.voxels.clone()
    }
}

// -- World factory ------------------------------------------------------------

/// Create a 64×64×64 empty world ready for viewer scenarios.
pub fn viewer_world() -> SimpleWorld {
    let size = 64i32;
    let bounds = Aabb {
        min_x: 0,
        min_y: 0,
        min_z: 0,
        max_x: size,
        max_y: size,
        max_z: size,
    };
    SimpleWorld::new(bounds)
}
