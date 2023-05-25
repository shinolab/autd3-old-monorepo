/*
 * File: slice_viewer.rs
 * Project: src
 * Created Date: 11/11/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 25/05/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2021 Hapis Lab. All rights reserved.
 *
 */

use std::sync::Arc;

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
            input_assembly::InputAssemblyState, vertex_input::Vertex, viewport::ViewportState,
        },
        GraphicsPipeline, Pipeline, PipelineBindPoint,
    },
    render_pass::Subpass,
    sampler::{Filter, Sampler, SamplerAddressMode, SamplerCreateInfo, SamplerMipmapMode},
    sync::GpuFuture,
};

use crate::{
    model::{Model, ModelVertex},
    renderer::Renderer,
    settings::Settings,
    Matrix4, Quaternion, Vector3, GL_SCALE,
};

#[allow(clippy::needless_question_mark)]
mod vs {
    vulkano_shaders::shader! {
        ty: "vertex",
        path: "./assets/shaders/base.vert"
    }
}

#[allow(clippy::needless_question_mark)]
mod fs {
    vulkano_shaders::shader! {
        ty: "fragment",
        path: "./assets/shaders/base.frag"
    }
}

pub struct DeviceViewer {
    vertices: Subbuffer<[ModelVertex]>,
    indices: Subbuffer<[u32]>,
    texture_desc_set: Arc<PersistentDescriptorSet>,
    pipeline: Arc<GraphicsPipeline>,
}

impl DeviceViewer {
    pub fn new(renderer: &Renderer, model: &Model) -> Self {
        let device = renderer.device();
        let vertices = Self::create_vertices(renderer, &model.vertices);
        let indices = Self::create_indices(renderer, &model.indices);

        let vs = vs::load(device.clone()).unwrap();
        let fs = fs::load(device.clone()).unwrap();

        let subpass = Subpass::from(renderer.render_pass(), 0).unwrap();
        let pipeline = GraphicsPipeline::start()
            .vertex_input_state([ModelVertex::per_vertex()])
            .vertex_shader(vs.entry_point("main").unwrap(), ())
            .input_assembly_state(InputAssemblyState::new())
            .viewport_state(ViewportState::viewport_dynamic_scissor_irrelevant())
            .fragment_shader(fs.entry_point("main").unwrap(), ())
            .color_blend_state(ColorBlendState::new(subpass.num_color_attachments()).blend_alpha())
            .depth_stencil_state(DepthStencilState::simple_depth_test())
            .render_pass(subpass)
            .build(device)
            .unwrap();

        let texture_desc_set =
            Self::create_texture_desc_set(pipeline.clone(), renderer, &model.image);

        Self {
            vertices,
            indices,
            texture_desc_set,
            pipeline,
        }
    }

    pub fn render(
        &mut self,
        model: &Model,
        pos_rot: &[(Vector3, Quaternion)],
        view_proj: (Matrix4, Matrix4),
        setting: &Settings,
        builder: &mut AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>,
    ) {
        builder
            .bind_vertex_buffers(0, self.vertices.clone())
            .bind_index_buffer(self.indices.clone())
            .bind_descriptor_sets(
                PipelineBindPoint::Graphics,
                self.pipeline.layout().clone(),
                0,
                self.texture_desc_set.clone(),
            );

        let (view, proj) = view_proj;

        for (&(pos, rot), s) in pos_rot.iter().zip(setting.shows.iter()) {
            if !s {
                continue;
            }

            for primitive in &model.primitives {
                let material = &model.materials[primitive.material_index];

                let pcf = fs::PushConsts {
                    proj_view: (proj * view).into(),
                    model: (Matrix4::from_translation(pos) * Matrix4::from(rot)).into(),
                    lightPos: [
                        setting.light_pos_x * GL_SCALE,
                        setting.light_pos_y * GL_SCALE,
                        setting.light_pos_z * GL_SCALE,
                        1.,
                    ],
                    viewPos: [
                        setting.camera_pos_x * GL_SCALE,
                        setting.camera_pos_y * GL_SCALE,
                        setting.camera_pos_z * GL_SCALE,
                        1.,
                    ],
                    ambient: setting.ambient,
                    specular: setting.specular,
                    lightPower: setting.light_power,
                    metallic: material.metallic_factor,
                    roughness: material.roughness_factor,
                    baseColorR: material.base_color_factor[0],
                    baseColorG: material.base_color_factor[1],
                    baseColorB: material.base_color_factor[2],
                    hasTexture: if material.base_color_texture_idx.is_some() {
                        1
                    } else {
                        0
                    },
                };

                builder
                    .bind_pipeline_graphics(self.pipeline.clone())
                    .push_constants(self.pipeline.layout().clone(), 0, pcf)
                    .draw_indexed(primitive.index_count, 1, primitive.first_index, 0, 0)
                    .unwrap();
            }
        }
    }

    fn create_vertices(renderer: &Renderer, vertices: &[ModelVertex]) -> Subbuffer<[ModelVertex]> {
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
            vertices.iter().cloned(),
        )
        .unwrap()
    }

    fn create_indices(renderer: &Renderer, indices: &[u32]) -> Subbuffer<[u32]> {
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
            indices.iter().cloned(),
        )
        .unwrap()
    }

    fn create_texture_desc_set(
        pipeline: Arc<GraphicsPipeline>,
        renderer: &Renderer,
        image: &gltf::image::Data,
    ) -> Arc<PersistentDescriptorSet> {
        let (uploads, texture) = Self::load_image(renderer, image);
        let sampler = Sampler::new(
            pipeline.device().clone(),
            SamplerCreateInfo {
                mag_filter: Filter::Linear,
                min_filter: Filter::Linear,
                mipmap_mode: SamplerMipmapMode::Linear,
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

    fn load_image(
        renderer: &Renderer,
        image: &gltf::image::Data,
    ) -> (PrimaryAutoCommandBuffer, Arc<dyn ImageViewAbstract>) {
        let dimensions = ImageDimensions::Dim2d {
            width: image.width,
            height: image.height,
            array_layers: 1,
        };
        let mut image_data = Vec::new();
        image_data.resize((image.width * image.height * 4) as usize, 0);
        image_data.copy_from_slice(&image.pixels);

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
            MipmapsCount::Log2,
            Format::R8G8B8A8_SRGB,
            &mut uploads,
        )
        .unwrap();
        let image = ImageView::new_default(image).unwrap();

        (uploads.build().unwrap(), image)
    }
}
