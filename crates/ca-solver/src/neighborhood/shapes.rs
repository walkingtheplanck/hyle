//! Built-in shape functions for CPU neighborhood buffers.

/// Moore: all cells within Chebyshev distance R (full cube).
pub fn moore(_dx: i32, _dy: i32, _dz: i32, _radius: u32) -> bool {
    true
}

/// Von Neumann: cells within Manhattan distance R.
pub fn von_neumann(dx: i32, dy: i32, dz: i32, radius: u32) -> bool {
    (dx.unsigned_abs() + dy.unsigned_abs() + dz.unsigned_abs()) <= radius
}

/// Spherical: cells within Euclidean distance R.
pub fn spherical(dx: i32, dy: i32, dz: i32, radius: u32) -> bool {
    ((dx * dx + dy * dy + dz * dz) as u32) <= radius * radius
}
