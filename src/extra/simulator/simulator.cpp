// File: simulator.cpp
// Project: simulator
// Created Date: 30/09/2022
// Author: Shun Suzuki
// -----
// Last Modified: 13/10/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#include "autd3/extra/simulator/simulator.hpp"

#include <atomic>
#include <mutex>
#include <queue>
#include <thread>

#if WIN32
#include <WS2tcpip.h>
#else
#include <arpa/inet.h>
#include <netinet/in.h>
#include <sys/ioctl.h>
#include <sys/socket.h>
#include <sys/types.h>
#include <unistd.h>
#endif

#include <vulkan_context.hpp>
#include <window_handler.hpp>

#include "autd3/driver/cpu/datagram.hpp"
#include "autd3/driver/cpu/ec_config.hpp"
#include "autd3/extra/firmware_emulator/cpu/emulator.hpp"
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

namespace autd3::extra::simulator {

#if WIN32
using socklen_t = int;
#endif

void Simulator::run() {
  SoundSources sources;

  std::vector<firmware_emulator::cpu::CPU> cpus;

#if WIN32
  SOCKET sock = {};
#else
  int sock = 0;
#endif
  sockaddr_in addr = {};

#if WIN32
#pragma warning(push)
#pragma warning(disable : 6031)
  WSAData wsa_data{};
  WSAStartup(MAKEWORD(2, 0), &wsa_data);
#pragma warning(pop)
#endif

  sock = socket(AF_INET, SOCK_DGRAM, 0);
#if WIN32
  if (sock == INVALID_SOCKET)
#else
  if (sock < 0)
#endif
    throw std::runtime_error("cannot connect to emulator");

  addr.sin_family = AF_INET;
  addr.sin_port = htons(_settings->port);
#if WIN32
  inet_pton(AF_INET, _settings->ip.c_str(), &addr.sin_addr.S_un.S_addr);
#else
  addr.sin_addr.s_addr = inet_addr(_settings->ip.c_str());
#endif

  if (bind(sock, reinterpret_cast<sockaddr*>(&addr), sizeof addr) != 0)
    throw std::runtime_error("failed to bind socket: " + std::to_string(_settings->port));

  u_long val = 1;
#if WIN32
  ioctlsocket(sock, FIONBIO, &val);
#else
  ioctl(sock, FIONBIO, &val);
#endif

  std::queue<driver::TxDatagram> recv_queue;
  std::mutex recv_mtx;

  std::atomic run_recv = true;
  _th = std::thread([sock, &run_recv, &recv_queue, &recv_mtx] {
    std::vector<char> buf(65536);
    while (run_recv.load()) {
      sockaddr_in addr_in{};
      auto addr_len = static_cast<socklen_t>(sizeof addr_in);
      if (const auto len = recvfrom(sock, buf.data(), 65536, 0, reinterpret_cast<sockaddr*>(&addr_in), &addr_len); len >= 0) {
        const auto recv_len = static_cast<size_t>(len);
        if (recv_len < driver::HEADER_SIZE) {
          std::cerr << "Unknown data size: " << recv_len << std::endl;
          continue;
        }
        const auto body_len = recv_len - driver::HEADER_SIZE;
        if (body_len % driver::BODY_SIZE != 0) {
          std::cerr << "Unknown data size: " << recv_len << std::endl;
          continue;
        }
        const auto dev_num = body_len / driver::BODY_SIZE;
        driver::TxDatagram tx(dev_num);
        tx.num_bodies = dev_num;
        std::memcpy(tx.data().data(), buf.data(), recv_len);
        {
          std::lock_guard lock(recv_mtx);
          recv_queue.push(tx.clone());
        }
      }
    }

#if WIN32
    closesocket(sock);
    WSACleanup();
#else
    ::close(sock);
#endif
  });

  const auto window = std::make_unique<helper::WindowHandler>(_settings->window_width, _settings->window_height);
  const auto context = std::make_unique<helper::VulkanContext>(_settings->gpu_idx, false);

  const auto imgui = std::make_unique<VulkanImGui>(window.get(), context.get());
  const auto renderer = std::make_unique<VulkanRenderer>(context.get(), window.get(), imgui.get(), _settings->vsync);

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
      vk::DescriptorPoolSize(vk::DescriptorType::eStorageBuffer, 100),
  };
  context->create_descriptor_pool(pool_size);

  renderer->create_command_buffers();
  renderer->create_sync_objects();

  const auto trans_viewer = std::make_unique<trans_viewer::TransViewer>(context.get(), renderer.get());
  const auto slice_viewer = std::make_unique<slice_viewer::SliceViewer>(context.get(), renderer.get());
  const auto field_compute = std::make_unique<FieldCompute>(context.get(), renderer.get());

  imgui->init(static_cast<uint32_t>(renderer->frames_in_flight()), renderer->render_pass(), *_settings);

  bool initialized = false;
  bool data_updated = false;
  while (!window->should_close()) {
    helper::WindowHandler::poll_events();
    glfwPollEvents();

    if (!recv_queue.empty()) {
      const auto& tx = recv_queue.front();
      for (auto& cpu : cpus) cpu.send(tx);

      if (initialized) {
        if (tx.header().msg_id == driver::MSG_SIMULATOR_CLOSE) {
          initialized = false;
          cpus.clear();
          sources.clear();
        } else {
          data_updated = true;
        }
      }

      if (tx.header().msg_id == driver::MSG_SIMULATOR_INIT) {
        cpus.clear();
        cpus.reserve(tx.size());
        for (size_t i = 0; i < tx.size(); i++) {
          firmware_emulator::cpu::CPU cpu(i);
          cpu.init();
          cpus.emplace_back(cpu);
        }

        sources.clear();
        for (size_t dev = 0; dev < tx.size(); dev++) {
          const auto* body = reinterpret_cast<const float*>(tx.bodies() + dev);
          const auto origin = glm::vec3(body[0], body[1], body[2]);
          const auto rot = glm::quat(body[3], body[4], body[5], body[6]);
          const auto matrix = translate(glm::identity<glm::mat4>(), origin) * mat4_cast(rot);
          for (size_t iy = 0; iy < driver::NUM_TRANS_Y; iy++)
            for (size_t ix = 0; ix < driver::NUM_TRANS_X; ix++) {
              if (driver::is_missing_transducer(ix, iy)) continue;
              const auto local_pos = glm::vec4(static_cast<float>(ix) * static_cast<float>(driver::TRANS_SPACING_MM),
                                               static_cast<float>(iy) * static_cast<float>(driver::TRANS_SPACING_MM), 0.0f, 1.0f);
              const auto global_pos = matrix * local_pos;
              sources.add(global_pos, rot, Drive(1.0f, 0.0f, 1.0f, 40e3, 340e3), 1.0f);
            }
        }

        imgui->set(sources);
        trans_viewer->init(sources);
        slice_viewer->init(imgui->slice_width, imgui->slice_height, imgui->pixel_size);
        field_compute->init(sources, imgui->slice_alpha, slice_viewer->images(), slice_viewer->image_size());

        initialized = true;
      }

      {
        std::lock_guard lock(recv_mtx);
        recv_queue.pop();
      }
    }

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
            sources.drives()[i].set_wave_num(freq, imgui->sound_speed * 1e3f);
          }
        }
        update_flags.set(UpdateFlags::UPDATE_SOURCE_DRIVE);
        data_updated = false;
      }
      if (is_stm_mode && update_flags.contains(UpdateFlags::UPDATE_SOURCE_DRIVE)) {
        size_t i = 0;
        for (auto& cpu : cpus) {
          const auto& cycles = cpu.fpga().cycles();
          const auto& [amps, phases] = cpu.fpga().drives();
          for (size_t tr = 0; tr < driver::NUM_TRANS_IN_UNIT; tr++, i++) {
            sources.drives()[i].amp = std::sin(glm::pi<float>() * static_cast<float>(amps[imgui->stm_idx][tr].duty) / static_cast<float>(cycles[tr]));
            sources.drives()[i].phase =
                2.0f * glm::pi<float>() * static_cast<float>(phases[imgui->stm_idx][tr].phase) / static_cast<float>(cycles[tr]);
            const auto freq = static_cast<float>(driver::FPGA_CLK_FREQ) / static_cast<float>(cycles[tr]);
            sources.drives()[i].set_wave_num(freq, imgui->sound_speed * 1e3f);
          }
        }
      }

      const auto& [view, proj] = imgui->get_view_proj(static_cast<float>(renderer->extent().width) / static_cast<float>(renderer->extent().height));
      const auto& slice_model = imgui->get_slice_model();
      slice_viewer->update(imgui->slice_width, imgui->slice_height, imgui->pixel_size, update_flags);
      trans_viewer->update(sources, update_flags);
      field_compute->update(sources, imgui->slice_alpha, slice_viewer->images(), slice_viewer->image_size(), update_flags);

      const Config config{static_cast<uint32_t>(sources.size()),
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

        auto [staging_buffer, staging_buffer_memory] = context->create_buffer(
            image_size, vk::BufferUsageFlagBits::eTransferDst, vk::MemoryPropertyFlagBits::eHostVisible | vk::MemoryPropertyFlagBits::eHostCoherent);
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
    } else {
      VulkanImGui::draw();
      const std::array background = {imgui->background.r, imgui->background.g, imgui->background.b, imgui->background.a};
      const auto& [command_buffer, image_index] = renderer->begin_frame(background);
      VulkanImGui::render(command_buffer);
      renderer->end_frame(command_buffer, image_index);
    }
  }

  run_recv.store(false);
  if (_th.joinable()) _th.join();

  context->device().waitIdle();
  VulkanImGui::cleanup();
  renderer->cleanup();

  imgui->save_settings(*_settings);
  const auto [window_width, window_height] = window->get_window_size();
  _settings->window_width = window_width;
  _settings->window_height = window_height;
}
}  // namespace autd3::extra::simulator
