// File: vulkan_imgui.hpp
// Project: include
// Created Date: 27/09/2022
// Author: Shun Suzuki
// -----
// Last Modified: 30/09/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <imgui.h>
#include <imgui_impl_glfw.h>
#include <imgui_impl_vulkan.h>

#include <glm/gtc/type_ptr.hpp>
#include <iostream>
#include <memory>
#include <string>
#include <utility>
#include <vector>

#include "model.hpp"
#include "vulkan_handler.hpp"

namespace autd3::extra::geometry_viewer {

class VulkanImGui {
 public:
  VulkanImGui() noexcept = default;
  ~VulkanImGui() = default;
  VulkanImGui(const VulkanImGui& v) = delete;
  VulkanImGui& operator=(const VulkanImGui& obj) = delete;
  VulkanImGui(VulkanImGui&& obj) = default;
  VulkanImGui& operator=(VulkanImGui&& obj) = default;

  void init(const vk_helper::WindowHandler& window, const vk_helper::VulkanContext& contex, const uint32_t image_count,
            const VkRenderPass renderer_pass, std::vector<gltf::Geometry> geometries, const std::string& font_path) {
    _geometries = std::move(geometries);

    const auto& [pos, rot] = _geometries[0];
    const auto rot_mat = mat4_cast(glm::quat(rot.w, rot.x, rot.z, -rot.y));

    const auto right = glm::vec3(rot_mat * glm::vec4(1.0f, 0.0f, 0.0f, 1.0f));
    const auto up = glm::vec3(rot_mat * glm::vec4(0.0f, 1.0f, 0.0f, 1.0f));
    const auto forward = glm::vec3(rot_mat * glm::vec4(0.0f, 0.0f, 1.0f, 1.0f));
    const auto center = pos + right * 192.0f / 2.0f + up * 151.4f / 2.0f;
    const auto center_v = glm::vec3(center[0], center[2], -center[1]) / 1000.0f;

    const auto cam_pos = pos + right * 192.0f / 2.0f - up * 151.4f + forward * 300.0f;
    const auto cam_pos_v = glm::vec3(cam_pos[0], cam_pos[2], -cam_pos[1]) / 1000.0f;
    const auto cam_view = lookAt(cam_pos_v, center_v, up);

    const auto r = glm::vec3(cam_view[0].x, cam_view[0].y, cam_view[0].z);
    const auto u = glm::vec3(cam_view[1].x, cam_view[1].y, cam_view[1].z);
    const auto f = glm::vec3(cam_view[2].x, cam_view[2].y, cam_view[2].z);
    const auto angles = degrees(
        eulerAngles(quat_cast(transpose(glm::mat4(glm::vec4(r, 0.0f), glm::vec4(u, 0.0f), glm::vec4(f, 0.0f), glm::vec4(0.0f, 0.0f, 0.0f, 1.0f))))));

    camera_pos = {cam_pos[0], cam_pos[1], cam_pos[2]};
    camera_rot = {angles[0], -angles[2], angles[1]};
    light_pos = {cam_pos[0], cam_pos[1], cam_pos[2]};

    show = std::make_unique<bool[]>(_geometries.size());
    std::fill_n(show.get(), _geometries.size(), true);

    const auto [fst, snd] = window.scale();
    const auto factor = (fst + snd) / 2.0f;

    IMGUI_CHECKVERSION();
    ImGui::CreateContext();
    ImGuiIO& io = ImGui::GetIO();

    if (!font_path.empty()) {
      io.Fonts->AddFontFromFileTTF(font_path.c_str(), 16.0f * factor);
      io.FontGlobalScale = 1.0f / factor;
    } else
      io.Fonts->AddFontDefault();

    ImGui::StyleColorsDark();

    const auto [graphics_family, present_family] = contex.find_queue_families(contex.physical_device());
    ImGui_ImplGlfw_InitForVulkan(window.window(), true);
    ImGui_ImplVulkan_InitInfo init_info{contex.instance(),
                                        contex.physical_device(),
                                        contex.device(),
                                        graphics_family.value(),
                                        contex.graphics_queue(),
                                        nullptr,
                                        contex.descriptor_pool(),
                                        0,
                                        image_count,
                                        image_count,
                                        static_cast<VkSampleCountFlagBits>(contex.msaa_samples()),
                                        nullptr,
                                        check_vk_result};
    ImGui_ImplVulkan_Init(&init_info, renderer_pass);

    {
      contex.device().resetCommandPool(contex.command_pool());
      const vk::CommandBufferAllocateInfo alloc_info(contex.command_pool(), vk::CommandBufferLevel::ePrimary, 1);
      auto command_buffers = contex.device().allocateCommandBuffersUnique(alloc_info);
      vk::UniqueCommandBuffer command_buffer = std::move(command_buffers[0]);
      const vk::CommandBufferBeginInfo begin_info(vk::CommandBufferUsageFlagBits::eOneTimeSubmit);
      command_buffer->begin(begin_info);
      ImGui_ImplVulkan_CreateFontsTexture(command_buffer.get());
      vk::SubmitInfo end_info(0, nullptr, nullptr, 1, &command_buffer.get(), 0, nullptr);
      command_buffer->end();
      contex.graphics_queue().submit(end_info);
      contex.device().waitIdle();
      ImGui_ImplVulkan_DestroyFontUploadObjects();
    }
  }

  void draw(ImFont* font) {
    ImGui_ImplVulkan_NewFrame();
    ImGui_ImplGlfw_NewFrame();
    ImGui::NewFrame();

    {
      const auto& io = ImGui::GetIO();

      const auto rot = glm::quat(radians(glm::vec3(camera_rot[0], camera_rot[2], -camera_rot[1])));
      const auto model = mat4_cast(rot);

      const auto r = make_vec3(model[0]);
      const auto u = make_vec3(model[1]);
      const auto f = make_vec3(model[2]);

      if (!io.WantCaptureMouse) {
        const auto mouse_wheel = io.MouseWheel;
        const auto trans = -f * mouse_wheel * _cam_move_speed;
        camera_pos[0] += trans.x;
        camera_pos[1] -= trans.z;
        camera_pos[2] += trans.y;
      }

      if (!io.WantCaptureMouse) {
        const auto mouse_delta = io.MouseDelta;
        if (io.MouseDown[0]) {
          if (io.KeyShift) {
            const auto delta = glm::vec2(mouse_delta.x, mouse_delta.y) * _cam_move_speed / 3000.0f;
            const auto trans_x = -r * delta.x;
            const auto trans_y = u * delta.y;
            const auto to = trans_x + trans_y + f;

            const auto rotation = quaternion_to(f, to);

            const auto cam_r = rotation * r;
            const auto cam_u = rotation * u;
            const auto cam_f = rotation * f;

            const auto angles = degrees(eulerAngles(
                quat_cast(glm::mat4(glm::vec4(cam_r, 0.0f), glm::vec4(cam_u, 0.0f), glm::vec4(cam_f, 0.0f), glm::vec4(0.0f, 0.0f, 0.0f, 1.0f)))));
            camera_rot = {angles[0], -angles[2], angles[1]};
          } else {
            const auto delta = glm::vec2(mouse_delta.x, mouse_delta.y) * _cam_move_speed / 10.0f;
            const auto trans_x = -r * delta.x;
            const auto trans_y = u * delta.y;
            const auto trans = trans_x + trans_y;
            camera_pos[0] += trans.x;
            camera_pos[1] -= trans.z;
            camera_pos[2] += trans.y;
          }
        }
      }
    }

    ImGui::PushFont(font);

    ImGui::Begin("Dear ImGui");
    if (ImGui::BeginTabBar("Settings")) {
      if (ImGui::BeginTabItem("Camera")) {
        ImGui::DragFloat("Camera X", camera_pos.data());
        ImGui::DragFloat("Camera Y", camera_pos.data() + 1);
        ImGui::DragFloat("Camera Z", camera_pos.data() + 2);
        ImGui::Separator();
        ImGui::DragFloat("Camera RX", camera_rot.data(), 1, -180, 180);
        ImGui::DragFloat("Camera RY", camera_rot.data() + 1, 1, -180, 180);
        ImGui::DragFloat("Camera RZ", camera_rot.data() + 2, 1, -180, 180);
        ImGui::Separator();
        ImGui::DragFloat("FOV", &fov, 1, 0, 180);
        ImGui::Separator();
        ImGui::DragFloat("Camera move speed", &_cam_move_speed);
        ImGui::EndTabItem();
      }

      if (ImGui::BeginTabItem("Lighting")) {
        ImGui::DragFloat("Light X", light_pos.data());
        ImGui::DragFloat("Light Y", light_pos.data() + 1);
        ImGui::DragFloat("Light Z", light_pos.data() + 2);
        ImGui::Separator();
        ImGui::DragFloat("Ambient", &lighting.ambient, 0.01f);
        ImGui::DragFloat("Specular", &lighting.specular);
        ImGui::EndTabItem();
      }

      if (ImGui::BeginTabItem("Config")) {
        for (size_t i = 0; i < _geometries.size(); i++) {
          ImGui::Text("Device %d", static_cast<int32_t>(i));
          ImGui::SameLine();
          const auto id = "show##" + std::to_string(i);
          ImGui::Checkbox(id.c_str(), &show[i]);
        }

        ImGui::Separator();

        ImGui::ColorPicker4("Background", background.data());
        ImGui::EndTabItem();
      }

      if (ImGui::BeginTabItem("Info")) {
        ImGui::Text("FPS: %4.2f fps", static_cast<double>(ImGui::GetIO().Framerate));

        ImGui::Separator();

        for (size_t i = 0; i < _geometries.size(); i++) {
          ImGui::Text("Device %d", static_cast<int32_t>(i));
          ImGui::Text("\tx: %4.2f, y: %4.2f, z: %4.2f", static_cast<double>(_geometries[i].pos.x), static_cast<double>(_geometries[i].pos.y),
                      static_cast<double>(_geometries[i].pos.z));
          ImGui::Text("\trw: %4.2f, rx: %4.2f, ry: %4.2f, rz: %4.2f", static_cast<double>(_geometries[i].rot.w),
                      static_cast<double>(_geometries[i].rot.x), static_cast<double>(_geometries[i].rot.y),
                      static_cast<double>(_geometries[i].rot.z));
        }

        ImGui::EndTabItem();
      }

      ImGui::EndTabBar();
    }
    ImGui::End();
    ImGui::PopFont();

    ImGui::Render();
  }

  static void cleanup() {
    ImGui_ImplVulkan_Shutdown();
    ImGui_ImplGlfw_Shutdown();
    ImGui::DestroyContext();
  }

  std::array<float, 4> background{};

  std::array<float, 3> camera_pos{};
  std::array<float, 3> camera_rot{};
  float fov = 45.0f;

  std::array<float, 3> light_pos{};
  gltf::Lighting lighting{0.1f, 32.0f};

  std::unique_ptr<bool[]> show;

 private:
  float _cam_move_speed = 10.0f;
  std::vector<gltf::Geometry> _geometries;

  static void check_vk_result(const VkResult err) {
    if (err == VK_SUCCESS) return;
    std::cerr << "[vulkan] Error: VkResult = " << err << std::endl;
    if (err < 0) std::abort();
  }

  static glm::quat quaternion_to(glm::vec3 v, glm::vec3 to) {
    const auto a = normalize(v);
    const auto b = normalize(to);

    const auto c = normalize(cross(b, a));
    if (std::isnan(c.x) || std::isnan(c.y) || std::isnan(c.z)) return {1, 0, 0, 0};

    const auto ip = dot(a, b);
    if (constexpr float eps = 1e-4f; length(c) < eps || 1.0f < ip) {
      if (ip < eps - 1.0f) {
        const auto a2 = glm::vec3(-a.y, a.z, a.x);
        const auto c2 = normalize(cross(a2, a));
        return {0.0, c2};
      }
      return {1, 0, 0, 0};
    }
    const auto e = c * std::sqrt(0.5f * (1.0f - ip));
    return {std::sqrt(0.5f * (1.0f + ip)), e};
  }
};

}  // namespace autd3::extra::geometry_viewer
