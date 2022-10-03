// File: simulator.cpp
// Project: simulator
// Created Date: 30/09/2022
// Author: Shun Suzuki
// -----
// Last Modified: 03/10/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#include "autd3/extra/simulator/simulator.hpp"

#include <vulkan_context.hpp>
#include <window_handler.hpp>

#include "sound_sources.hpp"
#include "vulkan_renderer.hpp"

namespace autd3::extra::simulator {
Simulator Simulator::start() {
  _is_running = true;

  _th = std::thread([this]() {
    helper::WindowHandler window(_width, _height);
    helper::VulkanContext context(_gpu_idx, true);

    VulkanImGui imgui(&window, &context);
    VulkanRenderer renderer(&context, &window, &imgui, _vsync);

    window.init("AUTD3 Simulator", nullptr, nullptr, nullptr);
    context.init_vulkan("AUTD3 Simulator", window);
    renderer.create_swapchain();
    renderer.create_image_views();
    renderer.create_render_pass();
    // create pipeline
    context.create_command_pool();
    renderer.create_depth_resources();
    renderer.create_color_resources();
    renderer.create_framebuffers();
    // texture
    // vertex, index, uniform
    const std::array pool_size = {
        vk::DescriptorPoolSize(vk::DescriptorType::eCombinedImageSampler, 100),
        vk::DescriptorPoolSize(vk::DescriptorType::eSampledImage, 100),
        vk::DescriptorPoolSize(vk::DescriptorType::eUniformBuffer, 100),
    };
    context.create_descriptor_pool(pool_size);
    // descriptor sets
    renderer.create_command_buffers();
    renderer.create_sync_objects();

    imgui.init(static_cast<uint32_t>(renderer.frames_in_flight()), renderer.render_pass(), _font);

    while (_is_running && !window.should_close()) {
      helper::WindowHandler::poll_events();
      glfwPollEvents();
      imgui.draw();
      const std::array background = {imgui.background.r, imgui.background.g, imgui.background.b, imgui.background.a};
      const auto& [command_buffer, image_index] = renderer.begin_frame(background);
      VulkanImGui::render(command_buffer);
      renderer.end_frame(command_buffer, image_index);
    }

    context.device().waitIdle();
    VulkanImGui::cleanup();
    renderer.cleanup();
  });

  return std::move(*this);
}

void Simulator::exit() {
  _is_running = false;
  if (_th.joinable()) _th.join();
}

}  // namespace autd3::extra::simulator
