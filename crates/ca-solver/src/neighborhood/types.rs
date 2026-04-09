//! Shared runtime types for CPU neighborhoods.

use hyle_ca_core::Cell;

/// Relative position of a neighbor to the center cell.
#[derive(Clone, Copy, Debug)]
pub struct Offset {
    /// X offset from center.
    pub dx: i32,
    /// Y offset from center.
    pub dy: i32,
    /// Z offset from center.
    pub dz: i32,
}

/// A single neighbor: its offset, cell value, and precomputed weight.
#[derive(Clone, Copy, Debug)]
pub struct Entry<C: Cell> {
    /// Position relative to the center cell.
    pub offset: Offset,
    /// The cell value at this offset.
    pub cell: C,
    /// Precomputed influence weight for this offset.
    pub weight: f32,
}

/// Shape function: returns whether offset `(dx, dy, dz)` is included at `radius`.
pub type ShapeFn = fn(dx: i32, dy: i32, dz: i32, radius: u32) -> bool;

/// Weight function: returns the influence weight for offset `(dx, dy, dz)`.
pub type WeightFn = fn(dx: i32, dy: i32, dz: i32) -> f32;
