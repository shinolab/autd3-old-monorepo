/*
 * File: renderer.rs
 * Project: src
 * Created Date: 11/11/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 15/06/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2021 Hapis Lab. All rights reserved.
 *
 */

use std::sync::Arc;

use camera_controllers::{Camera, CameraPerspective, FirstPerson, FirstPersonSettings};

use vulkano::{
    command_buffer::allocator::StandardCommandBufferAllocator,
    descriptor_set::allocator::StandardDescriptorSetAllocator,
    device::{
        physical::{PhysicalDevice, PhysicalDeviceType},
        Device, DeviceCreateInfo, DeviceExtensions, Features, Queue, QueueCreateInfo, QueueFlags,
    },
    format::{Format, FormatFeatures},
    image::{
        view::ImageView, AttachmentImage, ImageAccess, ImageUsage, SampleCount, SampleCounts,
        SwapchainImage,
    },
    instance::{
        debug::{
            DebugUtilsMessageSeverity, DebugUtilsMessageType, DebugUtilsMessenger,
            DebugUtilsMessengerCreateInfo,
        },
        Instance, InstanceCreateInfo,
    },
    memory::allocator::StandardMemoryAllocator,
    pipeline::graphics::viewport::Viewport,
    render_pass::{Framebuffer, FramebufferCreateInfo, RenderPass},
    swapchain::{
        self, AcquireError, ColorSpace, FullScreenExclusive, PresentMode, Surface,
        SurfaceTransform, Swapchain, SwapchainCreateInfo, SwapchainCreationError,
        SwapchainPresentInfo,
    },
    sync::{self, FlushError, GpuFuture},
    VulkanLibrary,
};
use vulkano_win::VkSurfaceBuild;
use winit::{
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

use crate::{camera_helper, Matrix4, Vector3, GL_SCALE};

pub struct Renderer {
    device: Arc<Device>,
    surface: Arc<Surface>,
    queue: Arc<Queue>,
    swap_chain: Arc<Swapchain>,
    image_index: u32,
    images: Vec<Arc<SwapchainImage>>,
    recreate_swapchain: bool,
    previous_frame_end: Option<Box<dyn GpuFuture>>,
    frame_buffers: Vec<Arc<Framebuffer>>,
    render_pass: Arc<RenderPass>,
    viewport: Viewport,
    command_buffer_allocator: StandardCommandBufferAllocator,
    descriptor_set_allocator: StandardDescriptorSetAllocator,
    memory_allocator: StandardMemoryAllocator,
    camera: Camera<f32>,
    depth_format: Format,
    _debug_callback: Option<DebugUtilsMessenger>,
    msaa_sample: SampleCounts,
}

impl Renderer {
    pub fn new(
        event_loop: &EventLoop<()>,
        title: &str,
        width: f64,
        height: f64,
        v_sync: bool,
    ) -> Self {
        let library = VulkanLibrary::new().unwrap();
        let mut required_extensions = vulkano_win::required_extensions(&library);
        if cfg!(feature = "enable_debug") {
            required_extensions.ext_debug_utils = true;
        }

        let instance = Instance::new(
            library,
            InstanceCreateInfo {
                enabled_extensions: required_extensions,
                enumerate_portability: true,
                ..Default::default()
            },
        )
        .expect("Failed to create instance");

        let _debug_callback = if cfg!(feature = "enable_debug") {
            unsafe {
                DebugUtilsMessenger::new(
                    instance.clone(),
                    DebugUtilsMessengerCreateInfo {
                        message_severity: DebugUtilsMessageSeverity::ERROR
                            | DebugUtilsMessageSeverity::WARNING
                            | DebugUtilsMessageSeverity::INFO
                            | DebugUtilsMessageSeverity::VERBOSE,
                        message_type: DebugUtilsMessageType::GENERAL
                            | DebugUtilsMessageType::VALIDATION
                            | DebugUtilsMessageType::PERFORMANCE,
                        ..DebugUtilsMessengerCreateInfo::user_callback(Arc::new(|msg| {
                            let severity = if msg
                                .severity
                                .intersects(DebugUtilsMessageSeverity::ERROR)
                            {
                                "error"
                            } else if msg.severity.intersects(DebugUtilsMessageSeverity::WARNING) {
                                "warning"
                            } else if msg.severity.intersects(DebugUtilsMessageSeverity::INFO) {
                                "information"
                            } else if msg.severity.intersects(DebugUtilsMessageSeverity::VERBOSE) {
                                "verbose"
                            } else {
                                panic!("no-impl");
                            };

                            let ty = if msg.ty.intersects(DebugUtilsMessageType::GENERAL) {
                                "general"
                            } else if msg.ty.intersects(DebugUtilsMessageType::VALIDATION) {
                                "validation"
                            } else if msg.ty.intersects(DebugUtilsMessageType::PERFORMANCE) {
                                "performance"
                            } else {
                                panic!("no-impl");
                            };

                            println!(
                                "{} {} {}: {}",
                                msg.layer_prefix.unwrap_or("unknown"),
                                ty,
                                severity,
                                msg.description
                            );
                        }))
                    },
                )
                .ok()
            }
        } else {
            None
        };

        let surface = WindowBuilder::new()
            .with_inner_size(winit::dpi::LogicalSize::new(width, height))
            .with_title(title)
            .build_vk_surface(event_loop, instance.clone())
            .unwrap();

        let (device, queue) = Self::create_device(instance, surface.clone());

        let msaa_sample = Self::get_max_usable_sample_count(device.physical_device().clone());

        let mut viewport = Viewport {
            origin: [0.0, 0.0],
            dimensions: [0.0, 0.0],
            depth_range: 0.0..1.0,
        };
        let (swap_chain, images) = Self::create_swap_chain(
            surface.clone(),
            device.physical_device().clone(),
            device.clone(),
            if v_sync {
                PresentMode::Fifo
            } else {
                PresentMode::Immediate
            },
        );
        let candidates = [
            Format::D32_SFLOAT,
            Format::D32_SFLOAT_S8_UINT,
            Format::D24_UNORM_S8_UINT,
            Format::D32_SFLOAT,
        ];
        let depth_format = candidates
            .into_iter()
            .find(|&f| {
                if let Ok(props) = device.physical_device().format_properties(f) {
                    props
                        .optimal_tiling_features
                        .contains(FormatFeatures::DEPTH_STENCIL_ATTACHMENT)
                } else {
                    false
                }
            })
            .unwrap_or(Format::D16_UNORM);
        let render_pass = vulkano::single_pass_renderpass!(
            device.clone(),
            attachments: {
                intermediary: {
                    load: Clear,
                    store: DontCare,
                    format: swap_chain.image_format(),
                    samples: msaa_sample.max_count(),
                },
                depth: {
                    load: Clear,
                    store: DontCare,
                    format: depth_format,
                    samples: msaa_sample.max_count(),
                },
                color: {
                    load: DontCare,
                    store: Store,
                    format: swap_chain.image_format(),
                    samples: 1,
                }
            },
            pass: {
                color: [intermediary],
                depth_stencil: {depth},
                resolve: [color],
            }
        )
        .unwrap();
        let memory_allocator = Arc::new(StandardMemoryAllocator::new_default(device.clone()));
        let frame_buffers = Self::window_size_dependent_setup(
            memory_allocator,
            &images,
            render_pass.clone(),
            &mut viewport,
            swap_chain.image_format(),
            depth_format,
            msaa_sample.max_count(),
        );

        let mut camera =
            FirstPerson::new([0., -500.0, 120.0], FirstPersonSettings::keyboard_wasd()).camera(0.);
        camera.set_yaw_pitch(0., -std::f32::consts::PI / 2.0);

        let command_buffer_allocator =
            StandardCommandBufferAllocator::new(queue.device().clone(), Default::default());
        let descriptor_set_allocator = StandardDescriptorSetAllocator::new(queue.device().clone());
        let memory_allocator = StandardMemoryAllocator::new_default(queue.device().clone());

        let previous_frame_end = Some(sync::now(device.clone()).boxed());

        Renderer {
            device,
            surface,
            queue,
            swap_chain,
            image_index: 0,
            images,
            previous_frame_end,
            recreate_swapchain: false,
            frame_buffers,
            render_pass,
            viewport,
            command_buffer_allocator,
            descriptor_set_allocator,
            memory_allocator,
            camera,
            depth_format,
            _debug_callback,
            msaa_sample,
        }
    }

    fn get_max_usable_sample_count(physical: Arc<PhysicalDevice>) -> SampleCounts {
        let properties = physical.properties();
        let counts =
            properties.framebuffer_color_sample_counts & properties.framebuffer_depth_sample_counts;
        [
            SampleCounts::SAMPLE_64,
            SampleCounts::SAMPLE_32,
            SampleCounts::SAMPLE_16,
            SampleCounts::SAMPLE_8,
            SampleCounts::SAMPLE_4,
            SampleCounts::SAMPLE_2,
        ]
        .into_iter()
        .find(|c| counts.contains(*c))
        .unwrap_or(SampleCounts::SAMPLE_1)
    }

    pub fn get_projection(&self, fov: f32, near_clip: f32, far_clip: f32) -> Matrix4 {
        let draw_size = self.window().inner_size();
        Matrix4::from({
            let mut projection = CameraPerspective {
                fov,
                near_clip: near_clip * GL_SCALE,
                far_clip: far_clip * GL_SCALE,
                aspect_ratio: (draw_size.width as f32) / (draw_size.height as f32),
            }
            .projection();
            projection[0][1] = -projection[0][1];
            projection[1][1] = -projection[1][1];
            projection[2][1] = -projection[2][1];
            projection
        })
    }

    pub fn get_view(&self) -> Matrix4 {
        Matrix4::from(self.camera.orthogonal())
    }

    fn create_device(instance: Arc<Instance>, surface: Arc<Surface>) -> (Arc<Device>, Arc<Queue>) {
        let device_extensions = DeviceExtensions {
            khr_swapchain: true,
            ..DeviceExtensions::default()
        };

        let (physical_device, queue_family) = instance
            .enumerate_physical_devices()
            .unwrap()
            .filter(|p| p.supported_extensions().contains(&device_extensions))
            .filter_map(|p| {
                p.queue_family_properties()
                    .iter()
                    .enumerate()
                    .position(|(i, q)| {
                        q.queue_flags.intersects(QueueFlags::GRAPHICS)
                            && p.surface_support(i as u32, &surface).unwrap_or(false)
                    })
                    .map(|i| (p, i as u32))
            })
            .min_by_key(|(p, _)| match p.properties().device_type {
                PhysicalDeviceType::DiscreteGpu => 0,
                PhysicalDeviceType::IntegratedGpu => 1,
                PhysicalDeviceType::VirtualGpu => 2,
                PhysicalDeviceType::Cpu => 3,
                PhysicalDeviceType::Other => 4,
                _ => 5,
            })
            .unwrap();

        let features = Features::default();
        let (device, mut queues) = {
            Device::new(
                physical_device,
                DeviceCreateInfo {
                    enabled_extensions: device_extensions,
                    enabled_features: features,
                    queue_create_infos: vec![QueueCreateInfo {
                        queue_family_index: queue_family,
                        ..Default::default()
                    }],
                    ..Default::default()
                },
            )
            .unwrap()
        };
        (device, queues.next().unwrap())
    }

    fn create_swap_chain(
        surface: Arc<Surface>,
        physical: Arc<PhysicalDevice>,
        device: Arc<Device>,
        present_mode: PresentMode,
    ) -> (Arc<Swapchain>, Vec<Arc<SwapchainImage>>) {
        let caps = physical
            .surface_capabilities(&surface, Default::default())
            .unwrap();
        let alpha = caps.supported_composite_alpha.into_iter().next().unwrap();
        let format = physical
            .surface_formats(&surface, Default::default())
            .unwrap()
            .into_iter()
            .find(|&(f, c)| f == Format::B8G8R8A8_UNORM && c == ColorSpace::SrgbNonLinear);
        let image_extent: [u32; 2] = surface
            .object()
            .unwrap()
            .downcast_ref::<Window>()
            .unwrap()
            .inner_size()
            .into();
        Swapchain::new(
            device,
            surface,
            SwapchainCreateInfo {
                min_image_count: caps.min_image_count,
                image_format: format.map(|f| f.0),
                image_color_space: format.map_or(ColorSpace::SrgbNonLinear, |f| f.1),
                image_extent,
                image_array_layers: 1,
                image_usage: ImageUsage::TRANSFER_DST | ImageUsage::COLOR_ATTACHMENT,
                pre_transform: SurfaceTransform::Identity,
                composite_alpha: alpha,
                present_mode,
                clipped: true,
                full_screen_exclusive: FullScreenExclusive::Default,
                ..Default::default()
            },
        )
        .unwrap()
    }

    pub fn device(&self) -> Arc<Device> {
        self.device.clone()
    }

    pub fn color_format(&self) -> Format {
        self.swap_chain.image_format()
    }

    pub fn sample_count(&self) -> SampleCount {
        self.msaa_sample.max_count()
    }

    pub fn window(&self) -> &Window {
        self.surface
            .object()
            .unwrap()
            .downcast_ref::<Window>()
            .unwrap()
    }

    pub fn queue(&self) -> Arc<Queue> {
        self.queue.clone()
    }

    pub fn frame_buffer(&self) -> Arc<Framebuffer> {
        self.frame_buffers[self.image_index as usize].clone()
    }

    pub fn image(&self) -> Arc<SwapchainImage> {
        self.images[self.image_index as usize].clone()
    }

    pub fn render_pass(&self) -> Arc<RenderPass> {
        self.render_pass.clone()
    }

    pub fn viewport(&self) -> Viewport {
        self.viewport.clone()
    }

    pub fn command_buffer_allocator(&self) -> &StandardCommandBufferAllocator {
        &self.command_buffer_allocator
    }

    pub fn descriptor_set_allocator(&self) -> &StandardDescriptorSetAllocator {
        &self.descriptor_set_allocator
    }

    pub fn memory_allocator(&self) -> &StandardMemoryAllocator {
        &self.memory_allocator
    }

    pub fn resize(&mut self) {
        self.recreate_swapchain = true
    }

    pub fn start_frame(&mut self) -> Result<Box<dyn GpuFuture>, AcquireError> {
        if self.recreate_swapchain {
            self.recreate_swapchain_and_views();
        }

        let (image_num, suboptimal, acquire_future) =
            match swapchain::acquire_next_image(self.swap_chain.clone(), None) {
                Ok(r) => r,
                Err(AcquireError::OutOfDate) => {
                    self.recreate_swapchain = true;
                    return Err(AcquireError::OutOfDate);
                }
                Err(e) => panic!("Failed to acquire next image: {:?}", e),
            };
        if suboptimal {
            self.recreate_swapchain = true;
        }
        self.image_index = image_num as _;

        let future = self.previous_frame_end.take().unwrap().join(acquire_future);

        Ok(future.boxed())
    }

    pub fn finish_frame(&mut self, after_future: Box<dyn GpuFuture>) {
        let future = after_future
            .then_swapchain_present(
                self.queue.clone(),
                SwapchainPresentInfo::swapchain_image_index(
                    self.swap_chain.clone(),
                    self.image_index,
                ),
            )
            .then_signal_fence_and_flush();
        match future {
            Ok(future) => {
                match future.wait(None) {
                    Ok(x) => x,
                    Err(err) => println!("{:?}", err),
                }
                self.previous_frame_end = Some(future.boxed());
            }
            Err(FlushError::OutOfDate) => {
                self.recreate_swapchain = true;
                self.previous_frame_end = Some(sync::now(self.device.clone()).boxed());
            }
            Err(e) => {
                println!("Failed to flush future: {:?}", e);
                self.previous_frame_end = Some(sync::now(self.device.clone()).boxed());
            }
        }
    }

    fn recreate_swapchain_and_views(&mut self) {
        let dimensions: [u32; 2] = self.window().inner_size().into();
        let (new_swapchain, new_images) = match self.swap_chain.recreate(SwapchainCreateInfo {
            image_extent: dimensions,
            ..self.swap_chain.create_info()
        }) {
            Ok(r) => r,
            Err(SwapchainCreationError::ImageExtentNotSupported {
                provided,
                min_supported,
                max_supported,
            }) => {
                println!(
                    "provided {:?}, min_supported = {:?}, max_supported = {:?}",
                    provided, min_supported, max_supported
                );
                return;
            }
            Err(e) => panic!("Failed to recreate swapchain: {:?}", e),
        };

        self.swap_chain = new_swapchain;
        let memory_allocator = Arc::new(StandardMemoryAllocator::new_default(self.device.clone()));
        let format = self.color_format();
        self.frame_buffers = Self::window_size_dependent_setup(
            memory_allocator,
            &new_images,
            self.render_pass.clone(),
            &mut self.viewport,
            format,
            self.depth_format,
            self.msaa_sample.max_count(),
        );
        self.images = new_images;
        self.recreate_swapchain = false;
    }

    fn window_size_dependent_setup(
        memory_allocator: Arc<StandardMemoryAllocator>,
        images: &[Arc<SwapchainImage>],
        render_pass: Arc<RenderPass>,
        viewport: &mut Viewport,
        color_format: Format,
        depth_format: Format,
        samples: SampleCount,
    ) -> Vec<Arc<Framebuffer>> {
        let dimensions = images[0].dimensions().width_height();
        viewport.dimensions = [dimensions[0] as f32, dimensions[1] as f32];

        let color_image = ImageView::new_default(
            AttachmentImage::transient_multisampled(
                &memory_allocator,
                dimensions,
                samples,
                color_format,
            )
            .unwrap(),
        )
        .unwrap();

        let depth_buffer = ImageView::new_default(
            AttachmentImage::transient_multisampled(
                &memory_allocator,
                dimensions,
                samples,
                depth_format,
            )
            .unwrap(),
        )
        .unwrap();

        images
            .iter()
            .map(|image| {
                let view = ImageView::new_default(image.clone()).unwrap();
                Framebuffer::new(
                    render_pass.clone(),
                    FramebufferCreateInfo {
                        attachments: vec![color_image.clone(), depth_buffer.clone(), view],
                        ..Default::default()
                    },
                )
                .unwrap()
            })
            .collect::<Vec<_>>()
    }

    pub fn move_camera(&mut self, pos: Vector3, rot: Vector3) {
        let pos = pos * GL_SCALE;
        self.camera.position = [pos.x, pos.y, pos.z];
        camera_helper::set_camera_angle(&mut self.camera, rot);
    }
}
