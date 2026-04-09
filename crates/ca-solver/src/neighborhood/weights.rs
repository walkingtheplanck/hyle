//! Built-in weight functions for CPU neighborhood buffers.

/// Equal weight for all neighbors (1.0).
pub fn unweighted(_dx: i32, _dy: i32, _dz: i32) -> f32 {
    1.0
}

/// Inverse square: `1 / (dx^2 + dy^2 + dz^2)`.
pub fn inverse_square(dx: i32, dy: i32, dz: i32) -> f32 {
    let d_sq = (dx * dx + dy * dy + dz * dz) as f32;
    1.0 / d_sq
}
