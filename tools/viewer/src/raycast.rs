//! DDA ray-voxel traversal with AABB early-out.
//!
//! 1. Intersect ray with world AABB — skip entirely on miss.
//! 2. Start DDA from the entry point (not the camera).
//! 3. Exit DDA as soon as the ray leaves the AABB — don't waste steps in void.

use glam::Vec3;
use hyle_core::voxel::Voxel;
use hyle_core::VoxelAccess;

use crate::world::Aabb;

/// Result of a successful ray–voxel intersection.
pub struct Hit {
    pub voxel: Voxel,
    pub pos: (i32, i32, i32),
    pub normal: Vec3,
}

// Precomputed axis normals to avoid Vec3 construction in the hot loop.
const NEG_X: Vec3 = Vec3::new(-1.0, 0.0, 0.0);
const POS_X: Vec3 = Vec3::new(1.0, 0.0, 0.0);
const NEG_Y: Vec3 = Vec3::new(0.0, -1.0, 0.0);
const POS_Y: Vec3 = Vec3::new(0.0, 1.0, 0.0);
const NEG_Z: Vec3 = Vec3::new(0.0, 0.0, -1.0);
const POS_Z: Vec3 = Vec3::new(0.0, 0.0, 1.0);

/// Slab-method ray-AABB intersection. Returns (t_enter, t_exit) or None.
#[inline]
fn intersect_aabb(origin: Vec3, inv_dir: Vec3, aabb: &Aabb) -> Option<(f32, f32)> {
    let (bmin_x, bmin_y, bmin_z) = aabb.min_f();
    let (bmax_x, bmax_y, bmax_z) = aabb.max_f();

    let t1x = (bmin_x - origin.x) * inv_dir.x;
    let t2x = (bmax_x - origin.x) * inv_dir.x;
    let t1y = (bmin_y - origin.y) * inv_dir.y;
    let t2y = (bmax_y - origin.y) * inv_dir.y;
    let t1z = (bmin_z - origin.z) * inv_dir.z;
    let t2z = (bmax_z - origin.z) * inv_dir.z;

    let t_near = t1x.min(t2x).max(t1y.min(t2y)).max(t1z.min(t2z));
    let t_far = t1x.max(t2x).min(t1y.max(t2y)).min(t1z.max(t2z));

    if t_near <= t_far && t_far >= 0.0 {
        Some((t_near.max(0.0), t_far))
    } else {
        None
    }
}

/// Cast a ray through `world` with AABB culling and bounded DDA.
#[inline]
pub fn cast_ray(
    world: &impl VoxelAccess,
    origin: Vec3,
    dir: Vec3,
    aabb: &Aabb,
    max_steps: u32,
) -> Option<Hit> {
    // Precompute inverse direction (one div per axis, not per step).
    let inv_dir = Vec3::new(
        if dir.x != 0.0 { 1.0 / dir.x } else { f32::INFINITY },
        if dir.y != 0.0 { 1.0 / dir.y } else { f32::INFINITY },
        if dir.z != 0.0 { 1.0 / dir.z } else { f32::INFINITY },
    );

    // AABB early-out: skip rays that miss the world entirely.
    let (t_enter, _t_exit) = intersect_aabb(origin, inv_dir, aabb)?;

    // Advance origin to the AABB entry point so DDA starts inside the volume.
    let start = if t_enter > 0.0 {
        origin + dir * (t_enter + 0.001)
    } else {
        origin
    };

    let mut x = start.x.floor() as i32;
    let mut y = start.y.floor() as i32;
    let mut z = start.z.floor() as i32;

    let step_x: i32 = if dir.x >= 0.0 { 1 } else { -1 };
    let step_y: i32 = if dir.y >= 0.0 { 1 } else { -1 };
    let step_z: i32 = if dir.z >= 0.0 { 1 } else { -1 };

    let t_delta_x = inv_dir.x.abs();
    let t_delta_y = inv_dir.y.abs();
    let t_delta_z = inv_dir.z.abs();

    let mut t_max_x = if dir.x != 0.0 {
        let boundary = if dir.x > 0.0 { (x + 1) as f32 } else { x as f32 };
        (boundary - start.x) * inv_dir.x
    } else {
        f32::INFINITY
    };
    let mut t_max_y = if dir.y != 0.0 {
        let boundary = if dir.y > 0.0 { (y + 1) as f32 } else { y as f32 };
        (boundary - start.y) * inv_dir.y
    } else {
        f32::INFINITY
    };
    let mut t_max_z = if dir.z != 0.0 {
        let boundary = if dir.z > 0.0 { (z + 1) as f32 } else { z as f32 };
        (boundary - start.z) * inv_dir.z
    } else {
        f32::INFINITY
    };

    // Select normal constants based on step direction (no per-step branching).
    let norm_x = if step_x > 0 { NEG_X } else { POS_X };
    let norm_y = if step_y > 0 { NEG_Y } else { POS_Y };
    let norm_z = if step_z > 0 { NEG_Z } else { POS_Z };

    let mut normal = Vec3::ZERO;

    for _ in 0..max_steps {
        // Bounds check: exit immediately if we've left the AABB.
        if x < aabb.min_x || x >= aabb.max_x
            || y < aabb.min_y || y >= aabb.max_y
            || z < aabb.min_z || z >= aabb.max_z
        {
            return None;
        }

        let voxel = world.get_voxel(x, y, z);
        if !voxel.is_air() {
            return Some(Hit { voxel, pos: (x, y, z), normal });
        }

        // Advance along the axis with the smallest t_max.
        if t_max_x < t_max_y {
            if t_max_x < t_max_z {
                x += step_x;
                t_max_x += t_delta_x;
                normal = norm_x;
            } else {
                z += step_z;
                t_max_z += t_delta_z;
                normal = norm_z;
            }
        } else if t_max_y < t_max_z {
            y += step_y;
            t_max_y += t_delta_y;
            normal = norm_y;
        } else {
            z += step_z;
            t_max_z += t_delta_z;
            normal = norm_z;
        }
    }

    None
}
