use super::{linear_index, Topology};

/// Coordinates wrap around each axis independently.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TorusTopology;

impl Topology for TorusTopology {
    fn resolve_index(
        &self,
        x: i32,
        y: i32,
        z: i32,
        width: u32,
        height: u32,
        depth: u32,
        guard_idx: usize,
    ) -> usize {
        if width == 0 || height == 0 || depth == 0 {
            return guard_idx;
        }

        let x = wrap_axis(x, width);
        let y = wrap_axis(y, height);
        let z = wrap_axis(z, depth);
        linear_index(x, y, z, width, height)
    }
}

#[inline]
fn wrap_axis(coord: i32, size: u32) -> u32 {
    let size = i64::from(size);
    i64::from(coord).rem_euclid(size) as u32
}
