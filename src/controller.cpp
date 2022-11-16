// File: controller.cpp
// Project: src
// Created Date: 16/11/2022
// Author: Shun Suzuki
// -----
// Last Modified: 16/11/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#include "autd3/controller.hpp"

#include "autd3/core/interface.hpp"

#if _MSC_VER
#pragma warning(push)
#pragma warning(disable : 6285 6385 26437 26800 26498 26451 26495 26450)
#endif
#if defined(__GNUC__) && !defined(__llvm__)
#pragma GCC diagnostic push
#endif
#ifdef __clang__
#pragma clang diagnostic push
#endif
#include <spdlog/spdlog.h>
#if _MSC_VER
#pragma warning(pop)
#endif
#if defined(__GNUC__) && !defined(__llvm__)
#pragma GCC diagnostic pop
#endif
#ifdef __clang__
#pragma clang diagnostic pop
#endif

namespace autd3 {
Controller::Controller(std::unique_ptr<const driver::Driver> driver)
    : force_fan(false),
      reads_fpga_info(false),
      _tx_buf(0),
      _rx_buf(0),
      _link(nullptr),
      _send_th_running(false),
      _last_send_res(false),
      _driver(std::move(driver)) {}

Controller::~Controller() noexcept {
  try {
    close();
  } catch (std::exception&) {
  }
}

core::Geometry& Controller::geometry() noexcept { return _geometry; }

const core::Geometry& Controller::geometry() const noexcept { return _geometry; }

std::unique_ptr<core::Mode>& Controller::mode() noexcept { return _geometry.mode(); }

bool Controller::open(core::LinkPtr link) {
  _tx_buf = driver::TxDatagram(_geometry.num_devices());
  _rx_buf = driver::RxDatagram(_geometry.num_devices());

  _link = std::move(link);
  if (_link != nullptr) _link->open(_geometry);

  _send_th_running = true;
  _send_th = std::thread([this] {
    std::unique_ptr<core::DatagramHeader> header = nullptr;
    std::unique_ptr<core::DatagramBody> body = nullptr;
    std::function<void(void)> pre;
    std::function<void(void)> post;
    while (_send_th_running) {
      if (header == nullptr && body == nullptr) {
        std::unique_lock lk(_send_mtx);
        _send_cond.wait(lk, [&] { return !_send_queue.empty() || !this->_send_th_running; });
        if (!this->_send_th_running) break;
        AsyncData a = std::move(_send_queue.front());
        header = std::move(a.header);
        body = std::move(a.body);
        pre = std::move(a.pre);
        post = std::move(a.post);
      }

      pre();

      header->init();
      body->init();

      _driver->force_fan(_tx_buf, force_fan);
      _driver->reads_fpga_info(_tx_buf, reads_fpga_info);

      const auto no_wait = _ack_check_timeout == std::chrono::high_resolution_clock::duration::zero();
      while (true) {
        const auto msg_id = get_id();
        header->pack(_driver, msg_id, _tx_buf);
        body->pack(_driver, _geometry, _tx_buf);
        _link->send(_tx_buf);
        const auto success = wait_msg_processed(_ack_check_timeout);
        if (!no_wait && !success) {
          spdlog::warn("Failed to send data. Trying to resend...");
          break;
        }
        if (header->is_finished() && body->is_finished()) {
          header = nullptr;
          body = nullptr;
          break;
        }
        if (no_wait) std::this_thread::sleep_for(_send_interval);
      }

      post();

      if (header == nullptr && body == nullptr) {
        std::unique_lock lk(_send_mtx);
        _send_queue.pop();
      }
    }
  });

  return is_open();
}

bool Controller::close() {
  if (!is_open()) return true;

  _send_th_running = false;
  _send_cond.notify_all();
  if (_send_th.joinable()) _send_th.join();

  if (!send(autd3::Stop{})) return false;
  if (!send(autd3::Clear{})) return false;
  _link->close();
  return true;
}

bool Controller::is_open() const noexcept { return (_link != nullptr) && _link->is_open(); }

std::vector<driver::FPGAInfo> Controller::read_fpga_info() {
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
    std::transform(_rx_buf.begin(), _rx_buf.end(), std::back_inserter(acks), [](driver::RxMessage msg) noexcept { return msg.ack; });
    return acks;
  };

  _driver->cpu_version(_tx_buf);
  const auto cpu_versions = pack_ack();
  if (cpu_versions.empty()) return firmware_infos;
  for (const auto version : cpu_versions)
    if (version != _driver->version_num())
      spdlog::error(
          "Driver version is {}, but found {} CPU firmware. This discrepancy may cause abnormal behavior. Please change the driver version to an "
          "appropriate one or update the firmware version.",
          driver::FirmwareInfo::firmware_version_map(_driver->version_num()), driver::FirmwareInfo::firmware_version_map(version));

  _driver->fpga_version(_tx_buf);
  const auto fpga_versions = pack_ack();
  if (fpga_versions.empty()) return firmware_infos;
  for (const auto version : fpga_versions)
    if (version != _driver->version_num())
      spdlog::error(
          "Driver version is {}, but found {} FPGA firmware. This discrepancy may cause abnormal behavior. Please change the driver version to an "
          "appropriate one or update the firmware version.",
          driver::FirmwareInfo::firmware_version_map(_driver->version_num()), driver::FirmwareInfo::firmware_version_map(version));

  _driver->fpga_functions(_tx_buf);
  const auto fpga_functions = pack_ack();
  if (fpga_functions.empty()) return firmware_infos;

  for (size_t i = 0; i < _geometry.num_devices(); i++) firmware_infos.emplace_back(i, cpu_versions.at(i), fpga_versions.at(i), fpga_functions.at(i));

  DriverLatest latest_driver;
  for (const auto& info : firmware_infos) {
    if (info.cpu_version_num() != info.fpga_version_num())
      spdlog::error("FPGA firmware version {} and CPU firmware version {} do not match. This discrepancy may cause abnormal behavior.",
                    info.fpga_version(), info.cpu_version());
    if ((info.cpu_version_num() != latest_driver.version_num()) || info.fpga_version_num() != latest_driver.version_num())
      spdlog::warn("You are using old firmware. Please consider updating to {}.",
                   driver::FirmwareInfo::firmware_version_map(latest_driver.version_num()));
  }

  return firmware_infos;
}

bool Controller::synchronize() { return send(Synchronize{}); }

bool Controller::update_flag() { return send(UpdateFlag{}); }

bool Controller::clear() { return send(Clear{}); }

bool Controller::stop() { return send(Stop{}); }

bool Controller::send(core::DatagramHeader* header, core::DatagramBody* body) {
  header->init();
  body->init();

  _driver->force_fan(_tx_buf, force_fan);
  _driver->reads_fpga_info(_tx_buf, reads_fpga_info);

  const auto no_wait = _ack_check_timeout == std::chrono::high_resolution_clock::duration::zero();
  while (true) {
    const auto msg_id = get_id();
    header->pack(_driver, msg_id, _tx_buf);
    body->pack(_driver, _geometry, _tx_buf);
    _link->send(_tx_buf);
    const auto success = wait_msg_processed(_ack_check_timeout);
    if (!no_wait && !success) return false;
    if (header->is_finished() && body->is_finished()) break;
    if (no_wait) std::this_thread::sleep_for(_send_interval);
  }
  return true;
}

bool Controller::send(SpecialData* s) {
  push_ack_check_timeout();
  if (s->ack_check_timeout_override()) _ack_check_timeout = s->ack_check_timeout();
  auto h = s->header();
  auto b = s->body();
  const auto res = send(h.get(), b.get());
  pop_ack_check_timeout();
  return res;
}

void Controller::send_async(SpecialData* s) {
  auto ack_check_timeout_override = s->ack_check_timeout_override();
  auto timeout = s->ack_check_timeout();
  send_async(
      s->header(), s->body(),
      [this, ack_check_timeout_override, timeout] {
        push_ack_check_timeout();
        if (ack_check_timeout_override) _ack_check_timeout = timeout;
      },
      [this] { pop_ack_check_timeout(); });
}

void Controller::send_async(std::unique_ptr<core::DatagramHeader> header, std::unique_ptr<core::DatagramBody> body, std::function<void()> pre,
                            std::function<void()> post) {
  {
    std::unique_lock lk(_send_mtx);
    AsyncData data;
    data.header = std::move(header);
    data.body = std::move(body);
    data.pre = std::move(pre);
    data.post = std::move(post);
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

std::chrono::high_resolution_clock::duration Controller::get_send_interval() const noexcept { return _send_interval; }

std::chrono::high_resolution_clock::duration Controller::get_ack_check_timeout() const noexcept { return _ack_check_timeout; }

uint8_t Controller::get_id() noexcept {
  static std::atomic id_body{driver::MSG_BEGIN};
  if (uint8_t expected = driver::MSG_END; !id_body.compare_exchange_weak(expected, driver::MSG_BEGIN)) id_body.fetch_add(0x01);
  return id_body.load();
}

bool Controller::wait_msg_processed(const std::chrono::high_resolution_clock::duration timeout) {
  const auto msg_id = _tx_buf.header().msg_id;
  auto start = std::chrono::high_resolution_clock::now();
  while (std::chrono::high_resolution_clock::now() - start < timeout) {
    if (_link->receive(_rx_buf) && _rx_buf.is_msg_processed(msg_id)) return true;
    std::this_thread::sleep_for(_send_interval);
  }
  return false;
}

void Controller::push_ack_check_timeout() { _ack_check_timeout_ = _ack_check_timeout; }

void Controller::pop_ack_check_timeout() { _ack_check_timeout = _ack_check_timeout_; }
}  // namespace autd3
