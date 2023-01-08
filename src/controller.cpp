// File: controller.cpp
// Project: src
// Created Date: 16/11/2022
// Author: Shun Suzuki
// -----
// Last Modified: 08/01/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#include "autd3/controller.hpp"

#include <atomic>

#include "autd3/core/datagram.hpp"
#include "autd3/core/mode.hpp"
#include "autd3/driver/operation/info.hpp"
#include "spdlog.hpp"

namespace autd3 {
Controller::Controller() : _mode(core::Mode::Legacy), _tx_buf({0}), _rx_buf(0), _link(nullptr), _send_th_running(false), _last_send_res(false) {}

Controller::~Controller() noexcept {
  try {
    close();
  } catch (std::exception&) {
  }
}

core::Geometry& Controller::geometry() noexcept { return _geometry; }

const core::Geometry& Controller::geometry() const noexcept { return _geometry; }

bool Controller::open(core::LinkPtr link) {
  if (_geometry.num_transducers() == 0) {
    spdlog::error("Please add devices before opening.");
    return false;
  }

  spdlog::debug("Open Controller with {} transducers.", _geometry.num_transducers());

  if (link == nullptr) {
    spdlog::error("link is null");
    return false;
  }
  _link = std::move(link);
  if (!_link->open(_geometry)) {
    spdlog::error("Failed to open link.");
    return false;
  }

  _tx_buf = driver::TxDatagram(_geometry.device_map());
  _rx_buf = driver::RxDatagram(_geometry.num_devices());

  _send_th_running = true;
  _send_th = std::thread([this] {
    AsyncData data{};
    while (_send_th_running) {
      if (data.header == nullptr && data.body == nullptr) {
        std::unique_lock lk(_send_mtx);
        _send_cond.wait(lk, [&] { return !_send_queue.empty() || !this->_send_th_running; });
        if (!this->_send_th_running) break;
        data = std::move(_send_queue.front());
      }

      data.header->init();
      data.body->init(_mode, _geometry);

      _force_fan.pack(_tx_buf);
      _reads_fpga_info.pack(_tx_buf);

      const auto no_wait = data.timeout == std::chrono::high_resolution_clock::duration::zero();
      while (true) {
        const auto msg_id = get_id();
        _tx_buf.header().msg_id = msg_id;
        data.header->pack(_tx_buf);
        data.body->pack(_tx_buf);
        spdlog::debug("Sending data ({}) asynchronously", msg_id);
        spdlog::debug("Timeout: {} [ms]",
                      static_cast<driver::autd3_float_t>(std::chrono::duration_cast<std::chrono::nanoseconds>(data.timeout).count()) / 1000 / 1000);
        if (!_link->send(_tx_buf)) {
          spdlog::warn("Failed to send data ({}). Trying to resend...", msg_id);
          break;
        }
        if (const auto success = wait_msg_processed(data.timeout); !no_wait && !success) {
          spdlog::warn("Could not confirm if the data ({}) was processed successfully.", msg_id);
          break;
        }
        spdlog::debug("Sending data ({}) succeeded.", msg_id);
        if (data.header->is_finished() && data.body->is_finished()) {
          data.header = nullptr;
          data.body = nullptr;
          spdlog::debug("All data has been sent successfully.");
          break;
        }
        if (no_wait) std::this_thread::sleep_for(_send_interval);
      }

      if (data.header == nullptr && data.body == nullptr) {
        std::unique_lock lk(_send_mtx);
        _send_queue.pop();
      }
    }
  });

  return is_open();
}

bool Controller::close() {
  if (!is_open()) {
    spdlog::debug("Controller is not opened.");
    return true;
  }

  _send_th_running = false;
  _send_cond.notify_all();
  spdlog::debug("Stopping asynchronous send thread...");
  if (_send_th.joinable()) _send_th.join();
  spdlog::debug("Stopping asynchronous send thread...done");

  if (!send(autd3::stop())) spdlog::error("Failed to stop outputting.");
  if (!send(autd3::clear())) spdlog::error("Failed to clear.");
  return _link->close();
}

bool Controller::is_open() const noexcept { return _link != nullptr && _link->is_open(); }

std::vector<driver::FPGAInfo> Controller::fpga_info() {
  std::vector<driver::FPGAInfo> fpga_info;
  if (!_link->receive(_rx_buf)) return fpga_info;
  std::transform(_rx_buf.begin(), _rx_buf.end(), std::back_inserter(fpga_info), [](const driver::RxMessage& rx) { return driver::FPGAInfo(rx.ack); });
  return fpga_info;
}

std::vector<driver::FirmwareInfo> Controller::firmware_infos() {
  std::vector<driver::FirmwareInfo> firmware_infos;

  const auto pack_ack = [&]() -> std::vector<uint8_t> {
    std::vector<uint8_t> acks;
    if (!_link->send(_tx_buf)) return acks;
    if (!wait_msg_processed(std::chrono::nanoseconds(200 * 1000 * 1000))) return acks;
    std::transform(_rx_buf.begin(), _rx_buf.end(), std::back_inserter(acks), [](const driver::RxMessage msg) noexcept { return msg.ack; });
    return acks;
  };

  driver::CPUVersion().pack(_tx_buf);
  const auto cpu_versions = pack_ack();
  if (cpu_versions.empty()) {
    spdlog::error("Failed to get firmware information.");
    return firmware_infos;
  }

  driver::FPGAVersion().pack(_tx_buf);
  const auto fpga_versions = pack_ack();
  if (fpga_versions.empty()) {
    spdlog::error("Failed to get firmware information.");
    return firmware_infos;
  }

  driver::FPGAFunctions().pack(_tx_buf);
  const auto fpga_functions = pack_ack();
  if (fpga_functions.empty()) {
    spdlog::error("Failed to get firmware information.");
    return firmware_infos;
  }
  for (size_t i = 0; i < cpu_versions.size(); i++) firmware_infos.emplace_back(i, cpu_versions[i], fpga_versions[i], fpga_functions[i]);

  for (const auto& info : firmware_infos) {
    if (info.cpu_version_num() != info.fpga_version_num())
      spdlog::error("FPGA firmware version {} and CPU firmware version {} do not match. This discrepancy may cause abnormal behavior.",
                    info.fpga_version(), info.cpu_version());
    if (info.cpu_version_num() != driver::VERSION_NUM || info.fpga_version_num() != driver::VERSION_NUM)
      spdlog::warn("You are using old firmware. Please consider updating to {}.", driver::FirmwareInfo::firmware_version_map(driver::VERSION_NUM));
  }

  return firmware_infos;
}

bool Controller::send(core::DatagramHeader* header, core::DatagramBody* body, const std::chrono::high_resolution_clock::duration timeout) {
  header->init();
  body->init(_mode, _geometry);

  _force_fan.pack(_tx_buf);
  _reads_fpga_info.pack(_tx_buf);

  const auto no_wait = timeout == std::chrono::high_resolution_clock::duration::zero();
  while (true) {
    const auto msg_id = get_id();
    _tx_buf.header().msg_id = msg_id;
    header->pack(_tx_buf);
    body->pack(_tx_buf);
    spdlog::debug("Sending data ({})", _tx_buf.header().msg_id);
    spdlog::debug("Timeout: {} [ms]",
                  static_cast<driver::autd3_float_t>(std::chrono::duration_cast<std::chrono::nanoseconds>(timeout).count()) / 1000 / 1000);
    if (!_link->send(_tx_buf)) {
      spdlog::warn("Failed to send data ({})", msg_id);
      return false;
    }
    if (const auto success = wait_msg_processed(timeout); !no_wait && !success) {
      spdlog::warn("Could not confirm if the data ({}) was processed successfully.", msg_id);
      return false;
    }
    if (header->is_finished() && body->is_finished()) {
      spdlog::debug("All data has been sent successfully.");
      break;
    }
    if (no_wait) std::this_thread::sleep_for(_send_interval);
  }
  return true;
}

bool Controller::send(SpecialData* s) {
  const auto timeout = s->ack_check_timeout_override() ? s->ack_check_timeout() : _ack_check_timeout;
  const auto h = s->header();
  const auto b = s->body();
  const auto res = send(h.get(), b.get(), timeout);
  return res;
}

void Controller::send_async(SpecialData* s) {
  const auto timeout = s->ack_check_timeout_override() ? s->ack_check_timeout() : _ack_check_timeout;
  send_async(s->header(), s->body(), timeout);
}

void Controller::send_async(std::unique_ptr<core::DatagramHeader> header, std::unique_ptr<core::DatagramBody> body) {
  send_async(std::move(header), std::move(body), _ack_check_timeout);
}

void Controller::send_async(std::unique_ptr<core::DatagramHeader> header, std::unique_ptr<core::DatagramBody> body,
                            const std::chrono::high_resolution_clock::duration timeout) {
  {
    std::unique_lock lk(_send_mtx);
    AsyncData data;
    data.header = std::move(header);
    data.body = std::move(body);
    data.timeout = timeout;
    _send_queue.emplace(std::move(data));
  }
  _send_cond.notify_all();
}

void Controller::wait() const {
  if (!is_open()) return;
  while (!_send_queue.empty()) std::this_thread::sleep_for(std::chrono::milliseconds(100));
}

void Controller::flush() {
  std::unique_lock lk(_send_mtx);
  std::queue<AsyncData>().swap(_send_queue);
}

bool Controller::reads_fpga_info() const noexcept { return _reads_fpga_info.value; }

std::chrono::high_resolution_clock::duration Controller::get_send_interval() const noexcept { return _send_interval; }

std::chrono::high_resolution_clock::duration Controller::get_ack_check_timeout() const noexcept { return _ack_check_timeout; }

driver::autd3_float_t Controller::set_sound_speed_from_temp(const driver::autd3_float_t temp, const driver::autd3_float_t k,
                                                            const driver::autd3_float_t r, const driver::autd3_float_t m) {
#ifdef AUTD3_USE_METER
  const auto sound_speed = std::sqrt(k * r * (static_cast<driver::autd3_float_t>(273.15) + temp) / m);
#else
  const auto sound_speed = std::sqrt(k * r * (static_cast<driver::autd3_float_t>(273.15) + temp) / m) * static_cast<driver::autd3_float_t>(1e3);
#endif
  _geometry.sound_speed = sound_speed;
  return sound_speed;
}

uint8_t Controller::get_id() noexcept {
  static std::atomic id_body{driver::MSG_BEGIN};
  if (uint8_t expected = driver::MSG_END; !id_body.compare_exchange_weak(expected, driver::MSG_BEGIN)) id_body.fetch_add(0x01);
  return id_body.load();
}

bool Controller::wait_msg_processed(const std::chrono::high_resolution_clock::duration timeout) {
  const auto msg_id = _tx_buf.header().msg_id;
  const auto start = std::chrono::high_resolution_clock::now();
  while (std::chrono::high_resolution_clock::now() - start < timeout) {
    if (_link->receive(_rx_buf) && _rx_buf.is_msg_processed(msg_id)) return true;
    std::this_thread::sleep_for(_send_interval);
  }
  return false;
}
}  // namespace autd3
