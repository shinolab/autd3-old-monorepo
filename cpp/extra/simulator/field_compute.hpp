// File: field_compute.hpp
// Project: simulator
// Created Date: 05/10/2022
// Author: Shun Suzuki
// -----
// Last Modified: 28/04/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <algorithm>
#include <utility>
#include <vector>

#include "shader.hpp"
#include "sound_sources.hpp"
#include "tinycolormap.hpp"
#include "update_flag.hpp"
#include "vulkan_renderer.hpp"

namespace autd3::extra::simulator {

struct Config {
  uint32_t source_num;
  float wave_num;
  float color_scale;
  uint32_t width;
  uint32_t height;
  float pixel_size;
  float scale;
  uint32_t dummy1;
  glm::mat4 model;
};

class FieldCompute {
 public:
  explicit FieldCompute(const helper::VulkanContext* context, const VulkanRenderer* renderer) : _context(context), _renderer(renderer) {}
  ~FieldCompute() = default;
  FieldCompute(const FieldCompute& v) = delete;
  FieldCompute& operator=(const FieldCompute& obj) = delete;
  FieldCompute(FieldCompute&& obj) = default;
  FieldCompute& operator=(FieldCompute&& obj) = default;

  void init(const std::vector<SoundSources>& sources, const float slice_alpha, const std::vector<vk::UniqueBuffer>& image_buffers,
            const size_t image_size, const tinycolormap::ColormapType type) {
    create_descriptor_set_layouts();

    const std::vector<uint8_t> shader_code = {
#include "shaders/field.comp.spv.txt"
    };

    auto [layout, pipeline] = create_pipeline(shader_code);

    _layout1 = std::move(layout);
    _pipeline1 = std::move(pipeline);

    const std::vector<uint8_t> shader_code2 = {
#include "shaders/field2.comp.spv.txt"
    };
    auto [layout2, pipeline2] = create_pipeline(shader_code2);

    _layout2 = std::move(layout2);
    _pipeline2 = std::move(pipeline2);

    create_source_drive(sources);
    create_source_pos(sources);
    create_color_map(slice_alpha, type);

    create_descriptor_sets(sources, image_buffers, image_size);

    update_source_drive(sources);
    update_source_pos(sources);
  }

  void update(const std::vector<SoundSources>& sources, const float slice_alpha, const std::vector<vk::UniqueBuffer>& image_buffers,
              const size_t image_size, const tinycolormap::ColormapType type, const driver::BitFlags<UpdateFlags> update_flags) {
    if (update_flags.contains(UpdateFlags::UpdateSliceSize)) create_descriptor_sets(sources, image_buffers, image_size);

    if (update_flags.contains(UpdateFlags::UpdateSourceDrive) || update_flags.contains(UpdateFlags::UpdateSourceFlag)) update_source_drive(sources);

    if (update_flags.contains(UpdateFlags::UpdateColorMap)) return create_color_map(slice_alpha, type);
  }

  void compute(const Config config, const bool show_radiation_pressure) {
    const auto current_image = _renderer->current_frame();

    auto command_buffer = _context->begin_single_time_commands();

    const auto pipeline = show_radiation_pressure ? _pipeline2.get() : _pipeline1.get();
    const auto layout = show_radiation_pressure ? _layout2.get() : _layout1.get();

    const std::array sets = {
        _descriptor_sets_0[current_image].get(),
        _descriptor_sets_1[current_image].get(),
        _descriptor_sets_2[current_image].get(),
        _descriptor_sets_3[current_image].get(),
    };

    command_buffer->bindPipeline(vk::PipelineBindPoint::eCompute, pipeline);
    command_buffer->bindDescriptorSets(vk::PipelineBindPoint::eCompute, layout, 0, sets, nullptr);
    command_buffer->pushConstants(layout, vk::ShaderStageFlagBits::eCompute, 0, sizeof(Config), &config);
    command_buffer->dispatch((config.width - 1) / 32 + 1, (config.height - 1) / 32 + 1, 1);

    _context->end_single_time_commands(command_buffer);
  }

 private:
  void create_descriptor_set_layouts() {
    std::array bindings_0 = {vk::DescriptorSetLayoutBinding()
                                 .setBinding(0)
                                 .setDescriptorType(vk::DescriptorType::eStorageBuffer)
                                 .setDescriptorCount(1)
                                 .setStageFlags(vk::ShaderStageFlagBits::eCompute)};
    _descriptor_set_layout_0 = _context->device().createDescriptorSetLayoutUnique(vk::DescriptorSetLayoutCreateInfo().setBindings(bindings_0));

    std::array bindings_1 = {vk::DescriptorSetLayoutBinding()
                                 .setBinding(0)
                                 .setDescriptorType(vk::DescriptorType::eStorageBuffer)
                                 .setDescriptorCount(1)
                                 .setStageFlags(vk::ShaderStageFlagBits::eCompute)};
    _descriptor_set_layout_1 = _context->device().createDescriptorSetLayoutUnique(vk::DescriptorSetLayoutCreateInfo().setBindings(bindings_1));

    std::array bindings_2 = {vk::DescriptorSetLayoutBinding()
                                 .setBinding(0)
                                 .setDescriptorType(vk::DescriptorType::eStorageBuffer)
                                 .setDescriptorCount(1)
                                 .setStageFlags(vk::ShaderStageFlagBits::eCompute)};
    _descriptor_set_layout_2 = _context->device().createDescriptorSetLayoutUnique(vk::DescriptorSetLayoutCreateInfo().setBindings(bindings_2));

    std::array bindings_3 = {vk::DescriptorSetLayoutBinding()
                                 .setBinding(0)
                                 .setDescriptorType(vk::DescriptorType::eCombinedImageSampler)
                                 .setDescriptorCount(1)
                                 .setStageFlags(vk::ShaderStageFlagBits::eCompute)};
    _descriptor_set_layout_3 = _context->device().createDescriptorSetLayoutUnique(vk::DescriptorSetLayoutCreateInfo().setBindings(bindings_3));
  }

  std::pair<vk::UniquePipelineLayout, vk::UniquePipeline> create_pipeline(const std::vector<uint8_t>& shader_code) {
    vk::UniqueShaderModule shader_module = helper::create_shader_module(_context->device(), shader_code);

    const vk::PipelineShaderStageCreateInfo shader_stage =
        vk::PipelineShaderStageCreateInfo().setStage(vk::ShaderStageFlagBits::eCompute).setModule(shader_module.get()).setPName("main");

    const std::array push_constant_ranges = {
        vk::PushConstantRange().setStageFlags(vk::ShaderStageFlagBits::eCompute).setSize(sizeof(Config)).setOffset(0)};
    const std::array layouts = {_descriptor_set_layout_0.get(), _descriptor_set_layout_1.get(), _descriptor_set_layout_2.get(),
                                _descriptor_set_layout_3.get()};
    const vk::PipelineLayoutCreateInfo pipeline_layout_info =
        vk::PipelineLayoutCreateInfo().setSetLayouts(layouts).setPushConstantRanges(push_constant_ranges);

    auto layout = _context->device().createPipelineLayoutUnique(pipeline_layout_info);

    if (auto result =
            _context->device().createComputePipelineUnique({}, vk::ComputePipelineCreateInfo().setStage(shader_stage).setLayout(layout.get()));
        result.result == vk::Result::eSuccess)
      return std::make_pair(std::move(layout), std::move(result.value));

    throw std::runtime_error("Failed to create a pipeline!");
  }

  void create_descriptor_sets(const std::vector<SoundSources>& sources, const std::vector<vk::UniqueBuffer>& image_buffers, const size_t image_size) {
    const auto size = std::accumulate(sources.begin(), sources.end(), size_t{0}, [](size_t acc, const auto& s) { return acc + s.size(); });

    {
      const std::vector layouts(_renderer->frames_in_flight(), _descriptor_set_layout_0.get());
      _descriptor_sets_0 = _context->device().allocateDescriptorSetsUnique(
          vk::DescriptorSetAllocateInfo().setDescriptorPool(_context->descriptor_pool()).setSetLayouts(layouts));
      for (size_t i = 0; i < _renderer->frames_in_flight(); i++) {
        const vk::DescriptorBufferInfo buffer_info = vk::DescriptorBufferInfo().setBuffer(image_buffers[i].get()).setOffset(0).setRange(image_size);
        std::array descriptor_writes{
            vk::WriteDescriptorSet()
                .setDstSet(_descriptor_sets_0[i].get())
                .setDstBinding(0)
                .setDstArrayElement(0)
                .setDescriptorCount(1)
                .setDescriptorType(vk::DescriptorType::eStorageBuffer)
                .setBufferInfo(buffer_info),
        };
        _context->device().updateDescriptorSets(descriptor_writes, {});
      }
    }

    {
      const std::vector layouts(_renderer->frames_in_flight(), _descriptor_set_layout_1.get());
      _descriptor_sets_1 = _context->device().allocateDescriptorSetsUnique(
          vk::DescriptorSetAllocateInfo().setDescriptorPool(_context->descriptor_pool()).setSetLayouts(layouts));
      for (size_t i = 0; i < _renderer->frames_in_flight(); i++) {
        const vk::DescriptorBufferInfo buffer_info =
            vk::DescriptorBufferInfo().setBuffer(_pos_buffers[i].get()).setOffset(0).setRange(sizeof(glm::vec4) * size);
        std::array descriptor_writes{
            vk::WriteDescriptorSet()
                .setDstSet(_descriptor_sets_1[i].get())
                .setDstBinding(0)
                .setDstArrayElement(0)
                .setDescriptorCount(1)
                .setDescriptorType(vk::DescriptorType::eStorageBuffer)
                .setBufferInfo(buffer_info),
        };
        _context->device().updateDescriptorSets(descriptor_writes, {});
      }
    }
    {
      const std::vector layouts(_renderer->frames_in_flight(), _descriptor_set_layout_2.get());
      _descriptor_sets_2 = _context->device().allocateDescriptorSetsUnique(
          vk::DescriptorSetAllocateInfo().setDescriptorPool(_context->descriptor_pool()).setSetLayouts(layouts));
      for (size_t i = 0; i < _renderer->frames_in_flight(); i++) {
        const vk::DescriptorBufferInfo buffer_info =
            vk::DescriptorBufferInfo().setBuffer(_drive_buffers[i].get()).setOffset(0).setRange(sizeof(Drive) * size);
        std::array descriptor_writes{
            vk::WriteDescriptorSet()
                .setDstSet(_descriptor_sets_2[i].get())
                .setDstBinding(0)
                .setDstArrayElement(0)
                .setDescriptorCount(1)
                .setDescriptorType(vk::DescriptorType::eStorageBuffer)
                .setBufferInfo(buffer_info),
        };
        _context->device().updateDescriptorSets(descriptor_writes, {});
      }
    }
  }

  void create_color_map(const float slice_alpha, const tinycolormap::ColormapType type) {
    constexpr size_t color_map_size = 100;
    constexpr size_t image_size = color_map_size * 4;
    std::vector<uint8_t> pixels;
    pixels.reserve(color_map_size * 4);
    const auto alpha = static_cast<uint8_t>(slice_alpha * 255.0f);
    for (size_t i = 0; i < color_map_size; i++) {
      const auto v = static_cast<double>(i) / static_cast<double>(color_map_size);
      const auto color = GetColor(v, type);

      pixels.emplace_back(static_cast<uint8_t>(color.r() * 255.0));
      pixels.emplace_back(static_cast<uint8_t>(color.g() * 255.0));
      pixels.emplace_back(static_cast<uint8_t>(color.b() * 255.0));
      pixels.emplace_back(alpha);
    }

    {
      auto [staging_buffer, staging_buffer_memory] = _context->create_buffer(
          image_size, vk::BufferUsageFlagBits::eTransferSrc, vk::MemoryPropertyFlagBits::eHostVisible | vk::MemoryPropertyFlagBits::eHostCoherent);

      void* data;
      if (_context->device().mapMemory(staging_buffer_memory.get(), 0, image_size, {}, &data) != vk::Result::eSuccess)
        throw std::runtime_error("Failed to map texture buffer.");

      std::memcpy(data, pixels.data(), image_size);
      _context->device().unmapMemory(staging_buffer_memory.get());

      const auto flag = vk::ImageUsageFlagBits::eTransferDst | vk::ImageUsageFlagBits::eSampled | vk::ImageUsageFlagBits::eTransferSrc;
      auto [texture_image, texture_image_memory] =
          _context->create_image(static_cast<uint32_t>(color_map_size), 1u, 1, vk::SampleCountFlagBits::e1, vk::Format::eR8G8B8A8Unorm,
                                 vk::ImageTiling::eOptimal, flag, vk::MemoryPropertyFlagBits::eDeviceLocal, vk::ImageType::e1D);
      _texture_image = std::move(texture_image);
      _texture_image_memory = std::move(texture_image_memory);

      _context->transition_image_layout(_texture_image, vk::Format::eR8G8B8A8Unorm, vk::ImageLayout::eUndefined, vk::ImageLayout::eTransferDstOptimal,
                                        1);
      _context->copy_buffer_to_image(staging_buffer, _texture_image, static_cast<uint32_t>(color_map_size), 1u);
      _context->transition_image_layout(_texture_image, vk::Format::eR8G8B8A8Unorm, vk::ImageLayout::eTransferDstOptimal,
                                        vk::ImageLayout::eShaderReadOnlyOptimal, 1);
    }

    {
      _texture_image_view =
          _context->create_image_view(_texture_image.get(), vk::Format::eR8G8B8A8Unorm, vk::ImageAspectFlagBits::eColor, 1, vk::ImageViewType::e1D);
    }

    {
      const auto properties = _context->physical_device().getProperties();

      _texture_sampler = _context->device().createSamplerUnique(vk::SamplerCreateInfo()
                                                                    .setMagFilter(vk::Filter::eLinear)
                                                                    .setMinFilter(vk::Filter::eLinear)
                                                                    .setMipmapMode(vk::SamplerMipmapMode::eNearest)
                                                                    .setAddressModeU(vk::SamplerAddressMode::eClampToEdge)
                                                                    .setAddressModeV(vk::SamplerAddressMode::eClampToEdge)
                                                                    .setAddressModeW(vk::SamplerAddressMode::eClampToEdge)
                                                                    .setMipLodBias(0.0f)
                                                                    .setAnisotropyEnable(true)
                                                                    .setMaxAnisotropy(properties.limits.maxSamplerAnisotropy)
                                                                    .setCompareEnable(false)
                                                                    .setCompareOp(vk::CompareOp::eAlways)
                                                                    .setMinLod(0.0f)
                                                                    .setMaxLod(1.0f)
                                                                    .setBorderColor(vk::BorderColor::eIntOpaqueBlack)
                                                                    .setUnnormalizedCoordinates(false));
    }

    {
      const std::vector layouts(_renderer->frames_in_flight(), _descriptor_set_layout_3.get());
      _descriptor_sets_3 = _context->device().allocateDescriptorSetsUnique(
          vk::DescriptorSetAllocateInfo().setDescriptorPool(_context->descriptor_pool()).setSetLayouts(layouts));
      for (size_t i = 0; i < _renderer->frames_in_flight(); i++) {
        const vk::DescriptorImageInfo image_info = vk::DescriptorImageInfo()
                                                       .setSampler(_texture_sampler.get())
                                                       .setImageView(_texture_image_view.get())
                                                       .setImageLayout(vk::ImageLayout::eShaderReadOnlyOptimal);
        std::array descriptor_writes{
            vk::WriteDescriptorSet()
                .setDstSet(_descriptor_sets_3[i].get())
                .setDstBinding(0)
                .setDstArrayElement(0)
                .setDescriptorCount(1)
                .setDescriptorType(vk::DescriptorType::eCombinedImageSampler)
                .setImageInfo(image_info),
        };
        _context->device().updateDescriptorSets(descriptor_writes, {});
      }
    }
  }

  void create_source_drive(const std::vector<SoundSources>& sources) {
    _drive_buffers.resize(_renderer->frames_in_flight());
    _drive_buffers_memory.resize(_renderer->frames_in_flight());
    for (size_t i = 0; i < _renderer->frames_in_flight(); i++) {
      const auto size = std::accumulate(sources.begin(), sources.end(), size_t{0}, [](size_t acc, const auto& s) { return acc + s.size(); });
      auto [buf, mem] = _context->create_buffer(sizeof(Drive) * size, vk::BufferUsageFlagBits::eStorageBuffer,
                                                vk::MemoryPropertyFlagBits::eHostVisible | vk::MemoryPropertyFlagBits::eHostCoherent);

      _drive_buffers[i] = std::move(buf);
      _drive_buffers_memory[i] = std::move(mem);
    }
  }

  void create_source_pos(const std::vector<SoundSources>& sources) {
    _pos_buffers.resize(_renderer->frames_in_flight());
    _pos_buffers_memory.resize(_renderer->frames_in_flight());
    for (size_t i = 0; i < _renderer->frames_in_flight(); i++) {
      const auto size = std::accumulate(sources.begin(), sources.end(), size_t{0}, [](size_t acc, const auto& s) { return acc + s.size(); });
      auto [buf, mem] = _context->create_buffer(sizeof(glm::vec4) * size, vk::BufferUsageFlagBits::eStorageBuffer,
                                                vk::MemoryPropertyFlagBits::eHostVisible | vk::MemoryPropertyFlagBits::eHostCoherent);

      _pos_buffers[i] = std::move(buf);
      _pos_buffers_memory[i] = std::move(mem);
    }
  }

  void update_source_drive(const std::vector<SoundSources>& sources) {
    const auto size = std::accumulate(sources.begin(), sources.end(), size_t{0}, [](size_t acc, const auto& s) { return acc + s.size(); });
    uint8_t* data = nullptr;
    for (auto& memory : _drive_buffers_memory) {
      if (_context->device().mapMemory(memory.get(), 0, sizeof(Drive) * size, {}, reinterpret_cast<void**>(&data)) != vk::Result::eSuccess)
        throw std::runtime_error("Failed to map uniform buffer memory");

      for (const auto& s : sources) {
        std::memcpy(data, s.drives().data(), sizeof(glm::vec4) * s.drives().size());
        data += sizeof(glm::vec4) * s.drives().size();
      }
      _context->device().unmapMemory(memory.get());
    }
  }

  void update_source_pos(const std::vector<SoundSources>& sources) {
    const auto size = std::accumulate(sources.begin(), sources.end(), size_t{0}, [](size_t acc, const auto& s) { return acc + s.size(); });
    uint8_t* data = nullptr;
    for (auto& memory : _pos_buffers_memory) {
      if (_context->device().mapMemory(memory.get(), 0, sizeof(glm::vec4) * size, {}, reinterpret_cast<void**>(&data)) != vk::Result::eSuccess)
        throw std::runtime_error("Failed to map uniform buffer memory");

      for (const auto& s : sources) {
        memcpy(data, s.positions().data(), sizeof(glm::vec4) * s.positions().size());
        data += sizeof(glm::vec4) * s.positions().size();
      }
      _context->device().unmapMemory(memory.get());
    }
  }

  const helper::VulkanContext* _context;
  const VulkanRenderer* _renderer;

  vk::UniqueImage _texture_image;
  vk::UniqueDeviceMemory _texture_image_memory;
  vk::UniqueImageView _texture_image_view;
  vk::UniqueSampler _texture_sampler;

  std::vector<vk::UniqueBuffer> _drive_buffers;
  std::vector<vk::UniqueDeviceMemory> _drive_buffers_memory;
  std::vector<vk::UniqueBuffer> _pos_buffers;
  std::vector<vk::UniqueDeviceMemory> _pos_buffers_memory;

  vk::UniquePipelineLayout _layout1;
  vk::UniquePipeline _pipeline1;
  vk::UniquePipelineLayout _layout2;
  vk::UniquePipeline _pipeline2;

  std::vector<vk::UniqueDescriptorSet> _descriptor_sets_0;
  std::vector<vk::UniqueDescriptorSet> _descriptor_sets_1;
  std::vector<vk::UniqueDescriptorSet> _descriptor_sets_2;
  std::vector<vk::UniqueDescriptorSet> _descriptor_sets_3;
  vk::UniqueDescriptorSetLayout _descriptor_set_layout_0;
  vk::UniqueDescriptorSetLayout _descriptor_set_layout_1;
  vk::UniqueDescriptorSetLayout _descriptor_set_layout_2;
  vk::UniqueDescriptorSetLayout _descriptor_set_layout_3;
};

}  // namespace autd3::extra::simulator
