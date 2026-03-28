//! Simple directional + ambient shading.

use glam::Vec3;
use hyle_core::MaterialAccess;

use crate::raycast::Hit;

/// Sky / miss colour.
const SKY: [f32; 3] = [0.53, 0.73, 0.92];

/// Directional light (normalised).
const SUN_DIR: Vec3 = Vec3::new(0.48, 0.64, 0.6);
const SUN_COLOR: [f32; 3] = [1.0, 0.96, 0.88];
const AMBIENT: f32 = 0.25;

/// Shade a hit and return a packed 0xRRGGBB `u32`.
pub fn shade(hit: &Hit, materials: &impl MaterialAccess) -> u32 {
    let mat = materials.get_material(hit.voxel.material_id);
    let base = &mat.visual.color;

    // Lambertian diffuse
    let n_dot_l = hit.normal.dot(SUN_DIR).max(0.0);
    let diffuse = AMBIENT + n_dot_l * (1.0 - AMBIENT);

    let r = (base[0] * diffuse * SUN_COLOR[0]).min(1.0);
    let g = (base[1] * diffuse * SUN_COLOR[1]).min(1.0);
    let b = (base[2] * diffuse * SUN_COLOR[2]).min(1.0);

    pack_rgb(r, g, b)
}

/// Return the sky colour as packed 0xRRGGBB.
pub fn sky_color() -> u32 {
    pack_rgb(SKY[0], SKY[1], SKY[2])
}

fn pack_rgb(r: f32, g: f32, b: f32) -> u32 {
    let ri = (r * 255.0) as u32;
    let gi = (g * 255.0) as u32;
    let bi = (b * 255.0) as u32;
    (ri << 16) | (gi << 8) | bi
}
