/*
 * File: simulator.rs
 * Project: src
 * Created Date: 24/05/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 27/06/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use std::{
    error::Error,
    f32::consts::PI,
    io::ErrorKind,
    net::ToSocketAddrs,
    pin::Pin,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
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

use futures_util::future::FutureExt;
use tokio::{
    runtime::Builder,
    sync::{mpsc, oneshot},
};
use tokio_stream::{wrappers::ReceiverStream, Stream, StreamExt};
use tonic::{transport::Server, Request, Response, Status, Streaming};

pub mod pb {
    tonic::include_proto!("autd3");
}

use pb::*;

type SimulatorServerResult<T> = Result<Response<T>, Status>;
type ResponseStream = Pin<Box<dyn Stream<Item = Result<Rx, Status>> + Send>>;

fn match_for_io_error(err_status: &Status) -> Option<&std::io::Error> {
    let mut err: &(dyn Error + 'static) = err_status;
    loop {
        if let Some(io_err) = err.downcast_ref::<std::io::Error>() {
            return Some(io_err);
        }
        if let Some(h2_err) = err.downcast_ref::<h2::Error>() {
            if let Some(io_err) = h2_err.get_io() {
                return Some(io_err);
            }
        }
        err = match err.source() {
            Some(err) => err,
            None => return None,
        };
    }
}

struct SimulatorServer {
    fin: Arc<AtomicBool>,
    rx_receiver: Receiver<Rx>,
    tx_sender: Sender<Tx>,
}

#[tonic::async_trait]
impl simulator_server::Simulator for SimulatorServer {
    type ReceiveDataStream = ResponseStream;

    async fn receive_data(
        &self,
        req: Request<Streaming<Tx>>,
    ) -> SimulatorServerResult<Self::ReceiveDataStream> {
        spdlog::info!(
            "Connect to client{}",
            if let Some(addr) = req.remote_addr() {
                format!(": {}", addr)
            } else {
                "".to_string()
            }
        );
        let in_stream = req
            .into_inner()
            .timeout_repeating(tokio::time::interval(std::time::Duration::from_millis(100)));

        let (tx, rx) = mpsc::channel(128);

        let fin = self.fin.clone();
        let rx_receiver = self.rx_receiver.clone();
        let tx2 = tx.clone();
        tokio::spawn(async move {
            while !fin.load(Ordering::Acquire) {
                if let Ok(rx) = rx_receiver.try_recv() {
                    match tx2.send(Ok(rx)).await {
                        Ok(_) => {}
                        Err(e) => {
                            spdlog::trace!("Send err: {}", e);
                            break;
                        }
                    }
                }
                tokio::time::sleep(std::time::Duration::from_millis(1)).await;
            }
        });
        let fin = self.fin.clone();
        let tx_sender = self.tx_sender.clone();
        tokio::spawn(async move {
            tokio::pin!(in_stream);
            loop {
                match in_stream.try_next().await {
                    Ok(Some(Ok(result))) => {
                        tx_sender.send(result).unwrap();
                    }
                    Ok(Some(Err(err))) => {
                        if let Some(io_err) = match_for_io_error(&err) {
                            if io_err.kind() == ErrorKind::BrokenPipe {
                                break;
                            }
                        }
                        match tx.send(Err(err)).await {
                            Ok(_) => (),
                            Err(e) => {
                                spdlog::trace!("Receive err: {}", e);
                                break;
                            }
                        }
                    }
                    Ok(None) => {
                        spdlog::trace!("Receive None");
                        break;
                    }
                    Err(_) => {
                        if fin.load(Ordering::Acquire) {
                            break;
                        }
                    }
                }
            }
            spdlog::info!("Disconnect from client");
            let _ = tx_sender.send(Tx {
                data: Some(pb::tx::Data::Close(Close {})),
            });
        });

        let out_stream = ReceiverStream::new(rx);

        Ok(Response::new(
            Box::pin(out_stream) as Self::ReceiveDataStream
        ))
    }
}

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

    pub fn with_window_size(mut self, width: u32, height: u32) -> Self {
        self.window_width = Some(width);
        self.window_height = Some(height);
        self
    }

    pub fn with_vsync(mut self, vsync: bool) -> Self {
        self.vsync = Some(vsync);
        self
    }

    pub fn with_port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }

    pub fn with_gpu_idx(mut self, gpu_idx: i32) -> Self {
        self.gpu_idx = Some(gpu_idx);
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

        let (sender_c2s, receiver_c2s) = bounded(128);
        let (sender_s2c, receiver_s2c) = bounded(128);

        let (tx, rx) = oneshot::channel::<()>();
        let exit = Arc::new(AtomicBool::new(false));
        let port = self.port.unwrap_or(self.settings.port);
        let server_th = Self::run_server(port, exit.clone(), receiver_c2s, sender_s2c, rx);
        self.run_simulator(server_th, exit, receiver_s2c, sender_c2s, tx)
    }

    fn run_server(
        port: u16,
        fin: Arc<AtomicBool>,
        rx_receiver: Receiver<Rx>,
        tx_sender: Sender<Tx>,
        shutdown: tokio::sync::oneshot::Receiver<()>,
    ) -> std::thread::JoinHandle<Result<(), tonic::transport::Error>> {
        std::thread::spawn(move || {
            spdlog::info!("Waiting for client connection on http://0.0.0.0:{}", port);
            let body = async {
                Server::builder()
                    .add_service(simulator_server::SimulatorServer::new(SimulatorServer {
                        fin,
                        rx_receiver,
                        tx_sender,
                    }))
                    .serve_with_shutdown(
                        format!("0.0.0.0:{port}")
                            .to_socket_addrs()
                            .unwrap()
                            .next()
                            .unwrap(),
                        shutdown.map(drop),
                    )
                    .await
            };
            Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(body)
        })
    }

    fn run_simulator(
        &mut self,
        server_th: std::thread::JoinHandle<Result<(), tonic::transport::Error>>,
        exit: Arc<AtomicBool>,
        receiver_c2s: Receiver<Tx>,
        sender_s2c: Sender<Rx>,
        shutdown: tokio::sync::oneshot::Sender<()>,
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

        let server_th_ref = &server_th;
        let res = event_loop.run_return(move |event, _, control_flow| {
            if let Ok(tx) = receiver_c2s.try_recv() {
                match tx.data {
                    Some(pb::tx::Data::Geometry(geometry)) => {
                        geometry.geometries.iter().for_each(|dev| {
                            let pos = dev.position.as_ref().unwrap();
                            let pos = autd3_core::geometry::Vector3::new(
                                pos.x as _, pos.y as _, pos.z as _,
                            );
                            let rot = dev.rotation.as_ref().unwrap();
                            let rot = autd3_core::geometry::UnitQuaternion::from_quaternion(
                                autd3_core::geometry::Quaternion::new(
                                    rot.w as _, rot.x as _, rot.y as _, rot.z as _,
                                ),
                            );
                            AUTD3::with_quaternion(pos, rot)
                                .get_transducers(0)
                                .iter()
                                .for_each(|(_, p, r)| {
                                    sources.add(
                                        to_gl_pos(Vector3::new(p.x as _, p.y as _, p.z as _)),
                                        to_gl_rot(Quaternion::new(
                                            r.w as _, r.i as _, r.j as _, r.k as _,
                                        )),
                                        Drive::new(1.0, 0.0, 1.0, 40e3, self.settings.sound_speed),
                                        1.0,
                                    );
                                });
                        });
                        sender_s2c
                            .send(Rx {
                                data: Some(pb::rx::Data::Geometry(GeometryResponse {})),
                            })
                            .unwrap();

                        cpus = (0..sources.len() / NUM_TRANS_IN_UNIT)
                            .map(|i| CPUEmulator::new(i, NUM_TRANS_IN_UNIT))
                            .collect();

                        field_compute_pipeline.init(&render, &sources);
                        trans_viewer.init(&render, &sources);
                        slice_viewer.init(&self.settings);
                        imgui.init(&cpus);

                        is_initialized = true;
                    }
                    Some(pb::tx::Data::Raw(raw)) => {
                        let len = raw.data.len();
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
                                raw.data[header_size..].as_ptr(),
                                tx.body_raw_mut().as_mut_ptr() as *mut u8,
                                body_len,
                            );
                            std::ptr::copy_nonoverlapping(
                                raw.data.as_ptr(),
                                tx.header_mut() as *mut _ as *mut u8,
                                header_size,
                            );
                        }
                        cpus.iter_mut().for_each(|cpu| {
                            cpu.send(&tx);
                        });
                        sender_s2c
                            .send(Rx {
                                data: Some(pb::rx::Data::Rx(RxMessage {
                                    data: cpus.iter().flat_map(|c| [c.ack(), c.msg_id()]).collect(),
                                })),
                            })
                            .unwrap();

                        is_source_update = true;
                    }
                    Some(pb::tx::Data::Read(_)) => {
                        cpus.iter_mut().for_each(|c| c.update());
                        sender_s2c
                            .send(Rx {
                                data: Some(pb::rx::Data::Rx(RxMessage {
                                    data: cpus.iter().flat_map(|c| [c.ack(), c.msg_id()]).collect(),
                                })),
                            })
                            .unwrap();
                    }
                    Some(pb::tx::Data::Close(_)) => {
                        is_initialized = false;
                        sources.clear();
                        cpus.clear();
                    }
                    None => {
                        unreachable!();
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
                    event: WindowEvent::Resized(..),
                    window_id,
                } if window_id == render.window().id() => {
                    render.resize();
                    imgui.resized(render.window(), &event);
                }
                Event::WindowEvent {
                    event:
                        WindowEvent::ScaleFactorChanged {
                            scale_factor,
                            new_inner_size,
                        },
                    window_id,
                } if window_id == render.window().id() => {
                    *new_inner_size = render
                        .window()
                        .inner_size()
                        .to_logical::<u32>(render.window().scale_factor())
                        .to_physical(scale_factor);
                    render.resize();
                    let event_imgui: Event<'_, ()> = Event::WindowEvent {
                        window_id,
                        event: WindowEvent::ScaleFactorChanged {
                            scale_factor,
                            new_inner_size,
                        },
                    };
                    imgui.resized(render.window(), &event_imgui);
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

                        let clear_values = vec![
                            Some(self.settings.background.into()),
                            Some(1f32.into()),
                            None,
                        ];
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
                            let slice_model = slice_viewer.model();
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
                                cpus.iter().for_each(|cpu| {
                                    let cycles = cpu.fpga().cycles();
                                    let drives = cpu.fpga().duties_and_phases(imgui.stm_idx());
                                    let m = if self.settings.mod_enable {
                                        cpu.fpga().modulation_at(imgui.mod_idx()) as f32 / 255.
                                    } else {
                                        1.
                                    };
                                    sources
                                        .drives_mut()
                                        .skip(cpu.id() * NUM_TRANS_IN_UNIT)
                                        .take(NUM_TRANS_IN_UNIT)
                                        .enumerate()
                                        .for_each(|(i, d)| {
                                            d.amp = (PI * drives[i].0 as f32 * m
                                                / cycles[i] as f32)
                                                .sin();
                                            d.phase =
                                                2. * PI * drives[i].1 as f32 / cycles[i] as f32;
                                            d.set_wave_number(
                                                FPGA_CLK_FREQ as f32 / cycles[i] as f32,
                                                self.settings.sound_speed,
                                            );
                                        });
                                });
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
                                img_buf
                                    .enumerate_pixels_mut()
                                    .zip(image_buffer_content.iter())
                                    .for_each(|((_, _, pixel), [r, g, b, a])| {
                                        let r = (r * 255.0) as u8;
                                        let g = (g * 255.0) as u8;
                                        let b = (b * 255.0) as u8;
                                        let a = (a * 255.0) as u8;
                                        *pixel = image::Rgba([r, g, b, a]);
                                    });
                                image::imageops::flip_vertical(&img_buf)
                                    .save(&self.settings.image_save_path)
                                    .unwrap();
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
                                model: slice_model.into(),
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

            if server_th_ref.is_finished() || !is_running {
                exit.store(true, Ordering::Release);

                spdlog::default_logger().flush();

                *control_flow = ControlFlow::Exit;
            }
        });

        let _ = shutdown.send(());
        if let Err(e) = server_th.join().unwrap() {
            match e.source() {
                Some(e) => spdlog::error!("Server error: {}", e),
                None => spdlog::error!("Server error: {}", e),
            }
        }

        res
    }
}
