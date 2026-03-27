//! Gas-phase cellular-automaton movement.

use glam::IVec3;
use hyle_core::state::CHUNK_SIZE;
use hyle_core::props::PhaseState;
use hyle_core::voxel::Voxel;
use hyle_core::{VoxelAccess, MaterialAccess, DirtyTracker};

/// Simulate one tick of gas CA movement.
pub fn gas_flow_step(
    world: &mut impl VoxelAccess,
    materials: &impl MaterialAccess,
    dirty: &mut impl DirtyTracker,
) {
    let cs = CHUNK_SIZE as i32;

    let gas_positions: Vec<(i32, i32, i32)> = world
        .iter_voxels()
        .into_iter()
        .filter(|(_, _, _, v)| {
            !v.is_air() && materials.get_material(v.material_id).phase.state == PhaseState::Gas
        })
        .map(|(x, y, z, _)| (x, y, z))
        .collect();

    for (x, y, z) in gas_positions {
        let voxel = world.get_voxel(x, y, z);
        if voxel.is_air() {
            continue;
        }

        let above = y + 1;

        if world.get_voxel(x, above, z).is_air() {
            world.set_voxel_or_create(x, y,     z, Voxel::AIR);
            world.set_voxel_or_create(x, above, z, voxel);

            dirty.mark_dirty(IVec3::new(x.div_euclid(cs), y.div_euclid(cs),     z.div_euclid(cs)));
            dirty.mark_dirty(IVec3::new(x.div_euclid(cs), above.div_euclid(cs), z.div_euclid(cs)));
        } else {
            const LATERALS: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
            for (dx, dz) in LATERALS {
                let nx = x + dx;
                let nz = z + dz;
                if world.get_voxel(nx, y, nz).is_air() {
                    world.set_voxel_or_create(x,  y, z,  Voxel::AIR);
                    world.set_voxel_or_create(nx, y, nz, voxel);

                    dirty.mark_dirty(IVec3::new(x.div_euclid(cs),  y.div_euclid(cs), z.div_euclid(cs)));
                    dirty.mark_dirty(IVec3::new(nx.div_euclid(cs), y.div_euclid(cs), nz.div_euclid(cs)));
                    break;
                }
            }
        }
    }
}
