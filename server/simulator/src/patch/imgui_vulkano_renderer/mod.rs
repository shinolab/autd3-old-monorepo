mod shader;

use std::{convert::TryInto, fmt, sync::Arc};

use bytemuck::{Pod, Zeroable};
use imgui::{internal::RawWrapper, DrawCmd, DrawCmdParams, DrawIdx, DrawVert, TextureId, Textures};
use vulkano::{
    buffer::{
        allocator::{SubbufferAllocator, SubbufferAllocatorCreateInfo},
        Buffer, BufferCreateInfo, BufferUsage, Subbuffer,
    },
    command_buffer::{
        allocator::StandardCommandBufferAllocator, AutoCommandBufferBuilder, CommandBufferUsage,
        CopyBufferToImageInfo, PrimaryAutoCommandBuffer, PrimaryCommandBufferAbstract,
        RenderPassBeginInfo, SubpassBeginInfo,
    },
    descriptor_set::{
        allocator::StandardDescriptorSetAllocator, PersistentDescriptorSet, WriteDescriptorSet,
    },
    device::{Device, Queue},
    format::Format,
    image::{
        sampler::{Sampler, SamplerCreateInfo},
        view::ImageView,
        Image, ImageCreateInfo, ImageType, ImageUsage,
    },
    memory::allocator::{AllocationCreateInfo, MemoryTypeFilter, StandardMemoryAllocator},
    pipeline::{
        graphics::{
            color_blend::{AttachmentBlend, ColorBlendAttachmentState, ColorBlendState},
            input_assembly::InputAssemblyState,
            multisample::MultisampleState,
            rasterization::RasterizationState,
            vertex_input::{Vertex, VertexDefinition},
            viewport::{Scissor, Viewport, ViewportState},
            GraphicsPipelineCreateInfo,
        },
        layout::PipelineDescriptorSetLayoutCreateInfo,
        DynamicState, GraphicsPipeline, Pipeline, PipelineBindPoint, PipelineLayout,
        PipelineShaderStageCreateInfo,
    },
    render_pass::{Framebuffer, FramebufferCreateInfo, RenderPass, Subpass},
    sync::GpuFuture,
    DeviceSize,
};

#[repr(C)]
#[derive(Default, Debug, Clone, Copy, Pod, Zeroable, Vertex)]
struct ImguiVertex {
    #[format(R32G32_SFLOAT)]
    pub pos: [f32; 2],
    #[format(R32G32_SFLOAT)]
    pub uv: [f32; 2],
    #[format(R32_UINT)]
    pub col: u32,
}

impl From<DrawVert> for ImguiVertex {
    fn from(v: DrawVert) -> ImguiVertex {
        unsafe { std::mem::transmute(v) }
    }
}

#[derive(Debug)]
pub enum RendererError {
    BadTexture(TextureId),
}

impl fmt::Display for RendererError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BadTexture(ref t) => {
                write!(f, "The Texture ID could not be found: {:?}", t)
            }
        }
    }
}

impl std::error::Error for RendererError {}

pub type Texture = (Arc<ImageView>, Arc<Sampler>);

pub struct Renderer {
    render_pass: Arc<RenderPass>,
    pipeline: Arc<GraphicsPipeline>,
    font_texture: Texture,
    textures: Textures<Texture>,
    vrt_buffer_pool: SubbufferAllocator,
    idx_buffer_pool: SubbufferAllocator,
    transform_buffer_pool: SubbufferAllocator,
}

impl Renderer {
    /// Initialize the renderer object, including vertex buffers, ImGui font textures,
    /// and the Vulkan graphics pipeline.
    ///
    /// ---
    ///
    /// `ctx`: the ImGui `Context` object
    ///
    /// `device`: the Vulkano `Device` object for the device you want to render the UI on.
    ///
    /// `queue`: the Vulkano `Queue` object for the queue the font atlas texture will be created on.
    ///
    /// `format`: the Vulkano `Format` that the render pass will use when storing the frame in the target image.
    pub fn init(
        ctx: &mut imgui::Context,
        device: Arc<Device>,
        queue: Arc<Queue>,
        format: Format,
    ) -> anyhow::Result<Renderer> {
        let vs = shader::vs::load(device.clone())?
            .entry_point("main")
            .unwrap();
        let fs = shader::fs::load(device.clone())?
            .entry_point("main")
            .unwrap();

        let render_pass = vulkano::single_pass_renderpass!(
            device.clone(),
            attachments: {
                color: {
                    format: format,
                    samples: 1,
                    load_op: Load,
                    store_op: Store,
                }
            },
            pass: {
                color: [color],
                depth_stencil: {}
            }
        )?;

        let vertex_input_state =
            ImguiVertex::per_vertex().definition(&vs.info().input_interface)?;
        let stages = [
            PipelineShaderStageCreateInfo::new(vs),
            PipelineShaderStageCreateInfo::new(fs),
        ];
        let layout = PipelineLayout::new(
            device.clone(),
            PipelineDescriptorSetLayoutCreateInfo::from_stages(&stages)
                .into_pipeline_layout_create_info(device.clone())?,
        )?;
        let subpass = Subpass::from(render_pass.clone(), 0).unwrap();
        let pipeline = GraphicsPipeline::new(
            device.clone(),
            None,
            GraphicsPipelineCreateInfo {
                stages: stages.into_iter().collect(),
                vertex_input_state: Some(vertex_input_state),
                input_assembly_state: Some(InputAssemblyState {
                    ..Default::default()
                }),
                viewport_state: Some(ViewportState::default()),
                rasterization_state: Some(RasterizationState::default()),
                color_blend_state: Some(ColorBlendState::with_attachment_states(
                    subpass.num_color_attachments(),
                    ColorBlendAttachmentState {
                        blend: Some(AttachmentBlend::alpha()),
                        ..Default::default()
                    },
                )),
                multisample_state: Some(MultisampleState {
                    ..MultisampleState::default()
                }),
                dynamic_state: [DynamicState::Viewport, DynamicState::Scissor]
                    .into_iter()
                    .collect(),
                subpass: Some(subpass.into()),
                ..GraphicsPipelineCreateInfo::layout(layout)
            },
        )?;

        let textures = Textures::new();

        let font_texture = Self::upload_font_texture(ctx.fonts(), device.clone(), queue)?;

        ctx.set_renderer_name(Some(format!(
            "imgui-vulkano-renderer {}",
            env!("CARGO_PKG_VERSION")
        )));

        let memory_allocator = Arc::new(StandardMemoryAllocator::new_default(device.clone()));
        let vrt_buffer_pool = SubbufferAllocator::new(
            memory_allocator.clone(),
            SubbufferAllocatorCreateInfo {
                buffer_usage: BufferUsage::VERTEX_BUFFER,
                memory_type_filter: MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                ..SubbufferAllocatorCreateInfo::default()
            },
        );
        let idx_buffer_pool = SubbufferAllocator::new(
            memory_allocator.clone(),
            SubbufferAllocatorCreateInfo {
                buffer_usage: BufferUsage::INDEX_BUFFER,
                memory_type_filter: MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                ..SubbufferAllocatorCreateInfo::default()
            },
        );
        let transform_buffer_pool = SubbufferAllocator::new(
            memory_allocator.clone(),
            SubbufferAllocatorCreateInfo {
                buffer_usage: BufferUsage::UNIFORM_BUFFER,
                memory_type_filter: MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                ..SubbufferAllocatorCreateInfo::default()
            },
        );

        Ok(Renderer {
            render_pass,
            pipeline,
            font_texture,
            textures,
            vrt_buffer_pool,
            idx_buffer_pool,
            transform_buffer_pool,
        })
    }

    /// Appends the draw commands for the UI frame to an `AutoCommandBufferBuilder`.
    ///
    /// ---
    ///
    /// `cmd_buf_builder`: An `AutoCommandBufferBuilder` from vulkano to add commands to
    ///
    /// `device`: the Vulkano `Device` object for the device you want to render the UI on
    ///
    /// `queue`: the Vulkano `Queue` object for buffer creation
    ///
    /// `target`: the target image to render to
    ///
    /// `draw_data`: the ImGui `DrawData` that each UI frame creates
    pub fn draw_commands(
        &mut self,
        cmd_buf_builder: &mut AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>,
        queue: Arc<Queue>,
        target: Arc<ImageView>,
        draw_data: &imgui::DrawData,
    ) -> anyhow::Result<()> {
        let fb_width = draw_data.display_size[0] * draw_data.framebuffer_scale[0];
        let fb_height = draw_data.display_size[1] * draw_data.framebuffer_scale[1];
        if !(fb_width > 0.0 && fb_height > 0.0) {
            return Ok(());
        }
        let left = draw_data.display_pos[0];
        let right = draw_data.display_pos[0] + draw_data.display_size[0];
        let top = draw_data.display_pos[1];
        let bottom = draw_data.display_pos[1] + draw_data.display_size[1];

        let descriptor_set_allocator =
            StandardDescriptorSetAllocator::new(queue.device().clone(), Default::default());

        let transform_data = [
            [(2.0 / (right - left)), 0.0, 0.0, 0.0],
            [0.0, (2.0 / (bottom - top)), 0.0, 0.0],
            [0.0, 0.0, -1.0, 0.0],
            [
                (right + left) / (left - right),
                (top + bottom) / (top - bottom),
                0.0,
                1.0,
            ],
        ];

        let transform_subbuffer = self.transform_buffer_pool.allocate_sized()?;
        *transform_subbuffer.write()? = transform_data;

        let extent = target.image().extent();

        let clip_off = draw_data.display_pos;
        let clip_scale = draw_data.framebuffer_scale;

        let layout = self.pipeline.layout().set_layouts()[0].clone();

        let framebuffer = Framebuffer::new(
            self.render_pass.clone(),
            FramebufferCreateInfo {
                attachments: vec![target],
                ..Default::default()
            },
        )?;

        cmd_buf_builder.begin_render_pass(
            RenderPassBeginInfo {
                clear_values: vec![None],
                ..RenderPassBeginInfo::framebuffer(framebuffer)
            },
            SubpassBeginInfo::default(),
        )?;

        for draw_list in draw_data.draw_lists() {
            let vertex_subbuffer: Subbuffer<[ImguiVertex]> = self
                .vrt_buffer_pool
                .allocate_unsized(draw_list.vtx_buffer().len() as _)?;
            for (b, v) in vertex_subbuffer
                .write()?
                .iter_mut()
                .zip(draw_list.vtx_buffer().iter())
            {
                *b = ImguiVertex::from(*v);
            }

            let index_subbuffer: Subbuffer<[DrawIdx]> = self
                .idx_buffer_pool
                .allocate_unsized(draw_list.idx_buffer().len() as _)?;
            for (b, &v) in index_subbuffer
                .write()?
                .iter_mut()
                .zip(draw_list.idx_buffer().iter())
            {
                *b = v;
            }

            for cmd in draw_list.commands() {
                match cmd {
                    DrawCmd::Elements {
                        count,
                        cmd_params:
                            DrawCmdParams {
                                clip_rect,
                                texture_id,
                                vtx_offset,
                                idx_offset,
                                ..
                            },
                    } => {
                        let clip_rect = [
                            (clip_rect[0] - clip_off[0]) * clip_scale[0],
                            (clip_rect[1] - clip_off[1]) * clip_scale[1],
                            (clip_rect[2] - clip_off[0]) * clip_scale[0],
                            (clip_rect[3] - clip_off[1]) * clip_scale[1],
                        ];

                        if clip_rect[0] < fb_width
                            && clip_rect[1] < fb_height
                            && clip_rect[2] >= 0.0
                            && clip_rect[3] >= 0.0
                        {
                            let tex = self.lookup_texture(texture_id)?;

                            let set = PersistentDescriptorSet::new(
                                &descriptor_set_allocator,
                                layout.clone(),
                                [
                                    WriteDescriptorSet::buffer(0, transform_subbuffer.clone()),
                                    WriteDescriptorSet::image_view_sampler(
                                        1,
                                        tex.0.clone(),
                                        tex.1.clone(),
                                    ),
                                ],
                                [],
                            )?;

                            let viewports = vec![Viewport {
                                offset: [0.0, 0.0],
                                extent: [extent[0] as f32, extent[1] as f32],
                                depth_range: 0.0..=1.0,
                            }];
                            let scissors = vec![Scissor {
                                offset: [
                                    f32::max(0.0, clip_rect[0]).floor() as u32,
                                    f32::max(0.0, clip_rect[1]).floor() as u32,
                                ],
                                extent: [
                                    (clip_rect[2] - clip_rect[0]).abs().ceil() as u32,
                                    (clip_rect[3] - clip_rect[1]).abs().ceil() as u32,
                                ],
                            }];
                            cmd_buf_builder
                                .set_viewport(0, viewports.into_iter().collect())?
                                .bind_pipeline_graphics(self.pipeline.clone())?
                                .bind_descriptor_sets(
                                    PipelineBindPoint::Graphics,
                                    self.pipeline.layout().clone(),
                                    0,
                                    set.clone(),
                                )?
                                .bind_vertex_buffers(0, vertex_subbuffer.clone())?
                                .bind_index_buffer(index_subbuffer.clone())?
                                .set_scissor(0, scissors.into_iter().collect())?
                                .draw_indexed(count as _, 1, idx_offset as _, vtx_offset as _, 0)?;
                        }
                    }
                    DrawCmd::ResetRenderState => (), // TODO
                    DrawCmd::RawCallback { callback, raw_cmd } => unsafe {
                        callback(draw_list.raw(), raw_cmd)
                    },
                }
            }
        }
        cmd_buf_builder.end_render_pass(Default::default())?;

        Ok(())
    }

    /// Update the ImGui font atlas texture.
    ///
    /// ---
    ///
    /// `ctx`: the ImGui `Context` object
    ///
    /// `device`: the Vulkano `Device` object for the device you want to render the UI on.
    ///
    /// `queue`: the Vulkano `Queue` object for the queue the font atlas texture will be created on.
    pub fn reload_font_texture(
        &mut self,
        ctx: &mut imgui::Context,
        device: Arc<Device>,
        queue: Arc<Queue>,
    ) -> anyhow::Result<()> {
        self.font_texture = Self::upload_font_texture(ctx.fonts(), device, queue)?;
        Ok(())
    }

    /// Get the texture library that the renderer uses
    pub fn textures(&mut self) -> &mut Textures<Texture> {
        &mut self.textures
    }

    fn upload_font_texture(
        fonts: &mut imgui::FontAtlas,
        device: Arc<Device>,
        queue: Arc<Queue>,
    ) -> anyhow::Result<Texture> {
        let command_buffer_allocator =
            StandardCommandBufferAllocator::new(queue.device().clone(), Default::default());

        let memory_allocator =
            Arc::new(StandardMemoryAllocator::new_default(queue.device().clone()));

        let texture = fonts.build_rgba32_texture();

        let upload_buffer = Buffer::new_slice(
            memory_allocator.clone(),
            BufferCreateInfo {
                usage: BufferUsage::TRANSFER_SRC,
                ..Default::default()
            },
            AllocationCreateInfo {
                memory_type_filter: MemoryTypeFilter::PREFER_HOST
                    | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                ..Default::default()
            },
            (texture.width * texture.height * 4) as DeviceSize,
        )?;

        upload_buffer.write()?.copy_from_slice(&texture.data);

        let mut uploads = AutoCommandBufferBuilder::primary(
            &command_buffer_allocator,
            queue.queue_family_index(),
            CommandBufferUsage::OneTimeSubmit,
        )?;

        let extent = [texture.width, texture.height, 1];
        let image = Image::new(
            memory_allocator,
            ImageCreateInfo {
                image_type: ImageType::Dim2d,
                format: Format::R8G8B8A8_SRGB,
                extent,
                usage: ImageUsage::TRANSFER_DST | ImageUsage::SAMPLED,
                mip_levels: 1,
                ..Default::default()
            },
            AllocationCreateInfo::default(),
        )?;

        uploads.copy_buffer_to_image(CopyBufferToImageInfo::buffer_image(
            upload_buffer,
            image.clone(),
        ))?;

        uploads
            .build()?
            .execute(queue.clone())?
            .then_signal_fence_and_flush()?
            .wait(None)?;

        let sampler = Sampler::new(device, SamplerCreateInfo::simple_repeat_linear())?;

        fonts.tex_id = TextureId::from(usize::MAX);
        Ok((ImageView::new_default(image)?, sampler))
    }

    fn lookup_texture(&self, texture_id: TextureId) -> Result<&Texture, RendererError> {
        if texture_id.id() == usize::MAX {
            Ok(&self.font_texture)
        } else if let Some(texture) = self.textures.get(texture_id) {
            Ok(texture)
        } else {
            Err(RendererError::BadTexture(texture_id))
        }
    }
}
