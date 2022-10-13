// File: vulkan_imgui.hpp
// Project: imgui_vulkan
// Created Date: 03/10/2022
// Author: Shun Suzuki
// -----
// Last Modified: 13/10/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <imgui.h>
#include <imgui_impl_glfw.h>
#include <imgui_impl_vulkan.h>

#include <algorithm>
#include <glm/gtc/type_ptr.hpp>
#include <iostream>
#include <limits>
#include <memory>
#include <string>
#include <transform.hpp>
#include <utility>
#include <vector>

namespace autd3::extra::simulator {

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
    const vk::SubmitInfo end_info(0, nullptr, nullptr, 1, &command_buffer.get(), 0, nullptr);
    command_buffer->end();
    _context->graphics_queue().submit(end_info);
    _context->device().waitIdle();
    ImGui_ImplVulkan_DestroyFontUploadObjects();
  }

  void load_settings(const Settings& setting) {
    slice_pos = glm::vec3(setting.slice_pos_x, setting.slice_pos_y, setting.slice_pos_z);
    slice_rot = glm::vec3(setting.slice_rot_x, setting.slice_rot_y, setting.slice_rot_z);
    slice_width = setting.slice_width;
    slice_height = setting.slice_height;
    pixel_size = setting.slice_pixel_size;
    color_scale = setting.slice_color_scale;
    slice_alpha = setting.slice_alpha;

    camera_pos = glm::vec3(setting.camera_pos_x, setting.camera_pos_y, setting.camera_pos_z);
    camera_rot = glm::vec3(setting.camera_rot_x, setting.camera_rot_y, setting.camera_rot_z);
    fov = setting.camera_fov;
    near_clip = setting.camera_near_clip;
    far_clip = setting.camera_far_clip;
    _cam_move_speed = setting.camera_move_speed;

    _font_size = setting.font_size;
    _font_path = setting.font_path;
    background = glm::vec4(setting.background_r, setting.background_g, setting.background_b, setting.background_a);

    _show_mod_plot = setting.show_mod_plot;
    _show_mod_plot_raw = setting.show_mod_plot_raw;

    setting.image_save_path.copy(save_path, 256);
  }

  void save_settings(Settings& settings) const {
    settings.slice_pos_x = slice_pos.x;
    settings.slice_pos_y = slice_pos.y;
    settings.slice_pos_z = slice_pos.z;
    settings.slice_rot_x = slice_rot.x;
    settings.slice_rot_y = slice_rot.y;
    settings.slice_rot_z = slice_rot.z;
    settings.slice_width = slice_width;
    settings.slice_height = slice_height;
    settings.slice_pixel_size = pixel_size;
    settings.slice_color_scale = color_scale;
    settings.slice_alpha = slice_alpha;

    settings.camera_pos_x = camera_pos.x;
    settings.camera_pos_y = camera_pos.y;
    settings.camera_pos_z = camera_pos.z;
    settings.camera_rot_x = camera_rot.x;
    settings.camera_rot_y = camera_rot.y;
    settings.camera_rot_z = camera_rot.z;
    settings.camera_fov = fov;
    settings.camera_near_clip = near_clip;
    settings.camera_far_clip = far_clip;
    settings.camera_move_speed = _cam_move_speed;

    settings.font_size = _font_size;
    settings.font_path = _font_path;

    settings.background_r = background.r;
    settings.background_g = background.g;
    settings.background_b = background.b;
    settings.background_a = background.a;

    settings.show_mod_plot = _show_mod_plot;
    settings.show_mod_plot_raw = _show_mod_plot_raw;

    settings.image_save_path = std::string(save_path);
  }

  void init(const uint32_t image_count, const VkRenderPass renderer_pass, const Settings& settings) {
    load_settings(settings);
    _default_settings = settings;

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

    set_font();
  }

  void set(const SoundSources& sources) {
    const auto dev_num = sources.size() / driver::NUM_TRANS_IN_UNIT;
    visible = std::make_unique<bool[]>(dev_num);
    enable = std::make_unique<bool[]>(dev_num);
    std::fill_n(visible.get(), dev_num, true);
    std::fill_n(enable.get(), dev_num, true);
  }

  static void draw() {
    ImGui_ImplVulkan_NewFrame();
    ImGui_ImplGlfw_NewFrame();
    ImGui::NewFrame();
    const auto& io = ImGui::GetIO();
    ImGui::PushFont(io.FontDefault);
    ImGui::Begin("Dear ImGui");
    ImGui::Text("Waiting for connection...");
    ImGui::End();
    ImGui::PopFont();
    ImGui::Render();
  }

  UpdateFlags draw(const std::vector<firmware_emulator::cpu::CPU>& cpus, SoundSources& sources) {
    auto flag = UpdateFlags(UpdateFlags::NONE);

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
        flag.set(UpdateFlags::UPDATE_CAMERA_POS);
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
          flag.set(UpdateFlags::UPDATE_CAMERA_POS);
        }
      }
    }

    ImGui::PushFont(io.FontDefault);

    ImGui::Begin("Dear ImGui");
    if (ImGui::BeginTabBar("Settings")) {
      if (ImGui::BeginTabItem("Slice")) {
        ImGui::Text("Position");
        if (ImGui::DragFloat("X##Slice", &slice_pos.x)) flag.set(UpdateFlags::UPDATE_SLICE_POS);
        if (ImGui::DragFloat("Y##Slice", &slice_pos.y)) flag.set(UpdateFlags::UPDATE_SLICE_POS);
        if (ImGui::DragFloat("Z##Slice", &slice_pos.z)) flag.set(UpdateFlags::UPDATE_SLICE_POS);
        ImGui::Separator();

        ImGui::Text("Rotation");
        if (ImGui::DragFloat("RX##Slice", &slice_rot.x, 1, -180, 180)) flag.set(UpdateFlags::UPDATE_SLICE_POS);
        if (ImGui::DragFloat("RY##Slice", &slice_rot.y, 1, -180, 180)) flag.set(UpdateFlags::UPDATE_SLICE_POS);
        if (ImGui::DragFloat("RZ##Slice", &slice_rot.z, 1, -180, 180)) flag.set(UpdateFlags::UPDATE_SLICE_POS);
        ImGui::Separator();

        ImGui::Text("Size");
        if (ImGui::DragInt("Width##Slice", &slice_width, 1, 1, 2000)) flag.set(UpdateFlags::UPDATE_SLICE_SIZE);
        if (ImGui::DragInt("Height##Slice", &slice_height, 1, 1, 2000)) flag.set(UpdateFlags::UPDATE_SLICE_SIZE);
        if (ImGui::DragInt("Pixel size##Slice", &pixel_size, 1, 1, 8)) flag.set(UpdateFlags::UPDATE_SLICE_SIZE);
        ImGui::Separator();

        ImGui::Text("Color settings");
        if (ImGui::DragFloat("Scale##Slice", &color_scale, 0.1f, 0.0f, std::numeric_limits<float>::infinity()))
          flag.set(UpdateFlags::UPDATE_COLOR_MAP);
        if (ImGui::DragFloat("Alpha##Slice", &slice_alpha, 0.01f, 0.0f, 1.0f)) flag.set(UpdateFlags::UPDATE_COLOR_MAP);

        ImGui::Separator();
        if (ImGui::SmallButton("xy")) {
          slice_rot = glm::vec3(0, 0, 0);
          flag.set(UpdateFlags::UPDATE_SLICE_POS);
        }
        ImGui::SameLine();
        if (ImGui::SmallButton("yz")) {
          slice_rot = glm::vec3(90, 0, 90);
          flag.set(UpdateFlags::UPDATE_SLICE_POS);
        }
        ImGui::SameLine();
        if (ImGui::SmallButton("zx")) {
          slice_rot = glm::vec3(90, 0, 0);
          flag.set(UpdateFlags::UPDATE_SLICE_POS);
        }

        ImGui::EndTabItem();
      }

      if (ImGui::BeginTabItem("Camera")) {
        ImGui::Text("Position");
        if (ImGui::DragFloat("X##Camera", &camera_pos.x)) flag.set(UpdateFlags::UPDATE_CAMERA_POS);
        if (ImGui::DragFloat("Y##Camera", &camera_pos.y)) flag.set(UpdateFlags::UPDATE_CAMERA_POS);
        if (ImGui::DragFloat("Z##Camera", &camera_pos.z)) flag.set(UpdateFlags::UPDATE_CAMERA_POS);
        ImGui::Separator();
        ImGui::Text("Rotation");
        if (ImGui::DragFloat("RX##Camera", &camera_rot.x, 1, -180, 180)) flag.set(UpdateFlags::UPDATE_CAMERA_POS);
        if (ImGui::DragFloat("RY##Camera", &camera_rot.y, 1, -180, 180)) flag.set(UpdateFlags::UPDATE_CAMERA_POS);
        if (ImGui::DragFloat("RZ##Camera", &camera_rot.z, 1, -180, 180)) flag.set(UpdateFlags::UPDATE_CAMERA_POS);
        ImGui::Separator();
        ImGui::Text("Perspective");
        if (ImGui::DragFloat("FOV", &fov, 1, 0, 180)) flag.set(UpdateFlags::UPDATE_CAMERA_POS);
        if (ImGui::DragFloat("Near clip", &near_clip, 1, 0, std::numeric_limits<float>::infinity())) flag.set(UpdateFlags::UPDATE_CAMERA_POS);
        if (ImGui::DragFloat("Far clip", &far_clip, 1, 0, std::numeric_limits<float>::infinity())) flag.set(UpdateFlags::UPDATE_CAMERA_POS);
        flag.set(UpdateFlags::UPDATE_CAMERA_POS);
        ImGui::Separator();
        ImGui::DragFloat("Move speed", &_cam_move_speed);
        ImGui::EndTabItem();
      }

      if (ImGui::BeginTabItem("Config")) {
        if (ImGui::DragFloat("Sound speed", &sound_speed, 1)) {
          for (size_t dev = 0; dev < cpus.size(); dev++) {
            const auto& cycles = cpus[dev].fpga().cycles();
            for (size_t i = 0; i < driver::NUM_TRANS_IN_UNIT; i++) {
              const auto freq = static_cast<float>(driver::FPGA_CLK_FREQ) / static_cast<float>(cycles[i]);
              sources.drives()[i + dev * driver::NUM_TRANS_IN_UNIT].set_wave_num(freq, sound_speed * 1e3f);
            }
          }
          flag.set(UpdateFlags::UPDATE_SOURCE_DRIVE);
        }

        ImGui::Separator();

        if (ImGui::DragFloat("Font size", &_font_size, 1, 1.0f, 256.0f)) _update_font = true;
        ImGui::Separator();
        ImGui::Text("Device index: show/enable");
        for (size_t i = 0; i < cpus.size(); i++) {
          ImGui::Text("Device %d: ", static_cast<int32_t>(i));
          ImGui::SameLine();
          const auto show_id = "##show" + std::to_string(i);
          if (ImGui::Checkbox(show_id.c_str(), &visible[i])) {
            flag.set(UpdateFlags::UPDATE_SOURCE_FLAG);
            for (size_t tr = 0; tr < driver::NUM_TRANS_IN_UNIT; tr++)
              sources.visibilities()[driver::NUM_TRANS_IN_UNIT * i + tr] = visible[i] ? 1.0f : 0.0f;
          }
          ImGui::SameLine();
          const auto enable_id = "##enable" + std::to_string(i);
          if (ImGui::Checkbox(enable_id.c_str(), &enable[i])) {
            flag.set(UpdateFlags::UPDATE_SOURCE_FLAG);
            for (size_t tr = 0; tr < driver::NUM_TRANS_IN_UNIT; tr++)
              sources.drives()[driver::NUM_TRANS_IN_UNIT * i + tr].enable = enable[i] ? 1.0f : 0.0f;
          }
        }

        ImGui::Separator();

        ImGui::ColorPicker4("Background", &background[0]);

        ImGui::EndTabItem();
      }

      if (ImGui::BeginTabItem("Info")) {
        ImGui::Text("FPS: %4.2f fps", static_cast<double>(ImGui::GetIO().Framerate));

        ImGui::Separator();
        ImGui::Text("Silencer");
        ImGui::Text("Cycle: %d", static_cast<int32_t>(cpus[0].fpga().silencer_cycle()));
        const auto freq = static_cast<double>(driver::FPGA_CLK_FREQ) / static_cast<double>(cpus[0].fpga().silencer_cycle());
        ImGui::Text("Sampling Frequency: %.3lf [Hz]", freq);
        ImGui::Text("Step: %d", static_cast<int32_t>(cpus[0].fpga().silencer_step()));

        {
          const auto& m = cpus[0].fpga().modulation();
          ImGui::Separator();
          ImGui::Text("Modulation");
          ImGui::Text("Size: %d", static_cast<int32_t>(m.size()));
          ImGui::Text("Frequency division: %d", cpus[0].fpga().modulation_frequency_division());
          const auto sampling_freq = static_cast<double>(driver::FPGA_CLK_FREQ) / static_cast<double>(cpus[0].fpga().modulation_frequency_division());
          ImGui::Text("Sampling Frequency: %.3lf [Hz]", sampling_freq);
          const auto sampling_period =
              1000000.0 * static_cast<double>(cpus[0].fpga().modulation_frequency_division()) / static_cast<double>(driver::FPGA_CLK_FREQ);
          ImGui::Text("Sampling period: %.3lf [us]", sampling_period);
          const auto period = sampling_period * static_cast<double>(m.size());
          ImGui::Text("Period: %.3lf [us]", period);

          if (!m.empty()) ImGui::Text("mod[0]: %d", m[0]);
          if (m.size() == 2 || m.size() == 3)
            ImGui::Text("mod[1]: %d", m[1]);
          else if (m.size() > 3)
            ImGui::Text("...");
          if (m.size() >= 3) ImGui::Text("mod[%d]: %d", static_cast<int32_t>(m.size() - 1), m[1]);

          if (ImGui::RadioButton("Show mod plot", _show_mod_plot)) _show_mod_plot = !_show_mod_plot;

          if (_show_mod_plot) {
            std::vector<float> mod_v;
            mod_v.resize(m.size());
            std::transform(m.begin(), m.end(), mod_v.begin(),
                           [](const uint8_t v) { return std::sin(static_cast<float>(v) / 512.0f * glm::pi<float>()); });
            ImGui::PlotLines("##mod plot", mod_v.data(), static_cast<int32_t>(mod_v.size()), 0, nullptr, 0.0f, 1.0f, _mod_plot_size);
          }

          if (ImGui::RadioButton("Show mod plot (raw)", _show_mod_plot_raw)) _show_mod_plot_raw = !_show_mod_plot_raw;

          if (_show_mod_plot_raw) {
            std::vector<float> mod_v;
            mod_v.resize(m.size());
            std::transform(m.begin(), m.end(), mod_v.begin(), [](const uint8_t v) { return static_cast<float>(v); });
            ImGui::PlotLines("##mod plot raw", mod_v.data(), static_cast<int32_t>(mod_v.size()), 0, nullptr, 0.0f, 255.0f, _mod_plot_size);
          }

          if (_show_mod_plot || _show_mod_plot_raw) ImGui::DragFloat2("plot size", &_mod_plot_size.x);
        }

        if (is_stm_mode) {
          ImGui::Separator();

          if (cpus[0].fpga().is_stm_gain_mode())
            ImGui::Text("Gain STM");
          else {
            ImGui::Text("Point STM");
            ImGui::Text("Sound speed: %.3lf [mm/s]", cpus[0].fpga().sound_speed() * 1000.0 / 1024.0);
          }

          ImGui::Text("Size: %d", static_cast<int32_t>(cpus[0].fpga().stm_cycle()));
          ImGui::Text("Frequency division: %d", cpus[0].fpga().stm_frequency_division());
          const auto sampling_freq = static_cast<double>(driver::FPGA_CLK_FREQ) / static_cast<double>(cpus[0].fpga().stm_frequency_division());
          ImGui::Text("Sampling Frequency: %.3lf [Hz]", sampling_freq);
          const auto sampling_period =
              1000000.0 * static_cast<double>(cpus[0].fpga().stm_frequency_division()) / static_cast<double>(driver::FPGA_CLK_FREQ);
          ImGui::Text("Sampling period: %.3lf [us]", sampling_period);
          const auto period = sampling_period * static_cast<double>(cpus[0].fpga().stm_cycle());
          ImGui::Text("Period: %.3lf [us]", period);

          if (ImGui::InputInt("Index##STM", &stm_idx, 1, 10)) flag.set(UpdateFlags::UPDATE_SOURCE_DRIVE);
          if (stm_idx >= stm_size) stm_idx = 0;
          if (stm_idx < 0) stm_idx = stm_size - 1;

          ImGui::Text("Time: %.3lf", sampling_period * static_cast<double>(stm_idx));
        }

        ImGui::Separator();
        ImGui::Text("MSG ID: %d", cpus[0].msg_id());
        ImGui::Text("CPU control flags");
        const auto cpu_flags = cpus[0].cpu_flags();
        if (auto mod = cpu_flags.contains(driver::CPUControlFlags::MOD); mod) {
          auto mod_begin = cpu_flags.contains(driver::CPUControlFlags::MOD_BEGIN);
          auto mod_end = cpu_flags.contains(driver::CPUControlFlags::MOD_END);
          ImGui::Checkbox("MOD", &mod);
          ImGui::Checkbox("MOD BEGIN", &mod_begin);
          ImGui::Checkbox("MOD END", &mod_end);
        } else if (auto config_en_n = cpu_flags.contains(driver::CPUControlFlags::CONFIG_EN_N); !config_en_n) {
          auto config_silencer = cpu_flags.contains(driver::CPUControlFlags::CONFIG_SILENCER);
          auto config_sync = cpu_flags.contains(driver::CPUControlFlags::CONFIG_SYNC);
          ImGui::Checkbox("CONFIG EN N", &config_en_n);
          ImGui::Checkbox("CONFIG SILENCER", &config_silencer);
          ImGui::Checkbox("CONFIG SYNC", &config_sync);
        }
        auto write_body = cpu_flags.contains(driver::CPUControlFlags::WRITE_BODY);
        auto stm_begin = cpu_flags.contains(driver::CPUControlFlags::STM_BEGIN);
        auto stm_end = cpu_flags.contains(driver::CPUControlFlags::STM_END);
        auto is_duty = cpu_flags.contains(driver::CPUControlFlags::IS_DUTY);
        auto mod_delay = cpu_flags.contains(driver::CPUControlFlags::MOD_DELAY);
        ImGui::Checkbox("WRITE BODY", &write_body);
        ImGui::Checkbox("STM BEGIN", &stm_begin);
        ImGui::Checkbox("STM END", &stm_end);
        ImGui::Checkbox("IS DUTY", &is_duty);
        ImGui::Checkbox("MOD DELAY", &mod_delay);

        ImGui::Separator();
        ImGui::Text("FPGA control flags");
        const auto fpga_flags = cpus[0].fpga_flags();
        auto is_legacy_mode = fpga_flags.contains(driver::FPGAControlFlags::LEGACY_MODE);
        auto force_fan = fpga_flags.contains(driver::FPGAControlFlags::FORCE_FAN);
        auto stm_mode = fpga_flags.contains(driver::FPGAControlFlags::STM_MODE);
        auto stm_gain_mode = fpga_flags.contains(driver::FPGAControlFlags::STM_GAIN_MODE);
        auto reads_fpga_info = fpga_flags.contains(driver::FPGAControlFlags::READS_FPGA_INFO);
        ImGui::Checkbox("LEGACY MODE", &is_legacy_mode);
        ImGui::Checkbox("FORCE FAN", &force_fan);
        ImGui::Checkbox("STM MODE", &stm_mode);
        ImGui::Checkbox("STM GAIN MODE", &stm_gain_mode);
        ImGui::Checkbox("READS FPGA INFO", &reads_fpga_info);

        ImGui::EndTabItem();
      }

      ImGui::EndTabBar();
    }

    ImGui::Separator();
    ImGui::Text("Save image as file");
    ImGui::InputText("path to image", save_path, 256);
    if (ImGui::SmallButton("save")) flag.set(UpdateFlags::SAVE_IMAGE);

    ImGui::Separator();

    if (ImGui::SmallButton("Auto")) {
      const auto sr = mat4_cast(glm::quat(radians(slice_rot)));
      const auto srf = glm::vec3(sr * glm::vec4(0, 0, 1, 1));
      camera_pos = slice_pos + srf * 600.0f;
      camera_rot = slice_rot;

      flag.set(UpdateFlags::UPDATE_CAMERA_POS);
    }
    ImGui::SameLine();
    if (ImGui::SmallButton("Reset")) {
      load_settings(_default_settings);
      flag.set(UpdateFlags::all().value());
    }

    ImGui::End();
    ImGui::PopFont();

    ImGui::Render();

    return flag;
  }

  static void render(const vk::CommandBuffer command_buffer) {
    ImDrawData* draw_data = ImGui::GetDrawData();
    ImGui_ImplVulkan_RenderDrawData(draw_data, command_buffer);
  }

  static void cleanup() {
    ImGui_ImplVulkan_Shutdown();
    ImGui_ImplGlfw_Shutdown();
    ImGui::DestroyContext();
  }

  [[nodiscard]] std::pair<glm::mat4, glm::mat4> get_view_proj(const float aspect) const {
    const auto rot = glm::quat(radians(camera_rot));
    const auto p = camera_pos;
    const auto view = helper::orthogonal(p, rot);
    auto proj = glm::perspective(glm::radians(fov), aspect, near_clip, far_clip);
    proj[1][1] *= -1;
    return std::make_pair(view, proj);
  }

  [[nodiscard]] glm::mat4 get_slice_model() const {
    return translate(glm::identity<glm::mat4>(), slice_pos) * mat4_cast(glm::quat(radians(slice_rot)));
  }

  int32_t slice_width{0};
  int32_t slice_height{0};
  glm::vec3 slice_pos{};
  glm::vec3 slice_rot{};
  float color_scale{2.0};
  float slice_alpha{1.0f};
  int32_t pixel_size{1};

  glm::vec3 camera_pos{};
  glm::vec3 camera_rot{};
  float fov = 45.0f;
  float near_clip = 0.1f;
  float far_clip = 1000.0f;

  float sound_speed = 340.0f;  // m/s

  glm::vec4 background{0.3f, 0.3f, 0.3f, 1.0f};

  bool is_stm_mode{false};
  int32_t stm_idx{0};
  int32_t stm_size{0};

  std::unique_ptr<bool[]> enable;
  std::unique_ptr<bool[]> visible;

  char save_path[256]{};

 private:
  const helper::WindowHandler* _window;
  const helper::VulkanContext* _context;
  Settings _default_settings;
  std::string _font_path;
  float _font_size = 16.0f;
  bool _update_font = false;
  float _cam_move_speed = 10.0f;

  bool _show_mod_plot{false};
  bool _show_mod_plot_raw{false};
  ImVec2 _mod_plot_size{200, 50};

  static void check_vk_result(const VkResult err) {
    if (err == VK_SUCCESS) return;
    std::cerr << "[vulkan] Error: VkResult = " << err << std::endl;
    if (err < 0) std::abort();
  }
};

}  // namespace autd3::extra::simulator
