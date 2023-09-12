/*
 * File: field_compute_pipeline.rs
 * Project: src
 * Created Date: 28/11/2021
 * Author: Shun Suzuki
 * -----
 * Last Modified: 18/06/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2021 Hapis Lab. All rights reserved.
 *
 */

use std::sync::Arc;

use bytemuck::{Pod, Zeroable};
use scarlet::{colormap::ColorMap, prelude::*};
use vulkano::{
    buffer::{Buffer, BufferCreateInfo, BufferUsage, Subbuffer},
    command_buffer::{AutoCommandBufferBuilder, CommandBufferUsage, PrimaryCommandBufferAbstract},
    descriptor_set::{PersistentDescriptorSet, WriteDescriptorSet},
    format::Format,
    image::{view::ImageView, ImageDimensions, ImmutableImage, MipmapsCount},
    memory::allocator::{AllocationCreateInfo, MemoryUsage},
    pipeline::{ComputePipeline, Pipeline, PipelineBindPoint},
    sampler::{Filter, Sampler, SamplerAddressMode, SamplerCreateInfo, SamplerMipmapMode},
    sync::GpuFuture,
};

use crate::{
    renderer::Renderer,
    sound_sources::{Drive, SoundSources},
    update_flag::UpdateFlag,
    viewer_settings::ViewerSettings,
};

#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Pod, Zeroable)]
pub struct Config {
    pub source_num: u32,
    pub _wave_num: f32,
    pub color_scale: f32,
    pub width: u32,
    pub height: u32,
    pub pixel_size: f32,
    pub scale: f32,
    pub _dummy_0: u32,
    pub model: [[f32; 4]; 4],
}

pub struct FieldComputePipeline {
    pipeline_pressure: Arc<ComputePipeline>,
    pipeline_radiation: Arc<ComputePipeline>,
    source_drive_buf: Option<Subbuffer<[Drive]>>,
    source_pos_buf: Option<Subbuffer<[[f32; 4]]>>,
    color_map_desc_set_pressure: Arc<PersistentDescriptorSet>,
    color_map_desc_set_radiation: Arc<PersistentDescriptorSet>,
}

impl FieldComputePipeline {
    pub fn new(renderer: &Renderer, settings: &ViewerSettings) -> Self {
        let pipeline_pressure = {
            let shader = cs_pressure::load(renderer.device()).unwrap();
            ComputePipeline::new(
                renderer.device(),
                shader.entry_point("main").unwrap(),
                &(),
                None,
                |_| {},
            )
            .unwrap()
        };
        let color_map_desc_set_pressure =
            Self::create_color_map_desc_set(renderer, pipeline_pressure.clone(), settings);

        let pipeline_radiation = {
            let shader = cs_radiation::load(renderer.device()).unwrap();
            ComputePipeline::new(
                renderer.device(),
                shader.entry_point("main").unwrap(),
                &(),
                None,
                |_| {},
            )
            .unwrap()
        };
        let color_map_desc_set_radiation =
            Self::create_color_map_desc_set(renderer, pipeline_radiation.clone(), settings);

        Self {
            pipeline_pressure,
            pipeline_radiation,
            source_pos_buf: None,
            source_drive_buf: None,
            color_map_desc_set_pressure,
            color_map_desc_set_radiation,
        }
    }

    fn create_color_map_desc_set(
        renderer: &Renderer,
        pipeline: Arc<ComputePipeline>,
        settings: &ViewerSettings,
    ) -> Arc<PersistentDescriptorSet> {
        let color_map_size = 100;
        let iter = (0..color_map_size).map(|x| x as f64 / color_map_size as f64);
        let mut uploads = AutoCommandBufferBuilder::primary(
            renderer.command_buffer_allocator(),
            renderer.queue().queue_family_index(),
            CommandBufferUsage::OneTimeSubmit,
        )
        .unwrap();
        let texture = {
            let color_map: Vec<RGBColor> = match settings.color_map_type {
                crate::viewer_settings::ColorMapType::Viridis => {
                    scarlet::colormap::ListedColorMap::viridis().transform(iter)
                }
                crate::viewer_settings::ColorMapType::Magma => {
                    scarlet::colormap::ListedColorMap::magma().transform(iter)
                }
                crate::viewer_settings::ColorMapType::Inferno => {
                    scarlet::colormap::ListedColorMap::inferno().transform(iter)
                }
                crate::viewer_settings::ColorMapType::Plasma => {
                    scarlet::colormap::ListedColorMap::plasma().transform(iter)
                }
            };

            let dimensions = ImageDimensions::Dim1d {
                width: color_map_size,
                array_layers: 1,
            };
            let alpha = (settings.slice_alpha * 255.) as u8;
            let texels = color_map
                .iter()
                .flat_map(|color| {
                    [
                        (color.r * 255.) as u8,
                        (color.g * 255.) as u8,
                        (color.b * 255.) as u8,
                        alpha,
                    ]
                })
                .collect::<Vec<_>>();

            let image = ImmutableImage::from_iter(
                renderer.memory_allocator(),
                texels,
                dimensions,
                MipmapsCount::One,
                Format::R8G8B8A8_UNORM,
                &mut uploads,
            )
            .unwrap();
            ImageView::new_default(image).unwrap()
        };

        uploads
            .build()
            .unwrap()
            .execute(renderer.queue())
            .unwrap()
            .then_signal_fence_and_flush()
            .unwrap()
            .wait(None)
            .unwrap();

        let sampler = Sampler::new(
            renderer.device(),
            SamplerCreateInfo {
                mag_filter: Filter::Linear,
                min_filter: Filter::Linear,
                mipmap_mode: SamplerMipmapMode::Nearest,
                address_mode: [SamplerAddressMode::ClampToEdge; 3],
                mip_lod_bias: 0.0,
                ..Default::default()
            },
        )
        .unwrap();

        let layout = pipeline.layout().set_layouts().get(3).unwrap();
        PersistentDescriptorSet::new(
            renderer.descriptor_set_allocator(),
            layout.clone(),
            [WriteDescriptorSet::image_view_sampler(0, texture, sampler)],
        )
        .unwrap()
    }

    pub fn init(&mut self, renderer: &Renderer, sources: &SoundSources) {
        self.source_drive_buf = Some(
            Buffer::from_iter(
                renderer.memory_allocator(),
                BufferCreateInfo {
                    usage: BufferUsage::STORAGE_BUFFER,
                    ..Default::default()
                },
                AllocationCreateInfo {
                    usage: MemoryUsage::Upload,
                    ..Default::default()
                },
                sources.drives().copied(),
            )
            .unwrap(),
        );
        self.source_pos_buf = Some(
            Buffer::from_iter(
                renderer.memory_allocator(),
                BufferCreateInfo {
                    usage: BufferUsage::STORAGE_BUFFER,
                    ..Default::default()
                },
                AllocationCreateInfo {
                    usage: MemoryUsage::Upload,
                    ..Default::default()
                },
                sources.positions().copied().map(|p| p.into()),
            )
            .unwrap(),
        );
    }

    pub fn update(
        &mut self,
        renderer: &Renderer,
        sources: &SoundSources,
        settings: &ViewerSettings,
        update_flag: &UpdateFlag,
    ) {
        if update_flag.contains(UpdateFlag::UPDATE_SOURCE_DRIVE)
            || update_flag.contains(UpdateFlag::UPDATE_SOURCE_FLAG)
        {
            self.update_source(sources);
        }

        if update_flag.contains(UpdateFlag::UPDATE_COLOR_MAP) {
            self.color_map_desc_set_pressure =
                Self::create_color_map_desc_set(renderer, self.pipeline_pressure.clone(), settings);
        }
    }

    pub fn compute(
        &mut self,
        renderer: &Renderer,
        config: Config,
        image: Subbuffer<[[f32; 4]]>,
        settings: &ViewerSettings,
    ) -> Box<dyn GpuFuture> {
        let pipeline = if settings.show_radiation_pressure {
            self.pipeline_radiation.clone()
        } else {
            self.pipeline_pressure.clone()
        };

        let pipeline_layout = pipeline.layout();
        let layout = pipeline_layout.set_layouts().get(0).unwrap();
        let set_0 = PersistentDescriptorSet::new(
            renderer.descriptor_set_allocator(),
            layout.clone(),
            [WriteDescriptorSet::buffer(0, image)],
        )
        .unwrap();

        let layout = pipeline_layout.set_layouts().get(1).unwrap();
        let set_1 = PersistentDescriptorSet::new(
            renderer.descriptor_set_allocator(),
            layout.clone(),
            [WriteDescriptorSet::buffer(
                0,
                self.source_pos_buf.clone().unwrap(),
            )],
        )
        .unwrap();

        let layout = pipeline_layout.set_layouts().get(1).unwrap();
        let set_2 = PersistentDescriptorSet::new(
            renderer.descriptor_set_allocator(),
            layout.clone(),
            [WriteDescriptorSet::buffer(
                0,
                self.source_drive_buf.clone().unwrap(),
            )],
        )
        .unwrap();

        let set_3 = if settings.show_radiation_pressure {
            self.color_map_desc_set_radiation.clone()
        } else {
            self.color_map_desc_set_pressure.clone()
        };

        let mut builder = AutoCommandBufferBuilder::primary(
            renderer.command_buffer_allocator(),
            renderer.queue().queue_family_index(),
            CommandBufferUsage::OneTimeSubmit,
        )
        .unwrap();

        builder
            .bind_pipeline_compute(pipeline.clone())
            .bind_descriptor_sets(
                PipelineBindPoint::Compute,
                pipeline_layout.clone(),
                0,
                (set_0, set_1, set_2, set_3),
            )
            .push_constants(pipeline_layout.clone(), 0, config)
            .dispatch([(config.width - 1) / 32 + 1, (config.height - 1) / 32 + 1, 1])
            .unwrap();
        let command_buffer = builder.build().unwrap();
        let finished = command_buffer.execute(renderer.queue()).unwrap();
        finished.then_signal_fence_and_flush().unwrap().boxed()
    }

    fn update_source(&mut self, sources: &SoundSources) {
        if let Some(data) = &mut self.source_drive_buf {
            data.write()
                .unwrap()
                .iter_mut()
                .zip(sources.drives())
                .for_each(|(d, &drive)| {
                    *d = drive;
                });
        }
    }
}

#[allow(clippy::needless_question_mark)]
mod cs_pressure {
    vulkano_shaders::shader! {
        ty: "compute",
        path: "./assets/shaders/pressure.comp"
    }
}

#[allow(clippy::needless_question_mark)]
mod cs_radiation {
    vulkano_shaders::shader! {
        ty: "compute",
        path: "./assets/shaders/pressure.comp",
        define: [("RADIATION", "1")]
    }
}
