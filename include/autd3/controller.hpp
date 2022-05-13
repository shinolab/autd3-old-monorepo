// File: controller.hpp
// Project: autd3
// Created Date: 10/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 13/05/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Hapis Lab. All rights reserved.
//

#pragma once

#include <algorithm>
#include <atomic>
#include <chrono>
#include <thread>
#include <type_traits>
#include <utility>
#include <vector>

#include "autd3/driver/cpu/datagram.hpp"
#include "autd3/driver/cpu/ec_config.hpp"
#include "autd3/gain/primitive.hpp"
#include "core/geometry/geometry.hpp"
#include "core/geometry/legacy_transducer.hpp"
#include "core/geometry/normal_transducer.hpp"
#include "core/interface.hpp"
#include "core/link.hpp"
#include "driver/firmware_version.hpp"
#include "silencer_config.hpp"

namespace autd3 {

template <typename T = core::LegacyTransducer>
class Controller {
 public:
  core::Geometry<T>& geometry() noexcept { return _geometry; }

  Controller() : force_fan(false), reads_fpga_info(false), check_ack(false), _geometry(), _tx_buf(0), _rx_buf(0), _link(nullptr) {}

  bool open(core::LinkPtr link) {
    _tx_buf = driver::TxDatagram(_geometry.num_devices());
    _rx_buf = driver::RxDatagram(_geometry.num_devices());

    _link = std::move(link);
    if (_link != nullptr) _link->open();
    return is_open();
  }

  bool is_open() { return (_link != nullptr) && _link->is_open(); }

  bool config_silencer(const SilencerConfig config) {
    _tx_buf.clear();

    driver::force_fan(_tx_buf, force_fan);
    driver::reads_fpga_info(_tx_buf, reads_fpga_info);

    const auto msg_id = get_id_header();
    driver::config_silencer(msg_id, config.cycle, config.step, _tx_buf);

    _link->send(_tx_buf);
    return wait_msg_processed(50);
  }

  bool synchronize() {
    _tx_buf.clear();

    driver::force_fan(_tx_buf, force_fan);
    driver::reads_fpga_info(_tx_buf, reads_fpga_info);

    const auto msg_id = get_id_body();
    std::vector<uint16_t> cycles;
    for (const auto& dev : _geometry) std::transform(dev.begin(), dev.end(), std::back_inserter(cycles), [](const T& tr) { return tr.cycle(); });

    sync(msg_id, _link->cycle_ticks(), cycles.data(), _tx_buf);

    _link->send(_tx_buf);
    return wait_msg_processed(50);
  }

  bool update_flag() {
    _tx_buf.clear();

    driver::force_fan(_tx_buf, force_fan);
    driver::reads_fpga_info(_tx_buf, reads_fpga_info);
    _tx_buf.header().msg_id = get_id_header();

    _link->send(_tx_buf);
    return wait_msg_processed(50);
  }

  std::vector<driver::FPGAInfo> read_fpga_info() {
    std::vector<driver::FPGAInfo> fpga_info;
    _link->receive(_rx_buf);
    std::transform(_rx_buf.begin(), _rx_buf.end(), std::back_inserter(fpga_info),
                   [](const driver::RxMessage& rx) { return driver::FPGAInfo(rx.ack); });
    return fpga_info;
  }

  /**
   * @brief Clear all data in hardware
   * \return if this function returns true and check_ack is true, it guarantees that the devices have processed the data.
   */
  bool clear() {
    const auto check_ack_ = check_ack;
    check_ack = true;
    _tx_buf.clear();
    driver::clear(_tx_buf);
    _link->send(_tx_buf);
    const auto success = wait_msg_processed(200);
    check_ack = check_ack_;
    return success;
  }

  /**
   * @brief Stop outputting
   * \return if this function returns true and check_ack is true, it guarantees that the devices have processed the data.
   */
  bool stop() {
    auto res = config_silencer(SilencerConfig());
    gain::Null<T> g;
    res &= send(g);
    return res;
  }

  /**
   * @brief Close the controller
   * \return if this function returns true and check_ack is true, it guarantees that the devices have processed the data.
   */
  bool close() {
    auto res = stop();
    res &= clear();
    _link->close();
    return res;
  }

  [[nodiscard]] std::vector<driver::FirmwareInfo> firmware_infos() {
    const auto check_ack_ = check_ack;
    check_ack = true;

    cpu_version(_tx_buf);
    _link->send(_tx_buf);
    wait_msg_processed(50);
    std::vector<uint8_t> cpu_versions;
    std::transform(_rx_buf.begin(), _rx_buf.end(), std::back_inserter(cpu_versions), [](driver::RxMessage msg) noexcept { return msg.ack; });

    fpga_version(_tx_buf);
    _link->send(_tx_buf);
    wait_msg_processed(50);
    std::vector<uint8_t> fpga_versions;
    std::transform(_rx_buf.begin(), _rx_buf.end(), std::back_inserter(fpga_versions), [](driver::RxMessage msg) noexcept { return msg.ack; });

    fpga_functions(_tx_buf);
    _link->send(_tx_buf);
    wait_msg_processed(50);
    std::vector<uint8_t> fpga_functions;
    std::transform(_rx_buf.begin(), _rx_buf.end(), std::back_inserter(fpga_functions), [](driver::RxMessage msg) noexcept { return msg.ack; });

    check_ack = check_ack_;

    std::vector<driver::FirmwareInfo> firmware_infos;
    for (size_t i = 0; i < _geometry.num_devices(); i++)
      firmware_infos.emplace_back(i, cpu_versions.at(i), fpga_versions.at(i), fpga_functions.at(i));

    return firmware_infos;
  }

  template <typename H>
  auto send(H& header) -> typename std::enable_if_t<std::is_base_of_v<core::DatagramHeader, H>, bool> {
    header.init();

    while (true) {
      _tx_buf.clear();

      driver::force_fan(_tx_buf, force_fan);
      driver::reads_fpga_info(_tx_buf, reads_fpga_info);

      const auto msg_id = get_id_header();
      header.pack(msg_id, _tx_buf);
      _link->send(_tx_buf);
      if (!wait_msg_processed(50)) return false;
      if (header.is_finished()) break;
    }
    return true;
  }

  template <typename B>
  auto send(B& body) -> typename std::enable_if_t<std::is_base_of_v<core::DatagramBody<T>, B>, bool> {
    body.init();

    while (true) {
      _tx_buf.clear();

      driver::force_fan(_tx_buf, force_fan);
      driver::reads_fpga_info(_tx_buf, reads_fpga_info);

      const auto msg_id = get_id_body();
      body.pack(msg_id, _geometry, _tx_buf);
      _link->send(_tx_buf);
      if (!wait_msg_processed(50)) return false;
      if (body.is_finished()) break;
    }
    return true;
  }

  template <typename H, typename B>
  auto send(H& header, B& body) ->
      typename std::enable_if_t<std::is_base_of_v<core::DatagramHeader, H> && std::is_base_of_v<core::DatagramBody<T>, B>, bool> {
    header.init();
    body.init();

    while (true) {
      _tx_buf.clear();

      driver::force_fan(_tx_buf, force_fan);
      driver::reads_fpga_info(_tx_buf, reads_fpga_info);

      const auto msg_id = get_id_body();
      header.pack(msg_id, _tx_buf);
      body.pack(msg_id, _geometry, _tx_buf);
      _link->send(_tx_buf);
      if (!wait_msg_processed(50)) return false;
      if (header.is_finished() && body.is_finished()) break;
    }
    return true;
  }

  bool force_fan;
  bool reads_fpga_info;
  bool check_ack;

 private:
  static uint8_t get_id_header() noexcept {
    static std::atomic id_header{driver::MSG_HEADER_ONLY_BEGINNING};
    if (uint8_t expected = driver::MSG_HEADER_ONLY_END; !id_header.compare_exchange_weak(expected, driver::MSG_HEADER_ONLY_BEGINNING))
      id_header.fetch_add(0x01);
    return id_header.load();
  }

  static uint8_t get_id_body() noexcept {
    static std::atomic id_body{driver::MSG_CONTAIN_BODY_BEGINNING};
    if (uint8_t expected = driver::MSG_CONTAIN_BODY_END; !id_body.compare_exchange_weak(expected, driver::MSG_CONTAIN_BODY_BEGINNING))
      id_body.fetch_add(0x01);
    return id_body.load();
  }

  bool wait_msg_processed(const size_t max_trial) {
    if (!check_ack) {
      return true;
    }
    const auto msg_id = _tx_buf.header().msg_id;
    const auto wait = static_cast<uint64_t>(std::ceil(driver::EC_TRAFFIC_DELAY * 1000.0 / static_cast<double>(driver::EC_DEVICE_PER_FRAME) *
                                                      static_cast<double>(_geometry.num_devices())));
    auto success = false;
    for (size_t i = 0; i < max_trial; i++) {
      if (!_link->receive(_rx_buf)) {
        continue;
      }
      if (_rx_buf.is_msg_processed(msg_id)) {
        success = true;
        break;
      }
      std::this_thread::sleep_for(std::chrono::milliseconds(wait));
    }

    return success;
  }

  core::Geometry<T> _geometry;
  driver::TxDatagram _tx_buf;
  driver::RxDatagram _rx_buf;
  core::LinkPtr _link;
};

}  // namespace autd3
