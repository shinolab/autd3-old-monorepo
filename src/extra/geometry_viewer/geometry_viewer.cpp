// File: geometry_viewer.cpp
// Project: geometry_viewer
// Created Date: 28/09/2022
// Author: Shun Suzuki
// -----
// Last Modified: 26/11/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#include "autd3/extra/geometry_viewer.hpp"

#include <fstream>

#include "model.hpp"
#include "models/autd3_model.hpp"
#include "vulkan_context.hpp"
#include "vulkan_handler.hpp"
#include "vulkan_imgui.hpp"
#include "vulkan_renderer.hpp"
#include "window_handler.hpp"

namespace {
[[nodiscard]] bool write_model() {
  if (std::filesystem::exists("models/AUTD3.glb")) return true;
  std::filesystem::create_directory("models");
  std::ofstream fs;
  fs.open("models/AUTD3.glb", std::ios::out | std::ios::binary | std::ios::trunc);
  if (!fs) {
    spdlog::error("Cannot write AUTD3 model.");
    return false;
  }
  for (size_t i = 0; i < model_size; i++) fs.write(reinterpret_cast<const char*>(&model_data[i]), sizeof(char));
  fs.close();
  return true;
}
}  // namespace

namespace autd3::extra {
[[nodiscard]] bool GeometryViewer::view(const core::Geometry& geometry) const {
  std::vector<geometry_viewer::gltf::Geometry> geometries;
  geometries.reserve(geometry.num_devices());

  for (const auto& tr_num : geometry.device_map())
    if (tr_num != AUTD3::NUM_TRANS_IN_UNIT) {
      spdlog::error("This is not AUTD3 device.");
      return false;
    }

  for (size_t dev = 0; dev < geometry.num_devices(); dev++) {
    const auto& tr = geometry[dev * AUTD3::NUM_TRANS_IN_UNIT];
    const auto pos = glm::vec3(static_cast<float>(tr.position().x()), static_cast<float>(tr.position().y()), static_cast<float>(tr.position().z()));
    const auto rot = glm::quat(static_cast<float>(tr.rotation().w()), static_cast<float>(tr.rotation().x()), static_cast<float>(tr.rotation().y()),
                               static_cast<float>(tr.rotation().z()));
    geometries.emplace_back(geometry_viewer::gltf::Geometry{pos, rot});
  }

  helper::WindowHandler window(_width, _height);
  helper::VulkanContext context(_gpu_idx, false);
  geometry_viewer::VulkanHandler handle(&context);
  geometry_viewer::VulkanImGui imgui(&window, &context);
  geometry_viewer::VulkanRenderer renderer(&context, &window, &handle, &imgui, _vsync);

  if (!write_model()) return false;
  const geometry_viewer::gltf::Model model("models/AUTD3.glb", geometries);

  window.init("Geometry Viewer", &renderer, geometry_viewer::VulkanRenderer::resize_callback, geometry_viewer::VulkanRenderer::pos_callback);
  if (!context.init_vulkan("Geometry Viewer", window)) return false;
  renderer.create_swapchain();
  renderer.create_image_views();
  if (!renderer.create_render_pass() || !renderer.create_graphics_pipeline(model)) return false;
  context.create_command_pool();
  if (!renderer.create_depth_resources() || !renderer.create_color_resources()) return false;
  renderer.create_framebuffers();
  if (!handle.create_texture_image(model.image_data(), model.image_size())) return false;
  handle.create_texture_image_view();
  handle.create_texture_sampler();
  if (!renderer.create_vertex_buffer(model) || !renderer.create_index_buffer(model) || !renderer.create_uniform_buffers()) return false;
  const std::array pool_size = {
      vk::DescriptorPoolSize(vk::DescriptorType::eCombinedImageSampler, 100),
      vk::DescriptorPoolSize(vk::DescriptorType::eSampledImage, 100),
      vk::DescriptorPoolSize(vk::DescriptorType::eUniformBuffer, 100),
  };
  context.create_descriptor_pool(pool_size);
  renderer.create_descriptor_sets();
  renderer.create_command_buffers();
  renderer.create_sync_objects();

  imgui.init(static_cast<uint32_t>(renderer.frames_in_flight()), renderer.render_pass(), geometries);

  while (!window.should_close()) {
    helper::WindowHandler::poll_events();
    glfwPollEvents();
    imgui.draw();
    if (!renderer.draw_frame(model, imgui)) return false;
  }

  context.device().waitIdle();
  geometry_viewer::VulkanImGui::cleanup();
  renderer.cleanup();

  return true;
}

}  // namespace autd3::extra
