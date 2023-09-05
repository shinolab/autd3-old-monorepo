/*
 * File: trans_viewer.rs
 * Project: src
 * Created Date: 30/11/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 05/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2021 Hapis Lab. All rights reserved.
 *
 */

use std::{f32::consts::PI, io::Cursor, sync::Arc};

use autd3::autd3_device::AUTD3;
use bytemuck::{Pod, Zeroable};
use vulkano::{
    buffer::{Buffer, BufferCreateInfo, BufferUsage, Subbuffer},
    command_buffer::{
        AutoCommandBufferBuilder, CommandBufferUsage, PrimaryAutoCommandBuffer,
        PrimaryCommandBufferAbstract,
    },
    descriptor_set::{PersistentDescriptorSet, WriteDescriptorSet},
    format::Format,
    image::{view::ImageView, ImageDimensions, ImageViewAbstract, ImmutableImage, MipmapsCount},
    memory::allocator::{AllocationCreateInfo, MemoryUsage},
    pipeline::{
        graphics::{
            color_blend::ColorBlendState, depth_stencil::DepthStencilState,
            input_assembly::InputAssemblyState, multisample::MultisampleState,
            vertex_input::Vertex, viewport::ViewportState,
        },
        GraphicsPipeline, Pipeline, PipelineBindPoint,
    },
    render_pass::Subpass,
    sampler::{Filter, Sampler, SamplerAddressMode, SamplerCreateInfo, SamplerMipmapMode},
    sync::GpuFuture,
};

use crate::{
    common::coloring_method::{coloring_hsv, ColoringMethod},
    renderer::Renderer,
    sound_sources::SoundSources,
    update_flag::UpdateFlag,
    Matrix4,
};

#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Zeroable, Pod, Vertex)]
struct CircleVertex {
    #[format(R32G32B32A32_SFLOAT)]
    position: [f32; 4],
    #[format(R32G32_SFLOAT)]
    tex_coords: [f32; 2],
}

#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Zeroable, Pod, Vertex)]
struct ModelInstanceData {
    #[format(R32G32B32A32_SFLOAT)]
    model: [[f32; 4]; 4],
}

#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Zeroable, Pod, Vertex)]
struct ColorInstanceData {
    #[format(R32G32B32A32_SFLOAT)]
    color: [f32; 4],
}

#[allow(clippy::needless_question_mark)]
mod vs {
    vulkano_shaders::shader! {
        ty: "vertex",
        path: "./assets/shaders/circle.vert"
    }
}

#[allow(clippy::needless_question_mark)]
mod fs {
    vulkano_shaders::shader! {
        ty: "fragment",
        path: "./assets/shaders/circle.frag"
    }
}

pub struct TransViewer {
    vertices: Subbuffer<[CircleVertex]>,
    indices: Subbuffer<[u32]>,
    model_instance_data: Option<Subbuffer<[ModelInstanceData]>>,
    color_instance_data: Option<Subbuffer<[ColorInstanceData]>>,
    pipeline: Arc<GraphicsPipeline>,
    texture_desc_set: Arc<PersistentDescriptorSet>,
    coloring_method: ColoringMethod,
}

impl TransViewer {
    pub fn new(renderer: &Renderer) -> Self {
        let device = renderer.device();
        let vertices = Self::create_vertices(renderer);
        let indices = Self::create_indices(renderer);

        let vs = vs::load(device.clone()).unwrap();
        let fs = fs::load(device.clone()).unwrap();

        let subpass = Subpass::from(renderer.render_pass(), 0).unwrap();
        let pipeline = GraphicsPipeline::start()
            .vertex_input_state([
                CircleVertex::per_vertex(),
                ModelInstanceData::per_instance(),
                ColorInstanceData::per_instance(),
            ])
            .vertex_shader(vs.entry_point("main").unwrap(), ())
            .input_assembly_state(InputAssemblyState::new())
            .viewport_state(ViewportState::viewport_dynamic_scissor_irrelevant())
            .fragment_shader(fs.entry_point("main").unwrap(), ())
            .color_blend_state(ColorBlendState::new(subpass.num_color_attachments()).blend_alpha())
            .depth_stencil_state(DepthStencilState::simple_depth_test())
            .multisample_state(MultisampleState {
                rasterization_samples: renderer.sample_count(),
                ..MultisampleState::default()
            })
            .render_pass(subpass)
            .build(device)
            .unwrap();

        let texture_desc_set = Self::create_texture_desc_set(pipeline.clone(), renderer);
        Self {
            vertices,
            indices,
            model_instance_data: None,
            color_instance_data: None,
            pipeline,
            texture_desc_set,
            coloring_method: coloring_hsv,
        }
    }

    pub fn init(&mut self, renderer: &Renderer, sources: &SoundSources) {
        self.color_instance_data = Some(Self::create_color_instance_data(
            renderer,
            sources,
            self.coloring_method,
        ));
        self.model_instance_data = Some(Self::create_model_instance_data(renderer, sources));
    }

    pub fn update(&mut self, sources: &SoundSources, update_flag: &UpdateFlag) {
        if update_flag.contains(UpdateFlag::UPDATE_SOURCE_DRIVE)
            || update_flag.contains(UpdateFlag::UPDATE_SOURCE_ALPHA)
            || update_flag.contains(UpdateFlag::UPDATE_SOURCE_FLAG)
        {
            self.update_color_instance_data(sources)
        }
    }

    pub fn render(
        &mut self,
        view: Matrix4,
        proj: Matrix4,
        builder: &mut AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>,
    ) {
        let pc = vs::PushConsts {
            proj_view: (proj * view).into(),
        };

        if let (Some(model), Some(color)) = (&self.model_instance_data, &self.color_instance_data) {
            builder
                .bind_pipeline_graphics(self.pipeline.clone())
                .bind_descriptor_sets(
                    PipelineBindPoint::Graphics,
                    self.pipeline.layout().clone(),
                    0,
                    self.texture_desc_set.clone(),
                )
                .push_constants(self.pipeline.layout().clone(), 0, pc)
                .bind_vertex_buffers(0, (self.vertices.clone(), model.clone(), color.clone()))
                .bind_index_buffer(self.indices.clone())
                .draw_indexed(self.indices.len() as u32, model.len() as u32, 0, 0, 0)
                .unwrap();
        }
    }

    fn create_model_instance_data(
        renderer: &Renderer,
        sources: &SoundSources,
    ) -> Subbuffer<[ModelInstanceData]> {
        Buffer::from_iter(
            renderer.memory_allocator(),
            BufferCreateInfo {
                usage: BufferUsage::STORAGE_BUFFER | BufferUsage::VERTEX_BUFFER,
                ..Default::default()
            },
            AllocationCreateInfo {
                usage: MemoryUsage::Upload,
                ..Default::default()
            },
            sources
                .positions()
                .zip(sources.rotations())
                .map(|(pos, rot)| {
                    #[allow(clippy::unnecessary_cast)]
                    let s = 0.5 * AUTD3::TRANS_SPACING as f32;
                    let mut m = Matrix4::from_scale(s);
                    m[3][0] = pos[0];
                    m[3][1] = pos[1];
                    m[3][2] = pos[2];
                    let rotm = Matrix4::from(*rot);
                    ModelInstanceData {
                        model: (m * rotm).into(),
                    }
                }),
        )
        .unwrap()
    }

    fn create_color_instance_data(
        renderer: &Renderer,
        sources: &SoundSources,
        coloring_method: ColoringMethod,
    ) -> Subbuffer<[ColorInstanceData]> {
        Buffer::from_iter(
            renderer.memory_allocator(),
            BufferCreateInfo {
                usage: BufferUsage::STORAGE_BUFFER | BufferUsage::VERTEX_BUFFER,
                ..Default::default()
            },
            AllocationCreateInfo {
                usage: MemoryUsage::Upload,
                ..Default::default()
            },
            sources
                .drives()
                .zip(sources.visibilities())
                .map(|(drive, &v)| {
                    let color = coloring_method(drive.phase / (2.0 * PI), drive.amp, v);
                    ColorInstanceData { color }
                }),
        )
        .unwrap()
    }

    fn create_vertices(renderer: &Renderer) -> Subbuffer<[CircleVertex]> {
        Buffer::from_iter(
            renderer.memory_allocator(),
            BufferCreateInfo {
                usage: BufferUsage::VERTEX_BUFFER,
                ..Default::default()
            },
            AllocationCreateInfo {
                usage: MemoryUsage::Upload,
                ..Default::default()
            },
            [
                CircleVertex {
                    position: [-1.0, -1.0, 0.0, 1.0],
                    tex_coords: [0.0, 1.0],
                },
                CircleVertex {
                    position: [1.0, -1.0, 0.0, 1.0],
                    tex_coords: [1.0, 1.0],
                },
                CircleVertex {
                    position: [1.0, 1.0, 0.0, 1.0],
                    tex_coords: [1.0, 0.0],
                },
                CircleVertex {
                    position: [-1.0, 1.0, 0.0, 1.0],
                    tex_coords: [0.0, 0.0],
                },
            ]
            .iter()
            .cloned(),
        )
        .unwrap()
    }

    fn create_indices(renderer: &Renderer) -> Subbuffer<[u32]> {
        let indices: Vec<u32> = vec![0, 1, 2, 2, 3, 0];
        Buffer::from_iter(
            renderer.memory_allocator(),
            BufferCreateInfo {
                usage: BufferUsage::INDEX_BUFFER,
                ..Default::default()
            },
            AllocationCreateInfo {
                usage: MemoryUsage::Upload,
                ..Default::default()
            },
            indices.into_iter(),
        )
        .unwrap()
    }

    fn create_texture_desc_set(
        pipeline: Arc<GraphicsPipeline>,
        renderer: &Renderer,
    ) -> Arc<PersistentDescriptorSet> {
        let (uploads, texture) = Self::load_image(renderer);
        let sampler = Sampler::new(
            pipeline.device().clone(),
            SamplerCreateInfo {
                mag_filter: Filter::Linear,
                min_filter: Filter::Linear,
                mipmap_mode: SamplerMipmapMode::Nearest,
                address_mode: [SamplerAddressMode::Repeat; 3],
                mip_lod_bias: 0.0,
                ..Default::default()
            },
        )
        .unwrap();
        let layout = pipeline.layout().set_layouts().get(0).unwrap();

        uploads
            .execute(renderer.queue())
            .unwrap()
            .then_signal_fence_and_flush()
            .unwrap()
            .wait(None)
            .unwrap();

        PersistentDescriptorSet::new(
            renderer.descriptor_set_allocator(),
            layout.clone(),
            [WriteDescriptorSet::image_view_sampler(0, texture, sampler)],
        )
        .unwrap()
    }

    fn load_image(renderer: &Renderer) -> (PrimaryAutoCommandBuffer, Arc<dyn ImageViewAbstract>) {
        let png_bytes = include_bytes!("../assets/textures/circle.png").to_vec();
        let cursor = Cursor::new(png_bytes);
        let decoder = png::Decoder::new(cursor);
        let mut reader = decoder.read_info().unwrap();
        let info = reader.info();
        let dimensions = ImageDimensions::Dim2d {
            width: info.width,
            height: info.height,
            array_layers: 1,
        };
        let mut image_data = Vec::new();
        image_data.resize((info.width * info.height * 4) as usize, 0);
        reader.next_frame(&mut image_data).unwrap();

        let mut uploads = AutoCommandBufferBuilder::primary(
            renderer.command_buffer_allocator(),
            renderer.queue().queue_family_index(),
            CommandBufferUsage::OneTimeSubmit,
        )
        .unwrap();

        let image = ImmutableImage::from_iter(
            renderer.memory_allocator(),
            image_data.iter().cloned(),
            dimensions,
            MipmapsCount::One,
            Format::R8G8B8A8_SRGB,
            &mut uploads,
        )
        .unwrap();
        let image = ImageView::new_default(image).unwrap();

        (uploads.build().unwrap(), image)
    }

    fn update_color_instance_data(&mut self, sources: &SoundSources) {
        if let Some(data) = &mut self.color_instance_data {
            data.write()
                .unwrap()
                .iter_mut()
                .zip(sources.drives().zip(sources.visibilities()))
                .for_each(|(d, (drive, &v))| {
                    d.color = (self.coloring_method)(drive.phase / (2.0 * PI), drive.amp, v);
                });
        }
    }
}
