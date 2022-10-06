// File: slice_viewer.hpp
// Project: simulator
// Created Date: 05/10/2022
// Author: Shun Suzuki
// -----
// Last Modified: 06/10/2022
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

#include "color.hpp"
#include "shader.hpp"
#include "update_flag.hpp"
#include "vulkan_renderer.hpp"

namespace autd3::extra::simulator::slice_viewer {

struct Vertex {
  glm::vec4 pos;
  glm::vec2 uv;
};

struct Data {
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
  explicit SliceViewer(const helper::VulkanContext* context, const VulkanRenderer* renderer, std::string shader)
      : _context(context), _renderer(renderer), _shader(std::move(shader)) {}
  ~SliceViewer() = default;
  SliceViewer(const SliceViewer& v) = delete;
  SliceViewer& operator=(const SliceViewer& obj) = delete;
  SliceViewer(SliceViewer&& obj) = default;
  SliceViewer& operator=(SliceViewer&& obj) = default;

  void init(const glm::mat4 model, const glm::mat4 view, const glm::mat4 proj, const uint32_t width, const uint32_t height) {
    create_pipeline();
    create_vertex_buffer(width, height);
    create_index_buffer();
    create_mvp_buffers();
    create_config_buffers();
    create_field_buffers(width, height);
    create_descriptor_sets();
    update_field_descriptor_sets(width, height);

    update_mvp_objects(model, view, proj);
    update_config_objects(width, height);
  }

  void render(const vk::CommandBuffer& command_buffer) {
    const vk::Buffer vertex_buffers[] = {_vertex_buffer.get()};
    constexpr vk::DeviceSize offsets[] = {0};

    command_buffer.bindPipeline(vk::PipelineBindPoint::eGraphics, _pipeline.get());
    command_buffer.bindDescriptorSets(vk::PipelineBindPoint::eGraphics, _layout.get(), 0, 1, &_descriptor_sets[_renderer->current_frame()].get(), 0,
                                      nullptr);
    command_buffer.bindVertexBuffers(0, 1, vertex_buffers, offsets);
    command_buffer.bindIndexBuffer(_index_buffer.get(), 0, vk::IndexType::eUint32);
    command_buffer.drawIndexed(6, 1, 0, 0, 0);
  }

  void update(const glm::mat4 model, const glm::mat4 view, const glm::mat4 proj, const uint32_t width, const uint32_t height,
              const UpdateFlags update_flag) {
    if (update_flag.contains(UpdateFlags::UPDATE_CAMERA_POS) || update_flag.contains(UpdateFlags::UPDATE_SLICE_POS))
      update_mvp_objects(model, view, proj);

    if (update_flag.contains(UpdateFlags::UPDATE_SLICE_SIZE)) {
      _context->device().waitIdle();
      create_field_buffers(width, height);
      update_field_descriptor_sets(width, height);
      create_vertex_buffer(width, height);
    }
  }

  [[nodiscard]] const std::vector<vk::UniqueBuffer>& images() const { return _field_buffers; }
  [[nodiscard]] size_t image_size() const { return _field_buffer_size; }

 private:
  void create_pipeline() {
    const auto vert_shader_code = helper::read_file(std::filesystem::path(_shader).append("slice.vert.spv").string());
    const auto frag_shader_code = helper::read_file(std::filesystem::path(_shader).append("slice.frag.spv").string());
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
                                                                    .setCullMode(vk::CullModeFlagBits::eBack)
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
                                 .setDescriptorType(vk::DescriptorType::eUniformBuffer)
                                 .setDescriptorCount(1)
                                 .setStageFlags(vk::ShaderStageFlagBits::eVertex),
                             vk::DescriptorSetLayoutBinding()
                                 .setBinding(1)
                                 .setDescriptorType(vk::DescriptorType::eUniformBuffer)
                                 .setDescriptorCount(1)
                                 .setStageFlags(vk::ShaderStageFlagBits::eFragment),
                             vk::DescriptorSetLayoutBinding()
                                 .setBinding(2)
                                 .setDescriptorType(vk::DescriptorType::eStorageBuffer)
                                 .setDescriptorCount(1)
                                 .setStageFlags(vk::ShaderStageFlagBits::eFragment)};
    const auto descriptor_set_layout_0 =
        _context->device().createDescriptorSetLayoutUnique(vk::DescriptorSetLayoutCreateInfo().setBindings(bindings_0));

    const std::array layouts = {descriptor_set_layout_0.get()};
    const vk::PipelineLayoutCreateInfo pipeline_layout_info = vk::PipelineLayoutCreateInfo().setSetLayouts(layouts);

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
        result.result == vk::Result::eSuccess)
      _pipeline = std::move(result.value);
    else
      throw std::runtime_error("failed to create a pipeline!");
  }

  void create_vertex_buffer(const uint32_t slice_width, const uint32_t slice_height) {
    const auto width = static_cast<float>(slice_width);
    const auto height = static_cast<float>(slice_height);
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

    void* data;
    if (_context->device().mapMemory(staging_buffer_memory.get(), 0, buffer_size, {}, &data) != vk::Result::eSuccess)
      throw std::runtime_error("failed to map vertex buffer memory!");
    std::memcpy(data, vertices.data(), buffer_size);
    _context->device().unmapMemory(staging_buffer_memory.get());

    auto [vertex_buffer, vertex_buffer_memory] = _context->create_buffer(
        buffer_size, vk::BufferUsageFlagBits::eVertexBuffer | vk::BufferUsageFlagBits::eTransferDst, vk::MemoryPropertyFlagBits::eDeviceLocal);

    _vertex_buffer = std::move(vertex_buffer);
    _vertex_buffer_memory = std::move(vertex_buffer_memory);

    _context->copy_buffer(staging_buffer.get(), _vertex_buffer.get(), buffer_size);
  }

  void create_index_buffer() {
    const std::vector indices = {0, 1, 2, 2, 3, 0};
    const vk::DeviceSize buffer_size = sizeof indices[0] * indices.size();

    auto [staging_buffer, staging_buffer_memory] = _context->create_buffer(
        buffer_size, vk::BufferUsageFlagBits::eTransferSrc, vk::MemoryPropertyFlagBits::eHostVisible | vk::MemoryPropertyFlagBits::eHostCoherent);

    void* data;
    if (_context->device().mapMemory(staging_buffer_memory.get(), 0, buffer_size, {}, &data) != vk::Result::eSuccess)
      throw std::runtime_error("failed to map vertex buffer memory!");
    std::memcpy(data, indices.data(), buffer_size);
    _context->device().unmapMemory(staging_buffer_memory.get());

    auto [index_buffer, index_buffer_memory] = _context->create_buffer(
        buffer_size, vk::BufferUsageFlagBits::eIndexBuffer | vk::BufferUsageFlagBits::eTransferDst, vk::MemoryPropertyFlagBits::eDeviceLocal);

    _index_buffer = std::move(index_buffer);
    _index_buffer_memory = std::move(index_buffer_memory);

    _context->copy_buffer(staging_buffer.get(), _index_buffer.get(), buffer_size);
  }

  void create_mvp_buffers() {
    _mvp_buffers.resize(_renderer->frames_in_flight());
    _mvp_buffers_memory.resize(_renderer->frames_in_flight());
    for (size_t i = 0; i < _renderer->frames_in_flight(); i++) {
      auto [buf, mem] = _context->create_buffer(sizeof(Data), vk::BufferUsageFlagBits::eUniformBuffer,
                                                vk::MemoryPropertyFlagBits::eHostVisible | vk::MemoryPropertyFlagBits::eHostCoherent);
      _mvp_buffers[i] = std::move(buf);
      _mvp_buffers_memory[i] = std::move(mem);
    }
  }

  void create_config_buffers() {
    _config_buffers.resize(_renderer->frames_in_flight());
    _config_buffers_memory.resize(_renderer->frames_in_flight());
    for (size_t i = 0; i < _renderer->frames_in_flight(); i++) {
      auto [buf, mem] = _context->create_buffer(sizeof(Config), vk::BufferUsageFlagBits::eUniformBuffer,
                                                vk::MemoryPropertyFlagBits::eHostVisible | vk::MemoryPropertyFlagBits::eHostCoherent);
      _config_buffers[i] = std::move(buf);
      _config_buffers_memory[i] = std::move(mem);
    }
  }

  void create_field_buffers(const uint32_t width, const uint32_t height) {
    _field_buffer_size = sizeof(glm::vec4) * width * height;
    _field_buffers.resize(_renderer->frames_in_flight());
    _field_buffers_memory.resize(_renderer->frames_in_flight());
    for (size_t i = 0; i < _renderer->frames_in_flight(); i++) {
      auto [buf, mem] = _context->create_buffer(_field_buffer_size, vk::BufferUsageFlagBits::eStorageBuffer,
                                                vk::MemoryPropertyFlagBits::eHostVisible | vk::MemoryPropertyFlagBits::eHostCoherent);
      _field_buffers[i] = std::move(buf);
      _field_buffers_memory[i] = std::move(mem);
    }
  }

  void create_descriptor_sets() {
    std::array bindings = {vk::DescriptorSetLayoutBinding()
                               .setBinding(0)
                               .setDescriptorType(vk::DescriptorType::eUniformBuffer)
                               .setDescriptorCount(1)
                               .setStageFlags(vk::ShaderStageFlagBits::eVertex),
                           vk::DescriptorSetLayoutBinding()
                               .setBinding(1)
                               .setDescriptorType(vk::DescriptorType::eUniformBuffer)
                               .setDescriptorCount(1)
                               .setStageFlags(vk::ShaderStageFlagBits::eFragment),
                           vk::DescriptorSetLayoutBinding()
                               .setBinding(2)
                               .setDescriptorType(vk::DescriptorType::eStorageBuffer)
                               .setDescriptorCount(1)
                               .setStageFlags(vk::ShaderStageFlagBits::eFragment)};
    _descriptor_set_layout = _context->device().createDescriptorSetLayoutUnique(vk::DescriptorSetLayoutCreateInfo().setBindings(bindings));
    std::vector layouts(_renderer->frames_in_flight(), _descriptor_set_layout.get());
    _descriptor_sets = _context->device().allocateDescriptorSetsUnique(
        vk::DescriptorSetAllocateInfo().setDescriptorPool(_context->descriptor_pool()).setSetLayouts(layouts));
    for (size_t i = 0; i < _renderer->frames_in_flight(); i++) {
      {
        const vk::DescriptorBufferInfo buffer_info = vk::DescriptorBufferInfo().setBuffer(_mvp_buffers[i].get()).setOffset(0).setRange(sizeof(Data));
        std::array descriptor_writes{
            vk::WriteDescriptorSet()
                .setDstSet(_descriptor_sets[i].get())
                .setDstBinding(0)
                .setDstArrayElement(0)
                .setDescriptorCount(1)
                .setDescriptorType(vk::DescriptorType::eUniformBuffer)
                .setBufferInfo(buffer_info),
        };
        _context->device().updateDescriptorSets(descriptor_writes, {});
      }
      {
        const vk::DescriptorBufferInfo config_info =
            vk::DescriptorBufferInfo().setBuffer(_config_buffers[i].get()).setOffset(0).setRange(sizeof(Config));
        std::array descriptor_writes{
            vk::WriteDescriptorSet()
                .setDstSet(_descriptor_sets[i].get())
                .setDstBinding(1)
                .setDstArrayElement(0)
                .setDescriptorCount(1)
                .setDescriptorType(vk::DescriptorType::eUniformBuffer)
                .setBufferInfo(config_info),
        };
        _context->device().updateDescriptorSets(descriptor_writes, {});
      }
    }
  }

  void update_field_descriptor_sets(const uint32_t width, const uint32_t height) {
    for (size_t i = 0; i < _renderer->frames_in_flight(); i++) {
      const vk::DescriptorBufferInfo field_info =
          vk::DescriptorBufferInfo().setBuffer(_field_buffers[i].get()).setOffset(0).setRange(sizeof(glm::vec4) * width * height);
      std::array descriptor_writes{
          vk::WriteDescriptorSet()
              .setDstSet(_descriptor_sets[i].get())
              .setDstBinding(2)
              .setDstArrayElement(0)
              .setDescriptorCount(1)
              .setDescriptorType(vk::DescriptorType::eStorageBuffer)
              .setBufferInfo(field_info),
      };
      _context->device().updateDescriptorSets(descriptor_writes, {});
    }
  }

  void update_mvp_objects(const glm::mat4 model, const glm::mat4 view, const glm::mat4 proj) {
    const Data ubo{model, view, proj};
    void* data;
    for (auto& mvp_buffer_memory : _mvp_buffers_memory) {
      if (_context->device().mapMemory(mvp_buffer_memory.get(), 0, sizeof ubo, {}, &data) != vk::Result::eSuccess)
        throw std::runtime_error("failed to map uniform buffer memory");
      memcpy(data, &ubo, sizeof ubo);
      _context->device().unmapMemory(mvp_buffer_memory.get());
    }
  }
  void update_config_objects(const uint32_t width, const uint32_t height) {
    const Config config{width, height, 0, 0};
    void* data;
    for (auto& config_buffer_memory : _config_buffers_memory) {
      if (_context->device().mapMemory(config_buffer_memory.get(), 0, sizeof config, {}, &data) != vk::Result::eSuccess)
        throw std::runtime_error("failed to map uniform buffer memory");
      memcpy(data, &config, sizeof config);
      _context->device().unmapMemory(config_buffer_memory.get());
    }
  }

  const helper::VulkanContext* _context;
  const VulkanRenderer* _renderer;
  std::string _shader;

  vk::UniqueBuffer _vertex_buffer;
  vk::UniqueDeviceMemory _vertex_buffer_memory;
  vk::UniqueBuffer _index_buffer;
  vk::UniqueDeviceMemory _index_buffer_memory;
  std::vector<vk::UniqueBuffer> _mvp_buffers;
  std::vector<vk::UniqueDeviceMemory> _mvp_buffers_memory;
  std::vector<vk::UniqueBuffer> _config_buffers;
  std::vector<vk::UniqueDeviceMemory> _config_buffers_memory;
  std::vector<vk::UniqueBuffer> _field_buffers;
  std::vector<vk::UniqueDeviceMemory> _field_buffers_memory;
  size_t _field_buffer_size{0};

  std::vector<vk::UniqueDescriptorSet> _descriptor_sets;
  vk::UniqueDescriptorSetLayout _descriptor_set_layout;

  vk::UniquePipelineLayout _layout;
  vk::UniquePipeline _pipeline;
};

}  // namespace autd3::extra::simulator::slice_viewer
