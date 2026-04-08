//! Built-in weight functions.

/// Equal weight for all neighbors (1.0).
///
/// `weighted_sum()` equals `count_alive()` as a float.
pub fn unweighted(_dx: i32, _dy: i32, _dz: i32) -> f32 {
    1.0
}

/// Inverse square: `1 / (dx² + dy² + dz²)`.
///
/// Closer neighbors have more influence.
pub fn inverse_square(dx: i32, dy: i32, dz: i32) -> f32 {
    let d_sq = (dx * dx + dy * dy + dz * dz) as f32;
    1.0 / d_sq
}
