// File: slice_viewer.hpp
// Project: simulator
// Created Date: 05/10/2022
// Author: Shun Suzuki
// -----
// Last Modified: 08/01/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <algorithm>
#include <utility>
#include <vector>

#include "color.hpp"
#include "shader.hpp"
#include "update_flag.hpp"
#include "vulkan_renderer.hpp"

namespace autd3::extra::simulator::slice_viewer {

struct Vertex {
  glm::vec4 pos;
  glm::vec2 uv;
};

struct Mvp {
  glm::mat4 model;
  glm::mat4 view;
  glm::mat4 proj;
};

struct Config {
  uint32_t width;
  uint32_t height;
  float dummy0;
  float dummy1;
};

class SliceViewer {
 public:
  explicit SliceViewer(const helper::VulkanContext* context, const VulkanRenderer* renderer) : _context(context), _renderer(renderer) {}
  ~SliceViewer() = default;
  SliceViewer(const SliceViewer& v) = delete;
  SliceViewer& operator=(const SliceViewer& obj) = delete;
  SliceViewer(SliceViewer&& obj) = default;
  SliceViewer& operator=(SliceViewer&& obj) = default;

  [[nodiscard]] bool init(const float width, const float height, const float pixel_width) {
    _width = static_cast<uint32_t>(width / pixel_width);
    _height = static_cast<uint32_t>(height / pixel_width);

    if (!create_pipeline() || !create_vertex_buffer(width, height) || !create_index_buffer() || !create_field_buffers(_width, _height)) return false;
    create_descriptor_sets();
    update_field_descriptor_sets(_width, _height);

    return true;
  }

  void render(const glm::mat4 model, const glm::mat4 view, const glm::mat4 proj, const vk::CommandBuffer& command_buffer) {
    const vk::Buffer vertex_buffers[] = {_vertex_buffer.get()};
    constexpr vk::DeviceSize offsets[] = {0};

    command_buffer.bindPipeline(vk::PipelineBindPoint::eGraphics, _pipeline.get());
    command_buffer.bindDescriptorSets(vk::PipelineBindPoint::eGraphics, _layout.get(), 0, 1, &_descriptor_sets[_renderer->current_frame()].get(), 0,
                                      nullptr);
    command_buffer.bindVertexBuffers(0, 1, vertex_buffers, offsets);
    command_buffer.bindIndexBuffer(_index_buffer.get(), 0, vk::IndexType::eUint32);
    const Mvp mvp{model, view, proj};
    command_buffer.pushConstants(_layout.get(), vk::ShaderStageFlagBits::eVertex, 0, sizeof(Mvp), &mvp);
    const Config config{_width, _height, 0, 0};
    command_buffer.pushConstants(_layout.get(), vk::ShaderStageFlagBits::eFragment, sizeof(Mvp), sizeof(Config), &config);
    command_buffer.drawIndexed(6, 1, 0, 0, 0);
  }

  [[nodiscard]] bool update(const float width, const float height, const float pixel_width, const UpdateFlags update_flag) {
    if (!update_flag.contains(UpdateFlags::UpdateSliceSize)) return true;

    _context->device().waitIdle();
    _width = static_cast<uint32_t>(width / pixel_width);
    _height = static_cast<uint32_t>(height / pixel_width);
    if (!create_field_buffers(_width, _height)) return false;
    update_field_descriptor_sets(_width, _height);
    return create_vertex_buffer(width, height);
  }

  [[nodiscard]] const std::vector<vk::UniqueBuffer>& images() const { return _field_buffers; }
  [[nodiscard]] size_t image_size() const { return _field_buffer_size; }

 private:
  [[nodiscard]] bool create_pipeline() {
    const std::vector<uint8_t> vert_shader_code = {
#include "shaders/slice.vert.spv.txt"
    };
    const std::vector<uint8_t> frag_shader_code = {
#include "shaders/slice.frag.spv.txt"
    };
    vk::UniqueShaderModule vert_shader_module = helper::create_shader_module(_context->device(), vert_shader_code);
    vk::UniqueShaderModule frag_shader_module = helper::create_shader_module(_context->device(), frag_shader_code);

    std::array shader_stages = {
        vk::PipelineShaderStageCreateInfo().setStage(vk::ShaderStageFlagBits::eVertex).setModule(vert_shader_module.get()).setPName("main"),
        vk::PipelineShaderStageCreateInfo().setStage(vk::ShaderStageFlagBits::eFragment).setModule(frag_shader_module.get()).setPName("main")};

    std::array binding_descriptions = {
        vk::VertexInputBindingDescription().setBinding(0).setStride(sizeof(Vertex)).setInputRate(vk::VertexInputRate::eVertex),
    };
    std::array attribute_descriptions = {vk::VertexInputAttributeDescription(0, 0, vk::Format::eR32G32B32A32Sfloat, offsetof(Vertex, pos)),
                                         vk::VertexInputAttributeDescription(1, 0, vk::Format::eR32G32Sfloat, offsetof(Vertex, uv))};
    const vk::PipelineVertexInputStateCreateInfo vertex_input_info = vk::PipelineVertexInputStateCreateInfo()
                                                                         .setVertexBindingDescriptions(binding_descriptions)
                                                                         .setVertexAttributeDescriptions(attribute_descriptions);

    const vk::PipelineInputAssemblyStateCreateInfo input_assembly =
        vk::PipelineInputAssemblyStateCreateInfo().setTopology(vk::PrimitiveTopology::eTriangleList);
    const vk::Viewport viewport = vk::Viewport()
                                      .setX(0.0f)
                                      .setY(0.0f)
                                      .setWidth(static_cast<float>(_renderer->extent().width))
                                      .setHeight(static_cast<float>(_renderer->extent().height))
                                      .setMinDepth(0.0f)
                                      .setMaxDepth(1.0f);
    const vk::Rect2D scissor = vk::Rect2D().setOffset({0, 0}).setExtent(_renderer->extent());
    const vk::PipelineViewportStateCreateInfo viewport_state = vk::PipelineViewportStateCreateInfo().setViewports(viewport).setScissors(scissor);

    const vk::PipelineRasterizationStateCreateInfo rasterizer = vk::PipelineRasterizationStateCreateInfo()
                                                                    .setDepthClampEnable(false)
                                                                    .setRasterizerDiscardEnable(false)
                                                                    .setPolygonMode(vk::PolygonMode::eFill)
                                                                    .setCullMode(vk::CullModeFlagBits::eNone)
                                                                    .setFrontFace(vk::FrontFace::eCounterClockwise)
                                                                    .setDepthBiasEnable(false)
                                                                    .setDepthBiasConstantFactor(0.0f)
                                                                    .setDepthBiasClamp(0.0f)
                                                                    .setDepthBiasSlopeFactor(0.0f)
                                                                    .setLineWidth(1.0f);

    const vk::PipelineMultisampleStateCreateInfo multi_sampling =
        vk::PipelineMultisampleStateCreateInfo().setRasterizationSamples(_context->msaa_samples()).setSampleShadingEnable(false);

    const vk::PipelineColorBlendAttachmentState color_blend_attachment =
        vk::PipelineColorBlendAttachmentState()
            .setBlendEnable(true)
            .setColorWriteMask(vk::ColorComponentFlagBits::eR | vk::ColorComponentFlagBits::eG | vk::ColorComponentFlagBits::eB |
                               vk::ColorComponentFlagBits::eA)
            .setSrcColorBlendFactor(vk::BlendFactor::eSrcAlpha)
            .setDstColorBlendFactor(vk::BlendFactor::eOneMinusSrcAlpha)
            .setColorBlendOp(vk::BlendOp::eAdd)
            .setSrcAlphaBlendFactor(vk::BlendFactor::eSrcAlpha)
            .setDstAlphaBlendFactor(vk::BlendFactor::eOneMinusSrcAlpha)
            .setAlphaBlendOp(vk::BlendOp::eAdd);

    vk::PipelineColorBlendStateCreateInfo color_blending =
        vk::PipelineColorBlendStateCreateInfo().setLogicOpEnable(false).setAttachments(color_blend_attachment);

    auto dynamic_states_ = std::array{vk::DynamicState::eViewport, vk::DynamicState::eScissor};
    const vk::PipelineDynamicStateCreateInfo dynamic_state = vk::PipelineDynamicStateCreateInfo().setDynamicStates(dynamic_states_);

    std::array bindings_0 = {vk::DescriptorSetLayoutBinding()
                                 .setBinding(0)
                                 .setDescriptorType(vk::DescriptorType::eStorageBuffer)
                                 .setDescriptorCount(1)
                                 .setStageFlags(vk::ShaderStageFlagBits::eFragment)};
    const auto descriptor_set_layout_0 =
        _context->device().createDescriptorSetLayoutUnique(vk::DescriptorSetLayoutCreateInfo().setBindings(bindings_0));

    const std::array push_constant_ranges = {
        vk::PushConstantRange().setStageFlags(vk::ShaderStageFlagBits::eVertex).setSize(sizeof(Mvp)).setOffset(0),
        vk::PushConstantRange().setStageFlags(vk::ShaderStageFlagBits::eFragment).setSize(sizeof(Config)).setOffset(sizeof(Mvp))};
    const std::array layouts = {descriptor_set_layout_0.get()};
    const vk::PipelineLayoutCreateInfo pipeline_layout_info =
        vk::PipelineLayoutCreateInfo().setSetLayouts(layouts).setPushConstantRanges(push_constant_ranges);

    _layout = _context->device().createPipelineLayoutUnique(pipeline_layout_info);

    const vk::PipelineDepthStencilStateCreateInfo depth_stencil = vk::PipelineDepthStencilStateCreateInfo()
                                                                      .setDepthTestEnable(true)
                                                                      .setDepthWriteEnable(true)
                                                                      .setDepthCompareOp(vk::CompareOp::eLess)
                                                                      .setDepthBoundsTestEnable(false)
                                                                      .setStencilTestEnable(false)
                                                                      .setMinDepthBounds(0.0f)
                                                                      .setMaxDepthBounds(1.0f);
    if (auto result = _context->device().createGraphicsPipelineUnique({}, vk::GraphicsPipelineCreateInfo()
                                                                              .setStages(shader_stages)
                                                                              .setPVertexInputState(&vertex_input_info)
                                                                              .setPInputAssemblyState(&input_assembly)
                                                                              .setPViewportState(&viewport_state)
                                                                              .setPRasterizationState(&rasterizer)
                                                                              .setPMultisampleState(&multi_sampling)
                                                                              .setPDepthStencilState(&depth_stencil)
                                                                              .setPColorBlendState(&color_blending)
                                                                              .setPDynamicState(&dynamic_state)
                                                                              .setLayout(_layout.get())
                                                                              .setRenderPass(_renderer->render_pass()));
        result.result == vk::Result::eSuccess) {
      _pipeline = std::move(result.value);
      return true;
    }
    throw std::runtime_error("Failed to create a pipeline!");
  }

  [[nodiscard]] bool create_vertex_buffer(const float width, const float height) {
    const std::vector vertices = {
        Vertex{
            {-width / 2.0f, -height / 2.0f, 0.0, 1.0},
            {0.0, 0.0},
        },
        Vertex{
            {width / 2.0f, -height / 2.0f, 0.0, 1.0},
            {1.0, 0.0},
        },
        Vertex{
            {width / 2.0f, height / 2.0f, 0.0, 1.0},
            {1.0, 1.0},
        },
        Vertex{
            {-width / 2.0f, height / 2.0f, 0.0, 1.0},
            {0.0, 1.0},
        },
    };
    const vk::DeviceSize buffer_size = sizeof vertices[0] * vertices.size();

    auto [staging_buffer, staging_buffer_memory] = _context->create_buffer(
        buffer_size, vk::BufferUsageFlagBits::eTransferSrc, vk::MemoryPropertyFlagBits::eHostVisible | vk::MemoryPropertyFlagBits::eHostCoherent);
    if (!staging_buffer || !staging_buffer_memory) return false;

    void* data;
    if (_context->device().mapMemory(staging_buffer_memory.get(), 0, buffer_size, {}, &data) != vk::Result::eSuccess)
      throw std::runtime_error("Failed to map vertex buffer memory!");

    std::memcpy(data, vertices.data(), buffer_size);
    _context->device().unmapMemory(staging_buffer_memory.get());

    auto [vertex_buffer, vertex_buffer_memory] = _context->create_buffer(
        buffer_size, vk::BufferUsageFlagBits::eVertexBuffer | vk::BufferUsageFlagBits::eTransferDst, vk::MemoryPropertyFlagBits::eDeviceLocal);
    if (!vertex_buffer || !vertex_buffer_memory) return false;

    _vertex_buffer = std::move(vertex_buffer);
    _vertex_buffer_memory = std::move(vertex_buffer_memory);

    _context->copy_buffer(staging_buffer.get(), _vertex_buffer.get(), buffer_size);

    return true;
  }

  [[nodiscard]] bool create_index_buffer() {
    const std::vector indices = {0, 1, 2, 2, 3, 0};
    const vk::DeviceSize buffer_size = sizeof indices[0] * indices.size();

    auto [staging_buffer, staging_buffer_memory] = _context->create_buffer(
        buffer_size, vk::BufferUsageFlagBits::eTransferSrc, vk::MemoryPropertyFlagBits::eHostVisible | vk::MemoryPropertyFlagBits::eHostCoherent);
    if (!staging_buffer || !staging_buffer_memory) return false;

    void* data;
    if (_context->device().mapMemory(staging_buffer_memory.get(), 0, buffer_size, {}, &data) != vk::Result::eSuccess)
      throw std::runtime_error("Failed to map vertex buffer memory!");

    std::memcpy(data, indices.data(), buffer_size);
    _context->device().unmapMemory(staging_buffer_memory.get());

    auto [index_buffer, index_buffer_memory] = _context->create_buffer(
        buffer_size, vk::BufferUsageFlagBits::eIndexBuffer | vk::BufferUsageFlagBits::eTransferDst, vk::MemoryPropertyFlagBits::eDeviceLocal);
    if (!index_buffer || !index_buffer_memory) return false;

    _index_buffer = std::move(index_buffer);
    _index_buffer_memory = std::move(index_buffer_memory);

    _context->copy_buffer(staging_buffer.get(), _index_buffer.get(), buffer_size);
    return true;
  }

  [[nodiscard]] bool create_field_buffers(const uint32_t width, const uint32_t height) {
    _field_buffer_size = sizeof(glm::vec4) * width * height;
    _field_buffers.resize(_renderer->frames_in_flight());
    _field_buffers_memory.resize(_renderer->frames_in_flight());
    for (size_t i = 0; i < _renderer->frames_in_flight(); i++) {
      auto [buf, mem] =
          _context->create_buffer(_field_buffer_size, vk::BufferUsageFlagBits::eStorageBuffer | vk::BufferUsageFlagBits::eTransferSrc, {});
      if (!buf || !mem) return false;

      _field_buffers[i] = std::move(buf);
      _field_buffers_memory[i] = std::move(mem);
    }
    return true;
  }

  void create_descriptor_sets() {
    std::array bindings = {vk::DescriptorSetLayoutBinding()
                               .setBinding(0)
                               .setDescriptorType(vk::DescriptorType::eStorageBuffer)
                               .setDescriptorCount(1)
                               .setStageFlags(vk::ShaderStageFlagBits::eFragment)};
    _descriptor_set_layout = _context->device().createDescriptorSetLayoutUnique(vk::DescriptorSetLayoutCreateInfo().setBindings(bindings));
    std::vector layouts(_renderer->frames_in_flight(), _descriptor_set_layout.get());
    _descriptor_sets = _context->device().allocateDescriptorSetsUnique(
        vk::DescriptorSetAllocateInfo().setDescriptorPool(_context->descriptor_pool()).setSetLayouts(layouts));
  }

  void update_field_descriptor_sets(const uint32_t width, const uint32_t height) {
    for (size_t i = 0; i < _renderer->frames_in_flight(); i++) {
      const vk::DescriptorBufferInfo field_info =
          vk::DescriptorBufferInfo().setBuffer(_field_buffers[i].get()).setOffset(0).setRange(sizeof(glm::vec4) * width * height);
      std::array descriptor_writes{
          vk::WriteDescriptorSet()
              .setDstSet(_descriptor_sets[i].get())
              .setDstBinding(0)
              .setDstArrayElement(0)
              .setDescriptorCount(1)
              .setDescriptorType(vk::DescriptorType::eStorageBuffer)
              .setBufferInfo(field_info),
      };
      _context->device().updateDescriptorSets(descriptor_writes, {});
    }
  }

  const helper::VulkanContext* _context;
  const VulkanRenderer* _renderer;

  vk::UniqueBuffer _vertex_buffer;
  vk::UniqueDeviceMemory _vertex_buffer_memory;
  vk::UniqueBuffer _index_buffer;
  vk::UniqueDeviceMemory _index_buffer_memory;
  std::vector<vk::UniqueBuffer> _field_buffers;
  std::vector<vk::UniqueDeviceMemory> _field_buffers_memory;
  size_t _field_buffer_size{0};

  std::vector<vk::UniqueDescriptorSet> _descriptor_sets;
  vk::UniqueDescriptorSetLayout _descriptor_set_layout;

  vk::UniquePipelineLayout _layout;
  vk::UniquePipeline _pipeline;

  uint32_t _width{0};
  uint32_t _height{0};
};

}  // namespace autd3::extra::simulator::slice_viewer
