// File: vulkan_renderer.hpp
// Project: simulator
// Created Date: 03/10/2022
// Author: Shun Suzuki
// -----
// Last Modified: 06/10/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <limits>
#include <string>
#include <utility>
#include <vector>

#include "vulkan_context.hpp"
#include "vulkan_imgui.hpp"

namespace autd3::extra::simulator {

class VulkanRenderer {
 public:
  std::string font_path;
  explicit VulkanRenderer(const helper::VulkanContext* context, const helper::WindowHandler* window, const VulkanImGui* imgui,
                          const bool vsync = true) noexcept
      : _context(context),
        _window(window),
        _imgui(imgui),
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
    for (size_t i = 0; i < _swap_chain_image_views.size(); i++)
      _swap_chain_image_views[i] = _context->create_image_view(_swap_chain_images[i], _swap_chain_image_format, vk::ImageAspectFlagBits::eColor, 1);
  }

  void create_framebuffers() {
    _swap_chain_framebuffers.resize(_swap_chain_image_views.size());
    for (size_t i = 0; i < _swap_chain_image_views.size(); i++) {
      std::array attachments{_color_image_view.get(), _depth_image_view.get(), _swap_chain_image_views[i].get()};
      vk::FramebufferCreateInfo framebuffer_create_info = vk::FramebufferCreateInfo()
                                                              .setRenderPass(_render_pass.get())
                                                              .setAttachments(attachments)
                                                              .setWidth(_swap_chain_extent.width)
                                                              .setHeight(_swap_chain_extent.height)
                                                              .setLayers(1);
      _swap_chain_framebuffers[i] = _context->device().createFramebufferUnique(framebuffer_create_info);
    }
  }

  void create_render_pass() {
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
                                   .setFormat(_context->find_depth_format())
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

  void create_depth_resources() {
    const auto depth_format = _context->find_depth_format();
    auto [depth_image, depth_image_memory] =
        _context->create_image(_swap_chain_extent.width, _swap_chain_extent.height, 1, _context->msaa_samples(), depth_format,
                               vk::ImageTiling::eOptimal, vk::ImageUsageFlagBits::eDepthStencilAttachment, vk::MemoryPropertyFlagBits::eDeviceLocal);
    _depth_image = std::move(depth_image);
    _depth_image_memory = std::move(depth_image_memory);
    _depth_image_view = _context->create_image_view(_depth_image.get(), depth_format, vk::ImageAspectFlagBits::eDepth, 1);

    _context->transition_image_layout(_depth_image, depth_format, vk::ImageLayout::eUndefined, vk::ImageLayout::eDepthStencilAttachmentOptimal, 1);
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

  std::pair<vk::CommandBuffer, uint32_t> begin_frame(const std::array<float, 4>& background) {
    if (_context->device().waitForFences(_in_flight_fences[_current_frame].get(), true, std::numeric_limits<uint64_t>::max()) != vk::Result::eSuccess)
      throw std::runtime_error("failed to wait fence!");

    uint32_t image_index;
    const auto result = _context->device().acquireNextImageKHR(_swap_chain.get(), std::numeric_limits<uint64_t>::max(),
                                                               _image_available_semaphores[_current_frame].get(), nullptr, &image_index);
    if (result == vk::Result::eErrorOutOfDateKHR) {
      recreate_swap_chain();
      return std::make_pair(nullptr, 0);
    }
    if (result != vk::Result::eSuccess && result != vk::Result::eSuboptimalKHR) throw std::runtime_error("failed to acquire next image!");

    _context->device().resetFences(_in_flight_fences[_current_frame].get());

    _command_buffers[_current_frame]->reset(vk::CommandBufferResetFlags{0});
    record_command_buffer(_command_buffers[_current_frame], image_index, background);

    return std::make_pair(_command_buffers[_current_frame].get(), image_index);
  }

  void end_frame(const vk::CommandBuffer& command_buffer, uint32_t image_index) {
    command_buffer.endRenderPass();
    command_buffer.end();

    vk::PipelineStageFlags wait_stage(vk::PipelineStageFlagBits::eColorAttachmentOutput);
    vk::SubmitInfo submit_info(_image_available_semaphores[_current_frame].get(), wait_stage, _command_buffers[_current_frame].get(),
                               _render_finished_semaphores[_current_frame].get());
    _context->graphics_queue().submit(submit_info, _in_flight_fences[_current_frame].get());

    const vk::PresentInfoKHR present_info(_render_finished_semaphores[_current_frame].get(), _swap_chain.get(), image_index);
    if (const auto result = _context->present_queue().presentKHR(&present_info);
        result == vk::Result::eErrorOutOfDateKHR || result == vk::Result::eSuboptimalKHR || _framebuffer_resized) {
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

    _context->device().waitIdle();

    cleanup();

    create_swapchain();
    create_image_views();
    create_depth_resources();
    create_color_resources();
    create_framebuffers();
  }

  void cleanup() {
    for (auto& framebuffer : _swap_chain_framebuffers) {
      _context->device().destroyFramebuffer(framebuffer.get(), nullptr);
      framebuffer.get() = nullptr;
    }
    for (auto& image_view : _swap_chain_image_views) {
      _context->device().destroyImageView(image_view.get(), nullptr);
      image_view.get() = nullptr;
    }
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
  [[nodiscard]] size_t current_frame() const { return _current_frame; }

 private:
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

  void record_command_buffer(vk::UniqueCommandBuffer& command_buffer, const uint32_t image_index, const std::array<float, 4>& background) {
    command_buffer->begin(vk::CommandBufferBeginInfo{});

    const vk::ClearColorValue clear_color_value = background;
    const vk::ClearValue clear_color(clear_color_value);
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
  }

  const helper::VulkanContext* _context{nullptr};
  const helper::WindowHandler* _window{nullptr};
  const VulkanImGui* _imgui{nullptr};

  vk::UniqueSwapchainKHR _swap_chain{nullptr};
  std::vector<vk::Image> _swap_chain_images{nullptr};
  vk::Format _swap_chain_image_format{};
  vk::Extent2D _swap_chain_extent{};

  std::vector<vk::UniqueImageView> _swap_chain_image_views{};
  std::vector<vk::UniqueFramebuffer> _swap_chain_framebuffers{};

  vk::UniqueRenderPass _render_pass{nullptr};

  vk::UniqueImage _depth_image{nullptr};
  vk::UniqueDeviceMemory _depth_image_memory{nullptr};
  vk::UniqueImageView _depth_image_view{nullptr};

  vk::UniqueImage _color_image{nullptr};
  vk::UniqueDeviceMemory _color_image_memory{nullptr};
  vk::UniqueImageView _color_image_view{nullptr};

  std::vector<vk::UniqueCommandBuffer> _command_buffers{};

  std::vector<vk::UniqueSemaphore> _image_available_semaphores{};
  std::vector<vk::UniqueSemaphore> _render_finished_semaphores{};
  std::vector<vk::UniqueFence> _in_flight_fences{};
  size_t _current_frame{0};

  bool _vsync{true};

  bool _framebuffer_resized{false};
  const size_t _max_frames_in_flight{2};
};

}  // namespace autd3::extra::simulator
