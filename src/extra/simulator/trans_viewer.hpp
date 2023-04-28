// File: trans_viewer.hpp
// Project: simulator
// Created Date: 03/10/2022
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

#include "coloring.hpp"
#include "shader.hpp"
#include "update_flag.hpp"
#include "vulkan_renderer.hpp"

namespace autd3::extra::simulator::trans_viewer {

struct ModelInstanceData {
  glm::mat4 model;
};

struct ColorInstanceData {
  glm::vec4 color;
};

struct Vertex {
  glm::vec4 pos;
  glm::vec2 uv;
};

struct Pc {
  glm::mat4 view;
  glm::mat4 proj;
};

class TransViewer {
 public:
  explicit TransViewer(const helper::VulkanContext* context, const VulkanRenderer* renderer)
      : _context(context), _renderer(renderer), _instance_count(0) {}
  ~TransViewer() = default;
  TransViewer(const TransViewer& v) = delete;
  TransViewer& operator=(const TransViewer& obj) = delete;
  TransViewer(TransViewer&& obj) = default;
  TransViewer& operator=(TransViewer&& obj) = default;

  void init(const std::vector<SoundSources>& sound_sources) {
    create_pipeline();
    create_texture();
    create_vertex_buffer();
    create_index_buffer();
    create_model_instance_buffer(sound_sources);
    create_color_instance_buffer(sound_sources);
    create_descriptor_sets();
  }

  void render(const glm::mat4 view, const glm::mat4 proj, const vk::CommandBuffer& command_buffer) {
    if (!_model_instance_buffer.get() || !_color_instance_buffer.get()) return;

    const vk::Buffer vertex_buffers[] = {_vertex_buffer.get()};
    const vk::Buffer model_instance_buffers[] = {_model_instance_buffer.get()};
    const vk::Buffer color_instance_buffers[] = {_color_instance_buffer.get()};
    constexpr vk::DeviceSize offsets[] = {0};

    command_buffer.bindVertexBuffers(0, 1, vertex_buffers, offsets);
    command_buffer.bindVertexBuffers(1, 1, model_instance_buffers, offsets);
    command_buffer.bindVertexBuffers(2, 1, color_instance_buffers, offsets);
    command_buffer.bindIndexBuffer(_index_buffer.get(), 0, vk::IndexType::eUint32);

    command_buffer.bindDescriptorSets(vk::PipelineBindPoint::eGraphics, _layout.get(), 0, 1, &_descriptor_sets[_renderer->current_frame()].get(), 0,
                                      nullptr);
    command_buffer.bindPipeline(vk::PipelineBindPoint::eGraphics, _pipeline.get());
    const Pc pc{view, proj};
    command_buffer.pushConstants(_layout.get(), vk::ShaderStageFlagBits::eVertex, 0, sizeof(Pc), &pc);
    command_buffer.drawIndexed(6, _instance_count, 0, 0, 0);
  }

  void update(const std::vector<SoundSources>& sound_sources, const driver::BitFlags<UpdateFlags> update_flag) {
    if (update_flag.contains(UpdateFlags::UpdateSourceDrive) || update_flag.contains(UpdateFlags::UpdateSourceAlpha) ||
        update_flag.contains(UpdateFlags::UpdateSourceFlag))
      return update_color_instance_buffer(sound_sources);
  }

 private:
  void create_pipeline() {
    const std::vector<uint8_t> vert_shader_code = {
#include "shaders/circle.vert.spv.txt"
    };
    const std::vector<uint8_t> frag_shader_code = {
#include "shaders/circle.frag.spv.txt"
    };

    vk::UniqueShaderModule vert_shader_module = helper::create_shader_module(_context->device(), vert_shader_code);
    vk::UniqueShaderModule frag_shader_module = helper::create_shader_module(_context->device(), frag_shader_code);

    std::array shader_stages = {
        vk::PipelineShaderStageCreateInfo().setStage(vk::ShaderStageFlagBits::eVertex).setModule(vert_shader_module.get()).setPName("main"),
        vk::PipelineShaderStageCreateInfo().setStage(vk::ShaderStageFlagBits::eFragment).setModule(frag_shader_module.get()).setPName("main")};

    std::array binding_descriptions = {
        vk::VertexInputBindingDescription().setBinding(0).setStride(sizeof(Vertex)).setInputRate(vk::VertexInputRate::eVertex),
        vk::VertexInputBindingDescription().setBinding(1).setStride(sizeof(ModelInstanceData)).setInputRate(vk::VertexInputRate::eInstance),
        vk::VertexInputBindingDescription().setBinding(2).setStride(sizeof(ColorInstanceData)).setInputRate(vk::VertexInputRate::eInstance),
    };
    std::array attribute_descriptions = {
        vk::VertexInputAttributeDescription(0, 0, vk::Format::eR32G32B32A32Sfloat, offsetof(Vertex, pos)),
        vk::VertexInputAttributeDescription(1, 0, vk::Format::eR32G32Sfloat, offsetof(Vertex, uv)),
        vk::VertexInputAttributeDescription(2, 1, vk::Format::eR32G32B32A32Sfloat, offsetof(ModelInstanceData, model)),
        vk::VertexInputAttributeDescription(3, 1, vk::Format::eR32G32B32A32Sfloat, offsetof(ModelInstanceData, model) + 1 * sizeof(glm::vec4)),
        vk::VertexInputAttributeDescription(4, 1, vk::Format::eR32G32B32A32Sfloat, offsetof(ModelInstanceData, model) + 2 * sizeof(glm::vec4)),
        vk::VertexInputAttributeDescription(5, 1, vk::Format::eR32G32B32A32Sfloat, offsetof(ModelInstanceData, model) + 3 * sizeof(glm::vec4)),
        vk::VertexInputAttributeDescription(6, 2, vk::Format::eR32G32B32A32Sfloat, offsetof(ColorInstanceData, color)),
    };
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
                                 .setDescriptorType(vk::DescriptorType::eCombinedImageSampler)
                                 .setDescriptorCount(1)
                                 .setStageFlags(vk::ShaderStageFlagBits::eFragment)};
    const auto descriptor_set_layout_0 =
        _context->device().createDescriptorSetLayoutUnique(vk::DescriptorSetLayoutCreateInfo().setBindings(bindings_0));

    const std::array push_constant_ranges = {
        vk::PushConstantRange().setStageFlags(vk::ShaderStageFlagBits::eVertex).setSize(sizeof(Pc)).setOffset(0)};
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
      return;
    }

    throw std::runtime_error("Failed to create a pipeline!");
  }

  void create_texture() {
    uint32_t mip_levels;
    {
      constexpr uint32_t tex_width = 128;
      constexpr uint32_t tex_height = 128;
      const std::vector<uint8_t> pixels = {
#include "textures/circle.png.txt"
      };
      constexpr auto image_size = tex_width * tex_height * 4;
      mip_levels = static_cast<uint32_t>(std::floor(std::log2(std::max(tex_width, tex_height)))) + 1;

      auto [staging_buffer, staging_buffer_memory] = _context->create_buffer(
          image_size, vk::BufferUsageFlagBits::eTransferSrc, vk::MemoryPropertyFlagBits::eHostVisible | vk::MemoryPropertyFlagBits::eHostCoherent);

      void* data;
      if (_context->device().mapMemory(staging_buffer_memory.get(), 0, image_size, {}, &data) != vk::Result::eSuccess)
        throw std::runtime_error("Failed to map texture buffer.");

      std::memcpy(data, pixels.data(), image_size);
      _context->device().unmapMemory(staging_buffer_memory.get());

      const auto flag = vk::ImageUsageFlagBits::eTransferDst | vk::ImageUsageFlagBits::eSampled | vk::ImageUsageFlagBits::eTransferSrc;
      auto [texture_image, texture_image_memory] =
          _context->create_image(tex_width, tex_height, mip_levels, vk::SampleCountFlagBits::e1, vk::Format::eR8G8B8A8Srgb, vk::ImageTiling::eOptimal,
                                 flag, vk::MemoryPropertyFlagBits::eDeviceLocal);
      _texture_image = std::move(texture_image);
      _texture_image_memory = std::move(texture_image_memory);

      _context->transition_image_layout(_texture_image, vk::Format::eR8G8B8A8Srgb, vk::ImageLayout::eUndefined, vk::ImageLayout::eTransferDstOptimal,
                                        mip_levels);
      _context->copy_buffer_to_image(staging_buffer, _texture_image, tex_width, tex_height);
      _context->generate_mipmaps(_texture_image, vk::Format::eR8G8B8A8Srgb, tex_width, tex_height, mip_levels);
    }

    { _texture_image_view = _context->create_image_view(_texture_image.get(), vk::Format::eR8G8B8A8Srgb, vk::ImageAspectFlagBits::eColor, 1); }
    {
      const auto properties = _context->physical_device().getProperties();

      _texture_sampler = _context->device().createSamplerUnique(vk::SamplerCreateInfo()
                                                                    .setMagFilter(vk::Filter::eLinear)
                                                                    .setMinFilter(vk::Filter::eLinear)
                                                                    .setMipmapMode(vk::SamplerMipmapMode::eLinear)
                                                                    .setAddressModeU(vk::SamplerAddressMode::eRepeat)
                                                                    .setAddressModeV(vk::SamplerAddressMode::eRepeat)
                                                                    .setAddressModeW(vk::SamplerAddressMode::eRepeat)
                                                                    .setMipLodBias(0.0f)
                                                                    .setAnisotropyEnable(true)
                                                                    .setMaxAnisotropy(properties.limits.maxSamplerAnisotropy)
                                                                    .setCompareEnable(false)
                                                                    .setCompareOp(vk::CompareOp::eAlways)
                                                                    .setMinLod(0.0f)
                                                                    .setMaxLod(static_cast<float>(mip_levels))
                                                                    .setBorderColor(vk::BorderColor::eIntOpaqueBlack)
                                                                    .setUnnormalizedCoordinates(false));
    }
  }

  void create_vertex_buffer() {
    const std::vector vertices = {
        Vertex{
            {-1.0, -1.0, 0.0, 1.0},
            {0.0, 1.0},
        },
        Vertex{
            {1.0, -1.0, 0.0, 1.0},
            {1.0, 1.0},
        },
        Vertex{
            {1.0, 1.0, 0.0, 1.0},
            {1.0, 0.0},
        },
        Vertex{
            {-1.0, 1.0, 0.0, 1.0},
            {0.0, 0.0},
        },
    };
    const vk::DeviceSize buffer_size = sizeof vertices[0] * vertices.size();

    auto [staging_buffer, staging_buffer_memory] = _context->create_buffer(
        buffer_size, vk::BufferUsageFlagBits::eTransferSrc, vk::MemoryPropertyFlagBits::eHostVisible | vk::MemoryPropertyFlagBits::eHostCoherent);

    void* data;
    if (_context->device().mapMemory(staging_buffer_memory.get(), 0, buffer_size, {}, &data) != vk::Result::eSuccess)
      throw std::runtime_error("Failed to map vertex buffer memory!");

    std::memcpy(data, vertices.data(), buffer_size);
    _context->device().unmapMemory(staging_buffer_memory.get());

    auto [vertex_buffer, vertex_buffer_memory] = _context->create_buffer(
        buffer_size, vk::BufferUsageFlagBits::eVertexBuffer | vk::BufferUsageFlagBits::eTransferDst, vk::MemoryPropertyFlagBits::eDeviceLocal);

    _vertex_buffer = std::move(vertex_buffer);
    _vertex_buffer_memory = std::move(vertex_buffer_memory);

    _context->copy_buffer(staging_buffer.get(), _vertex_buffer.get(), buffer_size);
  }

  void create_model_instance_buffer(const std::vector<SoundSources>& sound_sources) {
    std::vector<glm::mat4> models;
    _instance_count = static_cast<uint32_t>(
        std::accumulate(sound_sources.begin(), sound_sources.end(), size_t{0}, [](auto acc, const auto& s) { return acc + s.size(); }));
    models.reserve(_instance_count);

    for (const auto& sources : sound_sources) {
      const auto& positions = sources.positions();
      const auto& rotations = sources.rotations();
      std::transform(positions.begin(), positions.end(), rotations.begin(), std::back_inserter(models), [this](const auto& pos, const auto& rot) {
        constexpr auto s = 10.0f * 0.5f * scale;
        auto m = glm::scale(glm::identity<glm::mat4>(), glm::vec3(s, s, s));
        m[3].x = pos.x;
        m[3].y = pos.y;
        m[3].z = pos.z;
        m = m * mat4_cast(rot);
        return m;
      });
    }

    const vk::DeviceSize buffer_size = sizeof models[0] * models.size();

    auto [staging_buffer, staging_buffer_memory] = _context->create_buffer(
        buffer_size, vk::BufferUsageFlagBits::eTransferSrc, vk::MemoryPropertyFlagBits::eHostVisible | vk::MemoryPropertyFlagBits::eHostCoherent);

    void* data;
    if (_context->device().mapMemory(staging_buffer_memory.get(), 0, buffer_size, {}, &data) != vk::Result::eSuccess)
      throw std::runtime_error("Failed to map vertex buffer memory!");

    std::memcpy(data, models.data(), buffer_size);
    _context->device().unmapMemory(staging_buffer_memory.get());

    auto [model_instance_buffer, model_instance_buffer_memory] = _context->create_buffer(
        buffer_size, vk::BufferUsageFlagBits::eVertexBuffer | vk::BufferUsageFlagBits::eTransferDst, vk::MemoryPropertyFlagBits::eDeviceLocal);

    _model_instance_buffer = std::move(model_instance_buffer);
    _model_instance_buffer_memory = std::move(model_instance_buffer_memory);

    _context->copy_buffer(staging_buffer.get(), _model_instance_buffer.get(), buffer_size);
  }

  void create_color_instance_buffer(const std::vector<SoundSources>& sound_sources) {
    std::vector<glm::vec4> colors;
    colors.reserve(_instance_count);
    for (const auto& s : sound_sources)
      std::transform(
          s.drives().begin(), s.drives().end(), s.visibilities().begin(), std::back_inserter(colors),
          [](const auto& drive, const auto& visibility) { return coloring_hsv(drive.phase / (2.0f * glm::pi<float>()), drive.amp, visibility); });

    const vk::DeviceSize buffer_size = sizeof colors[0] * colors.size();

    auto [staging_buffer, staging_buffer_memory] = _context->create_buffer(
        buffer_size, vk::BufferUsageFlagBits::eTransferSrc, vk::MemoryPropertyFlagBits::eHostVisible | vk::MemoryPropertyFlagBits::eHostCoherent);

    void* data;
    if (_context->device().mapMemory(staging_buffer_memory.get(), 0, buffer_size, {}, &data) != vk::Result::eSuccess)
      throw std::runtime_error("Failed to map vertex buffer memory!");
    std::memcpy(data, colors.data(), buffer_size);
    _context->device().unmapMemory(staging_buffer_memory.get());

    auto [color_instance_buffer, color_instance_buffer_memory] = _context->create_buffer(
        buffer_size, vk::BufferUsageFlagBits::eVertexBuffer | vk::BufferUsageFlagBits::eTransferDst, vk::MemoryPropertyFlagBits::eDeviceLocal);

    _color_instance_buffer = std::move(color_instance_buffer);
    _color_instance_buffer_memory = std::move(color_instance_buffer_memory);

    _context->copy_buffer(staging_buffer.get(), _color_instance_buffer.get(), buffer_size);
  }

  void update_color_instance_buffer(const std::vector<SoundSources>& sources) {
    std::vector<glm::vec4> colors;
    colors.reserve(_instance_count);
    for (const auto& s : sources)
      std::transform(
          s.drives().begin(), s.drives().end(), s.visibilities().begin(), std::back_inserter(colors),
          [](const auto& drive, const auto& visibility) { return coloring_hsv(drive.phase / (2.0f * glm::pi<float>()), drive.amp, visibility); });

    const vk::DeviceSize buffer_size = sizeof colors[0] * colors.size();

    auto [staging_buffer, staging_buffer_memory] = _context->create_buffer(
        buffer_size, vk::BufferUsageFlagBits::eTransferSrc, vk::MemoryPropertyFlagBits::eHostVisible | vk::MemoryPropertyFlagBits::eHostCoherent);

    void* data;
    if (_context->device().mapMemory(staging_buffer_memory.get(), 0, buffer_size, {}, &data) != vk::Result::eSuccess)
      throw std::runtime_error("Failed to map vertex buffer memory!");

    std::memcpy(data, colors.data(), buffer_size);
    _context->device().unmapMemory(staging_buffer_memory.get());

    _context->copy_buffer(staging_buffer.get(), _color_instance_buffer.get(), buffer_size);
  }

  void create_index_buffer() {
    const std::vector indices = {0, 1, 2, 2, 3, 0};
    const vk::DeviceSize buffer_size = sizeof indices[0] * indices.size();

    auto [staging_buffer, staging_buffer_memory] = _context->create_buffer(
        buffer_size, vk::BufferUsageFlagBits::eTransferSrc, vk::MemoryPropertyFlagBits::eHostVisible | vk::MemoryPropertyFlagBits::eHostCoherent);

    void* data;
    if (_context->device().mapMemory(staging_buffer_memory.get(), 0, buffer_size, {}, &data) != vk::Result::eSuccess)
      throw std::runtime_error("Failed to map vertex buffer memory!");

    std::memcpy(data, indices.data(), buffer_size);
    _context->device().unmapMemory(staging_buffer_memory.get());

    auto [index_buffer, index_buffer_memory] = _context->create_buffer(
        buffer_size, vk::BufferUsageFlagBits::eIndexBuffer | vk::BufferUsageFlagBits::eTransferDst, vk::MemoryPropertyFlagBits::eDeviceLocal);

    _index_buffer = std::move(index_buffer);
    _index_buffer_memory = std::move(index_buffer_memory);

    _context->copy_buffer(staging_buffer.get(), _index_buffer.get(), buffer_size);
  }

  void create_descriptor_sets() {
    std::array bindings = {vk::DescriptorSetLayoutBinding()
                               .setBinding(0)
                               .setDescriptorType(vk::DescriptorType::eCombinedImageSampler)
                               .setDescriptorCount(1)
                               .setStageFlags(vk::ShaderStageFlagBits::eFragment)};
    _descriptor_set_layout = _context->device().createDescriptorSetLayoutUnique(vk::DescriptorSetLayoutCreateInfo().setBindings(bindings));
    std::vector layouts(_renderer->frames_in_flight(), _descriptor_set_layout.get());
    _descriptor_sets = _context->device().allocateDescriptorSetsUnique(
        vk::DescriptorSetAllocateInfo().setDescriptorPool(_context->descriptor_pool()).setSetLayouts(layouts));
    for (size_t i = 0; i < _renderer->frames_in_flight(); i++) {
      const vk::DescriptorImageInfo image_info = vk::DescriptorImageInfo()
                                                     .setSampler(_texture_sampler.get())
                                                     .setImageView(_texture_image_view.get())
                                                     .setImageLayout(vk::ImageLayout::eShaderReadOnlyOptimal);
      std::array descriptor_writes{
          vk::WriteDescriptorSet()
              .setDstSet(_descriptor_sets[i].get())
              .setDstBinding(0)
              .setDstArrayElement(0)
              .setDescriptorCount(1)
              .setDescriptorType(vk::DescriptorType::eCombinedImageSampler)
              .setImageInfo(image_info),
      };
      _context->device().updateDescriptorSets(descriptor_writes, {});
    }
  }

  const helper::VulkanContext* _context;
  const VulkanRenderer* _renderer;

  vk::UniqueImage _texture_image;
  vk::UniqueDeviceMemory _texture_image_memory;
  vk::UniqueImageView _texture_image_view;
  vk::UniqueSampler _texture_sampler;

  vk::UniqueBuffer _vertex_buffer;
  vk::UniqueDeviceMemory _vertex_buffer_memory;
  vk::UniqueBuffer _model_instance_buffer;
  vk::UniqueDeviceMemory _model_instance_buffer_memory;
  vk::UniqueBuffer _color_instance_buffer;
  vk::UniqueDeviceMemory _color_instance_buffer_memory;
  vk::UniqueBuffer _index_buffer;
  vk::UniqueDeviceMemory _index_buffer_memory;

  std::vector<vk::UniqueDescriptorSet> _descriptor_sets;
  vk::UniqueDescriptorSetLayout _descriptor_set_layout;

  vk::UniquePipelineLayout _layout;
  vk::UniquePipeline _pipeline;

  uint32_t _instance_count;
};

}  // namespace autd3::extra::simulator::trans_viewer
