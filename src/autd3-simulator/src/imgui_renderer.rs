/*
 * File: imgui_renderer.rs
 * Project: src
 * Created Date: 23/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 24/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use std::{ffi::CString, time::Instant};

use autd3_core::{
    autd3_device::NUM_TRANS_IN_UNIT, CPUControlFlags, FPGAControlFlags, FPGA_CLK_FREQ,
};
use autd3_firmware_emulator::CPUEmulator;
use imgui::{
    sys::{igDragFloat, igDragFloat2},
    Context, FontConfig, FontGlyphRanges, FontSource,
};
use imgui_winit_support::{HiDpiMode, WinitPlatform};
use vulkano::{
    command_buffer::{AutoCommandBufferBuilder, PrimaryAutoCommandBuffer},
    image::view::ImageView,
};
use winit::{event::Event, window::Window};

use crate::{
    renderer::Renderer,
    sound_sources::SoundSources,
    update_flag::UpdateFlag,
    viewer_settings::{ColorMapType, ViewerSettings},
    Matrix4, Vector4, SCALE,
};

pub struct ImGuiRenderer {
    imgui: Context,
    platform: WinitPlatform,
    imgui_renderer: imgui_vulkano_renderer::Renderer,
    last_frame: Instant,
    font_size: f32,
    hidpi_factor: f32,
    do_update_font: bool,
    visible: Vec<bool>,
    enable: Vec<bool>,
    thermal: Vec<bool>,
    mod_plot_size: [f32; 2],
    mod_idx: i32,
    stm_idx: i32,
    initial_settings: ViewerSettings,
}

impl ImGuiRenderer {
    pub fn new(initial_settings: ViewerSettings, renderer: &Renderer) -> Self {
        let mut imgui = Context::create();

        let mut platform = WinitPlatform::init(&mut imgui);
        platform.attach_window(imgui.io_mut(), renderer.window(), HiDpiMode::Default);

        let hidpi_factor = platform.hidpi_factor();
        let font_size = (16.0 * hidpi_factor) as f32;

        imgui.io_mut().font_global_scale = (1.0 / hidpi_factor) as f32;

        let imgui_renderer = imgui_vulkano_renderer::Renderer::init(
            &mut imgui,
            renderer.device(),
            renderer.queue(),
            vulkano::format::Format::B8G8R8A8_UNORM,
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
            visible: Vec::new(),
            enable: Vec::new(),
            thermal: Vec::new(),
            mod_plot_size: [200., 50.],
            mod_idx: 0,
            stm_idx: 0,
            initial_settings,
        }
    }

    pub fn init(&mut self, cpus: &[CPUEmulator]) {
        self.visible = vec![true; cpus.len()];
        self.enable = vec![true; cpus.len()];
        self.thermal = vec![false; cpus.len()];
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

    pub fn update(
        &mut self,
        cpus: &mut [CPUEmulator],
        sources: &mut SoundSources,
        render: &Renderer,
        builder: &mut AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>,
        settings: &mut ViewerSettings,
    ) -> UpdateFlag {
        self.update_font(render);

        let io = self.imgui.io_mut();

        let fps = io.framerate;

        self.platform
            .prepare_frame(io, render.window())
            .expect("Failed to start frame");

        let ui = self.imgui.new_frame();

        let mut update_flag = UpdateFlag::empty();
        let mut font_size = self.font_size;
        let mut update_font = false;
        ui.window("Dear ImGui").build(|| {
            if let Some(tab_bar) = ui.tab_bar("Settings") {
                if let Some(tab) = ui.tab_item("Slice") {
                    ui.text("Position");
                    unsafe {
                        if igDragFloat(
                            CString::new("X##Slice").unwrap().as_c_str().as_ptr(),
                            &mut settings.slice_pos_x as _,
                            1. * SCALE,
                            f32::MIN / 2.,
                            f32::MAX / 2.,
                            CString::new("%.3f").unwrap().as_c_str().as_ptr(),
                            0,
                        ) {
                            update_flag.set(UpdateFlag::UPDATE_SLICE_POS, true);
                        }
                        if igDragFloat(
                            CString::new("Y##Slice").unwrap().as_c_str().as_ptr(),
                            &mut settings.slice_pos_y as _,
                            1. * SCALE,
                            f32::MIN / 2.,
                            f32::MAX / 2.,
                            CString::new("%.3f").unwrap().as_c_str().as_ptr(),
                            0,
                        ) {
                            update_flag.set(UpdateFlag::UPDATE_SLICE_POS, true);
                        }
                        if igDragFloat(
                            CString::new("Z##Slice").unwrap().as_c_str().as_ptr(),
                            &mut settings.slice_pos_z as _,
                            1. * SCALE,
                            f32::MIN / 2.,
                            f32::MAX / 2.,
                            CString::new("%.3f").unwrap().as_c_str().as_ptr(),
                            0,
                        ) {
                            update_flag.set(UpdateFlag::UPDATE_SLICE_POS, true);
                        }
                    }
                    ui.separator();

                    ui.text("Rotation");
                    unsafe {
                        if igDragFloat(
                            CString::new("RX##Slice").unwrap().as_c_str().as_ptr(),
                            &mut settings.slice_rot_x as _,
                            1.,
                            -180.,
                            180.,
                            CString::new("%.3f").unwrap().as_c_str().as_ptr(),
                            0,
                        ) {
                            update_flag.set(UpdateFlag::UPDATE_SLICE_POS, true);
                        }
                        if igDragFloat(
                            CString::new("RY##Slice").unwrap().as_c_str().as_ptr(),
                            &mut settings.slice_rot_y as _,
                            1.,
                            -180.,
                            180.,
                            CString::new("%.3f").unwrap().as_c_str().as_ptr(),
                            0,
                        ) {
                            update_flag.set(UpdateFlag::UPDATE_SLICE_POS, true);
                        }
                        if igDragFloat(
                            CString::new("RZ##Slice").unwrap().as_c_str().as_ptr(),
                            &mut settings.slice_rot_z as _,
                            1.,
                            -180.,
                            180.,
                            CString::new("%.3f").unwrap().as_c_str().as_ptr(),
                            0,
                        ) {
                            update_flag.set(UpdateFlag::UPDATE_SLICE_POS, true);
                        }
                    }
                    ui.separator();

                    ui.text("Size");
                    unsafe {
                        if igDragFloat(
                            CString::new("Width##Slice").unwrap().as_c_str().as_ptr(),
                            &mut settings.slice_width as _,
                            1. * SCALE,
                            1. * SCALE,
                            2000. * SCALE,
                            CString::new("%.3f").unwrap().as_c_str().as_ptr(),
                            0,
                        ) {
                            if settings.slice_width < 1. * SCALE {
                                settings.slice_width = 1. * SCALE;
                            }
                            update_flag.set(UpdateFlag::UPDATE_SLICE_SIZE, true);
                        }
                        if igDragFloat(
                            CString::new("Height##Slice").unwrap().as_c_str().as_ptr(),
                            &mut settings.slice_height as _,
                            1. * SCALE,
                            1. * SCALE,
                            2000. * SCALE,
                            CString::new("%.3f").unwrap().as_c_str().as_ptr(),
                            0,
                        ) {
                            if settings.slice_height < 1. * SCALE {
                                settings.slice_height = 1. * SCALE;
                            }
                            update_flag.set(UpdateFlag::UPDATE_SLICE_SIZE, true);
                        }
                        if igDragFloat(
                            CString::new("Pixel size##Slice")
                                .unwrap()
                                .as_c_str()
                                .as_ptr(),
                            &mut settings.slice_pixel_size as _,
                            1. * SCALE,
                            0.1 * SCALE,
                            8. * SCALE,
                            CString::new("%.3f").unwrap().as_c_str().as_ptr(),
                            0,
                        ) {
                            if settings.slice_pixel_size < 0.1 * SCALE {
                                settings.slice_height = 0.1 * SCALE;
                            }
                            update_flag.set(UpdateFlag::UPDATE_SLICE_SIZE, true);
                        }
                    }
                    ui.separator();

                    if ui.radio_button_bool("Acoustic", !settings.show_radiation_pressure) {
                        settings.show_radiation_pressure = false;
                    }
                    if ui.radio_button_bool("Radiation", settings.show_radiation_pressure) {
                        settings.show_radiation_pressure = true;
                    }
                    ui.separator();

                    ui.text("Color settings");
                    let items = vec!["Viridis", "Magma", "Inferno", "Plasma"];
                    let selected_idx = match settings.color_map_type {
                        ColorMapType::Viridis => 0,
                        ColorMapType::Magma => 1,
                        ColorMapType::Inferno => 2,
                        ColorMapType::Plasma => 3,
                    };
                    let mut selected = &items[selected_idx];
                    if let Some(cb) = ui.begin_combo("Coloring", selected) {
                        for cur in &items {
                            if selected == cur {
                                ui.set_item_default_focus();
                            }
                            let clicked =
                                ui.selectable_config(cur).selected(selected == cur).build();
                            if clicked {
                                selected = cur;
                            }
                        }
                        match *selected {
                            "Viridis" => {
                                settings.color_map_type = ColorMapType::Viridis;
                                update_flag.set(UpdateFlag::UPDATE_COLOR_MAP, true);
                            }
                            "Magma" => {
                                settings.color_map_type = ColorMapType::Magma;
                                update_flag.set(UpdateFlag::UPDATE_COLOR_MAP, true);
                            }
                            "Inferno" => {
                                settings.color_map_type = ColorMapType::Inferno;
                                update_flag.set(UpdateFlag::UPDATE_COLOR_MAP, true);
                            }
                            "Plasma" => {
                                settings.color_map_type = ColorMapType::Magma;
                                update_flag.set(UpdateFlag::UPDATE_COLOR_MAP, true);
                            }
                            _ => {
                                settings.color_map_type = ColorMapType::Inferno;
                                update_flag.set(UpdateFlag::UPDATE_COLOR_MAP, true);
                            }
                        }
                        cb.end();
                    }
                    unsafe {
                        igDragFloat(
                            CString::new("Scale##Slice").unwrap().as_c_str().as_ptr(),
                            &mut settings.slice_color_scale as _,
                            0.1,
                            0.0,
                            std::f32::MAX / 2.,
                            CString::new("%.3f").unwrap().as_c_str().as_ptr(),
                            0,
                        );
                        if igDragFloat(
                            CString::new("Alpha##Slice").unwrap().as_c_str().as_ptr(),
                            &mut settings.slice_alpha as _,
                            0.01,
                            0.0,
                            1.,
                            CString::new("%.3f").unwrap().as_c_str().as_ptr(),
                            0,
                        ) {
                            update_flag.set(UpdateFlag::UPDATE_COLOR_MAP, true);
                        }
                    }
                    ui.separator();

                    if ui.small_button("xy") {
                        settings.slice_rot_x = 0.;
                        settings.slice_rot_y = 0.;
                        settings.slice_rot_z = 0.;
                        update_flag.set(UpdateFlag::UPDATE_SLICE_POS, true);
                    }
                    ui.same_line();
                    if ui.small_button("yz") {
                        settings.slice_rot_x = 0.;
                        settings.slice_rot_y = 90.;
                        settings.slice_rot_z = 0.;
                        update_flag.set(UpdateFlag::UPDATE_SLICE_POS, true);
                    }
                    ui.same_line();
                    if ui.small_button("zx") {
                        settings.slice_rot_x = 90.;
                        settings.slice_rot_y = 0.;
                        settings.slice_rot_z = 0.;
                        update_flag.set(UpdateFlag::UPDATE_SLICE_POS, true);
                    }

                    tab.end();
                }

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

                    unsafe {
                        igDragFloat(
                            CString::new("Move speed").unwrap().as_c_str().as_ptr(),
                            &mut settings.camera_move_speed as _,
                            1. * SCALE,
                            0.,
                            std::f32::MAX / 2.,
                            CString::new("%.3f").unwrap().as_c_str().as_ptr(),
                            0,
                        );
                    }

                    tab.end();
                }

                if let Some(tab) = ui.tab_item("Config") {
                    unsafe {
                        if igDragFloat(
                            CString::new("Sound speed").unwrap().as_c_str().as_ptr(),
                            &mut settings.sound_speed as _,
                            1. * SCALE,
                            0.,
                            std::f32::MAX / 2.,
                            CString::new("%.3f").unwrap().as_c_str().as_ptr(),
                            0,
                        ) {
                            for (dev, cpu) in cpus.iter().enumerate() {
                                let cycles = cpu.fpga().cycles();
                                sources
                                    .drives_mut()
                                    .skip(dev * NUM_TRANS_IN_UNIT)
                                    .enumerate()
                                    .for_each(|(i, s)| {
                                        let freq = FPGA_CLK_FREQ as f32 / cycles[i] as f32;
                                        s.set_wave_number(freq, settings.sound_speed);
                                    });
                            }
                            update_flag.set(UpdateFlag::UPDATE_SOURCE_DRIVE, true);
                        }
                    }
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

                    ui.text("Device index: show/enable/overheat");

                    for (i, cpu) in cpus.iter_mut().enumerate() {
                        ui.text(format!("Device {}: ", i));
                        ui.same_line();
                        if ui.checkbox(format!("##show{}", i), &mut self.visible[i]) {
                            update_flag.set(UpdateFlag::UPDATE_SOURCE_FLAG, true);
                            let v = if self.visible[i] { 1. } else { 0. };
                            sources
                                .visibilities_mut()
                                .skip(i * NUM_TRANS_IN_UNIT)
                                .for_each(|s| *s = v);
                        }
                        ui.same_line();
                        if ui.checkbox(format!("##enable{}", i), &mut self.enable[i]) {
                            update_flag.set(UpdateFlag::UPDATE_SOURCE_FLAG, true);
                            let v = if self.enable[i] { 1. } else { 0. };
                            sources
                                .drives_mut()
                                .skip(i * NUM_TRANS_IN_UNIT)
                                .for_each(|s| s.enable = v);
                        }
                        ui.same_line();
                        if ui.checkbox(format!("##overheat{}", i), &mut self.thermal[i]) {
                            update_flag.set(UpdateFlag::UPDATE_DEVICE_INFO, true);
                            if self.thermal[i] {
                                cpu.fpga_mut().assert_thermal_sensor();
                            } else {
                                cpu.fpga_mut().deassert_thermal_sensor();
                            }
                        }
                    }
                    ui.separator();

                    ui.color_picker4("Background", &mut settings.background);

                    tab.end();
                }

                if let Some(tab) = ui.tab_item("Info") {
                    ui.text(format!("FPS: {:4.2}", fps));
                    ui.separator();

                    ui.text("Silencer");
                    ui.text(format!("Step: {}", cpus[0].fpga().silencer_step()));
                    ui.separator();

                    {
                        let m = cpus[0].fpga().modulation();
                        ui.text("Modulation");

                        let mod_size = m.len();
                        ui.text(format!("Size: {}", mod_size));
                        ui.text(format!(
                            "Frequency division: {}",
                            cpus[0].fpga().modulation_frequency_division()
                        ));
                        let sampling_freq = FPGA_CLK_FREQ as f32
                            / cpus[0].fpga().modulation_frequency_division() as f32;
                        ui.text(format!("Sampling Frequency: {:.3} [Hz]", sampling_freq));
                        let sampling_period = 1000000.0
                            * cpus[0].fpga().modulation_frequency_division() as f32
                            / FPGA_CLK_FREQ as f32;
                        ui.text(format!("Sampling period: {:.3} [us]", sampling_period));
                        let period = sampling_period * mod_size as f32;
                        ui.text(format!("Period: {:.3} [us]", period));

                        if !m.is_empty() {
                            ui.text(format!("mod[0]: {}", m[0]));
                        }
                        if mod_size == 2 || mod_size == 3 {
                            ui.text(format!("mod[1]: {}", m[1]));
                        } else if mod_size > 3 {
                            ui.text("...");
                        }
                        if mod_size >= 3 {
                            ui.text(format!("mod[{}]: {}", mod_size - 1, m[mod_size - 1]));
                        }

                        if ui.radio_button_bool("Show mod plot", settings.show_mod_plot) {
                            settings.show_mod_plot = !settings.show_mod_plot;
                        }
                        if settings.show_mod_plot {
                            let mod_v: Vec<f32> = m
                                .iter()
                                .map(|&v| (v as f32 / 510.0 * std::f32::consts::PI).sin())
                                .collect();
                            ui.plot_lines("##mod plot", &mod_v)
                                .graph_size(self.mod_plot_size)
                                .scale_min(0.)
                                .scale_max(1.)
                                .build();
                        }

                        if ui.radio_button_bool("Show mod plot (raw)", settings.show_mod_plot_raw) {
                            settings.show_mod_plot_raw = !settings.show_mod_plot_raw;
                        }
                        if settings.show_mod_plot_raw {
                            let mod_v: Vec<f32> = m.iter().map(|&v| v as f32 / 255.0).collect();
                            ui.plot_lines("##mod plot", &mod_v)
                                .graph_size(self.mod_plot_size)
                                .scale_min(0.)
                                .scale_max(1.)
                                .build();
                        }

                        if settings.show_mod_plot || settings.show_mod_plot_raw {
                            unsafe {
                                igDragFloat2(
                                    CString::new("plot size").unwrap().as_c_str().as_ptr(),
                                    self.mod_plot_size.as_mut_ptr(),
                                    1.,
                                    0.,
                                    std::f32::MAX / 2.,
                                    CString::new("%.0f").unwrap().as_c_str().as_ptr(),
                                    0,
                                );
                            }
                        }

                        if ui.checkbox("Enable##MOD", &mut settings.mod_enable) {
                            update_flag.set(UpdateFlag::UPDATE_SOURCE_DRIVE, true);
                        }

                        if settings.mod_enable {
                            if ui.input_int("Index##MOD", &mut self.mod_idx).build() {
                                update_flag.set(UpdateFlag::UPDATE_SOURCE_DRIVE, true);
                            }
                            ui.checkbox("Auto play##MOD", &mut settings.mod_auto_play);
                            if settings.mod_auto_play {
                                update_flag.set(UpdateFlag::UPDATE_SOURCE_DRIVE, true);
                                self.mod_idx += 1;
                            }
                            if self.mod_idx >= mod_size as _ {
                                self.mod_idx = 0;
                            }
                            if self.mod_idx < 0 {
                                self.mod_idx = mod_size as i32 - 1;
                            }

                            ui.text(format!(
                                "Time: {:.3} [us]",
                                sampling_period * self.mod_idx as f32
                            ));
                        }
                    }

                    if cpus.iter().any(|c| c.fpga().is_stm_mode()) {
                        ui.separator();

                        if cpus[0].fpga().is_stm_gain_mode() {
                            ui.text("Gain STM");
                        } else {
                            ui.text("Focus STM");
                            #[cfg(feature = "use_meter")]
                            ui.text(format!(
                                "Sound speed: {:.3} [m/s]",
                                cpus[0].fpga().sound_speed() as f32 / 1024.0
                            ));
                            #[cfg(not(feature = "use_meter"))]
                            ui.text(format!(
                                "Sound speed: {:.3} [mm/s]",
                                cpus[0].fpga().sound_speed() as f32 * 1000. / 1024.0
                            ));
                        }

                        if let Some(start_idx) = cpus[0].fpga().stm_start_idx() {
                            ui.text(format!("Start idx: {}", start_idx));
                        }
                        if let Some(finish_idx) = cpus[0].fpga().stm_finish_idx() {
                            ui.text(format!("Finish idx: {}", finish_idx));
                        }

                        let stm_size = cpus[0].fpga().stm_cycle();
                        ui.text(format!("Size: {}", stm_size));
                        ui.text(format!(
                            "Frequency division: {}",
                            cpus[0].fpga().stm_frequency_division()
                        ));
                        let sampling_freq =
                            FPGA_CLK_FREQ as f32 / cpus[0].fpga().stm_frequency_division() as f32;
                        ui.text(format!("Sampling Frequency: {:.3} [Hz]", sampling_freq));
                        let sampling_period = 1000000.0
                            * cpus[0].fpga().stm_frequency_division() as f32
                            / FPGA_CLK_FREQ as f32;
                        ui.text(format!("Sampling period: {:.3} [us]", sampling_period));
                        let period = sampling_period / stm_size as f32;
                        ui.text(format!("Period: {:.3} [us]", period));

                        if ui.input_int("Index##STM", &mut self.stm_idx).build() {
                            update_flag.set(UpdateFlag::UPDATE_SOURCE_DRIVE, true);
                        }
                        ui.checkbox("Auto play##STM", &mut settings.stm_auto_play);
                        if settings.stm_auto_play {
                            update_flag.set(UpdateFlag::UPDATE_SOURCE_DRIVE, true);
                            self.stm_idx += 1;
                        }
                        if self.stm_idx >= stm_size as _ {
                            self.stm_idx = 0;
                        }
                        if self.stm_idx < 0 {
                            self.stm_idx = stm_size as i32 - 1;
                        }

                        ui.text(format!(
                            "Time: {:.3} [us]",
                            sampling_period * self.stm_idx as f32
                        ));
                    }

                    ui.separator();
                    ui.text(format!("MSG ID: {}", cpus[0].msg_id()));
                    ui.text("CPU control flags");
                    let cpu_flags = cpus[0].cpu_flags();
                    if cpu_flags.contains(CPUControlFlags::MOD) {
                        let mut mod_ = cpu_flags.contains(CPUControlFlags::MOD);
                        let mut mod_begin = cpu_flags.contains(CPUControlFlags::MOD_BEGIN);
                        let mut mod_end = cpu_flags.contains(CPUControlFlags::MOD_END);
                        ui.checkbox("MOD", &mut mod_);
                        ui.checkbox("MOD BEGIN", &mut mod_begin);
                        ui.checkbox("MOD END", &mut mod_end);
                    } else {
                        let mut config_en_n = !cpu_flags.contains(CPUControlFlags::CONFIG_EN_N);
                        let mut config_silencer =
                            cpu_flags.contains(CPUControlFlags::CONFIG_SILENCER);
                        let mut config_sync = cpu_flags.contains(CPUControlFlags::CONFIG_SYNC);
                        ui.checkbox("CONFIG EN", &mut config_en_n);
                        ui.checkbox("CONFIG SILENCER", &mut config_silencer);
                        ui.checkbox("CONFIG SYNC", &mut config_sync);
                    }
                    let mut write_body = cpu_flags.contains(CPUControlFlags::WRITE_BODY);
                    let mut stm_begin = cpu_flags.contains(CPUControlFlags::STM_BEGIN);
                    let mut stm_end = cpu_flags.contains(CPUControlFlags::STM_END);
                    let mut is_duty = cpu_flags.contains(CPUControlFlags::IS_DUTY);
                    let mut mod_delay = cpu_flags.contains(CPUControlFlags::MOD_DELAY);
                    ui.checkbox("WRITE BODY", &mut write_body);
                    ui.checkbox("STM BEGIN", &mut stm_begin);
                    ui.checkbox("STM END", &mut stm_end);
                    ui.checkbox("IS DUTY", &mut is_duty);
                    ui.checkbox("MOD DELAY", &mut mod_delay);

                    ui.separator();
                    ui.text("FPGA control flags");
                    let fpga_flags = cpus[0].fpga_flags();
                    let mut is_legacy_mode = fpga_flags.contains(FPGAControlFlags::LEGACY_MODE);
                    let mut use_stm_start_idx =
                        fpga_flags.contains(FPGAControlFlags::USE_START_IDX);
                    let mut use_stm_finish_idx =
                        fpga_flags.contains(FPGAControlFlags::USE_FINISH_IDX);
                    let mut force_fan = fpga_flags.contains(FPGAControlFlags::FORCE_FAN);
                    let mut stm_mode = fpga_flags.contains(FPGAControlFlags::STM_MODE);
                    let mut stm_gain_mode = fpga_flags.contains(FPGAControlFlags::STM_GAIN_MODE);
                    let mut reads_fpga_info =
                        fpga_flags.contains(FPGAControlFlags::READS_FPGA_INFO);

                    ui.checkbox("LEGACY MODE", &mut is_legacy_mode);
                    ui.checkbox("USE STM START IDX", &mut use_stm_start_idx);
                    ui.checkbox("USE STM FINISH IDX", &mut use_stm_finish_idx);
                    ui.checkbox("FORCE FAN", &mut force_fan);
                    ui.checkbox("STM MODE", &mut stm_mode);
                    ui.checkbox("STM GAIN MODE", &mut stm_gain_mode);
                    ui.checkbox("READS FPGA INFO", &mut reads_fpga_info);

                    tab.end();
                }

                tab_bar.end();
            }

            ui.separator();
            ui.text("Save image as file");
            ui.input_text("path to image", &mut settings.image_save_path)
                .build();
            if ui.small_button("save") {
                update_flag.set(UpdateFlag::SAVE_IMAGE, true);
            }

            ui.separator();

            if ui.small_button("Auto") {
                let rot = settings.slice_rotation();
                let sr = Matrix4::from(rot);
                let srf = sr * Vector4::new(0.0, 0.0, 1.0, 1.0);

                let camera_pos = settings.slice_pos() + srf * 600.0 * SCALE;
                settings.set_camera_pos(camera_pos.truncate());
                settings.set_camera_rot(settings.slice_rotation());
            }
            ui.same_line();
            if ui.small_button("Reset") {
                update_flag.set(UpdateFlag::all(), true);
                update_flag.remove(UpdateFlag::SAVE_IMAGE);
                *settings = self.initial_settings.clone();
            }
            ui.same_line();
            if ui.small_button("Default") {
                update_flag.set(UpdateFlag::all(), true);
                update_flag.remove(UpdateFlag::SAVE_IMAGE);
                *settings = Default::default();
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

        update_flag
    }

    pub(crate) fn waiting(
        &mut self,
        render: &Renderer,
        builder: &mut AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>,
    ) {
        self.update_font(render);

        let io = self.imgui.io_mut();

        self.platform
            .prepare_frame(io, render.window())
            .expect("Failed to start frame");

        let ui = self.imgui.new_frame();

        ui.window("Dear ImGui").build(|| {
            ui.text("Waiting for client connection...");
        });

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

    pub(crate) fn stm_idx(&self) -> usize {
        self.stm_idx as _
    }

    pub(crate) fn mod_idx(&self) -> usize {
        self.mod_idx as _
    }
}
