// File: geometry_viewer.cpp
// Project: geometry_viewer
// Created Date: 28/09/2022
// Author: Shun Suzuki
// -----
// Last Modified: 28/09/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#include "autd3/extra/geometry_viewer/geometry_viewer.hpp"

#include "model.hpp"
#include "vulkan_handler.hpp"
#include "vulkan_imgui.hpp"
#include "vulkan_renderer.hpp"
#include "window_handler.hpp"

namespace autd3::extra::geometry_viewer {
void GeometryViewer::view(const core::Geometry& geometry) const {
  std::vector<gltf::Geometry> geometries;
  geometries.reserve(geometry.num_devices());
  for (const auto& g : geometry) {
    const auto pos = glm::vec3(g.origin().x(), g.origin().y(), g.origin().z());
    const auto rot = glm::quat(g.rotation().w(), g.rotation().x(), g.rotation().y(), g.rotation().z());
    geometries.emplace_back(gltf::Geometry{pos, rot});
  }

  WindowHandler window(_width, _height);
  VulkanHandler handle;
  VulkanRenderer renderer(&handle, &window, _vsync);
  gltf::Model model("models/AUTD.glb", geometries);
  VulkanImGui imgui{};

  window.init(&renderer, VulkanRenderer::resize_callback, VulkanRenderer::pos_callback);
  handle.init_vulkan(window);
  renderer.init();
  renderer.create_swapchain();
  renderer.create_image_views();
  renderer.create_render_pass();
  renderer.create_graphics_pipeline(model);
  handle.create_command_pool();
  renderer.create_depth_resources();
  renderer.create_color_resources();
  renderer.create_framebuffers();
  handle.create_texture_image(model.image_data(), model.image_size());
  handle.create_texture_image_view();
  handle.create_texture_sampler();
  renderer.create_vertex_buffer(model);
  renderer.create_index_buffer(model);
  renderer.create_uniform_buffers();
  handle.create_descriptor_pool();
  renderer.create_descriptor_sets(model);
  renderer.create_command_buffers();
  renderer.create_sync_objects();

  imgui.init(window, handle, static_cast<uint32_t>(renderer.frames_in_flight()), renderer.render_pass(), geometries);

  while (!window.should_close()) {
    WindowHandler::poll_events();
    glfwPollEvents();
    imgui.draw(renderer.font());
    renderer.draw_frame(model, imgui);
  }

  handle.device().waitIdle();
  VulkanImGui::cleanup();
  renderer.cleanup();
}

}  // namespace autd3::extra::geometry_viewer
