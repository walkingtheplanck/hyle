//! DDA (Digital Differential Analyzer) ray-voxel traversal.
//!
//! Steps through a uniform grid one cell at a time along a ray,
//! reporting the first non-air voxel hit.

use glam::Vec3;
use hyle_core::voxel::Voxel;
use hyle_core::VoxelAccess;

/// Result of a successful ray–voxel intersection.
pub struct Hit {
    /// The voxel that was hit.
    pub voxel: Voxel,
    /// World-space position of the hit cell.
    #[allow(dead_code)]
    pub pos: (i32, i32, i32),
    /// Face normal of the side that was hit (axis-aligned unit vector).
    pub normal: Vec3,
    /// Parametric distance along the ray.
    #[allow(dead_code)]
    pub t: f32,
}

/// Cast a ray through `world`, returning the first non-air hit within
/// `max_steps` grid traversals.
pub fn cast_ray(
    world: &impl VoxelAccess,
    origin: Vec3,
    dir: Vec3,
    max_steps: u32,
) -> Option<Hit> {
    // Current voxel coordinates
    let mut x = origin.x.floor() as i32;
    let mut y = origin.y.floor() as i32;
    let mut z = origin.z.floor() as i32;

    // Step direction (+1 or -1) per axis
    let step_x: i32 = if dir.x >= 0.0 { 1 } else { -1 };
    let step_y: i32 = if dir.y >= 0.0 { 1 } else { -1 };
    let step_z: i32 = if dir.z >= 0.0 { 1 } else { -1 };

    // How far along the ray (in t) to cross one full cell on each axis.
    // Infinite if the ray is parallel to that axis.
    let t_delta_x = if dir.x != 0.0 { (1.0 / dir.x).abs() } else { f32::INFINITY };
    let t_delta_y = if dir.y != 0.0 { (1.0 / dir.y).abs() } else { f32::INFINITY };
    let t_delta_z = if dir.z != 0.0 { (1.0 / dir.z).abs() } else { f32::INFINITY };

    // Distance to the *next* cell boundary on each axis.
    let mut t_max_x = if dir.x != 0.0 {
        let boundary = if dir.x > 0.0 { (x + 1) as f32 } else { x as f32 };
        (boundary - origin.x) / dir.x
    } else {
        f32::INFINITY
    };
    let mut t_max_y = if dir.y != 0.0 {
        let boundary = if dir.y > 0.0 { (y + 1) as f32 } else { y as f32 };
        (boundary - origin.y) / dir.y
    } else {
        f32::INFINITY
    };
    let mut t_max_z = if dir.z != 0.0 {
        let boundary = if dir.z > 0.0 { (z + 1) as f32 } else { z as f32 };
        (boundary - origin.z) / dir.z
    } else {
        f32::INFINITY
    };

    let mut normal = Vec3::ZERO;

    for _ in 0..max_steps {
        let voxel = world.get_voxel(x, y, z);
        if !voxel.is_air() {
            let t = t_max_x.min(t_max_y).min(t_max_z);
            return Some(Hit {
                voxel,
                pos: (x, y, z),
                normal,
                t,
            });
        }

        // Advance along whichever axis has the nearest boundary.
        if t_max_x < t_max_y {
            if t_max_x < t_max_z {
                x += step_x;
                t_max_x += t_delta_x;
                normal = Vec3::new(-step_x as f32, 0.0, 0.0);
            } else {
                z += step_z;
                t_max_z += t_delta_z;
                normal = Vec3::new(0.0, 0.0, -step_z as f32);
            }
        } else if t_max_y < t_max_z {
            y += step_y;
            t_max_y += t_delta_y;
            normal = Vec3::new(0.0, -step_y as f32, 0.0);
        } else {
            z += step_z;
            t_max_z += t_delta_z;
            normal = Vec3::new(0.0, 0.0, -step_z as f32);
        }
    }

    None
}
