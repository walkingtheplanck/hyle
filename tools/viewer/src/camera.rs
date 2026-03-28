//! Orbital camera — yaw/pitch around a target, generates per-pixel rays.

use glam::Vec3;

pub struct Camera {
    /// Point the camera orbits around.
    pub target: Vec3,
    /// Distance from target.
    pub distance: f32,
    /// Horizontal angle (radians, 0 = +Z).
    pub yaw: f32,
    /// Vertical angle (radians, 0 = horizon).
    pub pitch: f32,
    /// Vertical field of view in radians.
    pub fov_y: f32,
}

impl Camera {
    pub fn new(target: Vec3, distance: f32) -> Self {
        Self {
            target,
            distance,
            yaw: 0.8,
            pitch: 0.5,
            fov_y: std::f32::consts::FRAC_PI_4,
        }
    }

    /// World-space eye position.
    pub fn eye(&self) -> Vec3 {
        let (sy, cy) = self.yaw.sin_cos();
        let (sp, cp) = self.pitch.sin_cos();
        self.target
            + Vec3::new(
                cy * cp * self.distance,
                sp * self.distance,
                sy * cp * self.distance,
            )
    }

    /// Generate a ray direction for pixel `(px, py)` in a viewport of
    /// `(width, height)`. Returns a normalised direction vector.
    pub fn ray_dir(&self, px: u32, py: u32, width: u32, height: u32) -> Vec3 {
        let aspect = width as f32 / height as f32;
        let half_h = (self.fov_y * 0.5).tan();
        let half_w = half_h * aspect;

        // NDC: [-1, 1]
        let ndc_x = (2.0 * (px as f32 + 0.5) / width as f32) - 1.0;
        let ndc_y = 1.0 - (2.0 * (py as f32 + 0.5) / height as f32);

        let eye = self.eye();
        let forward = (self.target - eye).normalize();
        let right = forward.cross(Vec3::Y).normalize();
        let up = right.cross(forward);

        (forward + right * (ndc_x * half_w) + up * (ndc_y * half_h)).normalize()
    }

    /// Rotate by mouse delta (pixels).
    pub fn rotate(&mut self, dx: f32, dy: f32) {
        let sensitivity = 0.005;
        self.yaw += dx * sensitivity;
        self.pitch = (self.pitch + dy * sensitivity).clamp(-1.4, 1.4);
    }

    /// Zoom by scroll amount.
    pub fn zoom(&mut self, scroll: f32) {
        self.distance = (self.distance - scroll * 2.0).clamp(3.0, 120.0);
    }
}
