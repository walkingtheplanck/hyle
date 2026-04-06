//! Orbital camera with precomputed per-frame basis.

use glam::Vec3;

pub struct Camera {
    pub target: Vec3,
    pub distance: f32,
    pub yaw: f32,
    pub pitch: f32,
    pub fov_y: f32,
}

/// Precomputed camera frame — compute once, use for all pixels.
pub struct CameraFrame {
    pub eye: Vec3,
    pub forward: Vec3,
    pub right: Vec3,
    pub up: Vec3,
    pub half_w: f32,
    pub half_h: f32,
    pub inv_w: f32,
    pub inv_h: f32,
}

impl Camera {
    pub fn new(target: Vec3, distance: f32) -> Self {
        Self {
            target,
            distance,
            yaw: 0.6,
            pitch: 0.35,
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

    /// Horizontal forward direction (for panning).
    pub fn flat_forward(&self) -> Vec3 {
        let (sy, cy) = self.yaw.sin_cos();
        Vec3::new(-cy, 0.0, -sy).normalize()
    }

    /// Horizontal right direction (for panning).
    pub fn flat_right(&self) -> Vec3 {
        let (sy, cy) = self.yaw.sin_cos();
        Vec3::new(sy, 0.0, -cy).normalize()
    }

    /// Build the per-frame basis.
    pub fn frame(&self, width: u32, height: u32) -> CameraFrame {
        let eye = self.eye();

        let forward = (self.target - eye).normalize();
        let right = forward.cross(Vec3::Y).normalize();
        let up = right.cross(forward);

        let aspect = width as f32 / height as f32;
        let half_h = (self.fov_y * 0.5).tan();
        let half_w = half_h * aspect;

        CameraFrame {
            eye,
            forward,
            right,
            up,
            half_w,
            half_h,
            inv_w: 1.0 / width as f32,
            inv_h: 1.0 / height as f32,
        }
    }

    /// Orbit by pixel delta.
    pub fn rotate(&mut self, dx: f32, dy: f32) {
        self.yaw -= dx * 0.005;
        self.pitch = (self.pitch + dy * 0.005).clamp(-1.4, 1.4);
    }

    /// Pan the target in the camera's horizontal plane.
    pub fn pan(&mut self, dx: f32, dy: f32) {
        let speed = self.distance * 0.002;
        self.target += self.flat_right() * (dx * speed);
        self.target += self.flat_forward() * (dy * speed);
    }

    /// Zoom by scroll amount.
    pub fn zoom(&mut self, scroll: f32) {
        self.distance = (self.distance * (1.0 - scroll * 0.1)).clamp(3.0, 500.0);
    }
}
