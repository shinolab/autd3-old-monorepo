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

#include "autd3/driver/hardware.hpp"
#include "sound_sources.hpp"
#include "trans_viewer.hpp"
#include "vulkan_renderer.hpp"

namespace autd3::extra::simulator {
void Simulator::start(bool* run) const {
  SoundSources sources;
  for (size_t y = 0; y < driver::NUM_TRANS_Y; y++) {
    for (size_t x = 0; x < driver::NUM_TRANS_X; x++) {
      if (driver::is_missing_transducer(x, y)) continue;
      const auto px = static_cast<float>(x) * static_cast<float>(driver::TRANS_SPACING_MM);
      const auto py = static_cast<float>(y) * static_cast<float>(driver::TRANS_SPACING_MM);
      const auto pz = 0.0f;
      sources.add(glm::vec3(px, py, pz), glm::identity<glm::quat>(), Drive{1.0f, 0.0f, 1.0f, 40e3, 340e3}, 1.0f);
    }
  }

  helper::WindowHandler window(_width, _height);
  helper::VulkanContext context(_gpu_idx, true);

  VulkanImGui imgui(&window, &context);
  VulkanRenderer renderer(&context, &window, &imgui, _vsync);

  trans_viewer::TransViewer trans_viewer(&context, &renderer, _shader, _texture);

  window.init("AUTD3 Simulator", nullptr, nullptr, nullptr);
  context.init_vulkan("AUTD3 Simulator", window);
  renderer.create_swapchain();
  renderer.create_image_views();
  renderer.create_render_pass();

  trans_viewer.create_pipeline();

  context.create_command_pool();
  renderer.create_depth_resources();
  renderer.create_color_resources();
  renderer.create_framebuffers();

  trans_viewer.create_texture();

  trans_viewer.create_vertex_buffer();
  trans_viewer.create_model_instance_buffer(sources);
  trans_viewer.create_color_instance_buffer(sources);
  trans_viewer.create_index_buffer();
  trans_viewer.create_uniform_buffers();

  const std::array pool_size = {
      vk::DescriptorPoolSize(vk::DescriptorType::eCombinedImageSampler, 100),
      vk::DescriptorPoolSize(vk::DescriptorType::eSampledImage, 100),
      vk::DescriptorPoolSize(vk::DescriptorType::eUniformBuffer, 100),
  };
  context.create_descriptor_pool(pool_size);

  trans_viewer.create_descriptor_sets();

  renderer.create_command_buffers();
  renderer.create_sync_objects();

  imgui.init(static_cast<uint32_t>(renderer.frames_in_flight()), renderer.render_pass(), _font, sources);

  *run = true;
  while (run && !window.should_close()) {
    helper::WindowHandler::poll_events();
    glfwPollEvents();
    imgui.draw();
    const std::array background = {imgui.background.r, imgui.background.g, imgui.background.b, imgui.background.a};
    const auto& [command_buffer, image_index] = renderer.begin_frame(background);
    trans_viewer.render(command_buffer);
    VulkanImGui::render(command_buffer);
    renderer.end_frame(command_buffer, image_index);

    const auto& [view, proj] = imgui.get_view_proj(static_cast<float>(renderer.extent().width) / static_cast<float>(renderer.extent().height));
    trans_viewer.update_uniform_objects(view, proj);
  }

  context.device().waitIdle();
  VulkanImGui::cleanup();
  renderer.cleanup();
}

}  // namespace autd3::extra::simulator
