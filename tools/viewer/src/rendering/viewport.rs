//! Viewport rendering — GPU sync and camera rendering.

use eframe::egui::{self, Sense, Vec2};

use crate::ca::{Materials, SimpleWorld};
use crate::input::InputState;
use crate::rendering::{Camera, GpuRaytracer, VoxelUpload};

/// Upload voxels to the GPU if the world changed.
fn sync_voxels(
    gpu: &mut GpuRaytracer,
    world: &SimpleWorld,
    device: &wgpu::Device,
    queue: &wgpu::Queue,
) {
    let data = world.export_material_ids();
    let b = &world.bounds;
    gpu.upload_voxels(
        device,
        queue,
        &VoxelUpload {
            data: &data,
            sx: b.size_x() as u32,
            sy: b.size_y() as u32,
            sz: b.size_z() as u32,
        },
    );
}

/// Render the viewport. Handles GPU sync, camera rendering, and mouse orbit/zoom.
#[allow(clippy::too_many_arguments)]
pub fn render(
    ui: &mut egui::Ui,
    render_state: &eframe::egui_wgpu::RenderState,
    gpu: &mut GpuRaytracer,
    world: &SimpleWorld,
    _materials: &Materials,
    camera: &mut Camera,
    world_dirty: &mut bool,
    input: &mut InputState,
) {
    let avail = ui.available_size();
    let viewport_w = avail.x.max(1.0) as u32;
    let viewport_h = avail.y.max(1.0) as u32;

    let device = &render_state.device;
    let queue = &render_state.queue;

    {
        let mut renderer = render_state.renderer.write();
        gpu.resize(device, &mut renderer, viewport_w, viewport_h);
    }

    if *world_dirty {
        sync_voxels(gpu, world, device, queue);
        *world_dirty = false;
    }

    let cam_frame = camera.frame(viewport_w, viewport_h);
    gpu.render(device, queue, &cam_frame, &world.bounds);

    let tex_id = gpu.texture_id();
    let sized =
        egui::load::SizedTexture::new(tex_id, Vec2::new(viewport_w as f32, viewport_h as f32));
    let response = ui.add(
        egui::Image::from_texture(sized)
            .fit_to_exact_size(avail)
            .sense(Sense::click_and_drag()),
    );

    input.handle_mouse(&response, camera);
}
