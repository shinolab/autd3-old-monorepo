/*
 * File: imgui_renderer.rs
 * Project: src
 * Created Date: 23/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 28/07/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use std::{ffi::CString, time::Instant};

use crate::patch::{
    imgui_vulkano_renderer,
    imgui_winit_support::{HiDpiMode, WinitPlatform},
};
use cgmath::{Deg, Euler, InnerSpace};
use imgui::{sys::igDragFloat, Context, FontConfig, FontGlyphRanges, FontSource};
use vulkano::{
    command_buffer::{AutoCommandBufferBuilder, PrimaryAutoCommandBuffer},
    image::view::ImageView,
};
use winit::{event::Event, window::Window};

use crate::{renderer::Renderer, settings::Settings, Quaternion, Vector3, SCALE, ZPARITY};

fn quaternion_to(v: Vector3, to: Vector3) -> Quaternion {
    let a = v.normalize();
    let b = to.normalize();
    let c = b.cross(a).normalize();
    if c.x.is_nan() || c.y.is_nan() || c.z.is_nan() {
        return Quaternion::new(1., 0., 0., 0.);
    }
    let ip = a.dot(b);
    const EPS: f32 = 1e-4;
    if c.magnitude() < EPS || 1. < ip {
        if ip < EPS - 1. {
            let a2 = Vector3::new(-a.y, a.z, a.x);
            let c2 = a2.cross(a).normalize();
            return Quaternion::new(0., c2.x, c2.y, c2.z);
        }
        return Quaternion::new(1., 0., 0., 0.);
    }
    let e = c * (0.5 * (1. - ip)).sqrt();
    Quaternion::new((0.5 * (1. + ip)).sqrt(), e.x, e.y, e.z)
}

pub struct ImGuiRenderer {
    imgui: Context,
    platform: WinitPlatform,
    imgui_renderer: imgui_vulkano_renderer::Renderer,
    last_frame: Instant,
    font_size: f32,
    hidpi_factor: f32,
    do_update_font: bool,
}

impl ImGuiRenderer {
    pub fn new(renderer: &Renderer) -> Self {
        let mut imgui = Context::create();

        let mut platform = WinitPlatform::init(&mut imgui);
        platform.attach_window(imgui.io_mut(), renderer.window(), HiDpiMode::Default);

        let hidpi_factor = platform.hidpi_factor();
        let font_size = 16.0;

        imgui.io_mut().font_global_scale = (1.0 / hidpi_factor) as f32;

        let imgui_renderer = imgui_vulkano_renderer::Renderer::init(
            &mut imgui,
            renderer.device(),
            renderer.queue(),
            renderer.color_format(),
        )
        .expect("Failed to initialize renderer");

        Self {
            imgui,
            platform,
            imgui_renderer,
            last_frame: Instant::now(),
            font_size,
            hidpi_factor: hidpi_factor as _,
            do_update_font: true,
        }
    }

    pub fn resized<T>(&mut self, window: &Window, event: &Event<T>) {
        self.platform
            .handle_event(self.imgui.io_mut(), window, event);
    }

    pub fn prepare_frame(&mut self, window: &Window) {
        self.platform
            .prepare_frame(self.imgui.io_mut(), window)
            .expect("Failed to prepare frame");
    }

    pub fn update_delta_time(&mut self) {
        let now = Instant::now();
        self.imgui
            .io_mut()
            .update_delta_time(now.duration_since(self.last_frame));
        self.last_frame = now;
    }

    pub fn handle_event<T>(&mut self, window: &Window, event: &Event<T>) {
        self.platform
            .handle_event(self.imgui.io_mut(), window, event);
    }

    pub fn render(
        &mut self,
        render: &Renderer,
        pos_rot: &[(Vector3, Quaternion)],
        settings: &mut Settings,
        builder: &mut AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>,
    ) {
        self.update_font(render);

        let io = self.imgui.io_mut();

        let fps = io.framerate;

        self.platform
            .prepare_frame(io, render.window())
            .expect("Failed to start frame");

        let ui = self.imgui.new_frame();

        {
            let rotation = Quaternion::from(Euler {
                x: Deg(settings.camera_rot_x),
                y: Deg(settings.camera_rot_y),
                z: Deg(settings.camera_rot_z),
            });

            let r = rotation * Vector3::unit_x();
            let u = rotation * Vector3::unit_y();
            let f = rotation * Vector3::unit_z();

            if !ui.io().want_capture_mouse {
                let mouse_wheel = ui.io().mouse_wheel;
                let trans = -f * mouse_wheel * settings.camera_move_speed * ZPARITY;
                settings.camera_pos_x += trans.x;
                settings.camera_pos_y += trans.y;
                settings.camera_pos_z += trans.z;
            }

            if !ui.io().want_capture_mouse {
                let mouse_delta = ui.io().mouse_delta;
                if ui.io().mouse_down[0] {
                    if ui.io().key_shift {
                        let delta_x = mouse_delta[0] * settings.camera_move_speed / 3000.;
                        let delta_y = mouse_delta[1] * settings.camera_move_speed / 3000.;
                        let to = -r * delta_x + u * delta_y + f;
                        let rot = Euler::from(quaternion_to(f, to) * rotation);
                        settings.camera_rot_x = Deg::from(rot.x).0;
                        settings.camera_rot_y = Deg::from(rot.y).0;
                        settings.camera_rot_z = Deg::from(rot.z).0;
                    } else {
                        let delta_x = mouse_delta[0] * settings.camera_move_speed / 10.;
                        let delta_y = mouse_delta[1] * settings.camera_move_speed / 10.;
                        let trans = -r * delta_x + u * delta_y;
                        settings.camera_pos_x += trans.x;
                        settings.camera_pos_y += trans.y;
                        settings.camera_pos_z += trans.z;
                    }
                }
            }
        }

        let mut font_size = self.font_size;
        let mut update_font = false;
        ui.window("Dear ImGui").build(|| {
            if let Some(tab_bar) = ui.tab_bar("Settings") {
                if let Some(tab) = ui.tab_item("Camera") {
                    ui.text("Position");
                    unsafe {
                        igDragFloat(
                            CString::new("X##Camera").unwrap().as_c_str().as_ptr(),
                            &mut settings.camera_pos_x as _,
                            1. * SCALE,
                            f32::MIN / 2.,
                            f32::MAX / 2.,
                            CString::new("%.3f").unwrap().as_c_str().as_ptr(),
                            0,
                        );
                        igDragFloat(
                            CString::new("Y##Camera").unwrap().as_c_str().as_ptr(),
                            &mut settings.camera_pos_y as _,
                            1. * SCALE,
                            f32::MIN / 2.,
                            f32::MAX / 2.,
                            CString::new("%.3f").unwrap().as_c_str().as_ptr(),
                            0,
                        );
                        igDragFloat(
                            CString::new("Z##Camera").unwrap().as_c_str().as_ptr(),
                            &mut settings.camera_pos_z as _,
                            1. * SCALE,
                            f32::MIN / 2.,
                            f32::MAX / 2.,
                            CString::new("%.3f").unwrap().as_c_str().as_ptr(),
                            0,
                        );
                    }
                    ui.separator();

                    ui.text("Rotation");
                    unsafe {
                        igDragFloat(
                            CString::new("RX##Camera").unwrap().as_c_str().as_ptr(),
                            &mut settings.camera_rot_x as _,
                            1.,
                            -180.,
                            180.,
                            CString::new("%.3f").unwrap().as_c_str().as_ptr(),
                            0,
                        );
                        igDragFloat(
                            CString::new("RY##Camera").unwrap().as_c_str().as_ptr(),
                            &mut settings.camera_rot_y as _,
                            1.,
                            -180.,
                            180.,
                            CString::new("%.3f").unwrap().as_c_str().as_ptr(),
                            0,
                        );
                        igDragFloat(
                            CString::new("RZ##Camera").unwrap().as_c_str().as_ptr(),
                            &mut settings.camera_rot_z as _,
                            1.,
                            -180.,
                            180.,
                            CString::new("%.3f").unwrap().as_c_str().as_ptr(),
                            0,
                        );
                    }
                    ui.separator();

                    unsafe {
                        igDragFloat(
                            CString::new("Move speed").unwrap().as_c_str().as_ptr(),
                            &mut settings.camera_move_speed as _,
                            1.,
                            1. * SCALE,
                            100. * SCALE,
                            CString::new("%.3f").unwrap().as_c_str().as_ptr(),
                            0,
                        );
                    }

                    ui.separator();

                    ui.text("Perspective");
                    unsafe {
                        igDragFloat(
                            CString::new("FOV").unwrap().as_c_str().as_ptr(),
                            &mut settings.camera_fov as _,
                            1.,
                            0.,
                            180.,
                            CString::new("%.3f").unwrap().as_c_str().as_ptr(),
                            0,
                        );
                        igDragFloat(
                            CString::new("Near clip").unwrap().as_c_str().as_ptr(),
                            &mut settings.camera_near_clip as _,
                            1. * SCALE,
                            0.,
                            std::f32::MAX / 2.,
                            CString::new("%.3f").unwrap().as_c_str().as_ptr(),
                            0,
                        );
                        igDragFloat(
                            CString::new("Far clip").unwrap().as_c_str().as_ptr(),
                            &mut settings.camera_far_clip as _,
                            1. * SCALE,
                            0.,
                            std::f32::MAX / 2.,
                            CString::new("%.3f").unwrap().as_c_str().as_ptr(),
                            0,
                        );
                    }
                    ui.separator();

                    tab.end();
                }

                if let Some(tab) = ui.tab_item("Lighting") {
                    ui.text("Position");
                    unsafe {
                        igDragFloat(
                            CString::new("X##Light").unwrap().as_c_str().as_ptr(),
                            &mut settings.light_pos_x as _,
                            1. * SCALE,
                            f32::MIN / 2.,
                            f32::MAX / 2.,
                            CString::new("%.3f").unwrap().as_c_str().as_ptr(),
                            0,
                        );
                        igDragFloat(
                            CString::new("Y##Light").unwrap().as_c_str().as_ptr(),
                            &mut settings.light_pos_y as _,
                            1. * SCALE,
                            f32::MIN / 2.,
                            f32::MAX / 2.,
                            CString::new("%.3f").unwrap().as_c_str().as_ptr(),
                            0,
                        );
                        igDragFloat(
                            CString::new("Z##Light").unwrap().as_c_str().as_ptr(),
                            &mut settings.light_pos_z as _,
                            1. * SCALE,
                            f32::MIN / 2.,
                            f32::MAX / 2.,
                            CString::new("%.3f").unwrap().as_c_str().as_ptr(),
                            0,
                        );
                    }
                    ui.separator();

                    ui.text("Properties");
                    unsafe {
                        igDragFloat(
                            CString::new("Power").unwrap().as_c_str().as_ptr(),
                            &mut settings.light_power as _,
                            0.1,
                            0.,
                            std::f32::MAX / 2.,
                            CString::new("%.3f").unwrap().as_c_str().as_ptr(),
                            0,
                        );
                        igDragFloat(
                            CString::new("Ambient").unwrap().as_c_str().as_ptr(),
                            &mut settings.ambient as _,
                            0.1,
                            0.,
                            std::f32::MAX / 2.,
                            CString::new("%.3f").unwrap().as_c_str().as_ptr(),
                            0,
                        );
                        igDragFloat(
                            CString::new("Specular").unwrap().as_c_str().as_ptr(),
                            &mut settings.specular as _,
                            0.1,
                            0.,
                            std::f32::MAX / 2.,
                            CString::new("%.3f").unwrap().as_c_str().as_ptr(),
                            0,
                        );
                    }

                    tab.end();
                }

                if let Some(tab) = ui.tab_item("Config") {
                    ui.text("Device index: show");

                    (0..settings.shows.len()).for_each(|i| {
                        ui.text(format!("Device {}: ", i));
                        ui.same_line();
                        if ui.checkbox(format!("##show{}", i), &mut settings.shows[i]) {}
                    });
                    ui.separator();

                    unsafe {
                        if igDragFloat(
                            CString::new("Font size").unwrap().as_c_str().as_ptr(),
                            &mut font_size as _,
                            1.,
                            1.,
                            256.,
                            CString::new("%.3f").unwrap().as_c_str().as_ptr(),
                            0,
                        ) {
                            update_font = true;
                        }
                    }
                    ui.separator();

                    ui.color_picker4("Background", &mut settings.background);

                    tab.end();
                }

                if let Some(tab) = ui.tab_item("Info") {
                    ui.text(format!("FPS: {:4.2}", fps));
                    ui.separator();

                    pos_rot.iter().enumerate().for_each(|(i, (pos, rot))| {
                        ui.text(format!("Device {}: ", i));
                        ui.text(format!(
                            "  x: {:.2}, y: {:.2}, z: {:.2}",
                            pos.x, pos.y, pos.z
                        ));
                        ui.text(format!(
                            "  rw: {:.2}, rx: {:.2}, ry: {:.2}, rx: {:.2}",
                            rot.s, rot.v.x, rot.v.y, rot.v.z
                        ));
                    });

                    tab.end()
                }

                tab_bar.end();
            }
        });

        self.font_size = font_size;
        self.do_update_font = update_font;

        self.platform.prepare_render(ui, render.window());
        let draw_data = self.imgui.render();
        self.imgui_renderer
            .draw_commands(
                builder,
                render.queue(),
                ImageView::new_default(render.image()).unwrap(),
                draw_data,
            )
            .expect("Rendering failed");
    }

    fn update_font(&mut self, render: &Renderer) {
        if self.do_update_font {
            self.imgui.fonts().clear();
            self.imgui.fonts().add_font(&[FontSource::TtfData {
                data: include_bytes!("../assets/fonts/NotoSans-Regular.ttf"),
                size_pixels: self.font_size * self.hidpi_factor,
                config: Some(FontConfig {
                    rasterizer_multiply: 1.,
                    glyph_ranges: FontGlyphRanges::default(),
                    ..FontConfig::default()
                }),
            }]);
            self.imgui_renderer
                .reload_font_texture(&mut self.imgui, render.device(), render.queue())
                .expect("Failed to reload fonts");
            self.do_update_font = false;
        }
    }
}
