// File: vulkan_renderer.hpp
// Project: include
// Created Date: 24/09/2022
// Author: Shun Suzuki
// -----
// Last Modified: 29/09/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <imgui.h>
#include <imgui_impl_vulkan.h>

#include <fstream>
#include <limits>
#include <string>
#include <utility>
#include <vector>

#define GLM_FORCE_DEPTH_ZERO_TO_ONE
#include <glm/glm.hpp>
#include <glm/gtc/matrix_transform.hpp>

#include "model.hpp"
#include "vulkan_handler.hpp"
#include "vulkan_imgui.hpp"

namespace autd3::extra::geometry_viewer {

struct UniformBufferObject {
  glm::mat4 view;
  glm::mat4 proj;
  glm::vec4 light_pos;
  glm::vec4 view_pos;
};

class VulkanRenderer {
 public:
  std::string font_path;
  explicit VulkanRenderer(const VulkanHandler* handler, const WindowHandler* window, std::string shader, std::string font_path,
                          const bool vsync = true) noexcept
      : _handler(handler),
        _window(window),
        _shader(std::move(shader)),
        _font_path(std::move(font_path)),
        _swap_chain(nullptr),
        _render_pass(nullptr),
        _depth_image(nullptr),
        _depth_image_memory(nullptr),
        _depth_image_view(nullptr),
        _color_image(nullptr),
        _color_image_memory(nullptr),
        _color_image_view(nullptr),
        _vertex_buffer(nullptr),
        _vertex_buffer_memory(nullptr),
        _index_buffer(nullptr),
        _index_buffer_memory(nullptr),
        _vsync(vsync) {}
  ~VulkanRenderer() = default;
  VulkanRenderer(const VulkanRenderer& v) = delete;
  VulkanRenderer& operator=(const VulkanRenderer& obj) = delete;
  VulkanRenderer(VulkanRenderer&& obj) = default;
  VulkanRenderer& operator=(VulkanRenderer&& obj) = delete;

  void init() {
    const auto [fst, snd] = _window->scale();
    _last_font_factor = (fst + snd) / 2.0f;
  }

  void create_swapchain() {
    const auto [capabilities, formats, present_modes] = _handler->query_swap_chain_support(_handler->physical_device());

    const vk::SurfaceFormatKHR surface_format = choose_swap_surface_format(formats);
    const vk::PresentModeKHR present_mode = _vsync ? vk::PresentModeKHR::eFifo : choose_swap_present_mode(present_modes);
    const vk::Extent2D extent = choose_swap_extent(capabilities, _window->window());

    uint32_t image_count = capabilities.minImageCount + 1;
    if (capabilities.maxImageCount > 0 && image_count > capabilities.maxImageCount) image_count = capabilities.maxImageCount;

    vk::SwapchainCreateInfoKHR create_info = vk::SwapchainCreateInfoKHR()
                                                 .setSurface(_handler->surface())
                                                 .setMinImageCount(image_count)
                                                 .setImageFormat(surface_format.format)
                                                 .setImageColorSpace(surface_format.colorSpace)
                                                 .setImageExtent(extent)
                                                 .setImageArrayLayers(1)
                                                 .setImageUsage(vk::ImageUsageFlagBits::eColorAttachment)
                                                 .setImageSharingMode(vk::SharingMode::eExclusive)
                                                 .setPreTransform(capabilities.currentTransform)
                                                 .setCompositeAlpha(vk::CompositeAlphaFlagBitsKHR::eOpaque)
                                                 .setPresentMode(present_mode)
                                                 .setClipped(true);

    if (const auto [graphics_family, present_family] = _handler->find_queue_families(_handler->physical_device());
        graphics_family != present_family) {
      create_info.setImageSharingMode(vk::SharingMode::eConcurrent);
      std::vector queue_family_indices = {graphics_family.value(), present_family.value()};
      create_info.setQueueFamilyIndices(queue_family_indices);
    }

    _swap_chain = _handler->device().createSwapchainKHRUnique(create_info);
    _swap_chain_images = _handler->device().getSwapchainImagesKHR(_swap_chain.get());
    _swap_chain_image_format = surface_format.format;
    _swap_chain_extent = extent;
  }

  void create_image_views() {
    _swap_chain_image_views.resize(_swap_chain_images.size());
    for (size_t i = 0; i < _swap_chain_image_views.size(); i++)
      _swap_chain_image_views[i] = _handler->create_image_view(_swap_chain_images[i], _swap_chain_image_format, vk::ImageAspectFlagBits::eColor, 1);
  }

  void create_framebuffers() {
    _swap_chain_framebuffers.resize(_swap_chain_image_views.size());
    for (size_t i = 0; i < _swap_chain_image_views.size(); i++) {
      std::vector<vk::ImageView> attachments;
      if (_handler->msaa_enable())
        attachments.emplace_back(_color_image_view.get());
      else
        attachments.emplace_back(_swap_chain_image_views[i].get());
      attachments.emplace_back(_depth_image_view.get());
      if (_handler->msaa_enable()) attachments.emplace_back(_swap_chain_image_views[i].get());
      vk::FramebufferCreateInfo framebuffer_create_info = vk::FramebufferCreateInfo()
                                                              .setRenderPass(_render_pass.get())
                                                              .setAttachments(attachments)
                                                              .setWidth(_swap_chain_extent.width)
                                                              .setHeight(_swap_chain_extent.height)
                                                              .setLayers(1);
      _swap_chain_framebuffers[i] = _handler->device().createFramebufferUnique(framebuffer_create_info);
    }
  }

  void create_render_pass() {
    std::vector attachments = {
        vk::AttachmentDescription()
            .setFormat(_swap_chain_image_format)
            .setSamples(_handler->msaa_samples())
            .setLoadOp(vk::AttachmentLoadOp::eClear)
            .setStoreOp(vk::AttachmentStoreOp::eStore)
            .setStencilLoadOp(vk::AttachmentLoadOp::eDontCare)
            .setStencilStoreOp(vk::AttachmentStoreOp::eDontCare)
            .setInitialLayout(vk::ImageLayout::eUndefined)
            .setFinalLayout(_handler->msaa_enable() ? vk::ImageLayout::eColorAttachmentOptimal : vk::ImageLayout::ePresentSrcKHR),
        vk::AttachmentDescription()
            .setFormat(_handler->find_depth_format())
            .setSamples(_handler->msaa_samples())
            .setLoadOp(vk::AttachmentLoadOp::eClear)
            .setStoreOp(vk::AttachmentStoreOp::eDontCare)
            .setStencilLoadOp(vk::AttachmentLoadOp::eDontCare)
            .setStencilStoreOp(vk::AttachmentStoreOp::eDontCare)
            .setInitialLayout(vk::ImageLayout::eUndefined)
            .setFinalLayout(vk::ImageLayout::eDepthStencilAttachmentOptimal)};
    if (_handler->msaa_enable())
      attachments.emplace_back(vk::AttachmentDescription()
                                   .setFormat(_swap_chain_image_format)
                                   .setSamples(vk::SampleCountFlagBits::e1)
                                   .setLoadOp(vk::AttachmentLoadOp::eDontCare)
                                   .setStoreOp(vk::AttachmentStoreOp::eStore)
                                   .setStencilLoadOp(vk::AttachmentLoadOp::eDontCare)
                                   .setStencilStoreOp(vk::AttachmentStoreOp::eDontCare)
                                   .setInitialLayout(vk::ImageLayout::eUndefined)
                                   .setFinalLayout(vk::ImageLayout::ePresentSrcKHR));
    const std::array color_attachments = {vk::AttachmentReference().setAttachment(0).setLayout(vk::ImageLayout::eColorAttachmentOptimal)};
    const vk::AttachmentReference depth_attachment =
        vk::AttachmentReference().setAttachment(1).setLayout(vk::ImageLayout::eDepthStencilAttachmentOptimal);
    const std::array resolve_attachments = {vk::AttachmentReference().setAttachment(2).setLayout(vk::ImageLayout::eColorAttachmentOptimal)};
    vk::SubpassDescription subpass = vk::SubpassDescription()
                                         .setPipelineBindPoint(vk::PipelineBindPoint::eGraphics)
                                         .setColorAttachments(color_attachments)
                                         .setPDepthStencilAttachment(&depth_attachment);
    if (_handler->msaa_enable()) subpass.setResolveAttachments(resolve_attachments);
    const std::array subpasses = {subpass};
    const std::array dependencies = {
        vk::SubpassDependency()
            .setSrcSubpass(VK_SUBPASS_EXTERNAL)
            .setDstSubpass(0)
            .setSrcStageMask(vk::PipelineStageFlagBits::eColorAttachmentOutput | vk::PipelineStageFlagBits::eEarlyFragmentTests)
            .setDstStageMask(vk::PipelineStageFlagBits::eColorAttachmentOutput | vk::PipelineStageFlagBits::eEarlyFragmentTests)
            .setDstAccessMask(vk::AccessFlagBits::eColorAttachmentWrite | vk::AccessFlagBits::eDepthStencilAttachmentWrite)};
    const vk::RenderPassCreateInfo render_pass_info =
        vk::RenderPassCreateInfo().setAttachments(attachments).setSubpasses(subpasses).setDependencies(dependencies);
    _render_pass = _handler->device().createRenderPassUnique(render_pass_info);
  }

  void create_graphics_pipeline(const gltf::Model& model) {
    const auto vert_shader_code = read_file(std::filesystem::path(_shader).append("vert.spv").string());
    const auto frag_shader_code = read_file(std::filesystem::path(_shader).append("frag.spv").string());

    vk::UniqueShaderModule vert_shader_module = create_shader_module(_handler->device(), vert_shader_code);
    vk::UniqueShaderModule frag_shader_module = create_shader_module(_handler->device(), frag_shader_code);

    std::array shader_stages = {
        vk::PipelineShaderStageCreateInfo().setStage(vk::ShaderStageFlagBits::eVertex).setModule(vert_shader_module.get()).setPName("main"),
        vk::PipelineShaderStageCreateInfo().setStage(vk::ShaderStageFlagBits::eFragment).setModule(frag_shader_module.get()).setPName("main")};

    auto binding_description = gltf::Vertex::get_binding_description();
    auto attribute_descriptions = gltf::Vertex::get_attribute_descriptions();
    const vk::PipelineVertexInputStateCreateInfo vertex_input_info = vk::PipelineVertexInputStateCreateInfo()
                                                                         .setVertexBindingDescriptions(binding_description)
                                                                         .setVertexAttributeDescriptions(attribute_descriptions);

    const vk::PipelineInputAssemblyStateCreateInfo input_assembly =
        vk::PipelineInputAssemblyStateCreateInfo().setTopology(vk::PrimitiveTopology::eTriangleList);
    const vk::Viewport viewport = vk::Viewport()
                                      .setX(0.0f)
                                      .setY(0.0f)
                                      .setWidth(static_cast<float>(_swap_chain_extent.width))
                                      .setHeight(static_cast<float>(_swap_chain_extent.height))
                                      .setMinDepth(0.0f)
                                      .setMaxDepth(1.0f);
    const vk::Rect2D scissor = vk::Rect2D().setOffset({0, 0}).setExtent(_swap_chain_extent);
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
        vk::PipelineMultisampleStateCreateInfo().setRasterizationSamples(_handler->msaa_samples()).setSampleShadingEnable(false);

    const vk::PipelineColorBlendAttachmentState color_blend_attachment =
        vk::PipelineColorBlendAttachmentState().setBlendEnable(false).setColorWriteMask(
            vk::ColorComponentFlagBits::eR | vk::ColorComponentFlagBits::eG | vk::ColorComponentFlagBits::eB | vk::ColorComponentFlagBits::eA);

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
                                 .setDescriptorType(vk::DescriptorType::eCombinedImageSampler)
                                 .setDescriptorCount(1)
                                 .setStageFlags(vk::ShaderStageFlagBits::eFragment)};
    const auto descriptor_set_layout_0 =
        _handler->device().createDescriptorSetLayoutUnique(vk::DescriptorSetLayoutCreateInfo().setBindings(bindings_0));

    const std::array push_constant_ranges = {
        vk::PushConstantRange().setStageFlags(vk::ShaderStageFlagBits::eVertex).setSize(sizeof(glm::mat4)),
        vk::PushConstantRange().setStageFlags(vk::ShaderStageFlagBits::eFragment).setSize(sizeof(gltf::Lighting)).setOffset(sizeof(glm::mat4))};
    const std::array layouts = {descriptor_set_layout_0.get()};
    const vk::PipelineLayoutCreateInfo pipeline_layout_info =
        vk::PipelineLayoutCreateInfo().setSetLayouts(layouts).setPushConstantRanges(push_constant_ranges);

    _pipeline_layout = _handler->device().createPipelineLayoutUnique(pipeline_layout_info);

    const vk::PipelineDepthStencilStateCreateInfo depth_stencil = vk::PipelineDepthStencilStateCreateInfo()
                                                                      .setDepthTestEnable(true)
                                                                      .setDepthWriteEnable(true)
                                                                      .setDepthCompareOp(vk::CompareOp::eLess)
                                                                      .setDepthBoundsTestEnable(false)
                                                                      .setStencilTestEnable(false)
                                                                      .setMinDepthBounds(0.0f)
                                                                      .setMaxDepthBounds(1.0f);
    _pipelines.resize(model.materials().size());
    for (size_t i = 0; i < model.materials().size(); i++) {
      const auto [base_color_factor, base_color_texture_idx, metallic_factor, roughness_factor] = model.materials()[i];
      struct MaterialSpecializationData {
        vk::Bool32 has_texture;
        float base_color_r;
        float base_color_g;
        float base_color_b;
      } material_specialization_data{};
      material_specialization_data.has_texture = base_color_texture_idx != -1;
      material_specialization_data.base_color_r = base_color_factor.r;
      material_specialization_data.base_color_g = base_color_factor.g;
      material_specialization_data.base_color_b = base_color_factor.b;
      const std::array specialization_map_entries = {vk::SpecializationMapEntry()
                                                         .setConstantID(0)
                                                         .setOffset(offsetof(MaterialSpecializationData, has_texture))
                                                         .setSize(sizeof material_specialization_data.has_texture),
                                                     vk::SpecializationMapEntry()
                                                         .setConstantID(1)
                                                         .setOffset(offsetof(MaterialSpecializationData, base_color_r))
                                                         .setSize(sizeof material_specialization_data.base_color_r),
                                                     vk::SpecializationMapEntry()
                                                         .setConstantID(2)
                                                         .setOffset(offsetof(MaterialSpecializationData, base_color_g))
                                                         .setSize(sizeof material_specialization_data.base_color_g),
                                                     vk::SpecializationMapEntry()
                                                         .setConstantID(3)
                                                         .setOffset(offsetof(MaterialSpecializationData, base_color_b))
                                                         .setSize(sizeof material_specialization_data.base_color_b)};
      const vk::SpecializationInfo specialization_info = vk::SpecializationInfo()
                                                             .setMapEntries(specialization_map_entries)
                                                             .setPData(&material_specialization_data)
                                                             .setDataSize(sizeof(MaterialSpecializationData));
      shader_stages[1].setPSpecializationInfo(&specialization_info);
      if (auto result = _handler->device().createGraphicsPipelineUnique({}, vk::GraphicsPipelineCreateInfo()
                                                                                .setStages(shader_stages)
                                                                                .setPVertexInputState(&vertex_input_info)
                                                                                .setPInputAssemblyState(&input_assembly)
                                                                                .setPViewportState(&viewport_state)
                                                                                .setPRasterizationState(&rasterizer)
                                                                                .setPMultisampleState(&multi_sampling)
                                                                                .setPDepthStencilState(&depth_stencil)
                                                                                .setPColorBlendState(&color_blending)
                                                                                .setPDynamicState(&dynamic_state)
                                                                                .setLayout(_pipeline_layout.get())
                                                                                .setRenderPass(_render_pass.get()));
          result.result == vk::Result::eSuccess)
        _pipelines[i] = std::move(result.value);
      else
        throw std::runtime_error("failed to create a pipeline!");
    }
  }

  void create_depth_resources() {
    const auto depth_format = _handler->find_depth_format();
    auto [depth_image, depth_image_memory] =
        _handler->create_image(_swap_chain_extent.width, _swap_chain_extent.height, 1, _handler->msaa_samples(), depth_format,
                               vk::ImageTiling::eOptimal, vk::ImageUsageFlagBits::eDepthStencilAttachment, vk::MemoryPropertyFlagBits::eDeviceLocal);
    _depth_image = std::move(depth_image);
    _depth_image_memory = std::move(depth_image_memory);
    _depth_image_view = _handler->create_image_view(_depth_image.get(), depth_format, vk::ImageAspectFlagBits::eDepth, 1);

    _handler->transition_image_layout(_depth_image, depth_format, vk::ImageLayout::eUndefined, vk::ImageLayout::eDepthStencilAttachmentOptimal, 1);
  }

  void create_color_resources() {
    const auto color_format = _swap_chain_image_format;

    auto [color_image, color_image_memory] = _handler->create_image(
        _swap_chain_extent.width, _swap_chain_extent.height, 1, _handler->msaa_samples(), color_format, vk::ImageTiling::eOptimal,
        vk::ImageUsageFlagBits::eTransientAttachment | vk::ImageUsageFlagBits::eColorAttachment, vk::MemoryPropertyFlagBits::eDeviceLocal);
    _color_image = std::move(color_image);
    _color_image_memory = std::move(color_image_memory);

    _color_image_view = _handler->create_image_view(_color_image.get(), color_format, vk::ImageAspectFlagBits::eColor, 1);
  }

  void create_vertex_buffer(const gltf::Model& model) {
    const auto& vertices = model.vertices();
    const vk::DeviceSize buffer_size = sizeof vertices[0] * vertices.size();

    auto [staging_buffer, staging_buffer_memory] = _handler->create_buffer(
        buffer_size, vk::BufferUsageFlagBits::eTransferSrc, vk::MemoryPropertyFlagBits::eHostVisible | vk::MemoryPropertyFlagBits::eHostCoherent);

    void* data;
    if (_handler->device().mapMemory(staging_buffer_memory.get(), 0, buffer_size, {}, &data) != vk::Result::eSuccess)
      throw std::runtime_error("failed to map vertex buffer memory!");
    std::memcpy(data, vertices.data(), buffer_size);
    _handler->device().unmapMemory(staging_buffer_memory.get());

    auto [vertex_buffer, vertex_buffer_memory] = _handler->create_buffer(
        buffer_size, vk::BufferUsageFlagBits::eVertexBuffer | vk::BufferUsageFlagBits::eTransferDst, vk::MemoryPropertyFlagBits::eDeviceLocal);

    _vertex_buffer = std::move(vertex_buffer);
    _vertex_buffer_memory = std::move(vertex_buffer_memory);

    _handler->copy_buffer(staging_buffer.get(), _vertex_buffer.get(), buffer_size);
  }

  void create_index_buffer(const gltf::Model& model) {
    const auto& indices = model.indices();
    const vk::DeviceSize buffer_size = sizeof indices[0] * indices.size();

    auto [staging_buffer, staging_buffer_memory] = _handler->create_buffer(
        buffer_size, vk::BufferUsageFlagBits::eTransferSrc, vk::MemoryPropertyFlagBits::eHostVisible | vk::MemoryPropertyFlagBits::eHostCoherent);

    void* data;
    if (_handler->device().mapMemory(staging_buffer_memory.get(), 0, buffer_size, {}, &data) != vk::Result::eSuccess)
      throw std::runtime_error("failed to map vertex buffer memory!");
    std::memcpy(data, indices.data(), buffer_size);
    _handler->device().unmapMemory(staging_buffer_memory.get());

    auto [index_buffer, index_buffer_memory] = _handler->create_buffer(
        buffer_size, vk::BufferUsageFlagBits::eIndexBuffer | vk::BufferUsageFlagBits::eTransferDst, vk::MemoryPropertyFlagBits::eDeviceLocal);

    _index_buffer = std::move(index_buffer);
    _index_buffer_memory = std::move(index_buffer_memory);

    _handler->copy_buffer(staging_buffer.get(), _index_buffer.get(), buffer_size);
  }

  void create_uniform_buffers() {
    _uniform_buffers.resize(_max_frames_in_flight);
    _uniform_buffers_memory.resize(_max_frames_in_flight);
    for (size_t i = 0; i < _max_frames_in_flight; i++) {
      auto [buf, mem] = _handler->create_buffer(sizeof(UniformBufferObject), vk::BufferUsageFlagBits::eUniformBuffer,
                                                vk::MemoryPropertyFlagBits::eHostVisible | vk::MemoryPropertyFlagBits::eHostCoherent);
      _uniform_buffers[i] = std::move(buf);
      _uniform_buffers_memory[i] = std::move(mem);
    }
  }

  void create_descriptor_sets() {
    std::vector bindings = {vk::DescriptorSetLayoutBinding()
                                .setBinding(0)
                                .setDescriptorType(vk::DescriptorType::eUniformBuffer)
                                .setDescriptorCount(1)
                                .setStageFlags(vk::ShaderStageFlagBits::eVertex)};
    bindings.emplace_back(vk::DescriptorSetLayoutBinding()
                              .setBinding(1)
                              .setDescriptorType(vk::DescriptorType::eCombinedImageSampler)
                              .setDescriptorCount(1)
                              .setStageFlags(vk::ShaderStageFlagBits::eFragment));
    _descriptor_set_layout = _handler->device().createDescriptorSetLayoutUnique(vk::DescriptorSetLayoutCreateInfo().setBindings(bindings));
    std::vector layouts(_max_frames_in_flight, _descriptor_set_layout.get());
    _descriptor_sets = _handler->device().allocateDescriptorSetsUnique(
        vk::DescriptorSetAllocateInfo().setDescriptorPool(_handler->descriptor_pool()).setSetLayouts(layouts));
    for (size_t i = 0; i < _max_frames_in_flight; i++) {
      {
        const vk::DescriptorBufferInfo buffer_info =
            vk::DescriptorBufferInfo().setBuffer(_uniform_buffers[i].get()).setOffset(0).setRange(sizeof(UniformBufferObject));
        std::array descriptor_writes{
            vk::WriteDescriptorSet()
                .setDstSet(_descriptor_sets[i].get())
                .setDstBinding(0)
                .setDstArrayElement(0)
                .setDescriptorCount(1)
                .setDescriptorType(vk::DescriptorType::eUniformBuffer)
                .setBufferInfo(buffer_info),
        };
        _handler->device().updateDescriptorSets(descriptor_writes, {});
      }
      const vk::DescriptorImageInfo image_info = vk::DescriptorImageInfo()
                                                     .setSampler(_handler->sampler())
                                                     .setImageView(_handler->image_view())
                                                     .setImageLayout(vk::ImageLayout::eShaderReadOnlyOptimal);
      std::array descriptor_writes{
          vk::WriteDescriptorSet()
              .setDstSet(_descriptor_sets[i].get())
              .setDstBinding(1)
              .setDstArrayElement(0)
              .setDescriptorCount(1)
              .setDescriptorType(vk::DescriptorType::eCombinedImageSampler)
              .setImageInfo(image_info),
      };

      _handler->device().updateDescriptorSets(descriptor_writes, {});
    }
  }

  void create_command_buffers() {
    const vk::CommandBufferAllocateInfo alloc_info = vk::CommandBufferAllocateInfo()
                                                         .setCommandPool(_handler->command_pool())
                                                         .setLevel(vk::CommandBufferLevel::ePrimary)
                                                         .setCommandBufferCount(static_cast<uint32_t>(_max_frames_in_flight));
    _command_buffers = _handler->device().allocateCommandBuffersUnique(alloc_info);
  }

  void create_sync_objects() {
    _image_available_semaphores.resize(_max_frames_in_flight);
    _render_finished_semaphores.resize(_max_frames_in_flight);
    _in_flight_fences.resize(_max_frames_in_flight);

    for (size_t i = 0; i < _max_frames_in_flight; i++) {
      _image_available_semaphores[i] = _handler->device().createSemaphoreUnique({});
      _render_finished_semaphores[i] = _handler->device().createSemaphoreUnique({});
      _in_flight_fences[i] = _handler->device().createFenceUnique({vk::FenceCreateFlagBits::eSignaled});
    }
  }

  void draw_frame(const gltf::Model& model, const VulkanImGui& imgui) {
    if (_handler->device().waitForFences(_in_flight_fences[_current_frame].get(), true, std::numeric_limits<uint64_t>::max()) != vk::Result::eSuccess)
      throw std::runtime_error("failed to wait fence!");

    uint32_t image_index;
    auto result = _handler->device().acquireNextImageKHR(_swap_chain.get(), std::numeric_limits<uint64_t>::max(),
                                                         _image_available_semaphores[_current_frame].get(), nullptr, &image_index);
    if (result == vk::Result::eErrorOutOfDateKHR) {
      recreate_swap_chain();
      return;
    }
    if (result != vk::Result::eSuccess && result != vk::Result::eSuboptimalKHR) throw std::runtime_error("failed to acquire next image!");

    _handler->device().resetFences(_in_flight_fences[_current_frame].get());

    _command_buffers[_current_frame]->reset(vk::CommandBufferResetFlags{0});
    record_command_buffer(_command_buffers[_current_frame], image_index, model, imgui);

    update_uniform_buffer(_current_frame, imgui);

    vk::PipelineStageFlags wait_stage(vk::PipelineStageFlagBits::eColorAttachmentOutput);
    vk::SubmitInfo submit_info(_image_available_semaphores[_current_frame].get(), wait_stage, _command_buffers[_current_frame].get(),
                               _render_finished_semaphores[_current_frame].get());
    _handler->graphics_queue().submit(submit_info, _in_flight_fences[_current_frame].get());

    const vk::PresentInfoKHR present_info(_render_finished_semaphores[_current_frame].get(), _swap_chain.get(), image_index);
    result = _handler->present_queue().presentKHR(&present_info);
    if (result == vk::Result::eErrorOutOfDateKHR || result == vk::Result::eSuboptimalKHR || _framebuffer_resized) {
      _framebuffer_resized = false;
      recreate_swap_chain();
    } else if (result != vk::Result::eSuccess)
      throw std::runtime_error("failed to wait fence!");

    _current_frame = (_current_frame + 1) % _max_frames_in_flight;
  }

  void recreate_swap_chain() {
    int width = 0, height = 0;
    glfwGetFramebufferSize(_window->window(), &width, &height);
    while (width == 0 || height == 0) {
      glfwGetFramebufferSize(_window->window(), &width, &height);
      glfwWaitEvents();
    }

    _handler->device().waitIdle();

    cleanup();

    create_swapchain();
    create_image_views();
    create_depth_resources();
    create_color_resources();
    create_framebuffers();
  }

  void cleanup() {
    for (auto& framebuffer : _swap_chain_framebuffers) {
      _handler->device().destroyFramebuffer(framebuffer.get(), nullptr);
      framebuffer.get() = nullptr;
    }
    for (auto& image_view : _swap_chain_image_views) {
      _handler->device().destroyImageView(image_view.get(), nullptr);
      image_view.get() = nullptr;
    }
    _handler->device().destroySwapchainKHR(_swap_chain.get(), nullptr);
    _swap_chain.get() = nullptr;
  }

  static void resize_callback(GLFWwindow* window, int, int) {
    static_cast<VulkanRenderer*>(glfwGetWindowUserPointer(window))->_framebuffer_resized = true;
  }

  static void pos_callback(GLFWwindow* window, int, int) {
    auto* renderer = static_cast<VulkanRenderer*>(glfwGetWindowUserPointer(window));
    const auto [fst, snd] = renderer->_window->scale();
    const auto factor = (fst + snd) / 2.0f;
    if (std::abs(renderer->_last_font_factor - factor) < 0.01f) return;
    renderer->_last_font_factor = factor;
    ImGuiIO& io = ImGui::GetIO();
    if (!renderer->_font_path.empty()) {
      renderer->_font = io.Fonts->AddFontFromFileTTF(renderer->_font_path.c_str(), 16.0f * factor);
      io.FontGlobalScale = 1.0f / factor;
    } else {
      renderer->_font = io.Fonts->AddFontDefault();
    }
    {
      renderer->_handler->device().waitIdle();

      // To destroy old texture image and image view, and to free memory
      struct ImGuiImplVulkanHFrameRenderBuffers;
      struct ImGuiImplVulkanHWindowRenderBuffers {
        uint32_t index;
        uint32_t count;
        ImGuiImplVulkanHFrameRenderBuffers* frame_render_buffers;
      };
      struct ImGuiImplVulkanData {
        ImGui_ImplVulkan_InitInfo vulkan_init_info;
        VkRenderPass render_pass;
        VkDeviceSize buffer_memory_alignment;
        VkPipelineCreateFlags pipeline_create_flags;
        VkDescriptorSetLayout descriptor_set_layout;
        VkPipelineLayout pipeline_layout;
        VkPipeline pipeline;
        uint32_t subpass;
        VkShaderModule shader_module_vert;
        VkShaderModule shader_module_frag;
        VkSampler font_sampler;
        VkDeviceMemory font_memory;
        VkImage font_image;
        VkImageView font_view;
        VkDescriptorSet font_descriptor_set;
        VkDeviceMemory upload_buffer_memory;
        VkBuffer upload_buffer;
        ImGuiImplVulkanHWindowRenderBuffers main_window_render_buffers;
      };
      const auto* bd = static_cast<ImGuiImplVulkanData*>(ImGui::GetIO().BackendRendererUserData);
      renderer->_handler->device().destroyImage(bd->font_image);
      renderer->_handler->device().destroyImageView(bd->font_view);
      renderer->_handler->device().freeMemory(bd->font_memory);

      renderer->_handler->device().resetCommandPool(renderer->_handler->command_pool());
      const vk::CommandBufferAllocateInfo alloc_info(renderer->_handler->command_pool(), vk::CommandBufferLevel::ePrimary, 1);
      auto command_buffers = renderer->_handler->device().allocateCommandBuffersUnique(alloc_info);
      vk::UniqueCommandBuffer command_buffer = std::move(command_buffers[0]);
      const vk::CommandBufferBeginInfo begin_info(vk::CommandBufferUsageFlagBits::eOneTimeSubmit);
      command_buffer->begin(begin_info);
      ImGui_ImplVulkan_CreateFontsTexture(command_buffer.get());
      vk::SubmitInfo end_info(0, nullptr, nullptr, 1, &command_buffer.get(), 0, nullptr);
      command_buffer->end();
      renderer->_handler->graphics_queue().submit(end_info);
      renderer->_handler->device().waitIdle();
      ImGui_ImplVulkan_DestroyFontUploadObjects();
    }
  }

  [[nodiscard]] vk::Format image_format() const { return _swap_chain_image_format; }
  [[nodiscard]] vk::Extent2D extent() const { return _swap_chain_extent; }
  [[nodiscard]] vk::RenderPass render_pass() const { return _render_pass.get(); }
  [[nodiscard]] vk::ImageView depth_image_view() const { return _depth_image_view.get(); }
  [[nodiscard]] size_t frames_in_flight() const { return _max_frames_in_flight; }
  [[nodiscard]] vk::Buffer uniform_buffer(const size_t i) const { return _uniform_buffers[i].get(); }
  [[nodiscard]] ImFont* font() const { return _font; }

 private:
  const VulkanHandler* _handler;
  const WindowHandler* _window;

  std::string _shader;
  std::string _font_path;

  vk::UniqueSwapchainKHR _swap_chain;
  std::vector<vk::Image> _swap_chain_images;
  vk::Format _swap_chain_image_format{};
  vk::Extent2D _swap_chain_extent{};

  std::vector<vk::UniqueImageView> _swap_chain_image_views;
  std::vector<vk::UniqueFramebuffer> _swap_chain_framebuffers;

  vk::UniqueRenderPass _render_pass;
  vk::UniqueDescriptorSetLayout _descriptor_set_layout;
  std::vector<vk::UniqueDescriptorSet> _descriptor_sets;
  std::vector<vk::UniquePipeline> _pipelines;
  vk::UniquePipelineLayout _pipeline_layout;

  vk::UniqueImage _depth_image;
  vk::UniqueDeviceMemory _depth_image_memory;
  vk::UniqueImageView _depth_image_view;

  vk::UniqueImage _color_image;
  vk::UniqueDeviceMemory _color_image_memory;
  vk::UniqueImageView _color_image_view;

  vk::UniqueBuffer _vertex_buffer;
  vk::UniqueDeviceMemory _vertex_buffer_memory;
  vk::UniqueBuffer _index_buffer;
  vk::UniqueDeviceMemory _index_buffer_memory;

  std::vector<vk::UniqueBuffer> _uniform_buffers;
  std::vector<vk::UniqueDeviceMemory> _uniform_buffers_memory;

  std::vector<vk::UniqueCommandBuffer> _command_buffers;

  std::vector<vk::UniqueSemaphore> _image_available_semaphores;
  std::vector<vk::UniqueSemaphore> _render_finished_semaphores;
  std::vector<vk::UniqueFence> _in_flight_fences;
  size_t _current_frame = 0;

  bool _vsync;

  ImFont* _font = nullptr;
  float _last_font_factor = 1.0f;

  bool _framebuffer_resized = false;
  const size_t _max_frames_in_flight = 2;

  [[nodiscard]] vk::SurfaceFormatKHR choose_swap_surface_format(const std::vector<vk::SurfaceFormatKHR>& available_formats) const {
    const auto it = std::find_if(available_formats.begin(), available_formats.end(), [](const auto& available_format) {
      return available_format.format == vk::Format::eB8G8R8A8Srgb && available_format.colorSpace == vk::ColorSpaceKHR::eSrgbNonlinear;
    });
    return it != available_formats.end() ? *it : available_formats[0];
  }

  [[nodiscard]] vk::PresentModeKHR choose_swap_present_mode(const std::vector<vk::PresentModeKHR>& available_present_modes) const {
    const auto it = std::find_if(available_present_modes.begin(), available_present_modes.end(),
                                 [](const auto& available_present_mode) { return available_present_mode == vk::PresentModeKHR::eMailbox; });
    return it != available_present_modes.end() ? *it : vk::PresentModeKHR::eFifo;
  }

  [[nodiscard]] vk::Extent2D choose_swap_extent(const vk::SurfaceCapabilitiesKHR& capabilities, GLFWwindow* window) const {
    if (capabilities.currentExtent.width != std::numeric_limits<uint32_t>::max()) return capabilities.currentExtent;

    int width, height;
    glfwGetFramebufferSize(window, &width, &height);

    vk::Extent2D actual_extent = {static_cast<uint32_t>(width), static_cast<uint32_t>(height)};

    actual_extent.width = std::clamp(actual_extent.width, capabilities.minImageExtent.width, capabilities.maxImageExtent.width);
    actual_extent.height = std::clamp(actual_extent.height, capabilities.minImageExtent.height, capabilities.maxImageExtent.height);

    return actual_extent;
  }

  static std::vector<char> read_file(const std::string& filename) {
    std::ifstream file(filename, std::ios::ate | std::ios::binary);
    if (!file.is_open()) throw std::runtime_error("failed to open file!");

    const size_t file_size = file.tellg();
    std::vector<char> buffer(file_size);

    file.seekg(0);
    file.read(buffer.data(), static_cast<std::streamsize>(file_size));

    file.close();

    return buffer;
  }

  static vk::UniqueShaderModule create_shader_module(const vk::Device& device, const std::vector<char>& code) {
    return device.createShaderModuleUnique(
        vk::ShaderModuleCreateInfo().setCodeSize(code.size()).setPCode(reinterpret_cast<const uint32_t*>(code.data())));
  }

  void record_command_buffer(vk::UniqueCommandBuffer& command_buffer, const uint32_t image_index, const gltf::Model& model,
                             const VulkanImGui& imgui) {
    command_buffer->begin(vk::CommandBufferBeginInfo{});

    const vk::ClearValue clear_color(vk::ClearColorValue(imgui.background));
    constexpr vk::ClearValue clear_depth_stencil(vk::ClearDepthStencilValue(1.0f, 0.0f));
    const std::array clear_values = {clear_color, clear_depth_stencil};
    const vk::RenderPassBeginInfo render_pass_info = vk::RenderPassBeginInfo()
                                                         .setRenderPass(_render_pass.get())
                                                         .setFramebuffer(_swap_chain_framebuffers[image_index].get())
                                                         .setRenderArea({{0, 0}, _swap_chain_extent})
                                                         .setClearValues(clear_values);

    command_buffer->beginRenderPass(render_pass_info, vk::SubpassContents::eInline);

    const vk::Viewport viewport = vk::Viewport()
                                      .setX(0.0f)
                                      .setY(0.0f)
                                      .setWidth(static_cast<float>(_swap_chain_extent.width))
                                      .setHeight(static_cast<float>(_swap_chain_extent.height))
                                      .setMinDepth(0.0f)
                                      .setMaxDepth(1.0f);
    command_buffer->setViewport(0, viewport);

    const vk::Rect2D scissor({0, 0}, _swap_chain_extent);
    command_buffer->setScissor(0, scissor);

    const vk::Buffer vertex_buffers[] = {_vertex_buffer.get()};
    constexpr vk::DeviceSize offsets[] = {0};
    command_buffer->bindVertexBuffers(0, 1, vertex_buffers, offsets);
    command_buffer->bindIndexBuffer(_index_buffer.get(), 0, vk::IndexType::eUint32);

    command_buffer->bindDescriptorSets(vk::PipelineBindPoint::eGraphics, _pipeline_layout.get(), 0, 1, &_descriptor_sets[_current_frame].get(), 0,
                                       nullptr);

    for (size_t dev = 0; dev < model.geometries().size(); dev++) {
      if (!imgui.show[dev]) continue;
      const auto [pos, rot] = model.geometries()[dev];
      auto matrix = translate(glm::identity<glm::mat4>(), glm::vec3(pos.x, pos.z, -pos.y) / 1000.0f);
      matrix = matrix * mat4_cast(glm::quat(rot.w, rot.x, rot.z, -rot.y));

      for (const auto& [first_index, index_count, material_index] : model.primitives()) {
        command_buffer->bindPipeline(vk::PipelineBindPoint::eGraphics, _pipelines[material_index].get());
        command_buffer->pushConstants(_pipeline_layout.get(), vk::ShaderStageFlagBits::eVertex, 0, sizeof(glm::mat4), &matrix);
        command_buffer->pushConstants(_pipeline_layout.get(), vk::ShaderStageFlagBits::eFragment, sizeof(glm::mat4), sizeof(gltf::Lighting),
                                      &imgui.lighting);
        command_buffer->drawIndexed(index_count, 1, first_index, 0, 0);
      }
    }

    ImDrawData* draw_data = ImGui::GetDrawData();
    ImGui_ImplVulkan_RenderDrawData(draw_data, command_buffer.get());

    command_buffer->endRenderPass();
    command_buffer->end();
  }

  void update_uniform_buffer(const size_t current_image, const VulkanImGui& imgui) {
    const auto rot = glm::quat(radians(glm::vec3(imgui.camera_rot[0], imgui.camera_rot[2], -imgui.camera_rot[1])));
    const auto model = mat4_cast(rot);
    const auto p = glm::vec3(imgui.camera_pos[0], imgui.camera_pos[2], -imgui.camera_pos[1]) / 1000.0f;
    const auto r = make_vec3(model[0]);
    const auto u = make_vec3(model[1]);
    const auto f = make_vec3(model[2]);
    const auto view = glm::mat4({r[0], u[0], f[0], 0.0f, r[1], u[1], f[1], 0.0f, r[2], u[2], f[2], 0.f, -dot(r, p), -dot(u, p), -dot(f, p), 1.0f});

    UniformBufferObject ubo{
        view,
        glm::perspective(glm::radians(imgui.fov), static_cast<float>(_swap_chain_extent.width) / static_cast<float>(_swap_chain_extent.height), 0.1f,
                         10.0f),
        glm::vec4(glm::vec3(imgui.light_pos[0], imgui.light_pos[2], -imgui.light_pos[1]) / 1000.0f, 1.0f), glm::vec4(p, 1.0f)};
    ubo.proj[1][1] *= -1;

    void* data;
    if (_handler->device().mapMemory(_uniform_buffers_memory[current_image].get(), 0, sizeof ubo, {}, &data) != vk::Result::eSuccess)
      throw std::runtime_error("failed to map uniform buffer memory");

    memcpy(data, &ubo, sizeof ubo);
    _handler->device().unmapMemory(_uniform_buffers_memory[current_image].get());
  }
};

}  // namespace autd3::extra::geometry_viewer
