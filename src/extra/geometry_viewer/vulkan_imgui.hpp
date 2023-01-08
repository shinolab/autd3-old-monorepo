// File: vulkan_imgui.hpp
// Project: include
// Created Date: 27/09/2022
// Author: Shun Suzuki
// -----
// Last Modified: 08/01/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <imgui.h>
#include <imgui_impl_glfw.h>
#include <imgui_impl_vulkan.h>

#include <iostream>
#include <memory>
#include <string>
#include <utility>
#include <vector>

#include "autd3/autd3_device.hpp"
#include "glm.hpp"
#include "model.hpp"
#include "transform.hpp"

namespace autd3::extra::geometry_viewer {

#ifdef AUTD3_USE_METER
constexpr float SCALE = 1e-3f;
#else
constexpr float SCALE = 1;
#endif

class VulkanImGui {
 public:
  explicit VulkanImGui(const helper::WindowHandler* window, const helper::VulkanContext* context) noexcept : _window(window), _context(context) {}
  ~VulkanImGui() = default;
  VulkanImGui(const VulkanImGui& v) = delete;
  VulkanImGui& operator=(const VulkanImGui& obj) = delete;
  VulkanImGui(VulkanImGui&& obj) = default;
  VulkanImGui& operator=(VulkanImGui&& obj) = default;

  void set_font() const {
    ImGuiIO& io = ImGui::GetIO();

    const auto [fst, snd] = _window->scale();
    const auto scale = (fst + snd) / 2.0f;

    _context->device().waitIdle();

    const std::vector<uint8_t> font_data = {
#include "fonts/NotoSans-Regular.ttf.txt"
    };
    auto* font_data_imgui = new uint8_t[font_data.size()];
    std::memcpy(font_data_imgui, font_data.data(), font_data.size());
    ImFont* font = io.Fonts->AddFontFromMemoryTTF(font_data_imgui, static_cast<int>(font_data.size()), _font_size * scale);
    io.FontGlobalScale = 1.0f / scale;
    io.FontDefault = font;

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
    if (const auto* bd = static_cast<ImGuiImplVulkanData*>(ImGui::GetIO().BackendRendererUserData); bd != nullptr) {
      if (bd->font_image != nullptr) _context->device().destroyImage(bd->font_image);
      if (bd->font_view != nullptr) _context->device().destroyImageView(bd->font_view);
      if (bd->font_memory != nullptr) _context->device().freeMemory(bd->font_memory);
    }

    _context->device().resetCommandPool(_context->command_pool());
    const vk::CommandBufferAllocateInfo alloc_info(_context->command_pool(), vk::CommandBufferLevel::ePrimary, 1);
    auto command_buffers = _context->device().allocateCommandBuffersUnique(alloc_info);
    vk::UniqueCommandBuffer command_buffer = std::move(command_buffers[0]);
    constexpr vk::CommandBufferBeginInfo begin_info(vk::CommandBufferUsageFlagBits::eOneTimeSubmit);
    command_buffer->begin(begin_info);
    ImGui_ImplVulkan_CreateFontsTexture(command_buffer.get());
    const vk::SubmitInfo end_info(0, nullptr, nullptr, 1, &command_buffer.get(), 0, nullptr);
    command_buffer->end();
    _context->graphics_queue().submit(end_info);
    _context->device().waitIdle();
    ImGui_ImplVulkan_DestroyFontUploadObjects();
  }

  [[nodiscard]] bool init(const uint32_t image_count, const VkRenderPass renderer_pass, std::vector<gltf::Geometry> geometries) {
    _geometries = std::move(geometries);

    const auto& [pos, rot] = _geometries[0];

    const auto rot_mat = mat4_cast(helper::to_gl_rot(rot));

    const auto right = glm::vec3(rot_mat * glm::vec4(1.0f, 0.0f, 0.0f, 1.0f));
    const auto up = glm::vec3(rot_mat * glm::vec4(0.0f, 1.0f, 0.0f, 1.0f));
    const auto forward = glm::vec3(rot_mat * glm::vec4(0.0f, 0.0f, 1.0f, 1.0f));

    const auto center = pos + right * static_cast<float>(AUTD3::DEVICE_WIDTH) / 2.0f + up * static_cast<float>(AUTD3::DEVICE_HEIGHT) / 2.0f;
    const auto cam_pos =
        pos + right * static_cast<float>(AUTD3::DEVICE_WIDTH) / 2.0f - up * static_cast<float>(AUTD3::DEVICE_HEIGHT) + forward * 300.0f * SCALE;
    const auto cam_view = lookAt(cam_pos, center, forward);

    camera_pos = helper::to_gl_pos(cam_pos);
    camera_rot = degrees(eulerAngles(helper::to_gl_rot(quat_cast(transpose(cam_view)))));
    light_pos = camera_pos;

    show = std::make_unique<bool[]>(_geometries.size());
    std::fill_n(show.get(), _geometries.size(), true);

    IMGUI_CHECKVERSION();
    ImGui::CreateContext();

    ImGui::StyleColorsDark();
    const auto [graphics_family, present_family] = _context->find_queue_families(_context->physical_device());

    if (!graphics_family) {
      spdlog::error("Failed to find queue family.");
      return false;
    }

    ImGui_ImplGlfw_InitForVulkan(_window->window(), true);
    ImGui_ImplVulkan_InitInfo init_info{_context->instance(),
                                        _context->physical_device(),
                                        _context->device(),
                                        graphics_family.value(),
                                        _context->graphics_queue(),
                                        nullptr,
                                        _context->descriptor_pool(),
                                        0,
                                        image_count,
                                        image_count,
                                        static_cast<VkSampleCountFlagBits>(_context->msaa_samples()),
                                        nullptr,
                                        check_vk_result};
    ImGui_ImplVulkan_Init(&init_info, renderer_pass);

    set_font();

    return true;
  }

  void draw() {
    if (_update_font) {
      set_font();
      _update_font = false;
    }

    ImGui_ImplVulkan_NewFrame();
    ImGui_ImplGlfw_NewFrame();
    ImGui::NewFrame();

    const auto& io = ImGui::GetIO();
    {
      const auto rot = glm::quat(radians(camera_rot));
      const auto model = mat4_cast(rot);

      const auto r = make_vec3(model[0]);
      const auto u = make_vec3(model[1]);
      const auto f = make_vec3(model[2]);

      if (!io.WantCaptureMouse) {
        const auto mouse_wheel = io.MouseWheel;
#ifdef AUTD3_USE_LEFT_HANDED
        const auto trans = f * mouse_wheel * _cam_move_speed;
#else
        const auto trans = -f * mouse_wheel * _cam_move_speed;
#endif
        camera_pos[0] += trans.x;
        camera_pos[1] += trans.y;
        camera_pos[2] += trans.z;
      }

      if (!io.WantCaptureMouse) {
        const auto mouse_delta = io.MouseDelta;
        if (io.MouseDown[0]) {
          if (io.KeyShift) {
            const auto delta = glm::vec2(mouse_delta.x, mouse_delta.y) * _cam_move_speed / SCALE / 3000.0f;
            const auto to = -r * delta.x + u * delta.y + f;
            const auto rotation = helper::quaternion_to(f, to);
            camera_rot = degrees(eulerAngles(rotation * rot));
          } else {
            const auto delta = glm::vec2(mouse_delta.x, mouse_delta.y) * _cam_move_speed / 10.0f;
            const auto trans = -r * delta.x + u * delta.y;
            camera_pos[0] += trans.x;
            camera_pos[1] += trans.y;
            camera_pos[2] += trans.z;
          }
        }
      }
    }

    ImGui::PushFont(io.FontDefault);

    ImGui::Begin("Dear ImGui");
    if (ImGui::BeginTabBar("Settings")) {
      if (ImGui::BeginTabItem("Camera")) {
        ImGui::DragFloat("Camera X", &camera_pos.x, 1 * SCALE);
        ImGui::DragFloat("Camera Y", &camera_pos.y, 1 * SCALE);
        ImGui::DragFloat("Camera Z", &camera_pos.z, 1 * SCALE);
        ImGui::Separator();
        ImGui::DragFloat("Camera RX", &camera_rot.x, 1, -180, 180);
        ImGui::DragFloat("Camera RY", &camera_rot.y, 1, -180, 180);
        ImGui::DragFloat("Camera RZ", &camera_rot.z, 1, -180, 180);
        ImGui::Separator();
        ImGui::DragFloat("FOV", &fov, 1, 0, 180);
        ImGui::Separator();
        ImGui::DragFloat("Camera move speed", &_cam_move_speed, 1 * SCALE);
        ImGui::EndTabItem();
      }

      if (ImGui::BeginTabItem("Lighting")) {
        ImGui::DragFloat("Light X", &light_pos.x, 1 * SCALE);
        ImGui::DragFloat("Light Y", &light_pos.y, 1 * SCALE);
        ImGui::DragFloat("Light Z", &light_pos.z, 1 * SCALE);
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

        if (ImGui::DragFloat("Font size", &_font_size, 1, 1.0f, 256.0f)) _update_font = true;

        ImGui::Separator();

        ImGui::ColorPicker4("Background", &background[0]);

        ImGui::EndTabItem();
      }

      if (ImGui::BeginTabItem("Info")) {
        ImGui::Text("FPS: %4.2f fps", static_cast<double>(ImGui::GetIO().Framerate));

        ImGui::Separator();
        size_t i = 0;
        std::for_each(_geometries.begin(), _geometries.end(), [&i](const auto geometry) {
          ImGui::Text("Device %d", static_cast<int32_t>(i++));
          ImGui::Text("\tx: %4.2f, y: %4.2f, z: %4.2f", static_cast<double>(geometry.pos.x), static_cast<double>(geometry.pos.y),
                      static_cast<double>(geometry.pos.z));
          ImGui::Text("\trw: %4.2f, rx: %4.2f, ry: %4.2f, rz: %4.2f", static_cast<double>(geometry.rot.w), static_cast<double>(geometry.rot.x),
                      static_cast<double>(geometry.rot.y), static_cast<double>(geometry.rot.z));
        });
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

  glm::vec4 background{0.3f, 0.3f, 0.3f, 1.0f};

  glm::vec3 camera_pos{};
  glm::vec3 camera_rot{};
  float fov = 45.0f;

  glm::vec3 light_pos{};
  gltf::Lighting lighting{0.1f, 32.0f};

  std::unique_ptr<bool[]> show{nullptr};

 private:
  const helper::WindowHandler* _window{nullptr};
  const helper::VulkanContext* _context{nullptr};
  float _font_size = 16.0f;
  bool _update_font = false;
  float _cam_move_speed = 10.0f * SCALE;
  std::vector<gltf::Geometry> _geometries{};

  static void check_vk_result(const VkResult err) {
    if (err == VK_SUCCESS) return;
    std::cerr << "[vulkan] Error: VkResult = " << err << std::endl;
    if (err < 0) std::abort();
  }
};

}  // namespace autd3::extra::geometry_viewer
