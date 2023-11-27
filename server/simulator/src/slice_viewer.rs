/*
 * File: slice_viewer.rs
 * Project: src
 * Created Date: 11/11/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 30/10/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2021 Hapis Lab. All rights reserved.
 *
 */

use std::sync::Arc;

use bytemuck::{Pod, Zeroable};
use vulkano::{
    buffer::{Buffer, BufferCreateInfo, BufferUsage, Subbuffer},
    command_buffer::{AutoCommandBufferBuilder, PrimaryAutoCommandBuffer},
    descriptor_set::{PersistentDescriptorSet, WriteDescriptorSet},
    memory::allocator::{AllocationCreateInfo, MemoryTypeFilter},
    pipeline::{
        graphics::{
            color_blend::{AttachmentBlend, ColorBlendAttachmentState, ColorBlendState},
            depth_stencil::{DepthState, DepthStencilState},
            input_assembly::{InputAssemblyState, PrimitiveTopology},
            multisample::MultisampleState,
            rasterization::RasterizationState,
            vertex_input::{Vertex, VertexDefinition},
            viewport::ViewportState,
            GraphicsPipelineCreateInfo,
        },
        layout::PipelineDescriptorSetLayoutCreateInfo,
        DynamicState, GraphicsPipeline, Pipeline, PipelineBindPoint, PipelineLayout,
        PipelineShaderStageCreateInfo,
    },
    render_pass::Subpass,
};

use crate::{
    common::transform::{to_gl_pos, to_gl_rot},
    renderer::Renderer,
    update_flag::UpdateFlag,
    viewer_settings::ViewerSettings,
    Matrix4,
};

#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Zeroable, Pod, Vertex)]
struct SliceVertex {
    #[format(R32G32B32A32_SFLOAT)]
    position: [f32; 4],
    #[format(R32G32_SFLOAT)]
    tex_coords: [f32; 2],
}

#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Zeroable, Pod)]
struct Data {
    world: [[f32; 4]; 4],
    view: [[f32; 4]; 4],
    proj: [[f32; 4]; 4],
    width: u32,
    height: u32,
    _dummy_0: u32,
    _dummy_1: u32,
}

#[allow(clippy::needless_question_mark)]
mod vs {
    vulkano_shaders::shader! {
        ty: "vertex",
        path: "./assets/shaders/slice.vert"
    }
}

#[allow(clippy::needless_question_mark)]
mod fs {
    vulkano_shaders::shader! {
        ty: "fragment",
        path: "./assets/shaders/slice.frag"
    }
}

pub struct SliceViewer {
    vertices: Subbuffer<[SliceVertex]>,
    indices: Subbuffer<[u32]>,
    pipeline: Arc<GraphicsPipeline>,
    model: Matrix4,
    field_image_view: Subbuffer<[[f32; 4]]>,
}

impl SliceViewer {
    pub fn new(renderer: &Renderer, settings: &ViewerSettings) -> anyhow::Result<Self> {
        let device = renderer.device();
        let vertices = Self::create_vertices(renderer, settings)?;
        let indices = Self::create_indices(renderer)?;

        let vs = vs::load(device.clone())?.entry_point("main").unwrap();
        let fs = fs::load(device.clone())?.entry_point("main").unwrap();

        let vertex_input_state =
            SliceVertex::per_vertex().definition(&vs.info().input_interface)?;
        let stages = [
            PipelineShaderStageCreateInfo::new(vs),
            PipelineShaderStageCreateInfo::new(fs),
        ];
        let layout = PipelineLayout::new(
            device.clone(),
            PipelineDescriptorSetLayoutCreateInfo::from_stages(&stages)
                .into_pipeline_layout_create_info(device.clone())?,
        )?;
        let subpass = Subpass::from(renderer.render_pass(), 0).unwrap();

        let pipeline = GraphicsPipeline::new(
            device.clone(),
            None,
            GraphicsPipelineCreateInfo {
                stages: stages.into_iter().collect(),
                vertex_input_state: Some(vertex_input_state),
                input_assembly_state: Some(InputAssemblyState {
                    topology: PrimitiveTopology::TriangleStrip,
                    ..Default::default()
                }),
                viewport_state: Some(ViewportState::default()),
                rasterization_state: Some(RasterizationState::default()),
                multisample_state: Some(MultisampleState {
                    rasterization_samples: renderer.sample_count(),
                    ..MultisampleState::default()
                }),
                color_blend_state: Some(ColorBlendState::with_attachment_states(
                    subpass.num_color_attachments(),
                    ColorBlendAttachmentState {
                        blend: Some(AttachmentBlend::alpha()),
                        ..Default::default()
                    },
                )),
                depth_stencil_state: Some(DepthStencilState {
                    depth: Some(DepthState::simple()),
                    ..Default::default()
                }),
                dynamic_state: [DynamicState::Viewport].into_iter().collect(),
                subpass: Some(subpass.into()),
                ..GraphicsPipelineCreateInfo::layout(layout)
            },
        )?;

        let width = (settings.slice_width / settings.slice_pixel_size) as u32;
        let height = (settings.slice_height / settings.slice_pixel_size) as u32;
        let field_image_view = Self::create_field_image_view(renderer, [width, height])?;

        Ok(Self {
            vertices,
            indices,
            pipeline,
            model: Matrix4::from_scale(1.),
            field_image_view,
        })
    }

    pub fn init(&mut self, settings: &ViewerSettings) {
        self.update_pos(settings);
    }

    fn update_pos(&mut self, settings: &ViewerSettings) {
        let rotation = to_gl_rot(settings.slice_rotation());
        let mut model = Matrix4::from(rotation);
        model[3] = to_gl_pos(settings.slice_pos()).extend(1.);
        self.model = model;
    }

    pub const fn model(&self) -> Matrix4 {
        self.model
    }

    pub fn field_image_view(&self) -> Subbuffer<[[f32; 4]]> {
        self.field_image_view.clone()
    }

    pub fn update(
        &mut self,
        renderer: &Renderer,
        settings: &ViewerSettings,
        update_flag: &UpdateFlag,
    ) -> anyhow::Result<()> {
        if update_flag.contains(UpdateFlag::UPDATE_SLICE_POS) {
            self.update_pos(settings);
        }

        if update_flag.contains(UpdateFlag::UPDATE_SLICE_SIZE) {
            let width = (settings.slice_width / settings.slice_pixel_size) as u32;
            let height = (settings.slice_height / settings.slice_pixel_size) as u32;
            self.field_image_view = Self::create_field_image_view(renderer, [width, height])?;
            self.vertices = Self::create_vertices(renderer, settings)?;
            self.indices = Self::create_indices(renderer)?;
        }

        Ok(())
    }

    pub fn render(
        &mut self,
        renderer: &Renderer,
        view: Matrix4,
        proj: Matrix4,
        settings: &ViewerSettings,
        builder: &mut AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>,
    ) -> anyhow::Result<()> {
        let pc = fs::PushConstsConfig {
            pvm: (proj * view * self.model).into(),
            width: (settings.slice_width / settings.slice_pixel_size) as _,
            height: (settings.slice_height / settings.slice_pixel_size) as _,
        };

        let layout = self.pipeline.layout().set_layouts().get(0).unwrap();
        let desc_set = PersistentDescriptorSet::new(
            renderer.descriptor_set_allocator(),
            layout.clone(),
            [WriteDescriptorSet::buffer(0, self.field_image_view())],
            [],
        )?;

        builder
            .bind_pipeline_graphics(self.pipeline.clone())?
            .bind_descriptor_sets(
                PipelineBindPoint::Graphics,
                self.pipeline.layout().clone(),
                0,
                desc_set,
            )?
            .push_constants(self.pipeline.layout().clone(), 0, pc)?
            .bind_vertex_buffers(0, self.vertices.clone())?
            .bind_index_buffer(self.indices.clone())?
            .draw_indexed(self.indices.len() as u32, 1, 0, 0, 0)?;

        Ok(())
    }

    fn create_field_image_view(
        renderer: &Renderer,
        view_size: [u32; 2],
    ) -> anyhow::Result<Subbuffer<[[f32; 4]]>> {
        let data_iter = vec![[0., 0., 0., 1.]; view_size[0] as usize * view_size[1] as usize];
        let buffer = Buffer::from_iter(
            renderer.memory_allocator(),
            BufferCreateInfo {
                usage: BufferUsage::STORAGE_BUFFER,
                ..Default::default()
            },
            AllocationCreateInfo {
                memory_type_filter: MemoryTypeFilter::PREFER_DEVICE
                    | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                ..Default::default()
            },
            data_iter,
        )?;
        Ok(buffer)
    }

    fn create_vertices(
        renderer: &Renderer,
        settings: &ViewerSettings,
    ) -> anyhow::Result<Subbuffer<[SliceVertex]>> {
        let width = settings.slice_width;
        let height = settings.slice_height;
        let buffer = Buffer::from_iter(
            renderer.memory_allocator(),
            BufferCreateInfo {
                usage: BufferUsage::VERTEX_BUFFER,
                ..Default::default()
            },
            AllocationCreateInfo {
                memory_type_filter: MemoryTypeFilter::PREFER_DEVICE
                    | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                ..Default::default()
            },
            [
                SliceVertex {
                    position: [-width / 2.0, -height / 2.0, 0.0, 1.0],
                    tex_coords: [0.0, 0.0],
                },
                SliceVertex {
                    position: [width / 2.0, -height / 2.0, 0.0, 1.0],
                    tex_coords: [1.0, 0.0],
                },
                SliceVertex {
                    position: [width / 2.0, height / 2.0, 0.0, 1.0],
                    tex_coords: [1.0, 1.0],
                },
                SliceVertex {
                    position: [-width / 2.0, height / 2.0, 0.0, 1.0],
                    tex_coords: [0.0, 1.0],
                },
            ]
            .iter()
            .cloned(),
        )?;

        Ok(buffer)
    }

    fn create_indices(renderer: &Renderer) -> anyhow::Result<Subbuffer<[u32]>> {
        let indices: Vec<u32> = vec![0, 2, 1, 0, 3, 2];
        let buffer = Buffer::from_iter(
            renderer.memory_allocator(),
            BufferCreateInfo {
                usage: BufferUsage::INDEX_BUFFER,
                ..Default::default()
            },
            AllocationCreateInfo {
                memory_type_filter: MemoryTypeFilter::PREFER_DEVICE
                    | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                ..Default::default()
            },
            indices,
        )?;

        Ok(buffer)
    }
}
