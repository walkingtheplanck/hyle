use crate::{AxisTopology, GridDims, TopologyDescriptor};

/// Canonical interpreted topology derived from a declarative descriptor.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ResolvedTopology {
    descriptor: TopologyDescriptor,
}

impl ResolvedTopology {
    /// Construct an interpreted topology from a declarative descriptor.
    pub const fn from_descriptor(descriptor: TopologyDescriptor) -> Self {
        Self { descriptor }
    }

    /// Return the source descriptor.
    pub const fn descriptor(&self) -> TopologyDescriptor {
        self.descriptor
    }

    /// Resolve a 3D coordinate to a linear cell index.
    pub fn resolve_index(&self, x: i32, y: i32, z: i32, dims: GridDims, guard_idx: usize) -> usize {
        match (
            resolve_axis(x, dims.width, self.descriptor.x),
            resolve_axis(y, dims.height, self.descriptor.y),
            resolve_axis(z, dims.depth, self.descriptor.z),
        ) {
            (Some(x), Some(y), Some(z)) => linear_index(x, y, z, dims.width, dims.height),
            _ => guard_idx,
        }
    }
}

/// Interpret a declarative topology descriptor into its canonical semantic form.
pub const fn interpret_topology(descriptor: TopologyDescriptor) -> ResolvedTopology {
    ResolvedTopology::from_descriptor(descriptor)
}

fn resolve_axis(coord: i32, size: u32, topology: AxisTopology) -> Option<u32> {
    match topology {
        AxisTopology::Bounded => {
            let coord = coord as u32;
            let max_dim = i32::MAX as u32;
            if size <= max_dim && coord < size {
                Some(coord)
            } else {
                None
            }
        }
        AxisTopology::Wrap => {
            if size == 0 {
                None
            } else {
                Some(i64::from(coord).rem_euclid(i64::from(size)) as u32)
            }
        }
    }
}

#[inline]
fn linear_index(x: u32, y: u32, z: u32, width: u32, height: u32) -> usize {
    (x as usize)
        + (y as usize) * (width as usize)
        + (z as usize) * (width as usize) * (height as usize)
}
