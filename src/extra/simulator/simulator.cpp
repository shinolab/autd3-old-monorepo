// File: simulator.cpp
// Project: simulator
// Created Date: 30/09/2022
// Author: Shun Suzuki
// -----
// Last Modified: 02/11/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#include "autd3/extra/simulator.hpp"

#include <atomic>
#include <mutex>
#include <queue>
#include <smem/smem.hpp>
#include <thread>
#include <vulkan_context.hpp>
#include <window_handler.hpp>

#include "autd3/driver/cpu/datagram.hpp"
#include "autd3/driver/cpu/ec_config.hpp"
#include "autd3/extra/cpu_emulator.hpp"
#include "field_compute.hpp"
#include "slice_viewer.hpp"
#include "sound_sources.hpp"
#include "trans_viewer.hpp"
#include "vulkan_renderer.hpp"

#if _MSC_VER
#pragma warning(push)
#endif
#if defined(__GNUC__) && !defined(__llvm__)
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wmissing-field-initializers"
#endif
#ifdef __clang__
#pragma clang diagnostic push
#pragma clang diagnostic ignored "-Wmissing-field-initializers"
#pragma clang diagnostic ignored "-Wdeprecated-declarations"
#endif
#define STB_IMAGE_WRITE_IMPLEMENTATION
#include "stb_image_write.h"
#if _MSC_VER
#pragma warning(pop)
#endif
#if defined(__GNUC__) && !defined(__llvm__)
#pragma GCC diagnostic pop
#endif
#ifdef __clang__
#pragma clang diagnostic pop
#endif

namespace autd3::extra {

void Simulator::run() {
  simulator::SoundSources sources;

  std::vector<CPU> cpus;

  const auto window = std::make_unique<helper::WindowHandler>(_settings->window_width, _settings->window_height);
  const auto context = std::make_unique<helper::VulkanContext>(_settings->gpu_idx, false);

  const auto imgui = std::make_unique<simulator::VulkanImGui>(window.get(), context.get());
  const auto renderer = std::make_unique<simulator::VulkanRenderer>(context.get(), window.get(), imgui.get(), _settings->vsync);

  window->init("AUTD3 Simulator", renderer.get(), simulator::VulkanRenderer::resize_callback, simulator::VulkanRenderer::pos_callback);
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
      vk::DescriptorPoolSize(vk::DescriptorType::eStorageBuffer, 100),
  };
  context->create_descriptor_pool(pool_size);

  renderer->create_command_buffers();
  renderer->create_sync_objects();

  const auto trans_viewer = std::make_unique<simulator::trans_viewer::TransViewer>(context.get(), renderer.get(), imgui.get());
  const auto slice_viewer = std::make_unique<simulator::slice_viewer::SliceViewer>(context.get(), renderer.get());
  const auto field_compute = std::make_unique<simulator::FieldCompute>(context.get(), renderer.get());

  imgui->init(static_cast<uint32_t>(renderer->frames_in_flight()), renderer->render_pass(), *_settings);

  auto smem = smem::SMem();
  const auto size = driver::HEADER_SIZE + static_cast<size_t>(_settings->max_dev_num) * (driver::BODY_SIZE + driver::EC_INPUT_FRAME_SIZE);
  smem.create("autd3_simulator_smem", size);
  volatile auto* ptr = static_cast<uint8_t*>(smem.map());
  for (size_t i = 0; i < size; i++) ptr[i] = 0;

  bool initialized = false;
  std::atomic run_recv = true;
  std::atomic data_updated = false;
  _th = std::thread([this, ptr, &cpus, &sources, &initialized, &run_recv, &data_updated, &imgui, &trans_viewer, &slice_viewer, &field_compute] {
    while (run_recv.load()) {
      const auto* header = reinterpret_cast<driver::GlobalHeader*>(const_cast<uint8_t*>(ptr));
      if (!initialized) {
        if (header->msg_id != driver::MSG_SIMULATOR_INIT) {
          std::this_thread::sleep_for(std::chrono::milliseconds(100));
          continue;
        }
        const auto dev_num = static_cast<size_t>(header->size);

        cpus.clear();
        cpus.reserve(dev_num);
        for (size_t i = 0; i < dev_num; i++) {
          CPU cpu(i);
          cpu.init();
          cpus.emplace_back(cpu);
        }

        sources.clear();
        for (size_t dev = 0; dev < dev_num; dev++) {
          const auto* body = reinterpret_cast<float*>(const_cast<uint8_t*>(ptr + sizeof(driver::GlobalHeader) + dev * sizeof(driver::Body)));
          const auto origin = imgui->to_gl_pos(glm::vec3(body[0], body[1], body[2])) * imgui->scale();
          const auto rot = imgui->to_gl_rot(glm::quat(body[3], body[4], body[5], body[6]));
          const auto matrix = translate(glm::identity<glm::mat4>(), origin) * mat4_cast(rot);
          for (size_t iy = 0; iy < driver::NUM_TRANS_Y; iy++)
            for (size_t ix = 0; ix < driver::NUM_TRANS_X; ix++) {
              if (driver::is_missing_transducer(ix, iy)) continue;
              const auto local_pos = glm::vec4(static_cast<float>(ix) * static_cast<float>(driver::TRANS_SPACING_MM) * imgui->scale(),
                                               static_cast<float>(iy) * static_cast<float>(driver::TRANS_SPACING_MM) * imgui->scale(), 0.0f, 1.0f);
              const auto global_pos = matrix * local_pos;
              sources.add(global_pos, rot, simulator::Drive(1.0f, 0.0f, 1.0f, 40e3, _settings->use_meter ? 340 : 340e3), 1.0f);
            }
        }

        imgui->set(sources);
        trans_viewer->init(sources);
        slice_viewer->init(imgui->slice_width, imgui->slice_height, imgui->pixel_size);
        field_compute->init(sources, imgui->slice_alpha, slice_viewer->images(), slice_viewer->image_size(), imgui->coloring_method);

        ptr[0] = 0x00;
        initialized = true;
      } else {
        if (header->msg_id == driver::MSG_SIMULATOR_CLOSE) {
          initialized = false;
          cpus.clear();
          sources.clear();
        } else {
          for (size_t i = 0; i < cpus.size(); i++) {
            const auto* body = reinterpret_cast<driver::Body*>(const_cast<uint8_t*>(ptr + sizeof(driver::GlobalHeader) + i * sizeof(driver::Body)));
            cpus[i].send(header, body);
          }
          data_updated.store(true);
          for (size_t i = 0; i < cpus.size(); i++) {
            auto* input = reinterpret_cast<driver::RxMessage*>(
                const_cast<uint8_t*>(ptr + sizeof(driver::GlobalHeader) + cpus.size() * sizeof(driver::Body) + i * driver::EC_INPUT_FRAME_SIZE));
            input->msg_id = cpus[i].msg_id();
            input->ack = cpus[i].ack();
          }
        }
      }
    }
  });

  while (!window->should_close()) {
    helper::WindowHandler::poll_events();
    glfwPollEvents();

    if (initialized) {
      const bool is_stm_mode = std::any_of(cpus.begin(), cpus.end(), [](const auto& cpu) { return cpu.fpga().is_stm_mode(); });
      imgui->is_stm_mode = is_stm_mode;
      if (is_stm_mode) imgui->stm_size = static_cast<int32_t>(cpus[0].fpga().stm_cycle());
      auto update_flags = imgui->draw(cpus, sources);
      if (data_updated) {
        const auto idx = is_stm_mode ? imgui->stm_idx : 0;
        size_t i = 0;
        for (auto& cpu : cpus) {
          const auto& cycles = cpu.fpga().cycles();
          const auto& [amps, phases] = cpu.fpga().drives();
          for (size_t tr = 0; tr < driver::NUM_TRANS_IN_UNIT; tr++, i++) {
            sources.drives()[i].amp = std::sin(glm::pi<float>() * static_cast<float>(amps[idx][tr].duty) / static_cast<float>(cycles[tr]));
            sources.drives()[i].phase = 2.0f * glm::pi<float>() * static_cast<float>(phases[idx][tr].phase) / static_cast<float>(cycles[tr]);
            const auto freq = static_cast<float>(driver::FPGA_CLK_FREQ) / static_cast<float>(cycles[tr]);
            sources.drives()[i].set_wave_num(freq, imgui->sound_speed);
          }
        }
        update_flags.set(simulator::UpdateFlags::UPDATE_SOURCE_DRIVE);
        data_updated = false;
      }
      if (is_stm_mode && update_flags.contains(simulator::UpdateFlags::UPDATE_SOURCE_DRIVE)) {
        size_t i = 0;
        for (auto& cpu : cpus) {
          const auto& cycles = cpu.fpga().cycles();
          const auto& [amps, phases] = cpu.fpga().drives();
          for (size_t tr = 0; tr < driver::NUM_TRANS_IN_UNIT; tr++, i++) {
            sources.drives()[i].amp = std::sin(glm::pi<float>() * static_cast<float>(amps[imgui->stm_idx][tr].duty) / static_cast<float>(cycles[tr]));
            sources.drives()[i].phase =
                2.0f * glm::pi<float>() * static_cast<float>(phases[imgui->stm_idx][tr].phase) / static_cast<float>(cycles[tr]);
            const auto freq = static_cast<float>(driver::FPGA_CLK_FREQ) / static_cast<float>(cycles[tr]);
            sources.drives()[i].set_wave_num(freq, imgui->sound_speed);
          }
        }
      }

      const auto& [view, proj] = imgui->get_view_proj(static_cast<float>(renderer->extent().width) / static_cast<float>(renderer->extent().height));
      const auto& slice_model = imgui->get_slice_model();
      slice_viewer->update(imgui->slice_width, imgui->slice_height, imgui->pixel_size, update_flags);
      trans_viewer->update(sources, update_flags);
      field_compute->update(sources, imgui->slice_alpha, slice_viewer->images(), slice_viewer->image_size(), imgui->coloring_method, update_flags);

      const simulator::Config config{static_cast<uint32_t>(sources.size()),
                                     0,
                                     imgui->color_scale,
                                     static_cast<uint32_t>(imgui->slice_width / imgui->pixel_size),
                                     static_cast<uint32_t>(imgui->slice_height / imgui->pixel_size),
                                     imgui->pixel_size,
                                     imgui->scale(),
                                     0,
                                     slice_model};
      field_compute->compute(config, imgui->show_radiation_pressure);

      if (update_flags.contains(simulator::UpdateFlags::SAVE_IMAGE)) {
        const auto& image = slice_viewer->images()[renderer->current_frame()].get();
        const auto image_size = slice_viewer->image_size();

        auto [staging_buffer, staging_buffer_memory] = context->create_buffer(
            image_size, vk::BufferUsageFlagBits::eTransferDst, vk::MemoryPropertyFlagBits::eHostVisible | vk::MemoryPropertyFlagBits::eHostCoherent);
        context->copy_buffer(image, staging_buffer.get(), image_size);
        void* data;
        if (context->device().mapMemory(staging_buffer_memory.get(), 0, image_size, {}, &data) != vk::Result::eSuccess)
          throw std::runtime_error("failed to map texture buffer.");
        const auto* image_data = static_cast<float*>(data);
        std::vector<uint8_t> pixels;
        const auto image_width = static_cast<int32_t>(imgui->slice_width / imgui->pixel_size);
        const auto image_height = static_cast<int32_t>(imgui->slice_height / imgui->pixel_size);
        pixels.reserve(static_cast<size_t>(image_width) * static_cast<size_t>(image_height) * 4);
        for (int32_t i = image_height - 1; i >= 0; i--) {
          for (int32_t j = 0; j < image_width; j++) {
            const auto idx = image_width * static_cast<size_t>(i) + static_cast<size_t>(j);
            pixels.emplace_back(static_cast<uint8_t>(255.0f * image_data[4 * idx]));
            pixels.emplace_back(static_cast<uint8_t>(255.0f * image_data[4 * idx + 1]));
            pixels.emplace_back(static_cast<uint8_t>(255.0f * image_data[4 * idx + 2]));
            pixels.emplace_back(static_cast<uint8_t>(255.0f * image_data[4 * idx + 3]));
          }
        }
        stbi_write_png(imgui->save_path, image_width, image_height, 4, pixels.data(), image_width * 4);
        context->device().unmapMemory(staging_buffer_memory.get());
      }

      const std::array background = {imgui->background.r, imgui->background.g, imgui->background.b, imgui->background.a};
      const auto& [command_buffer, image_index] = renderer->begin_frame(background);

      slice_viewer->render(slice_model, view, proj, command_buffer);
      trans_viewer->render(view, proj, command_buffer);
      simulator::VulkanImGui::render(command_buffer);
      renderer->end_frame(command_buffer, image_index);
    } else {
      simulator::VulkanImGui::draw();
      const std::array background = {imgui->background.r, imgui->background.g, imgui->background.b, imgui->background.a};
      const auto& [command_buffer, image_index] = renderer->begin_frame(background);
      simulator::VulkanImGui::render(command_buffer);
      renderer->end_frame(command_buffer, image_index);
    }
  }

  run_recv.store(false);
  if (_th.joinable()) _th.join();

  context->device().waitIdle();
  simulator::VulkanImGui::cleanup();
  renderer->cleanup();

  imgui->save_settings(*_settings);
  const auto [window_width, window_height] = window->get_window_size();
  _settings->window_width = window_width;
  _settings->window_height = window_height;
}
}  // namespace autd3::extra
