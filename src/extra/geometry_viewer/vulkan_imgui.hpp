// File: vulkan_imgui.hpp
// Project: include
// Created Date: 27/09/2022
// Author: Shun Suzuki
// -----
// Last Modified: 03/10/2022
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
#include "transform.hpp"

namespace autd3::extra::geometry_viewer {

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

    ImFont* font;
    if (!_font_path.empty()) {
      font = io.Fonts->AddFontFromFileTTF(_font_path.c_str(), _font_size * scale);
      io.FontGlobalScale = 1.0f / scale;
    } else
      font = io.Fonts->AddFontDefault();
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
    const vk::CommandBufferBeginInfo begin_info(vk::CommandBufferUsageFlagBits::eOneTimeSubmit);
    command_buffer->begin(begin_info);
    ImGui_ImplVulkan_CreateFontsTexture(command_buffer.get());
    vk::SubmitInfo end_info(0, nullptr, nullptr, 1, &command_buffer.get(), 0, nullptr);
    command_buffer->end();
    _context->graphics_queue().submit(end_info);
    _context->device().waitIdle();
    ImGui_ImplVulkan_DestroyFontUploadObjects();
  }

  void init(const uint32_t image_count, const VkRenderPass renderer_pass, std::vector<gltf::Geometry> geometries, const std::string& font_path) {
    _geometries = std::move(geometries);

    const auto& [pos, rot] = _geometries[0];

    const auto rot_mat = mat4_cast(rot);

    const auto right = glm::vec3(rot_mat * glm::vec4(1.0f, 0.0f, 0.0f, 1.0f));
    const auto up = glm::vec3(rot_mat * glm::vec4(0.0f, 1.0f, 0.0f, 1.0f));
    const auto forward = glm::vec3(rot_mat * glm::vec4(0.0f, 0.0f, 1.0f, 1.0f));

    const auto center = helper::to_gl_pos(pos + right * 192.0f / 2.0f + up * 151.4f / 2.0f);

    const auto cam_pos_autd = pos + right * 192.0f / 2.0f - up * 151.4f + forward * 300.0f;
    const auto cam_pos = helper::to_gl_pos(cam_pos_autd);

    const auto cam_view = lookAt(cam_pos, center, forward);

    const auto angles = degrees(eulerAngles(quat_cast(transpose(cam_view))));

    camera_pos = cam_pos_autd;
    camera_rot = angles;
    light_pos = cam_pos_autd;

    show = std::make_unique<bool[]>(_geometries.size());
    std::fill_n(show.get(), _geometries.size(), true);

    IMGUI_CHECKVERSION();
    ImGui::CreateContext();

    ImGui::StyleColorsDark();
    const auto [graphics_family, present_family] = _context->find_queue_families(_context->physical_device());

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

    _font_path = font_path;
    set_font();
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
        const auto trans = -f * mouse_wheel * _cam_move_speed;
        camera_pos[0] += trans.x;
        camera_pos[1] += trans.y;
        camera_pos[2] += trans.z;
      }

      if (!io.WantCaptureMouse) {
        const auto mouse_delta = io.MouseDelta;
        if (io.MouseDown[0]) {
          if (io.KeyShift) {
            const auto delta = glm::vec2(mouse_delta.x, mouse_delta.y) * _cam_move_speed / 3000.0f;
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
        ImGui::DragFloat("Camera X", &camera_pos.x);
        ImGui::DragFloat("Camera Y", &camera_pos.y);
        ImGui::DragFloat("Camera Z", &camera_pos.z);
        ImGui::Separator();
        ImGui::DragFloat("Camera RX", &camera_rot.x, 1, -180, 180);
        ImGui::DragFloat("Camera RY", &camera_rot.y, 1, -180, 180);
        ImGui::DragFloat("Camera RZ", &camera_rot.z, 1, -180, 180);
        ImGui::Separator();
        ImGui::DragFloat("FOV", &fov, 1, 0, 180);
        ImGui::Separator();
        ImGui::DragFloat("Camera move speed", &_cam_move_speed);
        ImGui::EndTabItem();
      }

      if (ImGui::BeginTabItem("Lighting")) {
        ImGui::DragFloat("Light X", &light_pos.x);
        ImGui::DragFloat("Light Y", &light_pos.y);
        ImGui::DragFloat("Light Z", &light_pos.z);
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

  glm::vec4 background{};

  glm::vec3 camera_pos{};
  glm::vec3 camera_rot{};
  float fov = 45.0f;

  glm::vec3 light_pos{};
  gltf::Lighting lighting{0.1f, 32.0f};

  std::unique_ptr<bool[]> show;

 private:
  const helper::WindowHandler* _window;
  const helper::VulkanContext* _context;
  std::string _font_path;
  float _font_size = 16.0f;
  bool _update_font = false;
  float _cam_move_speed = 10.0f;
  std::vector<gltf::Geometry> _geometries;

  static void check_vk_result(const VkResult err) {
    if (err == VK_SUCCESS) return;
    std::cerr << "[vulkan] Error: VkResult = " << err << std::endl;
    if (err < 0) std::abort();
  }
};

}  // namespace autd3::extra::geometry_viewer
