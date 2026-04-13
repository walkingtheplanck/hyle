/// Topology descriptor for device upload and serialization.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TopologyDescriptor {
    /// X axis behavior.
    pub x: AxisTopology,
    /// Y axis behavior.
    pub y: AxisTopology,
    /// Z axis behavior.
    pub z: AxisTopology,
}

impl TopologyDescriptor {
    /// Construct a descriptor for the three axes independently.
    pub const fn new(x: AxisTopology, y: AxisTopology, z: AxisTopology) -> Self {
        Self { x, y, z }
    }

    /// Construct a descriptor where all axes share the same behavior.
    pub const fn uniform(axis: AxisTopology) -> Self {
        Self {
            x: axis,
            y: axis,
            z: axis,
        }
    }

    /// Construct a bounded descriptor on all axes.
    pub const fn bounded() -> Self {
        Self::uniform(AxisTopology::Bounded)
    }

    /// Construct a wrapping descriptor on all axes.
    pub const fn wrap() -> Self {
        Self::uniform(AxisTopology::Wrap)
    }

    /// Construct a descriptor with per-axis behavior.
    pub const fn by_axis(x: AxisTopology, y: AxisTopology, z: AxisTopology) -> Self {
        Self::new(x, y, z)
    }
}

/// Boundary behavior for a single grid axis.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AxisTopology {
    /// Out-of-range coordinates have no mapped cell.
    #[default]
    Bounded,
    /// Out-of-range coordinates wrap around this axis.
    Wrap,
}
