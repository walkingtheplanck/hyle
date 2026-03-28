//! Simple directional + ambient shading.

use eframe::egui::Color32;
use glam::Vec3;
use hyle_core::MaterialAccess;

use crate::raycast::Hit;

/// Directional light.
const SUN_DIR: Vec3 = Vec3::new(0.48, 0.64, 0.6);
const SUN_R: f32 = 1.0;
const SUN_G: f32 = 0.96;
const SUN_B: f32 = 0.88;
const AMBIENT: f32 = 0.25;

/// Sky colour.
pub const SKY: Color32 = Color32::from_rgb(135, 186, 235);

/// Shade a hit → `Color32`.
#[inline]
pub fn shade(hit: &Hit, materials: &impl MaterialAccess) -> Color32 {
    let base = &materials.get_material(hit.voxel.material_id).visual.color;

    let n_dot_l = hit.normal.dot(SUN_DIR).max(0.0);
    let diffuse = AMBIENT + n_dot_l * (1.0 - AMBIENT);

    let r = (base[0] * diffuse * SUN_R).min(1.0);
    let g = (base[1] * diffuse * SUN_G).min(1.0);
    let b = (base[2] * diffuse * SUN_B).min(1.0);

    Color32::from_rgb(
        (r * 255.0) as u8,
        (g * 255.0) as u8,
        (b * 255.0) as u8,
    )
}
