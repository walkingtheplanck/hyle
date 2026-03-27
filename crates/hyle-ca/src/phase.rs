//! Temperature-driven phase transitions.

use glam::IVec3;
use hyle_core::state::CHUNK_SIZE;
use hyle_core::voxel::Voxel;
use hyle_core::{VoxelAccess, MaterialAccess, VoxelStateAccess, DirtyTracker};

/// Apply temperature-driven phase transitions for one tick.
pub fn phase_transition_step(
    world: &mut impl VoxelAccess,
    states: &impl VoxelStateAccess,
    materials: &impl MaterialAccess,
    dirty: &mut impl DirtyTracker,
) {
    let cs = CHUNK_SIZE as i32;

    let positions: Vec<(i32, i32, i32, Voxel)> = world.iter_voxels();

    for (x, y, z, voxel) in positions {
        let temp    = states.get_state(x, y, z).temperature;
        let thermal = &materials.get_material(voxel.material_id).thermal;

        let replacement: Option<Voxel> =
            if thermal.vaporisation_point.map_or(false, |t| temp >= t) {
                Some(Voxel::AIR)
            } else if thermal.melting_point.map_or(false, |t| temp >= t) {
                thermal.melt_product.map(Voxel::new)
            } else if thermal.freezing_point.map_or(false, |t| temp <= t) {
                thermal.freeze_product.map(Voxel::new)
            } else {
                None
            };

        if let Some(new_voxel) = replacement {
            world.set_voxel(x, y, z, new_voxel);
            dirty.mark_dirty(IVec3::new(
                x.div_euclid(cs),
                y.div_euclid(cs),
                z.div_euclid(cs),
            ));
        }
    }
}
