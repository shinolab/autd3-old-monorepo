// File: geometry_viewer.cpp
// Project: geometry_viewer
// Created Date: 28/09/2022
// Author: Shun Suzuki
// -----
// Last Modified: 03/10/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#include "autd3/extra/geometry_viewer/geometry_viewer.hpp"

#include "model.hpp"
#include "vulkan_context.hpp"
#include "vulkan_handler.hpp"
#include "vulkan_imgui.hpp"
#include "vulkan_renderer.hpp"
#include "window_handler.hpp"

namespace autd3::extra::geometry_viewer {
void GeometryViewer::view(const core::Geometry& geometry) const {
  std::vector<gltf::Geometry> geometries;
  geometries.reserve(geometry.num_devices());
  for (const auto& g : geometry) {
    const auto pos = glm::vec3(static_cast<float>(g.origin().x()), static_cast<float>(g.origin().y()), static_cast<float>(g.origin().z()));
    const auto rot = glm::quat(static_cast<float>(g.rotation().w()), static_cast<float>(g.rotation().x()), static_cast<float>(g.rotation().y()),
                               static_cast<float>(g.rotation().z()));
    geometries.emplace_back(gltf::Geometry{pos, rot});
  }

  helper::WindowHandler window(_width, _height);
  helper::VulkanContext context(_gpu_idx, true);
  VulkanHandler handle(&context);
  VulkanImGui imgui(&window, &context);
  VulkanRenderer renderer(&context, &window, &handle, &imgui, _shader, _vsync);
  const gltf::Model model(_model, geometries);

  window.init("Geometry Viewer", &renderer, VulkanRenderer::resize_callback, VulkanRenderer::pos_callback);
  context.init_vulkan("Geometry Viewer", window);
  renderer.create_swapchain();
  renderer.create_image_views();
  renderer.create_render_pass();
  renderer.create_graphics_pipeline(model);
  context.create_command_pool();
  renderer.create_depth_resources();
  renderer.create_color_resources();
  renderer.create_framebuffers();
  handle.create_texture_image(model.image_data(), model.image_size());
  handle.create_texture_image_view();
  handle.create_texture_sampler();
  renderer.create_vertex_buffer(model);
  renderer.create_index_buffer(model);
  renderer.create_uniform_buffers();
  const std::array pool_size = {
      vk::DescriptorPoolSize(vk::DescriptorType::eCombinedImageSampler, 100),
      vk::DescriptorPoolSize(vk::DescriptorType::eSampledImage, 100),
      vk::DescriptorPoolSize(vk::DescriptorType::eUniformBuffer, 100),
  };
  context.create_descriptor_pool(pool_size);
  renderer.create_descriptor_sets();
  renderer.create_command_buffers();
  renderer.create_sync_objects();

  imgui.init(static_cast<uint32_t>(renderer.frames_in_flight()), renderer.render_pass(), geometries, _font);

  while (!window.should_close()) {
    helper::WindowHandler::poll_events();
    glfwPollEvents();
    imgui.draw();
    renderer.draw_frame(model, imgui);
  }

  context.device().waitIdle();
  VulkanImGui::cleanup();
  renderer.cleanup();
}

}  // namespace autd3::extra::geometry_viewer
