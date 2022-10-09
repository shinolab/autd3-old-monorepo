// File: field_compute.hpp
// Project: simulator
// Created Date: 05/10/2022
// Author: Shun Suzuki
// -----
// Last Modified: 09/10/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <algorithm>
#include <filesystem>
#include <utility>
#include <vector>

#include "shader.hpp"
#include "sound_sources.hpp"
#include "update_flag.hpp"
#include "vulkan_renderer.hpp"

#if _MSC_VER
#pragma warning(push)
#pragma warning(disable : 26451)
#endif
#if defined(__GNUC__) && !defined(__llvm__)
#pragma GCC diagnostic push
#endif
#include "tinycolormap.hpp"
#if _MSC_VER
#pragma warning(pop)
#endif
#if defined(__GNUC__) && !defined(__llvm__)
#pragma GCC diagnostic pop
#endif

namespace autd3::extra::simulator {

struct Config {
  uint32_t source_num;
  float wave_num;
  float color_scale;
  uint32_t width;
  uint32_t height;
  uint32_t pixel_size;
  uint32_t dummy0;
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

  void init(const SoundSources& sources, const float slice_alpha, const std::vector<vk::UniqueBuffer>& image_buffers, const size_t image_size) {
    create_pipeline();

    create_source_drive(sources);
    create_source_pos(sources);
    create_color_map(slice_alpha);

    create_descriptor_sets(sources, image_buffers, image_size);

    update_source_drive(sources);
    update_source_pos(sources);
  }

  void update(const SoundSources& sources, const float slice_alpha, const std::vector<vk::UniqueBuffer>& image_buffers, const size_t image_size,
              const UpdateFlags update_flags) {
    if (update_flags.contains(UpdateFlags::UPDATE_SLICE_SIZE)) create_descriptor_sets(sources, image_buffers, image_size);

    if (update_flags.contains(UpdateFlags::UPDATE_SOURCE_DRIVE) || update_flags.contains(UpdateFlags::UPDATE_SOURCE_FLAG))
      update_source_drive(sources);

    if (update_flags.contains(UpdateFlags::UPDATE_COLOR_MAP)) create_color_map(slice_alpha);
  }

  void compute(const Config config) {
    const auto current_image = _renderer->current_frame();

    auto command_buffer = _context->begin_single_time_commands();

    const std::array sets = {
        _descriptor_sets_0[current_image].get(),
        _descriptor_sets_1[current_image].get(),
        _descriptor_sets_2[current_image].get(),
        _descriptor_sets_3[current_image].get(),
    };

    command_buffer->bindPipeline(vk::PipelineBindPoint::eCompute, _pipeline.get());
    command_buffer->bindDescriptorSets(vk::PipelineBindPoint::eCompute, _layout.get(), 0, sets, nullptr);
    command_buffer->pushConstants(_layout.get(), vk::ShaderStageFlagBits::eCompute, 0, sizeof(Config), &config);
    command_buffer->dispatch((config.width - 1) / 32 + 1, (config.height - 1) / 32 + 1, 1);

    _context->end_single_time_commands(command_buffer);
  }

 private:
  void create_pipeline() {
    const std::vector<uint8_t> shader_code = {
#include "field.comp.spv.txt"
    };
    vk::UniqueShaderModule shader_module = helper::create_shader_module(_context->device(), shader_code);

    const vk::PipelineShaderStageCreateInfo shader_stage =
        vk::PipelineShaderStageCreateInfo().setStage(vk::ShaderStageFlagBits::eCompute).setModule(shader_module.get()).setPName("main");

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

    const std::array push_constant_ranges = {
        vk::PushConstantRange().setStageFlags(vk::ShaderStageFlagBits::eCompute).setSize(sizeof(Config)).setOffset(0)};
    const std::array layouts = {_descriptor_set_layout_0.get(), _descriptor_set_layout_1.get(), _descriptor_set_layout_2.get(),
                                _descriptor_set_layout_3.get()};
    const vk::PipelineLayoutCreateInfo pipeline_layout_info =
        vk::PipelineLayoutCreateInfo().setSetLayouts(layouts).setPushConstantRanges(push_constant_ranges);

    _layout = _context->device().createPipelineLayoutUnique(pipeline_layout_info);

    if (auto result =
            _context->device().createComputePipelineUnique({}, vk::ComputePipelineCreateInfo().setStage(shader_stage).setLayout(_layout.get()));
        result.result == vk::Result::eSuccess)
      _pipeline = std::move(result.value);
    else
      throw std::runtime_error("failed to create a pipeline!");
  }

  void create_descriptor_sets(const SoundSources& sources, const std::vector<vk::UniqueBuffer>& image_buffers, const size_t image_size) {
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
            vk::DescriptorBufferInfo().setBuffer(_pos_buffers[i].get()).setOffset(0).setRange(sizeof(glm::vec4) * sources.size());
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
            vk::DescriptorBufferInfo().setBuffer(_drive_buffers[i].get()).setOffset(0).setRange(sizeof(Drive) * sources.size());
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

  void create_color_map(const float slice_alpha) {
    constexpr size_t color_map_size = 100;
    constexpr size_t image_size = color_map_size * 4;
    std::vector<uint8_t> pixels;
    pixels.reserve(color_map_size * 4);
    const auto alpha = static_cast<uint8_t>(slice_alpha * 255.0f);
    for (size_t i = 0; i < color_map_size; i++) {
      const auto v = static_cast<double>(i) / static_cast<double>(color_map_size);
      const auto color = GetColor(v, tinycolormap::ColormapType::Inferno);

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
        throw std::runtime_error("failed to map texture buffer.");
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

  void create_source_drive(const SoundSources& sources) {
    _drive_buffers.resize(_renderer->frames_in_flight());
    _drive_buffers_memory.resize(_renderer->frames_in_flight());
    for (size_t i = 0; i < _renderer->frames_in_flight(); i++) {
      auto [buf, mem] = _context->create_buffer(sizeof(Drive) * sources.size(), vk::BufferUsageFlagBits::eStorageBuffer,
                                                vk::MemoryPropertyFlagBits::eHostVisible | vk::MemoryPropertyFlagBits::eHostCoherent);
      _drive_buffers[i] = std::move(buf);
      _drive_buffers_memory[i] = std::move(mem);
    }
  }

  void create_source_pos(const SoundSources& sources) {
    _pos_buffers.resize(_renderer->frames_in_flight());
    _pos_buffers_memory.resize(_renderer->frames_in_flight());
    for (size_t i = 0; i < _renderer->frames_in_flight(); i++) {
      auto [buf, mem] = _context->create_buffer(sizeof(glm::vec4) * sources.size(), vk::BufferUsageFlagBits::eStorageBuffer,
                                                vk::MemoryPropertyFlagBits::eHostVisible | vk::MemoryPropertyFlagBits::eHostCoherent);
      _pos_buffers[i] = std::move(buf);
      _pos_buffers_memory[i] = std::move(mem);
    }
  }

  void update_source_drive(const SoundSources& sources) {
    const auto& drives = sources.drives();
    void* data;
    for (auto& memory : _drive_buffers_memory) {
      if (_context->device().mapMemory(memory.get(), 0, sizeof(Drive) * drives.size(), {}, &data) != vk::Result::eSuccess)
        throw std::runtime_error("failed to map uniform buffer memory");
      memcpy(data, drives.data(), sizeof(glm::vec4) * drives.size());
      _context->device().unmapMemory(memory.get());
    }
  }

  void update_source_pos(const SoundSources& sources) {
    const auto& positions = sources.positions();
    void* data;
    for (auto& memory : _pos_buffers_memory) {
      if (_context->device().mapMemory(memory.get(), 0, sizeof(glm::vec4) * positions.size(), {}, &data) != vk::Result::eSuccess)
        throw std::runtime_error("failed to map uniform buffer memory");
      memcpy(data, positions.data(), sizeof(glm::vec4) * positions.size());
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

  vk::UniquePipelineLayout _layout;
  vk::UniquePipeline _pipeline;

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
