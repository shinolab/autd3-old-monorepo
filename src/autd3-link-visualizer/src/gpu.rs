/*
 * File: gpu.rs
 * Project: src
 * Created Date: 15/06/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 06/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use std::sync::Arc;

use autd3_driver::{
    defined::{float, Complex},
    geometry::{Geometry, Vector3},
};
use vulkano::{
    buffer::{Buffer, BufferCreateInfo, BufferUsage},
    command_buffer::{
        allocator::StandardCommandBufferAllocator, AutoCommandBufferBuilder, CommandBufferUsage,
        PrimaryCommandBufferAbstract,
    },
    descriptor_set::{
        allocator::StandardDescriptorSetAllocator, PersistentDescriptorSet, WriteDescriptorSet,
    },
    device::{
        physical::PhysicalDeviceType, Device, DeviceCreateInfo, DeviceExtensions, Queue,
        QueueCreateInfo, QueueFlags,
    },
    instance::{Instance, InstanceCreateFlags, InstanceCreateInfo},
    memory::allocator::{AllocationCreateInfo, MemoryTypeFilter, StandardMemoryAllocator},
    pipeline::{
        compute::ComputePipelineCreateInfo, layout::PipelineDescriptorSetLayoutCreateInfo,
        ComputePipeline, Pipeline, PipelineBindPoint, PipelineLayout,
        PipelineShaderStageCreateInfo,
    },
    sync::GpuFuture,
    VulkanLibrary,
};

use crate::error::VisualizerError;

pub(crate) struct FieldCompute {
    pipeline: Arc<ComputePipeline>,
    queue: Arc<Queue>,
    command_buffer_allocator: StandardCommandBufferAllocator,
    descriptor_set_allocator: StandardDescriptorSetAllocator,
    memory_allocator: Arc<StandardMemoryAllocator>,
}

impl FieldCompute {
    pub(crate) fn new(gpu_idx: i32) -> Result<Self, VisualizerError> {
        let library = VulkanLibrary::new()?;
        let instance = Instance::new(
            library,
            InstanceCreateInfo {
                flags: InstanceCreateFlags::ENUMERATE_PORTABILITY,
                ..Default::default()
            },
        )
        .expect("Failed to create instance");

        let device_extensions = DeviceExtensions {
            khr_storage_buffer_storage_class: true,
            ..DeviceExtensions::empty()
        };
        let available_properties = instance
            .enumerate_physical_devices()?
            .filter(|p| p.supported_extensions().contains(&device_extensions))
            .filter_map(|p| {
                p.queue_family_properties()
                    .iter()
                    .position(|q| q.queue_flags.intersects(QueueFlags::COMPUTE))
                    .map(|i| (p, i as u32))
            })
            .collect::<Vec<_>>();

        let (physical_device, queue_family_index) = match gpu_idx {
            idx if idx < 0 || (idx as usize) >= available_properties.len() => available_properties
                .into_iter()
                .min_by_key(|(p, _)| match p.properties().device_type {
                    PhysicalDeviceType::DiscreteGpu => 0,
                    PhysicalDeviceType::IntegratedGpu => 1,
                    PhysicalDeviceType::VirtualGpu => 2,
                    PhysicalDeviceType::Cpu => 3,
                    PhysicalDeviceType::Other => 4,
                    _ => 5,
                })
                .unwrap(),
            _ => available_properties[gpu_idx as usize].clone(),
        };

        let (device, mut queues) = Device::new(
            physical_device,
            DeviceCreateInfo {
                enabled_extensions: device_extensions,
                queue_create_infos: vec![QueueCreateInfo {
                    queue_family_index,
                    ..Default::default()
                }],
                ..Default::default()
            },
        )?;

        let queue = queues.next().unwrap();

        let command_buffer_allocator =
            StandardCommandBufferAllocator::new(queue.device().clone(), Default::default());
        let descriptor_set_allocator =
            StandardDescriptorSetAllocator::new(queue.device().clone(), Default::default());
        let memory_allocator =
            Arc::new(StandardMemoryAllocator::new_default(queue.device().clone()));

        let pipeline = {
            let shader = cs::load(device.clone())?;
            let cs = shader.entry_point("main").unwrap();
            let stage = PipelineShaderStageCreateInfo::new(cs);
            let layout = PipelineLayout::new(
                device.clone(),
                PipelineDescriptorSetLayoutCreateInfo::from_stages([&stage])
                    .into_pipeline_layout_create_info(device.clone())?,
            )?;
            ComputePipeline::new(
                device.clone(),
                None,
                ComputePipelineCreateInfo::stage_layout(stage, layout),
            )?
        };

        Ok(Self {
            pipeline,
            queue,
            command_buffer_allocator,
            descriptor_set_allocator,
            memory_allocator,
        })
    }

    pub(crate) fn calc_field_of<
        'a,
        D: autd3_driver::acoustics::directivity::Directivity,
        I: IntoIterator<Item = &'a Vector3>,
    >(
        &self,
        observe_points: I,
        geometry: &Geometry,
        source_drive: Vec<[f32; 4]>,
    ) -> Result<Vec<Complex>, VisualizerError> {
        let pipeline_layout = self.pipeline.layout();
        let layout = pipeline_layout.set_layouts().get(0).unwrap();

        let observe_points = observe_points.into_iter().collect::<Vec<_>>();
        let size = observe_points.len();

        let data_buffer = Buffer::from_iter(
            self.memory_allocator.clone(),
            BufferCreateInfo {
                usage: BufferUsage::STORAGE_BUFFER,
                ..Default::default()
            },
            AllocationCreateInfo {
                memory_type_filter: MemoryTypeFilter::PREFER_DEVICE
                    | MemoryTypeFilter::HOST_RANDOM_ACCESS,
                ..Default::default()
            },
            (0..size).map(|_| [0f32, 0f32]),
        )?;
        let set_0 = PersistentDescriptorSet::new(
            &self.descriptor_set_allocator,
            layout.clone(),
            [WriteDescriptorSet::buffer(0, data_buffer.clone())],
            [],
        )?;

        let source_pos_buffer = Buffer::from_iter(
            self.memory_allocator.clone(),
            BufferCreateInfo {
                usage: BufferUsage::STORAGE_BUFFER,
                ..Default::default()
            },
            AllocationCreateInfo {
                memory_type_filter: MemoryTypeFilter::PREFER_DEVICE
                    | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                ..Default::default()
            },
            geometry
                .iter()
                .flat_map(|dev| {
                    dev.iter().map(|t| {
                        [
                            t.position().x as f32,
                            t.position().y as f32,
                            t.position().z as f32,
                            0.,
                        ]
                    })
                })
                .collect::<Vec<_>>(),
        )?;
        let set_1 = PersistentDescriptorSet::new(
            &self.descriptor_set_allocator,
            layout.clone(),
            [WriteDescriptorSet::buffer(0, source_pos_buffer.clone())],
            [],
        )?;

        let source_drive_buffer = Buffer::from_iter(
            self.memory_allocator.clone(),
            BufferCreateInfo {
                usage: BufferUsage::STORAGE_BUFFER,
                ..Default::default()
            },
            AllocationCreateInfo {
                memory_type_filter: MemoryTypeFilter::PREFER_DEVICE
                    | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                ..Default::default()
            },
            source_drive,
        )?;
        let set_2 = PersistentDescriptorSet::new(
            &self.descriptor_set_allocator,
            layout.clone(),
            [WriteDescriptorSet::buffer(0, source_drive_buffer.clone())],
            [],
        )?;

        let observe_pos_buffer = Buffer::from_iter(
            self.memory_allocator.clone(),
            BufferCreateInfo {
                usage: BufferUsage::STORAGE_BUFFER,
                ..Default::default()
            },
            AllocationCreateInfo {
                memory_type_filter: MemoryTypeFilter::PREFER_DEVICE
                    | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                ..Default::default()
            },
            observe_points
                .iter()
                .map(|p| [p.x as f32, p.y as f32, p.z as f32, 0.]),
        )?;
        let set_3 = PersistentDescriptorSet::new(
            &self.descriptor_set_allocator,
            layout.clone(),
            [WriteDescriptorSet::buffer(0, observe_pos_buffer.clone())],
            [],
        )?;

        let directivity_buffer = Buffer::from_iter(
            self.memory_allocator.clone(),
            BufferCreateInfo {
                usage: BufferUsage::STORAGE_BUFFER,
                ..Default::default()
            },
            AllocationCreateInfo {
                memory_type_filter: MemoryTypeFilter::PREFER_DEVICE
                    | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                ..Default::default()
            },
            (0..91).map(|x| D::directivity(x as float) as f32),
        )?;
        let set_4 = PersistentDescriptorSet::new(
            &self.descriptor_set_allocator,
            layout.clone(),
            [WriteDescriptorSet::buffer(0, directivity_buffer.clone())],
            [],
        )?;

        let source_dir_buffer = Buffer::from_iter(
            self.memory_allocator.clone(),
            BufferCreateInfo {
                usage: BufferUsage::STORAGE_BUFFER,
                ..Default::default()
            },
            AllocationCreateInfo {
                memory_type_filter: MemoryTypeFilter::PREFER_DEVICE
                    | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                ..Default::default()
            },
            geometry
                .iter()
                .flat_map(|dev| {
                    dev.iter().map(|t| {
                        [
                            t.z_direction().x as f32,
                            t.z_direction().y as f32,
                            t.z_direction().z as f32,
                            0.,
                        ]
                    })
                })
                .collect::<Vec<_>>(),
        )?;
        let set_5 = PersistentDescriptorSet::new(
            &self.descriptor_set_allocator,
            layout.clone(),
            [WriteDescriptorSet::buffer(0, source_dir_buffer.clone())],
            [],
        )?;

        let mut builder = AutoCommandBufferBuilder::primary(
            &self.command_buffer_allocator,
            self.queue.queue_family_index(),
            CommandBufferUsage::OneTimeSubmit,
        )?;

        let pc = cs::PushConsts {
            observe_num: size as u32,
            source_num: geometry.num_transducers() as u32,
            _dummy1: 0,
            _dummy2: 0,
        };

        builder
            .bind_pipeline_compute(self.pipeline.clone())?
            .bind_descriptor_sets(
                PipelineBindPoint::Compute,
                pipeline_layout.clone(),
                0,
                (set_0, set_1, set_2, set_3, set_4, set_5),
            )?
            .push_constants(pipeline_layout.clone(), 0, pc)?
            .dispatch([(size as u32 - 1) / 32 + 1, 1, 1])?;
        let command_buffer = builder.build()?;
        let finished = command_buffer.execute(self.queue.clone())?;
        let future = finished.then_signal_fence_and_flush()?;
        future.wait(None)?;

        let data_content = data_buffer.read()?;
        Ok(data_content
            .iter()
            .map(|d| Complex::new(d[0] as float, d[1] as float))
            .collect())
    }
}

#[allow(clippy::needless_question_mark)]
mod cs {
    vulkano_shaders::shader! {
        ty: "compute",
        path: "./assets/shaders/pressure.comp"
    }
}
