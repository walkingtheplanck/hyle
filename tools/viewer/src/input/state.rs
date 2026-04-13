//! Input handling — keyboard shortcuts and mouse interaction.

use eframe::egui;

use crate::rendering::Camera;

/// Tracks mouse capture state for orbit mode.
pub struct InputState {
    pub captured: bool,
    prev_capture_pos: Option<egui::Pos2>,
}

impl InputState {
    pub fn new() -> Self {
        Self {
            captured: false,
            prev_capture_pos: None,
        }
    }

    /// Read keyboard input and return what actions should be taken.
    pub fn handle_keyboard(&mut self, ctx: &egui::Context, camera: &mut Camera) -> KeyboardActions {
        let input = ctx.input(|i| {
            (
                i.key_pressed(egui::Key::Tab),
                i.key_pressed(egui::Key::Escape),
                i.key_pressed(egui::Key::R),
                i.key_down(egui::Key::W),
                i.key_down(egui::Key::A),
                i.key_down(egui::Key::S),
                i.key_down(egui::Key::D),
                i.key_down(egui::Key::Q),
                i.key_down(egui::Key::E),
                i.key_down(egui::Key::Space),
                i.modifiers.shift,
                i.smooth_scroll_delta.y,
            )
        });

        let (
            tab,
            esc,
            r_key,
            w_key,
            a_key,
            s_key,
            d_key,
            q_key,
            e_key,
            space_key,
            shift_key,
            scroll_y,
        ) = input;

        if tab {
            self.captured = !self.captured;
            self.prev_capture_pos = None;
        }
        if esc && self.captured {
            self.captured = false;
            self.prev_capture_pos = None;
        }

        // Camera movement
        let pan_speed = 0.5;
        if w_key {
            camera.target += camera.flat_forward() * pan_speed;
        }
        if s_key {
            camera.target -= camera.flat_forward() * pan_speed;
        }
        if a_key {
            camera.target -= camera.flat_right() * pan_speed;
        }
        if d_key {
            camera.target += camera.flat_right() * pan_speed;
        }
        if q_key {
            camera.target.y -= pan_speed;
        }
        if e_key {
            camera.target.y += pan_speed;
        }
        // Space = up, Shift = down (vertical camera movement)
        if space_key {
            camera.target.y += pan_speed;
        }
        if shift_key {
            camera.target.y -= pan_speed;
        }

        if scroll_y.abs() > 0.1 {
            camera.zoom(scroll_y / 30.0);
        }

        KeyboardActions { reset: r_key }
    }

    /// Handle mouse interaction on the viewport response (orbit, pan, capture).
    pub fn handle_mouse(&mut self, response: &egui::Response, camera: &mut Camera) {
        if self.captured {
            if let Some(pos) = response.hover_pos() {
                if let Some(prev) = self.prev_capture_pos {
                    let dx = pos.x - prev.x;
                    let dy = pos.y - prev.y;
                    if dx.abs() > 0.01 || dy.abs() > 0.01 {
                        camera.rotate(dx, dy);
                    }
                }
                self.prev_capture_pos = Some(pos);
            }
        } else {
            self.prev_capture_pos = None;

            if response.dragged_by(egui::PointerButton::Secondary) {
                let delta = response.drag_delta();
                camera.rotate(delta.x, delta.y);
            }

            if response.dragged_by(egui::PointerButton::Middle) {
                let delta = response.drag_delta();
                camera.pan(-delta.x, delta.y);
            }
        }
    }
}

/// Actions requested by keyboard input, consumed by the app orchestrator.
pub struct KeyboardActions {
    pub reset: bool,
}
