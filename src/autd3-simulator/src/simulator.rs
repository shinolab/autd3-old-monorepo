/*
 * File: simulator.rs
 * Project: src
 * Created Date: 24/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 12/06/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use std::{
    f32::consts::PI,
    io::{Read, Write},
    net::TcpListener,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread::JoinHandle,
};

use crate::{
    common::transform::{to_gl_pos, to_gl_rot},
    field_compute_pipeline::{Config, FieldComputePipeline},
    imgui_renderer::ImGuiRenderer,
    renderer::Renderer,
    slice_viewer::SliceViewer,
    sound_sources::{Drive, SoundSources},
    trans_viewer::TransViewer,
    update_flag::UpdateFlag,
    viewer_settings::ViewerSettings,
    Quaternion, Vector3, SCALE,
};
use autd3_core::{
    autd3_device::{AUTD3, NUM_TRANS_IN_UNIT},
    geometry::Device,
    TxDatagram, FPGA_CLK_FREQ,
};
use autd3_firmware_emulator::CPUEmulator;
use crossbeam_channel::{bounded, Receiver, Sender};
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

const WRITE: u8 = 1;
const READ: u8 = 2;
const GEOMETRY_CONFIG: u8 = 3;
const CLIENT_DISCONNECT: u8 = 4;

#[derive(Default)]
pub struct Simulator {
    window_width: Option<u32>,
    window_height: Option<u32>,
    vsync: Option<bool>,
    port: Option<u16>,
    gpu_idx: Option<i32>,
    settings: ViewerSettings,
}

impl Simulator {
    pub fn new() -> Self {
        Self {
            window_width: None,
            window_height: None,
            vsync: None,
            port: None,
            gpu_idx: None,
            settings: ViewerSettings::default(),
        }
    }

    #[deprecated(since = "11.1.0", note = "Use with_window_size instead")]
    pub fn window_size(mut self, width: u32, height: u32) -> Self {
        self.window_width = Some(width);
        self.window_height = Some(height);
        self
    }

    pub fn with_window_size(mut self, width: u32, height: u32) -> Self {
        self.window_width = Some(width);
        self.window_height = Some(height);
        self
    }

    #[deprecated(since = "11.1.0", note = "Use with_vsync instead")]
    pub fn vsync(mut self, vsync: bool) -> Self {
        self.vsync = Some(vsync);
        self
    }

    pub fn with_vsync(mut self, vsync: bool) -> Self {
        self.vsync = Some(vsync);
        self
    }

    #[deprecated(since = "11.1.0", note = "Use with_port instead")]
    pub fn port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }

    pub fn with_port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }

    #[deprecated(since = "11.1.0", note = "Use with_gpu_idx instead")]
    pub fn gpu_idx(mut self, gpu_idx: i32) -> Self {
        self.gpu_idx = Some(gpu_idx);
        self
    }

    pub fn with_gpu_idx(mut self, gpu_idx: i32) -> Self {
        self.gpu_idx = Some(gpu_idx);
        self
    }

    #[deprecated(since = "11.1.0", note = "Use with_settings instead")]
    pub fn settings(mut self, settings: ViewerSettings) -> Self {
        self.settings = settings;
        self
    }

    pub fn with_settings(mut self, settings: ViewerSettings) -> Self {
        self.settings = settings;
        self
    }

    pub fn get_settings(&self) -> &ViewerSettings {
        &self.settings
    }

    pub fn run(&mut self) -> i32 {
        spdlog::info!("Initializing window...");

        let (sender_c2s, receiver_c2s) = bounded(32);
        let (sender_s2c, receiver_s2c) = bounded(32);

        let max_dev_num = self.settings.max_dev_num;
        let exit = Arc::new(AtomicBool::new(false));
        let exit_server = exit.clone();
        let port = self.port.unwrap_or(self.settings.port);
        let server_th = std::thread::spawn(move || {
            match Self::run_server(port, max_dev_num, receiver_s2c, sender_c2s, exit_server) {
                Ok(_) => {}
                Err(e) => {
                    spdlog::error!("{}", e);
                }
            }
        });

        self.run_simulator(server_th, exit, receiver_c2s, sender_s2c)
    }

    fn run_server(
        port: u16,
        max_dev_num: usize,
        receiver_s2c: Receiver<Vec<u8>>,
        sender_c2s: Sender<Vec<u8>>,
        exit: Arc<AtomicBool>,
    ) -> anyhow::Result<()> {
        let buf_size =
            1 + std::mem::size_of::<autd3_core::GlobalHeader>() + max_dev_num * NUM_TRANS_IN_UNIT;

        let addr = format!("0.0.0.0:{}", port);
        let listener = TcpListener::bind(addr)?;
        listener
            .set_nonblocking(true)
            .expect("Cannot set non-blocking");
        spdlog::info!("Waiting for client connection on {}", port);

        for stream in listener.incoming() {
            let mut rx = Vec::new();
            match stream {
                Ok(mut socket) => {
                    spdlog::info!("Connected to client: {}", socket.peer_addr()?);
                    let mut buf = vec![0x00; buf_size];
                    loop {
                        if let Ok(rx_) = receiver_s2c.try_recv() {
                            rx = rx_;
                        }
                        let len = match socket.read(&mut buf) {
                            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                                if exit.load(Ordering::Acquire) {
                                    break;
                                }
                                continue;
                            }
                            Err(e) if e.kind() == std::io::ErrorKind::ConnectionReset => {
                                spdlog::info!("Client disconnected");
                                sender_c2s.send(vec![CLIENT_DISCONNECT]).unwrap();
                                spdlog::info!("Waiting for client connection on {}", port);
                                break;
                            }
                            Err(e) => {
                                return Err(e.into());
                            }
                            Ok(len) => len,
                        };
                        if len == 0 {
                            spdlog::info!("Client disconnected");
                            sender_c2s.send(vec![CLIENT_DISCONNECT]).unwrap();
                            spdlog::info!("Waiting for client connection on {}", port);
                            break;
                        }
                        match buf[0] {
                            WRITE => {
                                sender_c2s.send(buf[..len].to_vec()).unwrap();
                            }
                            GEOMETRY_CONFIG => {
                                sender_c2s.send(buf[..len].to_vec()).unwrap();
                                if let Ok(rx_) = receiver_s2c.recv() {
                                    let _ = socket.write(&rx_)?;
                                }
                            }
                            READ => {
                                let _ = socket.write(&rx)?;
                            }
                            _ => {}
                        }
                    }
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    if exit.load(Ordering::Acquire) {
                        break;
                    }
                    continue;
                }
                Err(e) => {
                    return Err(e.into());
                }
            }
        }

        Ok(())
    }

    fn run_simulator(
        &mut self,
        server_th: JoinHandle<()>,
        exit: Arc<AtomicBool>,
        receiver_c2s: Receiver<Vec<u8>>,
        sender_s2c: Sender<Vec<u8>>,
    ) -> i32 {
        let mut event_loop = EventLoopBuilder::<()>::with_user_event().build();

        let mut render = Renderer::new(
            &event_loop,
            "AUTD Simulator",
            self.window_width.unwrap_or(self.settings.window_width) as _,
            self.window_height.unwrap_or(self.settings.window_height) as _,
            self.vsync.unwrap_or(self.settings.vsync),
            self.gpu_idx.unwrap_or(self.settings.gpu_idx),
        );

        render.move_camera(&self.settings);

        let mut sources = SoundSources::new();

        let mut cpus: Vec<CPUEmulator> = Vec::new();

        let mut field_compute_pipeline = FieldComputePipeline::new(&render, &self.settings);
        let mut slice_viewer = SliceViewer::new(&render, &self.settings);
        let mut imgui = ImGuiRenderer::new(self.settings.clone(), &render);
        let mut trans_viewer = TransViewer::new(&render);

        let mut is_initialized = false;
        let mut is_source_update = false;
        let mut is_running = true;

        let res = event_loop.run_return(move |event, _, control_flow| {
            if is_initialized {
                cpus.iter_mut().for_each(|c| c.update());
                let rx = cpus.iter().flat_map(|c| [c.ack(), c.msg_id()]).collect();
                sender_s2c.send(rx).unwrap();
            }

            if let Ok(buf) = receiver_c2s.try_recv() {
                match buf[0] {
                    WRITE => {
                        let len = buf.len() - 1;
                        let header_size = std::mem::size_of::<autd3_core::GlobalHeader>();
                        let body_size = std::mem::size_of::<u16>() * NUM_TRANS_IN_UNIT;
                        let body_num = if len > header_size {
                            if (len - header_size) % body_size != 0 {
                                spdlog::warn!("Invalid message size: {}", len);
                                0
                            } else {
                                (len - header_size) / body_size
                            }
                        } else {
                            0
                        };

                        let mut tx = TxDatagram::new(&vec![NUM_TRANS_IN_UNIT; body_num]);
                        tx.num_bodies = body_num;
                        let body_len = body_num * body_size;
                        unsafe {
                            std::ptr::copy_nonoverlapping(
                                buf[1 + header_size..].as_ptr(),
                                tx.body_raw_mut().as_mut_ptr() as *mut u8,
                                body_len,
                            );
                            std::ptr::copy_nonoverlapping(
                                buf[1..].as_ptr(),
                                tx.header_mut() as *mut _ as *mut u8,
                                header_size,
                            );
                        }
                        for cpu in &mut cpus {
                            cpu.send(&tx);
                        }

                        is_source_update = true;
                    }
                    GEOMETRY_CONFIG => {
                        unsafe {
                            let mut cursor: *const u8 = buf[1..].as_ptr();
                            let dev_num = std::ptr::read(cursor as *const u32) as usize;
                            cursor = cursor.add(std::mem::size_of::<u32>());
                            for _ in 0..dev_num {
                                let mut p = cursor as *const f32;
                                let x = std::ptr::read(p);
                                p = p.add(1);
                                let y = std::ptr::read(p);
                                p = p.add(1);
                                let z = std::ptr::read(p);
                                p = p.add(1);
                                let qw = std::ptr::read(p);
                                p = p.add(1);
                                let qx = std::ptr::read(p);
                                p = p.add(1);
                                let qy = std::ptr::read(p);
                                p = p.add(1);
                                let qz = std::ptr::read(p);
                                cursor = cursor.add(7 * std::mem::size_of::<f32>());

                                let pos =
                                    autd3_core::geometry::Vector3::new(x as _, y as _, z as _);
                                let rot = autd3_core::geometry::UnitQuaternion::from_quaternion(
                                    autd3_core::geometry::Quaternion::new(
                                        qw as _, qx as _, qy as _, qz as _,
                                    ),
                                );
                                for (_, p, r) in AUTD3::with_quaternion(pos, rot).get_transducers(0)
                                {
                                    sources.add(
                                        to_gl_pos(Vector3::new(p.x as _, p.y as _, p.z as _)),
                                        to_gl_rot(Quaternion::new(
                                            r.w as _, r.i as _, r.j as _, r.k as _,
                                        )),
                                        Drive::new(1.0, 0.0, 1.0, 40e3, self.settings.sound_speed),
                                        1.0,
                                    );
                                }
                            }
                        }
                        sender_s2c.send(vec![GEOMETRY_CONFIG]).unwrap();

                        for i in 0..sources.len() / NUM_TRANS_IN_UNIT {
                            let mut cpu = CPUEmulator::new(i, NUM_TRANS_IN_UNIT);
                            cpu.init();
                            cpus.push(cpu);
                        }

                        field_compute_pipeline.init(&render, &sources);
                        trans_viewer.init(&render, &sources);
                        slice_viewer.init(&self.settings);
                        imgui.init(&cpus);

                        is_initialized = true;
                    }
                    CLIENT_DISCONNECT => {
                        is_initialized = false;
                        sources.clear();
                        cpus.clear();
                    }
                    _ => {
                        unreachable!()
                    }
                }
            }

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
                            vec![Some(self.settings.background.into()), Some(1f32.into())];
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

                        if is_initialized {
                            render.move_camera(&self.settings);
                            let view = render.get_view();
                            let proj = render.get_projection(&self.settings);
                            trans_viewer.render(view, proj, &mut builder);
                            slice_viewer.render(&render, view, proj, &self.settings, &mut builder);
                            builder.end_render_pass().unwrap();

                            let mut update_flag = imgui.update(
                                &mut cpus,
                                &mut sources,
                                &render,
                                &mut builder,
                                &mut self.settings,
                            );
                            if is_source_update {
                                update_flag.set(UpdateFlag::UPDATE_SOURCE_DRIVE, true);
                                is_source_update = false;
                            }

                            if update_flag.contains(UpdateFlag::UPDATE_SOURCE_DRIVE) {
                                for cpu in &cpus {
                                    let cycles = cpu.fpga().cycles();
                                    let (amp, phase) = cpu.fpga().drives(imgui.stm_idx());
                                    let m = if self.settings.mod_enable {
                                        cpu.fpga().modulation_at(imgui.mod_idx()) as f32 / 255.
                                    } else {
                                        1.
                                    };
                                    for (i, d) in sources
                                        .drives_mut()
                                        .skip(cpu.id() * NUM_TRANS_IN_UNIT)
                                        .take(NUM_TRANS_IN_UNIT)
                                        .enumerate()
                                    {
                                        d.amp = (PI * amp[i] as f32 * m / cycles[i] as f32).sin();
                                        d.phase = 2. * PI * phase[i] as f32 / cycles[i] as f32;
                                        let freq = FPGA_CLK_FREQ as f32 / cycles[i] as f32;
                                        d.set_wave_number(freq, self.settings.sound_speed);
                                    }
                                }
                            }

                            field_compute_pipeline.update(
                                &render,
                                &sources,
                                &self.settings,
                                &update_flag,
                            );
                            slice_viewer.update(&render, &self.settings, &update_flag);
                            trans_viewer.update(&sources, &update_flag);

                            let command_buffer = builder.build().unwrap();

                            let field_image = slice_viewer.field_image_view();

                            if update_flag.contains(UpdateFlag::SAVE_IMAGE) {
                                let image_buffer_content = field_image.read().unwrap();
                                let img_x = (self.settings.slice_width
                                    / self.settings.slice_pixel_size)
                                    as u32;
                                let img_y = (self.settings.slice_height
                                    / self.settings.slice_pixel_size)
                                    as u32;
                                let mut img_buf = image::ImageBuffer::new(img_x, img_y);
                                for ((_, _, pixel), [r, g, b, a]) in img_buf
                                    .enumerate_pixels_mut()
                                    .zip(image_buffer_content.iter())
                                {
                                    let r = (r * 255.0) as u8;
                                    let g = (g * 255.0) as u8;
                                    let b = (b * 255.0) as u8;
                                    let a = (a * 255.0) as u8;
                                    *pixel = image::Rgba([r, g, b, a]);
                                }
                                let img_buf = image::imageops::flip_vertical(&img_buf);
                                img_buf.save(&self.settings.image_save_path).unwrap();
                            }

                            let config = Config {
                                source_num: sources.len() as _,
                                color_scale: self.settings.slice_color_scale,
                                width: (self.settings.slice_width / self.settings.slice_pixel_size)
                                    as _,
                                height: (self.settings.slice_height
                                    / self.settings.slice_pixel_size)
                                    as _,
                                pixel_size: self.settings.slice_pixel_size as _,
                                scale: SCALE,
                                model: slice_viewer.model().into(),
                                ..Default::default()
                            };
                            let after_compute = field_compute_pipeline
                                .compute(&render, config, field_image, &self.settings)
                                .join(before_pipeline_future);
                            let future = after_compute
                                .then_execute(render.queue(), command_buffer)
                                .unwrap();

                            future.boxed()
                        } else {
                            builder.end_render_pass().unwrap();

                            imgui.waiting(&render, &mut builder);

                            let command_buffer = builder.build().unwrap();

                            let future = before_pipeline_future
                                .then_execute(render.queue(), command_buffer)
                                .unwrap();

                            future.boxed()
                        }
                    };

                    render.finish_frame(after_future);
                }
                event => {
                    imgui.handle_event(render.window(), &event);
                }
            }

            if !is_running {
                exit.store(true, Ordering::Release);

                spdlog::default_logger().flush();

                *control_flow = ControlFlow::Exit;
            }
        });

        server_th.join().unwrap();

        res
    }
}
