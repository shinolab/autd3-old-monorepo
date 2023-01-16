// File: vulkan_renderer.hpp
// Project: include
// Created Date: 24/09/2022
// Author: Shun Suzuki
// -----
// Last Modified: 17/01/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <imgui.h>
#include <imgui_impl_vulkan.h>

#include <algorithm>
#include <limits>
#include <string>
#include <utility>
#include <vector>

#include "glm.hpp"
#include "model.hpp"
#include "shader.hpp"
#include "vulkan_context.hpp"
#include "vulkan_handler.hpp"
#include "vulkan_imgui.hpp"

namespace autd3::extra::geometry_viewer {

#ifdef AUTD3_USE_METER
constexpr float GL_SCALE = 1;
#else
constexpr float GL_SCALE = 1e-3f;
#endif

struct UniformBufferObject {
  glm::mat4 view;
  glm::mat4 proj;
  glm::vec4 light_pos;
  glm::vec4 view_pos;
};

class VulkanRenderer {
 public:
  std::string font_path;
  explicit VulkanRenderer(const helper::VulkanContext* context, const helper::WindowHandler* window, const VulkanHandler* handler,
                          const VulkanImGui* imgui, const bool vsync = true) noexcept
      : _context(context),
        _window(window),
        _handler(handler),
        _imgui(imgui),
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

  void create_swapchain() {
    const auto [capabilities, formats, present_modes] = _context->query_swap_chain_support(_context->physical_device());

    const vk::SurfaceFormatKHR surface_format = choose_swap_surface_format(formats);
    const vk::PresentModeKHR present_mode = _vsync ? vk::PresentModeKHR::eFifo : choose_swap_present_mode(present_modes);
    const vk::Extent2D extent = choose_swap_extent(capabilities, _window->window());

    uint32_t image_count = capabilities.minImageCount + 1;
    if (capabilities.maxImageCount > 0 && image_count > capabilities.maxImageCount) image_count = capabilities.maxImageCount;

    vk::SwapchainCreateInfoKHR create_info = vk::SwapchainCreateInfoKHR()
                                                 .setSurface(_context->surface())
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

    if (const auto [graphics_family, present_family] = _context->find_queue_families(_context->physical_device());
        graphics_family != present_family) {
      if (!graphics_family || !present_family) throw std::runtime_error("Failed to find queue family.");

      create_info.setImageSharingMode(vk::SharingMode::eConcurrent);
      std::vector queue_family_indices = {graphics_family.value(), present_family.value()};
      create_info.setQueueFamilyIndices(queue_family_indices);
    }

    _swap_chain = _context->device().createSwapchainKHRUnique(create_info);
    _swap_chain_images = _context->device().getSwapchainImagesKHR(_swap_chain.get());
    _swap_chain_image_format = surface_format.format;
    _swap_chain_extent = extent;
  }

  void create_image_views() {
    _swap_chain_image_views.resize(_swap_chain_images.size());
    std::transform(_swap_chain_images.begin(), _swap_chain_images.end(), _swap_chain_image_views.begin(), [this](const auto& swap_chain_image) {
      return _context->create_image_view(swap_chain_image, _swap_chain_image_format, vk::ImageAspectFlagBits::eColor, 1);
    });
  }

  void create_framebuffers() {
    _swap_chain_framebuffers.resize(_swap_chain_image_views.size());
    std::transform(_swap_chain_image_views.begin(), _swap_chain_image_views.end(), _swap_chain_framebuffers.begin(),
                   [this](const auto& swap_chain_image_view) {
                     std::array attachments{_color_image_view.get(), _depth_image_view.get(), swap_chain_image_view.get()};
                     const vk::FramebufferCreateInfo framebuffer_create_info = vk::FramebufferCreateInfo()
                                                                                   .setRenderPass(_render_pass.get())
                                                                                   .setAttachments(attachments)
                                                                                   .setWidth(_swap_chain_extent.width)
                                                                                   .setHeight(_swap_chain_extent.height)
                                                                                   .setLayers(1);
                     return _context->device().createFramebufferUnique(framebuffer_create_info);
                   });
  }

  void create_render_pass() {
    const auto depth_format = _context->find_depth_format();
    std::vector attachments = {vk::AttachmentDescription()
                                   .setFormat(_swap_chain_image_format)
                                   .setSamples(_context->msaa_samples())
                                   .setLoadOp(vk::AttachmentLoadOp::eClear)
                                   .setStoreOp(vk::AttachmentStoreOp::eStore)
                                   .setStencilLoadOp(vk::AttachmentLoadOp::eDontCare)
                                   .setStencilStoreOp(vk::AttachmentStoreOp::eDontCare)
                                   .setInitialLayout(vk::ImageLayout::eUndefined)
                                   .setFinalLayout(vk::ImageLayout::eColorAttachmentOptimal),
                               vk::AttachmentDescription()
                                   .setFormat(depth_format)
                                   .setSamples(_context->msaa_samples())
                                   .setLoadOp(vk::AttachmentLoadOp::eClear)
                                   .setStoreOp(vk::AttachmentStoreOp::eDontCare)
                                   .setStencilLoadOp(vk::AttachmentLoadOp::eDontCare)
                                   .setStencilStoreOp(vk::AttachmentStoreOp::eDontCare)
                                   .setInitialLayout(vk::ImageLayout::eUndefined)
                                   .setFinalLayout(vk::ImageLayout::eDepthStencilAttachmentOptimal),
                               vk::AttachmentDescription()
                                   .setFormat(_swap_chain_image_format)
                                   .setSamples(vk::SampleCountFlagBits::e1)
                                   .setLoadOp(vk::AttachmentLoadOp::eDontCare)
                                   .setStoreOp(vk::AttachmentStoreOp::eStore)
                                   .setStencilLoadOp(vk::AttachmentLoadOp::eDontCare)
                                   .setStencilStoreOp(vk::AttachmentStoreOp::eDontCare)
                                   .setInitialLayout(vk::ImageLayout::eUndefined)
                                   .setFinalLayout(vk::ImageLayout::ePresentSrcKHR)};
    const std::array color_attachments = {vk::AttachmentReference().setAttachment(0).setLayout(vk::ImageLayout::eColorAttachmentOptimal)};
    const vk::AttachmentReference depth_attachment =
        vk::AttachmentReference().setAttachment(1).setLayout(vk::ImageLayout::eDepthStencilAttachmentOptimal);
    const std::array resolve_attachments = {vk::AttachmentReference().setAttachment(2).setLayout(vk::ImageLayout::eColorAttachmentOptimal)};
    const vk::SubpassDescription subpass = vk::SubpassDescription()
                                               .setPipelineBindPoint(vk::PipelineBindPoint::eGraphics)
                                               .setColorAttachments(color_attachments)
                                               .setPDepthStencilAttachment(&depth_attachment)
                                               .setResolveAttachments(resolve_attachments);
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
    _render_pass = _context->device().createRenderPassUnique(render_pass_info);
  }

  void create_graphics_pipeline(const gltf::Model& model) {
    const std::vector<uint8_t> vert_shader_code = {
#include "shaders/vert.spv.txt"
    };
    const std::vector<uint8_t> frag_shader_code = {
#include "shaders/frag.spv.txt"
    };

    vk::UniqueShaderModule vert_shader_module = helper::create_shader_module(_context->device(), vert_shader_code);
    vk::UniqueShaderModule frag_shader_module = helper::create_shader_module(_context->device(), frag_shader_code);

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
        vk::PipelineMultisampleStateCreateInfo().setRasterizationSamples(_context->msaa_samples()).setSampleShadingEnable(false);

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
        _context->device().createDescriptorSetLayoutUnique(vk::DescriptorSetLayoutCreateInfo().setBindings(bindings_0));

    const std::array push_constant_ranges = {
        vk::PushConstantRange().setStageFlags(vk::ShaderStageFlagBits::eVertex).setSize(sizeof(glm::mat4)),
        vk::PushConstantRange().setStageFlags(vk::ShaderStageFlagBits::eFragment).setSize(sizeof(gltf::Lighting)).setOffset(sizeof(glm::mat4))};
    const std::array layouts = {descriptor_set_layout_0.get()};
    const vk::PipelineLayoutCreateInfo pipeline_layout_info =
        vk::PipelineLayoutCreateInfo().setSetLayouts(layouts).setPushConstantRanges(push_constant_ranges);

    _pipeline_layout = _context->device().createPipelineLayoutUnique(pipeline_layout_info);

    const vk::PipelineDepthStencilStateCreateInfo depth_stencil = vk::PipelineDepthStencilStateCreateInfo()
                                                                      .setDepthTestEnable(true)
                                                                      .setDepthWriteEnable(true)
                                                                      .setDepthCompareOp(vk::CompareOp::eLess)
                                                                      .setDepthBoundsTestEnable(false)
                                                                      .setStencilTestEnable(false)
                                                                      .setMinDepthBounds(0.0f)
                                                                      .setMaxDepthBounds(1.0f);
    _pipelines.resize(model.materials().size());
    size_t i = 0;
    for (const auto& [base_color_factor, base_color_texture_idx, metallic_factor, roughness_factor] : model.materials()) {
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
                                                                                .setLayout(_pipeline_layout.get())
                                                                                .setRenderPass(_render_pass.get()));
          result.result == vk::Result::eSuccess)
        _pipelines[i++] = std::move(result.value);

      else
        throw std::runtime_error("Failed to create a pipeline!");
    }
  }

  void create_depth_resources() {
    const auto depth_format = _context->find_depth_format();
    auto [depth_image, depth_image_memory] =
        _context->create_image(_swap_chain_extent.width, _swap_chain_extent.height, 1, _context->msaa_samples(), depth_format,
                               vk::ImageTiling::eOptimal, vk::ImageUsageFlagBits::eDepthStencilAttachment, vk::MemoryPropertyFlagBits::eDeviceLocal);
    _depth_image = std::move(depth_image);
    _depth_image_memory = std::move(depth_image_memory);
    _depth_image_view = _context->create_image_view(_depth_image.get(), depth_format, vk::ImageAspectFlagBits::eDepth, 1);

    return _context->transition_image_layout(_depth_image, depth_format, vk::ImageLayout::eUndefined, vk::ImageLayout::eDepthStencilAttachmentOptimal,
                                             1);
  }

  void create_color_resources() {
    const auto color_format = _swap_chain_image_format;

    auto [color_image, color_image_memory] = _context->create_image(
        _swap_chain_extent.width, _swap_chain_extent.height, 1, _context->msaa_samples(), color_format, vk::ImageTiling::eOptimal,
        vk::ImageUsageFlagBits::eTransientAttachment | vk::ImageUsageFlagBits::eColorAttachment, vk::MemoryPropertyFlagBits::eDeviceLocal);
    _color_image = std::move(color_image);
    _color_image_memory = std::move(color_image_memory);

    _color_image_view = _context->create_image_view(_color_image.get(), color_format, vk::ImageAspectFlagBits::eColor, 1);
  }

  void create_vertex_buffer(const gltf::Model& model) {
    const auto& vertices = model.vertices();
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

  void create_index_buffer(const gltf::Model& model) {
    const auto& indices = model.indices();
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

  void create_uniform_buffers() {
    _uniform_buffers.resize(_max_frames_in_flight);
    _uniform_buffers_memory.resize(_max_frames_in_flight);
    for (size_t i = 0; i < _max_frames_in_flight; i++) {
      auto [buf, mem] = _context->create_buffer(sizeof(UniformBufferObject), vk::BufferUsageFlagBits::eUniformBuffer,
                                                vk::MemoryPropertyFlagBits::eHostVisible | vk::MemoryPropertyFlagBits::eHostCoherent);

      _uniform_buffers[i] = std::move(buf);
      _uniform_buffers_memory[i] = std::move(mem);
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
                               .setDescriptorType(vk::DescriptorType::eCombinedImageSampler)
                               .setDescriptorCount(1)
                               .setStageFlags(vk::ShaderStageFlagBits::eFragment)};
    _descriptor_set_layout = _context->device().createDescriptorSetLayoutUnique(vk::DescriptorSetLayoutCreateInfo().setBindings(bindings));
    std::vector layouts(_max_frames_in_flight, _descriptor_set_layout.get());
    _descriptor_sets = _context->device().allocateDescriptorSetsUnique(
        vk::DescriptorSetAllocateInfo().setDescriptorPool(_context->descriptor_pool()).setSetLayouts(layouts));
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
        _context->device().updateDescriptorSets(descriptor_writes, {});
      }
      {
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
        _context->device().updateDescriptorSets(descriptor_writes, {});
      }
    }
  }

  void create_command_buffers() {
    const vk::CommandBufferAllocateInfo alloc_info = vk::CommandBufferAllocateInfo()
                                                         .setCommandPool(_context->command_pool())
                                                         .setLevel(vk::CommandBufferLevel::ePrimary)
                                                         .setCommandBufferCount(static_cast<uint32_t>(_max_frames_in_flight));
    _command_buffers = _context->device().allocateCommandBuffersUnique(alloc_info);
  }

  void create_sync_objects() {
    _image_available_semaphores.resize(_max_frames_in_flight);
    _render_finished_semaphores.resize(_max_frames_in_flight);
    _in_flight_fences.resize(_max_frames_in_flight);

    for (size_t i = 0; i < _max_frames_in_flight; i++) {
      _image_available_semaphores[i] = _context->device().createSemaphoreUnique({});
      _render_finished_semaphores[i] = _context->device().createSemaphoreUnique({});
      _in_flight_fences[i] = _context->device().createFenceUnique({vk::FenceCreateFlagBits::eSignaled});
    }
  }

  void draw_frame(const gltf::Model& model, const VulkanImGui& imgui) {
    if (_context->device().waitForFences(_in_flight_fences[_current_frame].get(), true, std::numeric_limits<uint64_t>::max()) != vk::Result::eSuccess)
      throw std::runtime_error("Failed to wait fence!");

    uint32_t image_index;
    auto result = _context->device().acquireNextImageKHR(_swap_chain.get(), std::numeric_limits<uint64_t>::max(),
                                                         _image_available_semaphores[_current_frame].get(), nullptr, &image_index);
    if (result == vk::Result::eErrorOutOfDateKHR) return recreate_swap_chain();

    if (result != vk::Result::eSuccess && result != vk::Result::eSuboptimalKHR) throw std::runtime_error("Failed to acquire next image!");

    _context->device().resetFences(_in_flight_fences[_current_frame].get());

    _command_buffers[_current_frame]->reset(vk::CommandBufferResetFlags{0});
    record_command_buffer(_command_buffers[_current_frame], image_index, model, imgui);

    update_uniform_buffer(_current_frame, imgui);

    vk::PipelineStageFlags wait_stage(vk::PipelineStageFlagBits::eColorAttachmentOutput);
    const vk::SubmitInfo submit_info(_image_available_semaphores[_current_frame].get(), wait_stage, _command_buffers[_current_frame].get(),
                                     _render_finished_semaphores[_current_frame].get());
    _context->graphics_queue().submit(submit_info, _in_flight_fences[_current_frame].get());

    const vk::PresentInfoKHR present_info(_render_finished_semaphores[_current_frame].get(), _swap_chain.get(), image_index);
    result = _context->present_queue().presentKHR(&present_info);
    if (result == vk::Result::eErrorOutOfDateKHR || result == vk::Result::eSuboptimalKHR || _framebuffer_resized) {
      _framebuffer_resized = false;
      return recreate_swap_chain();
    }
    if (result != vk::Result::eSuccess) throw std::runtime_error("Failed to wait fence!");
    _current_frame = (_current_frame + 1) % _max_frames_in_flight;
  }

  void recreate_swap_chain() {
    int width = 0, height = 0;
    glfwGetFramebufferSize(_window->window(), &width, &height);
    while (width == 0 || height == 0) {
      glfwGetFramebufferSize(_window->window(), &width, &height);
      glfwWaitEvents();
    }

    _context->device().waitIdle();

    cleanup();

    create_swapchain();
    create_image_views();
    create_depth_resources();
    create_color_resources();
    create_framebuffers();
  }

  void cleanup() {
    std::for_each(_swap_chain_framebuffers.begin(), _swap_chain_framebuffers.end(), [this](auto& framebuffer) {
      _context->device().destroyFramebuffer(framebuffer.get(), nullptr);
      framebuffer.get() = nullptr;
    });
    std::for_each(_swap_chain_image_views.begin(), _swap_chain_image_views.end(), [this](auto& image_view) {
      _context->device().destroyImageView(image_view.get(), nullptr);
      image_view.get() = nullptr;
    });
    _context->device().destroySwapchainKHR(_swap_chain.get(), nullptr);
    _swap_chain.get() = nullptr;
  }

  static void resize_callback(GLFWwindow* window, int, int) {
    static_cast<VulkanRenderer*>(glfwGetWindowUserPointer(window))->_framebuffer_resized = true;
  }

  static void pos_callback(GLFWwindow* window, int, int) { static_cast<VulkanRenderer*>(glfwGetWindowUserPointer(window))->_imgui->set_font(); }

  [[nodiscard]] vk::Format image_format() const { return _swap_chain_image_format; }
  [[nodiscard]] vk::Extent2D extent() const { return _swap_chain_extent; }
  [[nodiscard]] vk::RenderPass render_pass() const { return _render_pass.get(); }
  [[nodiscard]] vk::ImageView depth_image_view() const { return _depth_image_view.get(); }
  [[nodiscard]] size_t frames_in_flight() const { return _max_frames_in_flight; }
  [[nodiscard]] vk::Buffer uniform_buffer(const size_t i) const { return _uniform_buffers[i].get(); }

 private:
  [[nodiscard]] static vk::SurfaceFormatKHR choose_swap_surface_format(const std::vector<vk::SurfaceFormatKHR>& available_formats) {
    const auto it = std::find_if(available_formats.begin(), available_formats.end(), [](const auto& available_format) {
      return available_format.format == vk::Format::eB8G8R8A8Srgb && available_format.colorSpace == vk::ColorSpaceKHR::eSrgbNonlinear;
    });
    return it != available_formats.end() ? *it : available_formats[0];
  }

  [[nodiscard]] static vk::PresentModeKHR choose_swap_present_mode(const std::vector<vk::PresentModeKHR>& available_present_modes) {
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

  void record_command_buffer(vk::UniqueCommandBuffer& command_buffer, const uint32_t image_index, const gltf::Model& model,
                             const VulkanImGui& imgui) {
    command_buffer->begin(vk::CommandBufferBeginInfo{});

    const std::array clear_value{imgui.background.r, imgui.background.g, imgui.background.b, imgui.background.a};
    const vk::ClearValue clear_color{vk::ClearColorValue(clear_value)};
    constexpr vk::ClearValue clear_depth_stencil{vk::ClearDepthStencilValue(1.0f, 0.0f)};
    const std::array clear_values{clear_color, clear_depth_stencil};
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

    size_t dev = 0;
    for (const auto& [pos, rot] : model.geometries()) {
      if (!imgui.show[dev++]) continue;
      auto matrix = translate(glm::identity<glm::mat4>(), helper::to_gl_pos(pos) * GL_SCALE);
      matrix = matrix * mat4_cast(helper::to_gl_rot(rot));

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
    const auto rot = helper::to_gl_rot(glm::quat(radians(imgui.camera_rot)));
    const auto p = helper::to_gl_pos(imgui.camera_pos) * GL_SCALE;
    const auto view = helper::orthogonal(p, rot);
    UniformBufferObject ubo{
        view,
        glm::perspective(glm::radians(imgui.fov), static_cast<float>(_swap_chain_extent.width) / static_cast<float>(_swap_chain_extent.height), 0.1f,
                         10.0f),
        glm::vec4(helper::to_gl_pos(imgui.light_pos) * GL_SCALE, 1.0f), glm::vec4(p, 1.0f)};
    ubo.proj[1][1] *= -1;

    void* data;
    if (_context->device().mapMemory(_uniform_buffers_memory[current_image].get(), 0, sizeof ubo, {}, &data) != vk::Result::eSuccess)
      throw std::runtime_error("Failed to map uniform buffer memory");

    memcpy(data, &ubo, sizeof ubo);
    _context->device().unmapMemory(_uniform_buffers_memory[current_image].get());
  }

  const helper::VulkanContext* _context;
  const helper::WindowHandler* _window;
  const VulkanHandler* _handler;
  const VulkanImGui* _imgui;

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

  bool _framebuffer_resized = false;
  const size_t _max_frames_in_flight = 2;
};

}  // namespace autd3::extra::geometry_viewer
