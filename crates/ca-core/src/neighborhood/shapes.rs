//! Built-in shape functions.

/// Moore: all cells within Chebyshev distance R (full cube).
///
/// R=1 → 26, R=2 → 124, R=3 → 342. Formula: `(2R+1)³ - 1`.
pub fn moore(_dx: i32, _dy: i32, _dz: i32, _radius: u32) -> bool {
    true
}

/// Von Neumann: cells within Manhattan distance R (diamond / octahedron).
///
/// R=1 → 6 (face-adjacent), R=2 → 24, R=3 → 62.
pub fn von_neumann(dx: i32, dy: i32, dz: i32, radius: u32) -> bool {
    (dx.unsigned_abs() + dy.unsigned_abs() + dz.unsigned_abs()) <= radius
}

/// Spherical: cells within Euclidean distance R (sphere).
///
/// R=1 → 6, R=2 → 32, R=3 → 122.
pub fn spherical(dx: i32, dy: i32, dz: i32, radius: u32) -> bool {
    ((dx * dx + dy * dy + dz * dz) as u32) <= radius * radius
}
