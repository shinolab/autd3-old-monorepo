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

  void init() { create_pipeline(); }

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

  const helper::VulkanContext* _context;
  std::string _shader;

  vk::UniquePipelineLayout _layout;
  vk::UniquePipeline _pipeline;
};

}  // namespace autd3::extra::simulator
