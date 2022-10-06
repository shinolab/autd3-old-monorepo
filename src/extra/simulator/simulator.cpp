// File: simulator.cpp
// Project: simulator
// Created Date: 30/09/2022
// Author: Shun Suzuki
// -----
// Last Modified: 06/10/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#include "autd3/extra/simulator/simulator.hpp"

#include <thread>
#include <vulkan_context.hpp>
#include <window_handler.hpp>

#include "autd3/extra/firmware_emulator/cpu/emulator.hpp"
#include "field_compute.hpp"
#include "slice_viewer.hpp"
#include "sound_sources.hpp"
#include "trans_viewer.hpp"
#include "vulkan_renderer.hpp"

#define STB_IMAGE_WRITE_IMPLEMENTATION
#include "stb_image_write.h"

namespace autd3::extra::simulator {

class SimulatorImpl final : public Simulator {
 public:
  SimulatorImpl(Settings setting, std::function<void(Settings)> callback) noexcept
      : _settings(std::move(setting)), _callback(std::move(callback)), _sources(std::make_unique<SoundSources>()) {}
  ~SimulatorImpl() override = default;
  SimulatorImpl(const SimulatorImpl& v) noexcept = delete;
  SimulatorImpl& operator=(const SimulatorImpl& obj) = delete;
  SimulatorImpl(SimulatorImpl&& obj) = delete;
  SimulatorImpl& operator=(SimulatorImpl&& obj) = delete;

  void start(const core::Geometry& geometry) override {
    _sources->clear();
    for (const auto& dev : geometry)
      for (const auto& trans : dev) {
        _sources->add(
            glm::vec3(static_cast<float>(trans.position().x()), static_cast<float>(trans.position().y()), static_cast<float>(trans.position().z())),
            glm::quat(static_cast<float>(dev.rotation().w()), static_cast<float>(dev.rotation().x()), static_cast<float>(dev.rotation().y()),
                      static_cast<float>(dev.rotation().z())),
            Drive{1.0f, 0.0f, 1.0f, static_cast<float>(trans.frequency()), static_cast<float>(geometry.sound_speed * 1e3)}, 1.0f);
      }

    _cpus.clear();
    _cpus.reserve(geometry.num_devices());
    for (size_t i = 0; i < geometry.num_devices(); i++) {
      firmware_emulator::cpu::CPU cpu(i);
      cpu.init();
      _cpus.emplace_back(cpu);
    }

    _th = std::make_unique<std::thread>([this] {
      const auto window = std::make_unique<helper::WindowHandler>(_settings.window_width, _settings.window_height);
      const auto context = std::make_unique<helper::VulkanContext>(_settings.gpu_idx, true);

      const auto imgui = std::make_unique<VulkanImGui>(window.get(), context.get());
      const auto renderer = std::make_unique<VulkanRenderer>(context.get(), window.get(), imgui.get(), _settings.vsync);

      window->init("AUTD3 Simulator", renderer.get(), VulkanRenderer::resize_callback, VulkanRenderer::pos_callback);
      context->init_vulkan("AUTD3 Simulator", *window);
      renderer->create_swapchain();
      renderer->create_image_views();
      renderer->create_render_pass();

      context->create_command_pool();
      renderer->create_depth_resources();
      renderer->create_color_resources();
      renderer->create_framebuffers();

      const std::array pool_size = {
          vk::DescriptorPoolSize(vk::DescriptorType::eCombinedImageSampler, 100),
          vk::DescriptorPoolSize(vk::DescriptorType::eSampledImage, 100),
          vk::DescriptorPoolSize(vk::DescriptorType::eUniformBuffer, 100),
      };
      context->create_descriptor_pool(pool_size);

      renderer->create_command_buffers();
      renderer->create_sync_objects();

      const auto trans_viewer = std::make_unique<trans_viewer::TransViewer>(context.get(), renderer.get());
      const auto slice_viewer = std::make_unique<slice_viewer::SliceViewer>(context.get(), renderer.get());
      const auto field_compute = std::make_unique<FieldCompute>(context.get(), renderer.get());

      // init
      {
        imgui->init(static_cast<uint32_t>(renderer->frames_in_flight()), renderer->render_pass(), _settings, _sources);
        trans_viewer->init(_sources);
        slice_viewer->init(imgui->slice_width, imgui->slice_height, imgui->pixel_size);
        field_compute->init(_sources, imgui->slice_alpha, slice_viewer->images(), slice_viewer->image_size());
      }

      _is_running = true;
      while (_is_running && !window->should_close()) {
        helper::WindowHandler::poll_events();
        glfwPollEvents();

        const bool is_stm_mode = std::any_of(_cpus.begin(), _cpus.end(), [](const auto& cpu) { return cpu.fpga().is_stm_mode(); });
        imgui->is_stm_mode = is_stm_mode;
        if (is_stm_mode) imgui->stm_size = static_cast<int32_t>(_cpus[0].fpga().stm_cycle());
        auto update_flags = imgui->draw(_cpus, _sources);
        if (_data_updated.load()) {
          const auto idx = is_stm_mode ? imgui->stm_idx : 0;
          size_t i = 0;
          for (auto& cpu : _cpus) {
            const auto& cycles = cpu.fpga().cycles();
            const auto& [amps, phases] = cpu.fpga().drives();
            for (size_t tr = 0; tr < driver::NUM_TRANS_IN_UNIT; tr++, i++) {
              _sources->drives()[i].amp = std::sin(glm::pi<float>() * static_cast<float>(amps[idx][tr].duty) / static_cast<float>(cycles[tr]));
              _sources->drives()[i].phase = 2.0f * glm::pi<float>() * static_cast<float>(phases[idx][tr].phase) / static_cast<float>(cycles[tr]);
            }
            update_flags.set(UpdateFlags::UPDATE_SOURCE_DRIVE);
          }
          _data_updated.store(false);
        }
        if (is_stm_mode && update_flags.contains(UpdateFlags::UPDATE_SOURCE_DRIVE)) {
          size_t i = 0;
          for (auto& cpu : _cpus) {
            const auto& cycles = cpu.fpga().cycles();
            const auto& [amps, phases] = cpu.fpga().drives();
            for (size_t tr = 0; tr < driver::NUM_TRANS_IN_UNIT; tr++, i++) {
              _sources->drives()[i].amp =
                  std::sin(glm::pi<float>() * static_cast<float>(amps[imgui->stm_idx][tr].duty) / static_cast<float>(cycles[tr]));
              _sources->drives()[i].phase =
                  2.0f * glm::pi<float>() * static_cast<float>(phases[imgui->stm_idx][tr].phase) / static_cast<float>(cycles[tr]);
            }
          }
        }

        const auto& [view, proj] = imgui->get_view_proj(static_cast<float>(renderer->extent().width) / static_cast<float>(renderer->extent().height));
        const auto& slice_model = imgui->get_slice_model();
        slice_viewer->update(imgui->slice_width, imgui->slice_height, imgui->pixel_size, update_flags);
        trans_viewer->update(_sources, update_flags);
        field_compute->update(_sources, imgui->slice_alpha, slice_viewer->images(), slice_viewer->image_size(), update_flags);

        const Config config{static_cast<uint32_t>(_sources->size()),
                            0,
                            imgui->color_scale,
                            static_cast<uint32_t>(imgui->slice_width / imgui->pixel_size),
                            static_cast<uint32_t>(imgui->slice_height / imgui->pixel_size),
                            static_cast<uint32_t>(imgui->pixel_size),
                            0,
                            0,
                            slice_model};
        field_compute->compute(config);

        if (update_flags.contains(UpdateFlags::SAVE_IMAGE)) {
          const auto& image = slice_viewer->images()[renderer->current_frame()].get();
          const auto image_size = slice_viewer->image_size();

          auto [staging_buffer, staging_buffer_memory] =
              context->create_buffer(image_size, vk::BufferUsageFlagBits::eTransferDst,
                                     vk::MemoryPropertyFlagBits::eHostVisible | vk::MemoryPropertyFlagBits::eHostCoherent);
          context->copy_buffer(image, staging_buffer.get(), image_size);
          void* data;
          if (context->device().mapMemory(staging_buffer_memory.get(), 0, image_size, {}, &data) != vk::Result::eSuccess)
            throw std::runtime_error("failed to map texture buffer.");
          const auto* image_data = static_cast<float*>(data);
          std::vector<uint8_t> pixels;
          pixels.reserve(static_cast<size_t>(imgui->slice_width) * static_cast<size_t>(imgui->slice_height) * 4);
          for (int32_t i = imgui->slice_height - 1; i >= 0; i--) {
            for (int32_t j = 0; j < imgui->slice_width; j++) {
              const auto idx = static_cast<size_t>(imgui->slice_width) * static_cast<size_t>(i) + static_cast<size_t>(j);
              pixels.emplace_back(static_cast<uint8_t>(255.0f * image_data[4 * idx]));
              pixels.emplace_back(static_cast<uint8_t>(255.0f * image_data[4 * idx + 1]));
              pixels.emplace_back(static_cast<uint8_t>(255.0f * image_data[4 * idx + 2]));
              pixels.emplace_back(static_cast<uint8_t>(255.0f * image_data[4 * idx + 3]));
            }
          }
          stbi_write_png(imgui->save_path, imgui->slice_width, imgui->slice_height, 4, pixels.data(), imgui->slice_width * 4);
          context->device().unmapMemory(staging_buffer_memory.get());
        }

        const std::array background = {imgui->background.r, imgui->background.g, imgui->background.b, imgui->background.a};
        const auto& [command_buffer, image_index] = renderer->begin_frame(background);

        slice_viewer->render(slice_model, view, proj, command_buffer);
        trans_viewer->render(view, proj, command_buffer);
        VulkanImGui::render(command_buffer);
        renderer->end_frame(command_buffer, image_index);
      }

      context->device().waitIdle();
      VulkanImGui::cleanup();
      renderer->cleanup();

      imgui->save_settings(_settings);
      const auto [window_width, window_height] = window->get_window_size();
      _settings.window_width = window_width;
      _settings.window_height = window_height;

      if (_callback != nullptr) _callback(_settings);
    });
  }

  void exit() override {
    _is_running = false;
    if (_th->joinable()) _th->join();
  }

  bool receive(driver::RxDatagram& rx) override {
    for (size_t i = 0; i < _cpus.size(); i++) {
      rx.messages()[i].msg_id = _cpus[i].msg_id();
      rx.messages()[i].ack = _cpus[i].ack();
    }
    return true;
  }

  bool send(const driver::TxDatagram& tx) override {
    for (size_t i = 0; i < _cpus.size(); i++) _cpus[i].send(tx.header(), tx.bodies()[i]);
    _data_updated.store(true);
    return true;
  }

 private:
  Settings _settings;
  std::function<void(Settings)> _callback;

  bool _is_running = false;
  std::unique_ptr<std::thread> _th;

  std::unique_ptr<SoundSources> _sources;
  std::vector<firmware_emulator::cpu::CPU> _cpus;

  std::atomic<bool> _data_updated{false};
};

std::unique_ptr<Simulator> Simulator::create(Settings setting, std::function<void(Settings)> callback) {
  return std::make_unique<SimulatorImpl>(std::move(setting), std::move(callback));
}

}  // namespace autd3::extra::simulator
