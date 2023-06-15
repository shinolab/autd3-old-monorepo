/*
 * File: simulator.rs
 * Project: src
 * Created Date: 24/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 15/06/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use crate::{
    device_viewer::DeviceViewer, imgui_renderer::ImGuiRenderer, model::Model, renderer::Renderer,
    settings::Settings, Quaternion, Vector3, GL_SCALE,
};

use autd3_core::{
    autd3_device::NUM_TRANS_IN_UNIT,
    geometry::{Geometry, Transducer},
};

use vulkano::{
    command_buffer::{
        AutoCommandBufferBuilder, CommandBufferUsage, RenderPassBeginInfo, SubpassContents,
    },
    sync::GpuFuture,
};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoopBuilder},
    platform::run_return::EventLoopExtRunReturn,
};

#[derive(Default)]
pub struct GeometryViewer {
    window_height: u32,
    window_width: u32,
    vsync: bool,
}

impl GeometryViewer {
    pub fn new() -> Self {
        Self {
            window_width: 800,
            window_height: 600,
            vsync: true,
        }
    }

    #[deprecated(since = "11.1.0", note = "Use with_window_size instead")]
    pub fn window_size(mut self, width: u32, height: u32) -> Self {
        self.window_width = width;
        self.window_height = height;
        self
    }

    pub fn with_window_size(mut self, width: u32, height: u32) -> Self {
        self.window_width = width;
        self.window_height = height;
        self
    }

    #[deprecated(since = "11.1.0", note = "Use with_vsync instead")]
    pub fn vsync(mut self, vsync: bool) -> Self {
        self.vsync = vsync;
        self
    }

    pub fn with_vsync(mut self, vsync: bool) -> Self {
        self.vsync = vsync;
        self
    }

    pub fn run<T: Transducer>(&mut self, geometry: &Geometry<T>) -> i32 {
        let mut event_loop = EventLoopBuilder::<()>::with_user_event().build();
        let mut render = Renderer::new(
            &event_loop,
            "AUTD GeometryViewer",
            self.window_width as _,
            self.window_height as _,
            self.vsync,
        );

        let model = Model::new();
        let mut device_viewer = DeviceViewer::new(&render, &model);

        let num_dev = geometry.num_devices();

        let geo: Vec<_> = geometry
            .into_iter()
            .step_by(NUM_TRANS_IN_UNIT)
            .map(|tr| {
                let pos = tr.position();
                let rot = tr.rotation();
                (
                    Vector3::new(pos.x as _, pos.y as _, pos.z as _) * GL_SCALE,
                    Quaternion::new(rot.w as _, rot.i as _, rot.j as _, rot.k as _),
                )
            })
            .collect();

        let mut settings = Settings::new(num_dev);

        render.move_camera(settings.camera_pos(), settings.camera_rot());

        let mut imgui = ImGuiRenderer::new(&render);

        let mut is_running = true;
        event_loop.run_return(move |event, _, control_flow| {
            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    is_running = false;
                    *control_flow = ControlFlow::Exit;
                }
                Event::WindowEvent {
                    event: WindowEvent::Resized(..) | WindowEvent::ScaleFactorChanged { .. },
                    ..
                } => {
                    render.resize();
                    imgui.resized(render.window(), &event);
                }
                Event::MainEventsCleared => {
                    imgui.prepare_frame(render.window());
                    render.window().request_redraw();
                }
                Event::NewEvents(_) => {
                    imgui.update_delta_time();
                    render.window().request_redraw();
                }
                Event::RedrawRequested(_) => {
                    let before_pipeline_future = match render.start_frame() {
                        Err(e) => {
                            eprintln!("{}", e);
                            return;
                        }
                        Ok(future) => future,
                    };
                    let after_future = {
                        let framebuffer = render.frame_buffer();

                        let mut builder = AutoCommandBufferBuilder::primary(
                            render.command_buffer_allocator(),
                            render.queue().queue_family_index(),
                            CommandBufferUsage::OneTimeSubmit,
                        )
                        .unwrap();

                        let clear_values =
                            vec![Some(settings.background.into()), Some(1f32.into()), None];
                        builder
                            .begin_render_pass(
                                RenderPassBeginInfo {
                                    clear_values,
                                    ..RenderPassBeginInfo::framebuffer(framebuffer)
                                },
                                SubpassContents::Inline,
                            )
                            .unwrap()
                            .set_viewport(0, [render.viewport()]);

                        render.move_camera(settings.camera_pos(), settings.camera_rot());
                        let view = render.get_view();
                        let proj = render.get_projection(
                            settings.camera_fov,
                            settings.camera_near_clip,
                            settings.camera_far_clip,
                        );

                        device_viewer.render(&model, &geo, (view, proj), &settings, &mut builder);

                        builder.end_render_pass().unwrap();

                        imgui.render(&render, &geo, &mut settings, &mut builder);

                        let command_buffer = builder.build().unwrap();

                        let future = before_pipeline_future
                            .then_execute(render.queue(), command_buffer)
                            .unwrap();

                        future.boxed()
                    };

                    render.finish_frame(after_future);
                }
                event => {
                    imgui.handle_event(render.window(), &event);
                }
            }
            if !is_running {
                *control_flow = ControlFlow::Exit;
            }
        })
    }
}
