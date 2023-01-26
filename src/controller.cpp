// File: controller.cpp
// Project: src
// Created Date: 16/11/2022
// Author: Shun Suzuki
// -----
// Last Modified: 27/01/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#include "autd3/controller.hpp"

#include <atomic>

#include "autd3/core/datagram.hpp"
#include "autd3/driver/operation/info.hpp"

namespace autd3 {
Controller::Controller() : _tx_buf({0}), _rx_buf(0), _link(nullptr), _send_th_running(false), _last_send_res(false) {}

Controller::~Controller() noexcept {
  try {
    close();
  } catch (std::exception&) {
  }
}

core::Geometry& Controller::geometry() noexcept { return _geometry; }

const core::Geometry& Controller::geometry() const noexcept { return _geometry; }

void Controller::open(core::LinkPtr link) {
  if (_geometry.num_transducers() == 0) throw std::runtime_error("Please add devices before opening.");

  if (link == nullptr) throw std::runtime_error("link is null");

  _link = std::move(link);
  if (!_link->open(_geometry)) throw std::runtime_error("Failed to open link.");

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

      const auto op_header = data.header->operation();
      const auto op_body = data.body->operation(_geometry);

      op_header->init();
      op_body->init();

      _force_fan.pack(_tx_buf);
      _reads_fpga_info.pack(_tx_buf);

      const auto no_wait = data.timeout == std::chrono::high_resolution_clock::duration::zero();
      while (true) {
        const auto msg_id = get_id();
        _tx_buf.header().msg_id = msg_id;
        op_header->pack(_tx_buf);
        op_body->pack(_tx_buf);
        if (!_link->send_receive(_tx_buf, _rx_buf, _send_interval, data.timeout)) {
          data.header = nullptr;
          data.body = nullptr;
          break;
        }
        if (op_header->is_finished() && op_body->is_finished()) {
          data.header = nullptr;
          data.body = nullptr;
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
}

bool Controller::close() {
  if (!is_open()) return true;
  flush();
  _send_th_running = false;
  _send_cond.notify_all();
  if (_send_th.joinable()) _send_th.join();
  auto res = send(stop());
  res &= send(clear());
  res &= _link->close();
  return res;
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
    if (!_link->send_receive(_tx_buf, _rx_buf, _send_interval, std::chrono::nanoseconds(200 * 1000 * 1000))) return acks;
    std::transform(_rx_buf.begin(), _rx_buf.end(), std::back_inserter(acks), [](const driver::RxMessage msg) noexcept { return msg.ack; });
    return acks;
  };

  driver::CPUVersion::pack(_tx_buf);
  const auto cpu_versions = pack_ack();
  if (cpu_versions.empty()) throw std::runtime_error("Failed to get firmware information.");

  driver::FPGAVersion::pack(_tx_buf);
  const auto fpga_versions = pack_ack();
  if (fpga_versions.empty()) throw std::runtime_error("Failed to get firmware information.");

  driver::FPGAFunctions::pack(_tx_buf);
  const auto fpga_functions = pack_ack();
  if (fpga_functions.empty()) throw std::runtime_error("Failed to get firmware information.");

  for (size_t i = 0; i < cpu_versions.size(); i++) firmware_infos.emplace_back(i, cpu_versions[i], fpga_versions[i], fpga_functions[i]);

  return firmware_infos;
}

bool Controller::send(core::DatagramHeader* header, core::DatagramBody* body, const std::chrono::high_resolution_clock::duration timeout) {
  const auto op_header = header->operation();
  const auto op_body = body->operation(_geometry);

  op_header->init();
  op_body->init();

  _force_fan.pack(_tx_buf);
  _reads_fpga_info.pack(_tx_buf);

  const auto no_wait = timeout == std::chrono::high_resolution_clock::duration::zero();
  while (true) {
    const auto msg_id = get_id();
    _tx_buf.header().msg_id = msg_id;

    op_header->pack(_tx_buf);
    op_body->pack(_tx_buf);

    if (!_link->send_receive(_tx_buf, _rx_buf, _send_interval, timeout)) return false;

    if (op_header->is_finished() && op_body->is_finished()) break;
    if (no_wait) std::this_thread::sleep_for(_send_interval);
  }
  return true;
}

bool Controller::send(SpecialData* s) {
  const auto timeout = s->ack_check_timeout_override() ? s->ack_check_timeout() : _ack_check_timeout;
  const auto h = s->header();
  const auto b = s->body();
  return send(h.get(), b.get(), timeout);
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

}  // namespace autd3
