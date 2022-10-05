// File: field_compute.hpp
// Project: simulator
// Created Date: 05/10/2022
// Author: Shun Suzuki
// -----
// Last Modified: 05/10/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <algorithm>
#include <filesystem>
#include <string>
#include <utility>
#include <vector>

#include "shader.hpp"
#include "sound_sources.hpp"
#include "tinycolormap.hpp"

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
  explicit FieldCompute(const helper::VulkanContext* context, std::string shader) : _context(context), _shader(std::move(shader)) {}
  ~FieldCompute() = default;
  FieldCompute(const FieldCompute& v) = delete;
  FieldCompute& operator=(const FieldCompute& obj) = delete;
  FieldCompute(FieldCompute&& obj) = default;
  FieldCompute& operator=(FieldCompute&& obj) = default;

  void init(const float slice_alpha) {
    create_pipeline();
    create_color_map(slice_alpha);
  }

  void update() {}

 private:
  void create_pipeline() {
    const auto shader_code = helper::read_file(std::filesystem::path(_shader).append("field.comp.spv").string());
    vk::UniqueShaderModule shader_module = helper::create_shader_module(_context->device(), shader_code);

    const vk::PipelineShaderStageCreateInfo shader_stage =
        vk::PipelineShaderStageCreateInfo().setStage(vk::ShaderStageFlagBits::eCompute).setModule(shader_module.get()).setPName("main");

    std::array bindings_0 = {vk::DescriptorSetLayoutBinding()
                                 .setBinding(0)
                                 .setDescriptorType(vk::DescriptorType::eStorageBuffer)
                                 .setDescriptorCount(1)
                                 .setStageFlags(vk::ShaderStageFlagBits::eCompute)};
    const auto descriptor_set_layout_0 =
        _context->device().createDescriptorSetLayoutUnique(vk::DescriptorSetLayoutCreateInfo().setBindings(bindings_0));

    std::array bindings_1 = {vk::DescriptorSetLayoutBinding()
                                 .setBinding(0)
                                 .setDescriptorType(vk::DescriptorType::eUniformBuffer)
                                 .setDescriptorCount(1)
                                 .setStageFlags(vk::ShaderStageFlagBits::eCompute)};
    const auto descriptor_set_layout_1 =
        _context->device().createDescriptorSetLayoutUnique(vk::DescriptorSetLayoutCreateInfo().setBindings(bindings_1));

    std::array bindings_2 = {vk::DescriptorSetLayoutBinding()
                                 .setBinding(0)
                                 .setDescriptorType(vk::DescriptorType::eStorageBuffer)
                                 .setDescriptorCount(1)
                                 .setStageFlags(vk::ShaderStageFlagBits::eCompute)};
    const auto descriptor_set_layout_2 =
        _context->device().createDescriptorSetLayoutUnique(vk::DescriptorSetLayoutCreateInfo().setBindings(bindings_2));

    std::array bindings_3 = {vk::DescriptorSetLayoutBinding()
                                 .setBinding(0)
                                 .setDescriptorType(vk::DescriptorType::eStorageBuffer)
                                 .setDescriptorCount(1)
                                 .setStageFlags(vk::ShaderStageFlagBits::eCompute)};
    const auto descriptor_set_layout_3 =
        _context->device().createDescriptorSetLayoutUnique(vk::DescriptorSetLayoutCreateInfo().setBindings(bindings_3));

    std::array bindings_4 = {vk::DescriptorSetLayoutBinding()
                                 .setBinding(0)
                                 .setDescriptorType(vk::DescriptorType::eCombinedImageSampler)
                                 .setDescriptorCount(1)
                                 .setStageFlags(vk::ShaderStageFlagBits::eCompute)};
    const auto descriptor_set_layout_4 =
        _context->device().createDescriptorSetLayoutUnique(vk::DescriptorSetLayoutCreateInfo().setBindings(bindings_4));

    const std::array layouts = {descriptor_set_layout_0.get(), descriptor_set_layout_1.get(), descriptor_set_layout_2.get(),
                                descriptor_set_layout_3.get(), descriptor_set_layout_4.get()};
    const vk::PipelineLayoutCreateInfo pipeline_layout_info = vk::PipelineLayoutCreateInfo().setSetLayouts(layouts);

    _layout = _context->device().createPipelineLayoutUnique(pipeline_layout_info);

    if (auto result =
            _context->device().createComputePipelineUnique({}, vk::ComputePipelineCreateInfo().setStage(shader_stage).setLayout(_layout.get()));
        result.result == vk::Result::eSuccess)
      _pipeline = std::move(result.value);
    else
      throw std::runtime_error("failed to create a pipeline!");
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
                                 vk::ImageTiling::eOptimal, flag, vk::MemoryPropertyFlagBits::eDeviceLocal);
      _texture_image = std::move(texture_image);
      _texture_image_memory = std::move(texture_image_memory);

      _context->transition_image_layout(_texture_image, vk::Format::eR8G8B8A8Unorm, vk::ImageLayout::eUndefined, vk::ImageLayout::eTransferDstOptimal,
                                        1);
      _context->copy_buffer_to_image(staging_buffer, _texture_image, static_cast<uint32_t>(color_map_size), 1u);
      _context->generate_mipmaps(_texture_image, vk::Format::eR8G8B8A8Unorm, color_map_size, 1, 1);
    }

    { _texture_image_view = _context->create_image_view(_texture_image.get(), vk::Format::eR8G8B8A8Unorm, vk::ImageAspectFlagBits::eColor, 1); }

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
  }

  const helper::VulkanContext* _context;
  std::string _shader;

  vk::UniqueImage _texture_image;
  vk::UniqueDeviceMemory _texture_image_memory;
  vk::UniqueImageView _texture_image_view;
  vk::UniqueSampler _texture_sampler;

  vk::UniquePipelineLayout _layout;
  vk::UniquePipeline _pipeline;
};

}  // namespace autd3::extra::simulator
