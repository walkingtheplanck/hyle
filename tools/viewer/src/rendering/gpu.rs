//! GPU compute-shader DDA raytracer using wgpu.

use eframe::egui;
use eframe::egui_wgpu;

use crate::ca::{Aabb, Materials, SimpleWorld};
use crate::rendering::CameraFrame;

/// GPU-side camera uniforms. Must match WGSL struct layout exactly.
#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct CameraUniforms {
    eye: [f32; 3],
    _pad0: f32,
    forward: [f32; 3],
    _pad1: f32,
    right: [f32; 3],
    _pad2: f32,
    up: [f32; 3],
    _pad3: f32,
    half_w: f32,
    half_h: f32,
    inv_w: f32,
    inv_h: f32,
    aabb_min: [f32; 3],
    _pad4: f32,
    aabb_max: [f32; 3],
    max_steps: f32,
    voxel_scale: f32, // 1.0 = full res, 2.0 = half, etc.
    _scale_pad: [f32; 3],
    // Brush preview
    brush_center: [f32; 3],
    brush_lo: f32,     // min offset (e.g. 0 for size 1, 0 for size 2, -1 for size 3)
    brush_hi: f32,     // max offset (e.g. 0 for size 1, 1 for size 2, 1 for size 3)
    brush_size: f32,   // for sphere radius computation
    brush_shape: u32,  // 0=cube, 1=sphere
    brush_mode: u32,   // 0=place, 1=delete
    brush_active: u32, // 0=inactive, 1=active
    _brush_pad: u32,
    _brush_pad2: u32,
    _brush_pad3: u32,
}

/// Parameters for voxel upload.
pub struct VoxelUpload<'a> {
    pub data: &'a [u16],
    pub sx: u32,
    pub sy: u32,
    pub sz: u32,
}

pub struct GpuRaytracer {
    pipeline: wgpu::ComputePipeline,
    bind_group_layout: wgpu::BindGroupLayout,
    camera_buffer: wgpu::Buffer,
    palette_buffer: wgpu::Buffer,
    voxel_texture: wgpu::Texture,
    voxel_view: wgpu::TextureView,
    output_texture: wgpu::Texture,
    output_view: wgpu::TextureView,
    output_texture_id: egui::TextureId,
    #[allow(dead_code)]
    grid_size: [u32; 3],
    width: u32,
    height: u32,
    bind_group: wgpu::BindGroup,
}

impl GpuRaytracer {
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        renderer: &mut egui_wgpu::Renderer,
        world: &SimpleWorld,
        materials: &Materials,
    ) -> Self {
        // Shader
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("raycast.wgsl"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../raycast.wgsl").into()),
        });

        // Bind group layout
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("raytracer_bgl"),
            entries: &[
                // 0: camera uniforms
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                // 1: voxel 3D texture
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Uint,
                        view_dimension: wgpu::TextureViewDimension::D3,
                        multisampled: false,
                    },
                    count: None,
                },
                // 2: palette storage buffer
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                // 3: output storage texture
                wgpu::BindGroupLayoutEntry {
                    binding: 3,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::StorageTexture {
                        access: wgpu::StorageTextureAccess::WriteOnly,
                        format: wgpu::TextureFormat::Rgba8Unorm,
                        view_dimension: wgpu::TextureViewDimension::D2,
                    },
                    count: None,
                },
            ],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("raytracer_pipeline_layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("raytracer_pipeline"),
            layout: Some(&pipeline_layout),
            module: &shader,
            entry_point: Some("main"),
            compilation_options: Default::default(),
            cache: None,
        });

        // Camera uniform buffer
        let camera_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("camera_uniform"),
            size: std::mem::size_of::<CameraUniforms>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Palette buffer
        let palette_data = materials.export_palette();
        let palette_bytes: &[u8] = bytemuck::cast_slice(&palette_data);
        let palette_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("palette"),
            size: palette_bytes.len().max(16) as u64,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        queue.write_buffer(&palette_buffer, 0, palette_bytes);

        // Voxel 3D texture
        let b = &world.bounds;
        let sx = b.size_x() as u32;
        let sy = b.size_y() as u32;
        let sz = b.size_z() as u32;
        let grid_size = [sx, sy, sz];

        // SimpleWorld memory layout: index = ly*(sx*sz) + lz*sx + lx
        // innermost=X, middle=Z, outermost=Y
        // wgpu 3D texture: width=row, height=rows_per_image, depth=layers
        // So we map: width=sx, height=sz, depth=sy (Y and Z swapped)
        // In the shader we sample with textureLoad(voxels, vec3(lx, lz, ly))
        let voxel_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("voxel_3d"),
            size: wgpu::Extent3d {
                width: sx,
                height: sz,
                depth_or_array_layers: sy,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D3,
            format: wgpu::TextureFormat::R16Uint,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        // Upload voxel data
        let material_ids = world.export_material_ids();
        let voxel_bytes: &[u8] = bytemuck::cast_slice(&material_ids);
        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &voxel_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            voxel_bytes,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(sx * 2), // R16Uint = 2 bytes per texel
                rows_per_image: Some(sz),    // middle dimension is Z
            },
            wgpu::Extent3d {
                width: sx,
                height: sz,
                depth_or_array_layers: sy,
            },
        );

        let voxel_view = voxel_texture.create_view(&wgpu::TextureViewDescriptor::default());

        // Initial output texture (small, will be resized)
        let init_w = 64u32;
        let init_h = 64u32;

        let output_texture = Self::create_output_texture(device, init_w, init_h);
        let output_view = output_texture.create_view(&wgpu::TextureViewDescriptor::default());

        // Register with egui
        let output_texture_id =
            renderer.register_native_texture(device, &output_view, wgpu::FilterMode::Nearest);

        // Bind group
        let bind_group = Self::make_bind_group(
            device,
            &bind_group_layout,
            &camera_buffer,
            &voxel_view,
            &palette_buffer,
            &output_view,
        );

        Self {
            pipeline,
            bind_group_layout,
            camera_buffer,
            palette_buffer,
            voxel_texture,
            voxel_view,
            output_texture,
            output_view,
            output_texture_id,
            grid_size,
            width: init_w,
            height: init_h,
            bind_group,
        }
    }

    fn create_output_texture(device: &wgpu::Device, w: u32, h: u32) -> wgpu::Texture {
        device.create_texture(&wgpu::TextureDescriptor {
            label: Some("output_texture"),
            size: wgpu::Extent3d {
                width: w,
                height: h,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::STORAGE_BINDING | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        })
    }

    fn make_bind_group(
        device: &wgpu::Device,
        layout: &wgpu::BindGroupLayout,
        camera_buf: &wgpu::Buffer,
        voxel_view: &wgpu::TextureView,
        palette_buf: &wgpu::Buffer,
        output_view: &wgpu::TextureView,
    ) -> wgpu::BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("raytracer_bg"),
            layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: camera_buf.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(voxel_view),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: palette_buf.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: wgpu::BindingResource::TextureView(output_view),
                },
            ],
        })
    }

    pub fn resize(
        &mut self,
        device: &wgpu::Device,
        renderer: &mut egui_wgpu::Renderer,
        w: u32,
        h: u32,
    ) {
        let w = w.max(1);
        let h = h.max(1);
        if w == self.width && h == self.height {
            return;
        }

        self.output_texture = Self::create_output_texture(device, w, h);
        self.output_view = self
            .output_texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        renderer.update_egui_texture_from_wgpu_texture(
            device,
            &self.output_view,
            wgpu::FilterMode::Nearest,
            self.output_texture_id,
        );

        self.bind_group = Self::make_bind_group(
            device,
            &self.bind_group_layout,
            &self.camera_buffer,
            &self.voxel_view,
            &self.palette_buffer,
            &self.output_view,
        );

        self.width = w;
        self.height = h;
    }

    /// Upload voxel data at the given scale. Recreates the 3D texture if dimensions changed.
    pub fn upload_voxels(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        upload: &VoxelUpload,
    ) {
        let sx = upload.sx;
        let sy = upload.sy;
        let sz = upload.sz;

        // Recreate texture if dimensions changed
        let cur_size = self.voxel_texture.size();
        if cur_size.width != sx || cur_size.height != sz || cur_size.depth_or_array_layers != sy {
            // Texture layout: width=X, height=Z, depth=Y
            self.voxel_texture = device.create_texture(&wgpu::TextureDescriptor {
                label: Some("voxel_3d"),
                size: wgpu::Extent3d {
                    width: sx,
                    height: sz,
                    depth_or_array_layers: sy,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D3,
                format: wgpu::TextureFormat::R16Uint,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                view_formats: &[],
            });
        }

        let voxel_bytes: &[u8] = bytemuck::cast_slice(upload.data);
        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &self.voxel_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            voxel_bytes,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(sx * 2),
                rows_per_image: Some(sz), // middle dimension is Z
            },
            wgpu::Extent3d {
                width: sx,
                height: sz,
                depth_or_array_layers: sy,
            },
        );

        self.voxel_view = self
            .voxel_texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        self.bind_group = Self::make_bind_group(
            device,
            &self.bind_group_layout,
            &self.camera_buffer,
            &self.voxel_view,
            &self.palette_buffer,
            &self.output_view,
        );
    }

    pub fn render(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        cam_frame: &CameraFrame,
        aabb: &Aabb,
    ) {
        let uniforms = CameraUniforms {
            eye: cam_frame.eye.into(),
            _pad0: 0.0,
            forward: cam_frame.forward.into(),
            _pad1: 0.0,
            right: cam_frame.right.into(),
            _pad2: 0.0,
            up: cam_frame.up.into(),
            _pad3: 0.0,
            half_w: cam_frame.half_w,
            half_h: cam_frame.half_h,
            inv_w: cam_frame.inv_w,
            inv_h: cam_frame.inv_h,
            aabb_min: [aabb.min_x as f32, aabb.min_y as f32, aabb.min_z as f32],
            _pad4: 0.0,
            aabb_max: [aabb.max_x as f32, aabb.max_y as f32, aabb.max_z as f32],
            max_steps: 512.0,
            voxel_scale: 1.0,
            _scale_pad: [0.0; 3],
            brush_center: [0.0; 3],
            brush_lo: 0.0,
            brush_hi: 0.0,
            brush_size: 1.0,
            brush_shape: 0,
            brush_mode: 0,
            brush_active: 0,
            _brush_pad: 0,
            _brush_pad2: 0,
            _brush_pad3: 0,
        };

        queue.write_buffer(&self.camera_buffer, 0, bytemuck::bytes_of(&uniforms));

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("raytracer_encoder"),
        });

        {
            let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("raytracer_pass"),
                timestamp_writes: None,
            });
            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, &self.bind_group, &[]);
            pass.dispatch_workgroups(self.width.div_ceil(8), self.height.div_ceil(8), 1);
        }

        queue.submit(std::iter::once(encoder.finish()));
    }

    pub fn texture_id(&self) -> egui::TextureId {
        self.output_texture_id
    }
}
