// File: simulator.cpp
// Project: simulator
// Created Date: 30/09/2022
// Author: Shun Suzuki
// -----
// Last Modified: 28/04/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#include "autd3/extra/simulator.hpp"

#ifdef WIN32
#include <SDKDDKVer.h>
#endif

#include <atomic>
#include <boost/asio.hpp>
#include <boost/interprocess/managed_shared_memory.hpp>
#include <boost/interprocess/sync/interprocess_mutex.hpp>
#include <mutex>
#include <numeric>
#include <thread>

#include "autd3/driver/cpu/datagram.hpp"
#include "autd3/driver/cpu/ec_config.hpp"
#include "autd3/extra/cpu_emulator.hpp"
#include "field_compute.hpp"
#include "slice_viewer.hpp"
#include "sound_sources.hpp"
#include "trans_viewer.hpp"
#include "vulkan_context.hpp"
#include "vulkan_renderer.hpp"
#include "window_handler.hpp"

#define STB_IMAGE_WRITE_IMPLEMENTATION
#include <autd3/autd3_device.hpp>

#include "stb_image_write.h"

namespace autd3::extra {

static constexpr std::string_view SHMEM_NAME{"autd3_simulator_shmem"};
static constexpr std::string_view SHMEM_MTX_NAME{"autd3_simulator_shmem_mtx"};
static constexpr std::string_view SHMEM_DATA_NAME{"autd3_simulator_shmem_ptr"};

constexpr size_t BUF_SIZE = 65536;

class Server {
 public:
  Server(boost::asio::io_context& io_context, const uint16_t port, std::vector<CPU>& cpus, std::vector<simulator::SoundSources>& sources,
         std::atomic<bool>& initialized, std::atomic<bool>& data_updated, std::atomic<bool>& do_init, std::atomic<bool>& do_close)
      : _acceptor(io_context, boost::asio::ip::tcp::endpoint(boost::asio::ip::tcp::v4(), port)),
        _cpus(cpus),
        _sources(sources),
        _initialized(initialized),
        _data_updated(data_updated),
        _do_init(do_init),
        _do_close(do_close) {
    spdlog::info("Waiting for client connection...");
    do_accept();
    io_context.run();
  }

 private:
  void do_accept() {
    _acceptor.async_accept([this](const boost::system::error_code ec, boost::asio::ip::tcp::socket socket) {
      if (ec) {
        spdlog::error("Accept error: {}", ec.message());
      } else {
        _socket = std::make_shared<boost::asio::ip::tcp::socket>(std::move(socket));
        do_read();
        do_accept();
      }
    });
  }

  void do_read() {
    _socket->async_read_some(boost::asio::buffer(_data), [this](boost::system::error_code ec, std::size_t len) {
      if (ec == boost::asio::error::eof || ec == boost::asio::error::connection_reset || ec == boost::asio::error::connection_aborted) {
        return;
      }
      if (ec) {
        spdlog::error("receive error: {}", ec.message());
        return;
      }
      auto* cursor = _data;
      const auto* header = reinterpret_cast<driver::GlobalHeader*>(cursor);
      if (!_initialized.load()) {
        if (header->msg_id == driver::MSG_SIMULATOR_INIT) {
          spdlog::info("Client connected");
          cursor++;

          const auto dev_num = *reinterpret_cast<uint32_t*>(cursor);
          cursor += sizeof(uint32_t);

          spdlog::info("Open simulator with {} devices", dev_num);

          _cpus.clear();
          _cpus.reserve(dev_num);

          _sources.clear();

          for (uint32_t dev = 0; dev < dev_num; dev++) {
            CPU cpu(dev, AUTD3::NUM_TRANS_IN_UNIT);
            cpu.init();
            _cpus.emplace_back(cpu);

            auto* p = reinterpret_cast<float*>(cursor);
            const auto transducers = AUTD3(Eigen::Vector3<float>(p[0], p[1], p[2]).cast<driver::float_t>(),
                                           Eigen::Quaternion<float>(p[3], p[4], p[5], p[6]).cast<driver::float_t>())
                                         .get_transducers(0);
            simulator::SoundSources s;
            for (const auto& tr : transducers) {
              const Eigen::Vector3<float> pos_f = tr.position().cast<float>();
              const Eigen::Quaternion<float> rot_f = tr.rotation().cast<float>();
              const auto pos = simulator::VulkanImGui::to_gl_pos(glm::vec3(pos_f.x(), pos_f.y(), pos_f.z()));
              const auto rot = simulator::VulkanImGui::to_gl_rot(glm::quat(rot_f.w(), rot_f.x(), rot_f.y(), rot_f.z()));
              s.add(pos, rot, simulator::Drive(1.0f, 0.0f, 1.0f, 40e3, 340e3f * simulator::scale), 1.0f);
            }
            cursor += sizeof(float) * 7;
            _sources.emplace_back(std::move(s));
          }

          _do_init.store(true);
          while (!_initialized.load()) std::this_thread::sleep_for(std::chrono::milliseconds(100));

          _send_buf[0] = driver::MSG_SIMULATOR_INIT;

          boost::system::error_code error;
          write(*_socket, boost::asio::buffer(_send_buf, 1), error);
          if (error) {
            spdlog::error("send error: {}", error.message());
            return;
          }
        }
      } else {
        if (header->msg_id == driver::MSG_SIMULATOR_CLOSE) {
          spdlog::info("Client disconnected");
          _do_close.store(true);
          std::memset(_data, 0, BUF_SIZE);
          while (_do_close.load()) std::this_thread::sleep_for(std::chrono::milliseconds(100));
          spdlog::info("Waiting for client connection...");
          return;
        }
        size_t c = 0;
        for (size_t i = 0; i < _cpus.size(); i++) {
          const auto* body = reinterpret_cast<driver::Body*>(_data + sizeof(driver::GlobalHeader) + c);
          _cpus[i].send(header, body);
          c += _sources[i].size() * sizeof(uint16_t);
        }
        for (size_t i = 0; i < _cpus.size(); i++) {
          auto* input = reinterpret_cast<driver::RxMessage*>(_send_buf + i * sizeof(driver::RxMessage));
          input->msg_id = _cpus[i].msg_id();
          input->ack = _cpus[i].ack();
        }
        async_write(*_socket, boost::asio::buffer(_send_buf, _cpus.size() * sizeof(driver::RxMessage)),
                    [this](const boost::system::error_code error, std::size_t) {
                      if (error == boost::asio::error::eof || error == boost::asio::error::connection_reset ||
                          error == boost::asio::error::connection_aborted) {
                        return;
                      }
                      if (error) spdlog::error("send error: {}", error.message());
                    });
        _data_updated.store(true);
      }
      do_read();
    });
  }

  boost::asio::ip::tcp::acceptor _acceptor;
  std::shared_ptr<boost::asio::ip::tcp::socket> _socket{nullptr};

  std::vector<CPU>& _cpus;
  std::vector<simulator::SoundSources>& _sources;
  std::atomic<bool>& _initialized;
  std::atomic<bool>& _data_updated;
  std::atomic<bool>& _do_init;
  std::atomic<bool>& _do_close;

  uint8_t _data[BUF_SIZE]{};
  uint8_t _send_buf[BUF_SIZE]{};
};

void Simulator::run() {
  std::vector<simulator::SoundSources> sources;

  std::vector<CPU> cpus;

  const auto window = std::make_unique<helper::WindowHandler>(_settings.window_width, _settings.window_height);
  const auto context = std::make_unique<helper::VulkanContext>(_settings.gpu_idx, false);

  const auto imgui = std::make_unique<simulator::VulkanImGui>(window.get(), context.get());
  const auto renderer = std::make_unique<simulator::VulkanRenderer>(context.get(), window.get(), imgui.get(), _settings.vsync);

  spdlog::info("Initializing window...");
  window->init("AUTD3 Simulator", renderer.get(), simulator::VulkanRenderer::resize_callback, simulator::VulkanRenderer::pos_callback);
  spdlog::info("Initializing vulkan...");
  context->init_vulkan("AUTD3 Simulator", *window);
  spdlog::info("Initializing renderer...");
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

  const auto trans_viewer = std::make_unique<simulator::trans_viewer::TransViewer>(context.get(), renderer.get());
  const auto slice_viewer = std::make_unique<simulator::slice_viewer::SliceViewer>(context.get(), renderer.get());
  const auto field_compute = std::make_unique<simulator::FieldCompute>(context.get(), renderer.get());

  imgui->init(static_cast<uint32_t>(renderer->frames_in_flight()), renderer->render_pass(), _settings);

  std::atomic initialized = false;
  std::atomic do_init = false;
  std::atomic do_close = false;
  std::atomic data_updated = false;
  std::atomic run_recv = true;

  if (_settings.remote)
    std::thread([this, &cpus, &sources, &initialized, &data_updated, &do_init, &do_close]() {
      boost::asio::io_context io_context;
      Server s(io_context, _settings.remote_port, cpus, sources, initialized, data_updated, do_init, do_close);
    }).detach();
  else
    std::thread([this, &cpus, &sources, &initialized, &run_recv, &data_updated, &do_init, &do_close] {
      spdlog::info("Initializing shared memory...");
      const auto size = sizeof(uint8_t) + sizeof(uint32_t) + sizeof(uint32_t) * _settings.max_dev_num + _settings.max_trans_num * sizeof(float) * 7;
      boost::interprocess::shared_memory_object::remove(std::string(SHMEM_NAME).c_str());
      boost::interprocess::managed_shared_memory segment(boost::interprocess::create_only, std::string(SHMEM_NAME).c_str(),
                                                         size + sizeof(boost::interprocess::interprocess_mutex) + 1024);
      auto* mtx = segment.construct<boost::interprocess::interprocess_mutex>(std::string(SHMEM_MTX_NAME).c_str())();
      volatile auto* ptr = segment.construct<uint8_t>(std::string(SHMEM_DATA_NAME).c_str())[size](0x00);

      spdlog::info("Initializing shared memory...done");

      uint8_t last_msg_id = 0;
      while (run_recv.load()) {
        auto* cursor = const_cast<uint8_t*>(ptr);
        const auto* header = reinterpret_cast<driver::GlobalHeader*>(cursor);
        if (!initialized.load()) {
          if (do_init.load() || header->msg_id != driver::MSG_SIMULATOR_INIT) {
            std::this_thread::sleep_for(std::chrono::milliseconds(100));
            continue;
          }

          {
            std::unique_lock lk(*mtx);

            spdlog::info("Client connected");
            cursor++;

            const auto dev_num = *reinterpret_cast<uint32_t*>(cursor);
            cursor += sizeof(uint32_t);

            spdlog::info("Open simulator with {} devices", dev_num);

            cpus.clear();
            cpus.reserve(dev_num);

            sources.clear();

            for (uint32_t dev = 0; dev < dev_num; dev++) {
              const auto tr_num = *reinterpret_cast<uint32_t*>(cursor);
              cursor += sizeof(uint32_t);

              spdlog::info("Add {}-th device with {} transducers", dev, tr_num);

              CPU cpu(dev, tr_num);
              cpu.init();
              std::vector<driver::Vector3> local_trans_pos;
              local_trans_pos.reserve(tr_num);
              auto* p = reinterpret_cast<float*>(cursor);
              const driver::Vector3 origin = Eigen::Vector3<float>(p[0], p[1], p[2]).cast<driver::float_t>();
              for (uint32_t tr = 0; tr < tr_num; tr++) {
                const driver::Vector3 pos = Eigen::Vector3<float>(p[0], p[1], p[2]).cast<driver::float_t>() - origin;
                local_trans_pos.emplace_back(pos);
                p += 7;
              }
              cpu.configure_local_trans_pos(local_trans_pos);
              cpus.emplace_back(cpu);

              simulator::SoundSources s;
              p = reinterpret_cast<float*>(cursor);
              for (uint32_t tr = 0; tr < tr_num; tr++) {
                const auto pos = simulator::VulkanImGui::to_gl_pos(glm::vec3(p[0], p[1], p[2]));
                const auto rot = simulator::VulkanImGui::to_gl_rot(glm::quat(p[3], p[4], p[5], p[6]));
                s.add(pos, rot, simulator::Drive(1.0f, 0.0f, 1.0f, 40e3, 340e3f * simulator::scale), 1.0f);
                p += 7;
                cursor += sizeof(float) * 7;
              }
              sources.emplace_back(std::move(s));
            }
          }

          do_init.store(true);
          while (do_init.load()) std::this_thread::sleep_for(std::chrono::milliseconds(100));
          ptr[0] = 0x00;
        } else {
          if (header->msg_id == driver::MSG_SIMULATOR_CLOSE) {
            spdlog::info("Client disconnected");
            do_close.store(true);
            while (do_close.load()) std::this_thread::sleep_for(std::chrono::milliseconds(100));
            for (size_t i = 0; i < size; i++) ptr[i] = 0;

            spdlog::info("Waiting for client connection...");
          } else {
            {
              std::unique_lock lk(*mtx);
              size_t c = 0;
              for (size_t i = 0; i < cpus.size(); i++) {
                const auto* body = reinterpret_cast<driver::Body*>(const_cast<uint8_t*>(ptr) + sizeof(driver::GlobalHeader) + c);
                cpus[i].send(header, body);
                c += sources[i].size() * sizeof(uint16_t);
              }
              for (size_t i = 0; i < cpus.size(); i++) {
                auto* input = reinterpret_cast<driver::RxMessage*>(
                    const_cast<uint8_t*>(ptr + sizeof(driver::GlobalHeader) + c + i * driver::EC_INPUT_FRAME_SIZE));
                input->msg_id = cpus[i].msg_id();
                input->ack = cpus[i].ack();
              }
            }
            if (last_msg_id != header->msg_id) {
              last_msg_id = header->msg_id;
              data_updated.store(true);
            }
          }
        }
      }
    }).detach();

  while (!window->should_close()) {
    helper::WindowHandler::poll_events();
    glfwPollEvents();

    if (initialized.load()) {
      if (do_close.load()) {
        cpus.clear();
        sources.clear();
        initialized.store(false);
        do_close.store(false);
      } else {
        auto update_flags = imgui->draw(cpus, sources);
        if (data_updated.load()) {
          data_updated.store(false);
          update_flags.set(simulator::UpdateFlags::UpdateSourceDrive);
        }
        if (update_flags.contains(simulator::UpdateFlags::UpdateSourceDrive)) {
          for (size_t dev = 0; dev < cpus.size(); dev++) {
            const auto& cpu = cpus[dev];
            const auto& cycles = cpu.fpga().cycles();
            const auto& [amps, phases] = cpu.fpga().drives(imgui->stm_idx);
            const auto m = imgui->mod_enable ? static_cast<float>(cpu.fpga().modulation(static_cast<size_t>(imgui->mod_idx))) / 255.0f : 1.0f;
            for (size_t tr = 0; tr < sources[dev].size(); tr++) {
              sources[dev].drives()[tr].amp = std::sin(glm::pi<float>() * static_cast<float>(amps[tr]) * m / static_cast<float>(cycles[tr]));
              sources[dev].drives()[tr].phase = 2.0f * glm::pi<float>() * static_cast<float>(phases[tr]) / static_cast<float>(cycles[tr]);
              const auto freq = static_cast<float>(driver::FPGA_CLK_FREQ) / static_cast<float>(cycles[tr]);
              sources[dev].drives()[tr].set_wave_num(freq, imgui->sound_speed);
            }
          }
        }

        const auto& [view, proj] = imgui->get_view_proj(static_cast<float>(renderer->extent().width) / static_cast<float>(renderer->extent().height));
        const auto& slice_model = imgui->get_slice_model();
        slice_viewer->update(imgui->slice_width, imgui->slice_height, imgui->pixel_size, update_flags);
        trans_viewer->update(sources, update_flags);
        field_compute->update(sources, imgui->slice_alpha, slice_viewer->images(), slice_viewer->image_size(), imgui->coloring_method, update_flags);

        if (update_flags.contains(simulator::UpdateFlags::UpdateDeviceInfo))
          for (size_t dev = 0; dev < cpus.size(); dev++) {
            auto& cpu = cpus[dev];
            auto& fpga = cpu.fpga();
            if (imgui->thermal_sensor[dev])
              fpga.assert_thermal_sensor();
            else
              fpga.deassert_thermal_sensor();
            cpu.update();
          }

        const simulator::Config config{static_cast<uint32_t>(std::accumulate(sources.begin(), sources.end(), size_t{0},
                                                                             [](const size_t acc, const auto& s) { return acc + s.size(); })),
                                       0,
                                       imgui->color_scale,
                                       static_cast<uint32_t>(imgui->slice_width / imgui->pixel_size),
                                       static_cast<uint32_t>(imgui->slice_height / imgui->pixel_size),
                                       imgui->pixel_size,
                                       simulator::scale,
                                       0,
                                       slice_model};
        field_compute->compute(config, imgui->show_radiation_pressure);

        if (update_flags.contains(simulator::UpdateFlags::SaveImage)) {
          const auto& image = slice_viewer->images()[renderer->current_frame()].get();
          const auto image_size = slice_viewer->image_size();

          auto [staging_buffer, staging_buffer_memory] =
              context->create_buffer(image_size, vk::BufferUsageFlagBits::eTransferDst,
                                     vk::MemoryPropertyFlagBits::eHostVisible | vk::MemoryPropertyFlagBits::eHostCoherent);

          context->copy_buffer(image, staging_buffer.get(), image_size);
          void* data;
          if (context->device().mapMemory(staging_buffer_memory.get(), 0, image_size, {}, &data) != vk::Result::eSuccess)
            throw std::runtime_error("Failed to map texture buffer.");

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
        if (!command_buffer) continue;
        slice_viewer->render(slice_model, view, proj, command_buffer);
        trans_viewer->render(view, proj, command_buffer);
        simulator::VulkanImGui::render(command_buffer);
        renderer->end_frame(command_buffer, image_index);
      }
    } else {
      if (do_init.load()) {
        imgui->set(sources);
        trans_viewer->init(sources);
        slice_viewer->init(imgui->slice_width, imgui->slice_height, imgui->pixel_size);
        field_compute->init(sources, imgui->slice_alpha, slice_viewer->images(), slice_viewer->image_size(), imgui->coloring_method);
        initialized.store(true);
        do_init.store(false);
      } else {
        simulator::VulkanImGui::draw();
        const std::array background = {imgui->background.r, imgui->background.g, imgui->background.b, imgui->background.a};
        const auto& [command_buffer, image_index] = renderer->begin_frame(background);
        if (!command_buffer) continue;
        simulator::VulkanImGui::render(command_buffer);
        renderer->end_frame(command_buffer, image_index);
      }
    }
  }

  run_recv.store(false);

  context->device().waitIdle();
  simulator::VulkanImGui::cleanup();
  renderer->cleanup();

  imgui->save_settings(_settings);
  const auto [window_width, window_height] = window->get_window_size();
  _settings.window_width = window_width;
  _settings.window_height = window_height;

  boost::interprocess::shared_memory_object::remove(std::string(SHMEM_NAME).c_str());
}
}  // namespace autd3::extra
