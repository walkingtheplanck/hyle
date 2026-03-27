//! Granular-voxel gravity simulation.

use std::collections::HashSet;
use glam::IVec3;
use hyle_core::state::CHUNK_SIZE;
use hyle_core::props::PhaseState;
use hyle_core::voxel::Voxel;
use hyle_core::{VoxelAccess, MaterialAccess, DirtyTracker};

/// Move every granular voxel down by one if the cell directly below is air.
///
/// Marks changed chunks via `dirty`.
pub fn gravity_step(
    world: &mut impl VoxelAccess,
    materials: &impl MaterialAccess,
    dirty: &mut impl DirtyTracker,
) {
    let cs = CHUNK_SIZE as i32;

    let positions: Vec<(i32, i32, i32)> = world
        .iter_voxels()
        .into_iter()
        .filter(|(_, _, _, v)| {
            !v.is_air()
                && materials.get_material(v.material_id).phase.state == PhaseState::Granular
        })
        .map(|(x, y, z, _)| (x, y, z))
        .collect();

    for (x, y, z) in positions {
        let below = y - 1;
        if below < materials.bedrock_y() {
            continue;
        }
        if world.get_voxel(x, below, z).is_air() {
            let voxel = world.get_voxel(x, y, z);
            world.set_voxel_or_create(x, y,     z, Voxel::AIR);
            world.set_voxel_or_create(x, below, z, voxel);

            dirty.mark_dirty(IVec3::new(
                x.div_euclid(cs),
                y.div_euclid(cs),
                z.div_euclid(cs),
            ));
            dirty.mark_dirty(IVec3::new(
                x.div_euclid(cs),
                below.div_euclid(cs),
                z.div_euclid(cs),
            ));
        }
    }
}

/// Repeatedly apply gravity until the world is stable or `max_steps` is reached.
pub fn settle(
    world: &mut impl VoxelAccess,
    materials: &impl MaterialAccess,
    dirty: &mut impl DirtyTracker,
    max_steps: u32,
) {
    for _ in 0..max_steps {
        let mut step_dirty = HashSet::<IVec3>::new();
        gravity_step_into(world, materials, &mut step_dirty);
        if step_dirty.is_empty() {
            break;
        }
        for pos in step_dirty {
            dirty.mark_dirty(pos);
        }
    }
}

/// Internal gravity step that collects dirty into a HashSet for emptiness check.
fn gravity_step_into(
    world: &mut impl VoxelAccess,
    materials: &impl MaterialAccess,
    dirty: &mut HashSet<IVec3>,
) {
    let cs = CHUNK_SIZE as i32;

    let positions: Vec<(i32, i32, i32)> = world
        .iter_voxels()
        .into_iter()
        .filter(|(_, _, _, v)| {
            !v.is_air()
                && materials.get_material(v.material_id).phase.state == PhaseState::Granular
        })
        .map(|(x, y, z, _)| (x, y, z))
        .collect();

    for (x, y, z) in positions {
        let below = y - 1;
        if below < materials.bedrock_y() {
            continue;
        }
        if world.get_voxel(x, below, z).is_air() {
            let voxel = world.get_voxel(x, y, z);
            world.set_voxel_or_create(x, y,     z, Voxel::AIR);
            world.set_voxel_or_create(x, below, z, voxel);

            dirty.insert(IVec3::new(
                x.div_euclid(cs),
                y.div_euclid(cs),
                z.div_euclid(cs),
            ));
            dirty.insert(IVec3::new(
                x.div_euclid(cs),
                below.div_euclid(cs),
                z.div_euclid(cs),
            ));
        }
    }
}
