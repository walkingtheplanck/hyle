//! Voxel editing tools — place, delete, inspect.

use hyle_core::voxel::{MaterialId, Voxel};
use hyle_core::VoxelAccess;

use crate::raycast::Hit;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Tool {
    Place,
    Delete,
    Inspect,
}

impl Tool {
    pub fn label(self) -> &'static str {
        match self {
            Tool::Place => "Place",
            Tool::Delete => "Delete",
            Tool::Inspect => "Inspect",
        }
    }

    #[allow(dead_code)]
    pub fn hint(self) -> &'static str {
        match self {
            Tool::Place => "Left-click to place selected material",
            Tool::Delete => "Left-click to remove voxel",
            Tool::Inspect => "Hover to inspect voxel properties",
        }
    }
}

/// Info about the voxel under the cursor.
pub struct HoverInfo {
    pub world_pos: (i32, i32, i32),
    #[allow(dead_code)]
    pub place_pos: (i32, i32, i32),
    #[allow(dead_code)]
    pub voxel: Voxel,
    pub material_name: String,
}

impl HoverInfo {
    pub fn from_hit(hit: &Hit, materials: &impl hyle_core::MaterialAccess) -> Self {
        let (x, y, z) = hit.pos;
        let n = hit.normal;
        Self {
            world_pos: (x, y, z),
            place_pos: (
                x + n.x as i32,
                y + n.y as i32,
                z + n.z as i32,
            ),
            voxel: hit.voxel,
            material_name: materials
                .get_material(hit.voxel.material_id)
                .name
                .clone(),
        }
    }
}

/// Apply the active tool at the given hit location.
pub fn apply_tool(
    tool: Tool,
    hit: &Hit,
    world: &mut impl VoxelAccess,
    selected_material: MaterialId,
) {
    let (x, y, z) = hit.pos;
    let n = hit.normal;

    match tool {
        Tool::Place => {
            let px = x + n.x as i32;
            let py = y + n.y as i32;
            let pz = z + n.z as i32;
            world.set_voxel_or_create(px, py, pz, Voxel::new(selected_material));
        }
        Tool::Delete => {
            world.set_voxel(x, y, z, Voxel::AIR);
        }
        Tool::Inspect => {
            // No mutation — info is shown in hover_info.
        }
    }
}
