//! Main application state and eframe::App implementation.

use std::collections::VecDeque;
use std::time::Instant;

use eframe::egui;
use glam::Vec3;
use hyle_ca_analysis::{analyze_spec, analyze_step_report, RuntimeReport, SpecAnalysis};
use hyle_ca_interface::{CaSolverProvider, MaterialSet};

use crate::ca::{viewer_world, Materials, Scenario, SimpleWorld, Simulation};
use crate::input::InputState;
use crate::rendering::{
    draw_runtime_analysis_window, draw_static_analysis_window, draw_toolbar, render, Camera,
    GpuRaytracer,
};

pub struct ViewerApp<P>
where
    P: CaSolverProvider,
{
    world: SimpleWorld,
    materials: Materials,
    camera: Camera,
    gpu: GpuRaytracer,
    sim: Simulation<P>,
    input: InputState,
    world_dirty: bool,
    show_runtime_analysis: bool,
    show_static_analysis: bool,
    static_analysis: SpecAnalysis,
    runtime_analysis: Option<RuntimeReport>,
    frame_times: VecDeque<f64>,
    last_frame: Instant,
}

impl<P> ViewerApp<P>
where
    P: CaSolverProvider,
{
    pub fn new(cc: &eframe::CreationContext, provider: P) -> Self {
        let render_state = cc
            .wgpu_render_state
            .as_ref()
            .expect("wgpu backend required");
        let device = &render_state.device;
        let queue = &render_state.queue;
        let mut renderer = render_state.renderer.write();

        let mut world = viewer_world();
        let mut sim = Simulation::new(provider);
        sim.reset(&mut world);
        let materials = sim.materials();
        let static_analysis = analyze_spec(&sim.scenario().blueprint());

        let gpu = GpuRaytracer::new(device, queue, &mut renderer, &world, &materials);
        drop(renderer);

        // Look at the center of the 64³ world.
        let camera = Camera::new(Vec3::new(32.0, 32.0, 32.0), 120.0);

        Self {
            world,
            materials,
            camera,
            gpu,
            sim,
            input: InputState::new(),
            world_dirty: true,
            show_runtime_analysis: false,
            show_static_analysis: false,
            static_analysis,
            runtime_analysis: None,
            frame_times: VecDeque::with_capacity(60),
            last_frame: Instant::now(),
        }
    }

    fn fps(&self) -> f64 {
        if self.frame_times.is_empty() {
            return 0.0;
        }
        let avg = self.frame_times.iter().sum::<f64>() / self.frame_times.len() as f64;
        if avg > 0.0 {
            1000.0 / avg
        } else {
            0.0
        }
    }
}

impl<P> eframe::App for ViewerApp<P>
where
    P: CaSolverProvider,
{
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let dt_ms = self.last_frame.elapsed().as_secs_f64() * 1000.0;
        self.last_frame = Instant::now();
        self.frame_times.push_back(dt_ms);
        if self.frame_times.len() > 60 {
            self.frame_times.pop_front();
        }

        ctx.request_repaint();

        let auto_step = self
            .sim
            .maybe_auto_step(&mut self.world, self.show_runtime_analysis);
        if auto_step.stepped {
            self.world_dirty = true;
            self.update_runtime_analysis(auto_step.report);
        }

        let actions = self.input.handle_keyboard(ctx, &mut self.camera);
        if actions.reset {
            self.sim.reset(&mut self.world);
            self.world_dirty = true;
        }

        let vp_size = ctx.available_rect().size();
        let approx_vp = (
            vp_size.x.max(64.0) as u32,
            (vp_size.y - 36.0).max(64.0) as u32,
        );
        let fps = self.fps();

        let toolbar = draw_toolbar(
            ctx,
            self.sim.scenario(),
            &mut self.sim.auto_step,
            &mut self.sim.step_interval_ms,
            &mut self.show_runtime_analysis,
            &mut self.show_static_analysis,
            fps,
            approx_vp,
        );

        let render_state = frame
            .wgpu_render_state()
            .expect("wgpu render state")
            .clone();

        if let Some(scenario) = toolbar.scenario_selected {
            self.apply_scenario(&render_state, scenario);
        }

        if toolbar.step_requested {
            let step = self.sim.step(&mut self.world, self.show_runtime_analysis);
            self.world_dirty = true;
            self.update_runtime_analysis(step.report);
        }
        if toolbar.reset_requested {
            self.sim.reset(&mut self.world);
            self.world_dirty = true;
            self.runtime_analysis = None;
        }

        egui::CentralPanel::default()
            .frame(egui::Frame::NONE)
            .show(ctx, |ui| {
                render(
                    ui,
                    &render_state,
                    &mut self.gpu,
                    &self.world,
                    &self.materials,
                    &mut self.camera,
                    &mut self.world_dirty,
                    &mut self.input,
                );
            });

        if self.show_static_analysis {
            draw_static_analysis_window(ctx, &mut self.show_static_analysis, &self.static_analysis);
        }
        if self.show_runtime_analysis {
            draw_runtime_analysis_window(
                ctx,
                &mut self.show_runtime_analysis,
                self.runtime_analysis.as_ref(),
            );
        }
    }
}

impl<P> ViewerApp<P>
where
    P: CaSolverProvider,
{
    fn apply_scenario(
        &mut self,
        render_state: &eframe::egui_wgpu::RenderState,
        scenario: Scenario,
    ) {
        if !self.sim.set_scenario(scenario, &mut self.world) {
            return;
        }

        self.materials = self.sim.materials();
        self.static_analysis = analyze_spec(&self.sim.scenario().blueprint());
        self.runtime_analysis = None;
        self.gpu
            .upload_palette(&render_state.device, &render_state.queue, &self.materials);
        self.world_dirty = true;
    }

    fn update_runtime_analysis(&mut self, report: Option<hyle_ca_interface::StepReport>) {
        let Some(report) = report else {
            return;
        };

        let alive_materials = self
            .sim
            .scenario()
            .alive_materials()
            .iter()
            .map(|material| material.id())
            .collect::<Vec<_>>();
        self.runtime_analysis = Some(analyze_step_report(&report, &alive_materials));
    }
}
