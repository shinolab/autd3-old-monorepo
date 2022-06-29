// File: controller.hpp
// Project: autd3
// Created Date: 10/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 28/06/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
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
#include "core/amplitudes.hpp"
#include "core/geometry.hpp"
#include "core/interface.hpp"
#include "core/link.hpp"
#include "core/silencer_config.hpp"
#include "driver/firmware_version.hpp"

namespace autd3 {

/**
 * @brief AUTD Controller
 */
class Controller {
 public:
  Controller() : force_fan(false), reads_fpga_info(false), check_trials(0), send_interval(1), _geometry(), _tx_buf(0), _rx_buf(0), _link(nullptr) {}

  /**
   * @brief Geometry of the devices
   */
  core::Geometry& geometry() noexcept { return _geometry; }

  /**
   * @brief Geometry of the devices
   */
  [[nodiscard]] const core::Geometry& geometry() const noexcept { return _geometry; }

  bool open(core::LinkPtr link) {
    _tx_buf = driver::TxDatagram(_geometry.num_devices());
    _rx_buf = driver::RxDatagram(_geometry.num_devices());

    _link = std::move(link);
    if (_link != nullptr) _link->open();
    return is_open();
  }

  /**
   * @brief Verify the device is properly connected
   */
  [[nodiscard]] bool is_open() const noexcept { return (_link != nullptr) && _link->is_open(); }

  /**
   * @brief Synchronize devices
   * \return if this function returns true and check_trials > 0, it guarantees that the devices have processed the data.
   */
  bool synchronize() {
    driver::force_fan(_tx_buf, force_fan);
    driver::reads_fpga_info(_tx_buf, reads_fpga_info);

    const auto msg_id = get_id();
    std::vector<uint16_t> cycles;
    std::for_each(_geometry.begin(), _geometry.end(), [&](const auto& dev) {
      std::transform(dev.begin(), dev.end(), std::back_inserter(cycles), [](const core::Transducer& tr) { return tr.cycle(); });
    });

    sync(msg_id, cycles.data(), _tx_buf);

    if (!_link->send(_tx_buf)) return false;

    const auto success = wait_msg_processed(200) != 200;
    return success;
  }

  /**
   * @brief Update flags (force fan and reads_fpga_info)
   * \return if this function returns true and check_trials > 0, it guarantees that the devices have processed the data.
   */
  bool update_flag() {
    core::NullHeader h;
    core::NullBody b;
    return send(h, b);
  }

  /**
   * @brief FPGA info
   *  \return veetor of FPGAInfo. If failed, the vector is empty
   */
  std::vector<driver::FPGAInfo> read_fpga_info() {
    std::vector<driver::FPGAInfo> fpga_info;
    if (!_link->receive(_rx_buf)) return fpga_info;
    std::transform(_rx_buf.begin(), _rx_buf.end(), std::back_inserter(fpga_info),
                   [](const driver::RxMessage& rx) { return driver::FPGAInfo(rx.ack); });
    return fpga_info;
  }

  /**
   * @brief Clear all data in devices
   * \return if this function returns true and check_trials > 0, it guarantees that the devices have processed the data.
   */
  bool clear() {
    driver::clear(_tx_buf);
    if (!_link->send(_tx_buf)) return false;
    const auto success = wait_msg_processed(200) != 200;
    return success;
  }

  /**
   * @brief Stop outputting
   * \return if this function returns true and check_trials > 0, it guarantees that the devices have processed the data.
   */
  bool stop() {
    SilencerConfig config;
    auto null = core::Amplitudes(0.0);
    return send(config, null);
  }

  /**
   * @brief Close the controller
   * \return if this function returns true and check_trials > 0, it guarantees that the devices have processed the data.
   */
  bool close() {
    if (!stop()) return false;
    if (!clear()) return false;
    _link->close();
    return true;
  }

  /**
   * @brief Enumerate firmware information
   * \return vector of driver::FirmwareInfo. If failed, the vector is empty.
   */
  [[nodiscard]] std::vector<driver::FirmwareInfo> firmware_infos() {
    std::vector<driver::FirmwareInfo> firmware_infos;

    const auto pack_ack = [&]() -> std::vector<uint8_t> {
      std::vector<uint8_t> acks;
      if (!_link->send(_tx_buf)) return acks;
      if (wait_msg_processed(200) == 200) return acks;
      std::transform(_rx_buf.begin(), _rx_buf.end(), std::back_inserter(acks), [](driver::RxMessage msg) noexcept { return msg.ack; });
      return acks;
    };

    cpu_version(_tx_buf);
    const auto cpu_versions = pack_ack();
    if (cpu_versions.empty()) return firmware_infos;

    fpga_version(_tx_buf);
    const auto fpga_versions = pack_ack();
    if (fpga_versions.empty()) return firmware_infos;

    fpga_functions(_tx_buf);
    const auto fpga_functions = pack_ack();
    if (fpga_functions.empty()) return firmware_infos;

    for (size_t i = 0; i < _geometry.num_devices(); i++)
      firmware_infos.emplace_back(i, cpu_versions.at(i), fpga_versions.at(i), fpga_functions.at(i));

    return firmware_infos;
  }

  /**
   * @brief Send header data to devices
   * \return if this function returns true and check_trials > 0, it guarantees that the devices have processed the data.
   */
  template <typename H>
  auto send(H& header) -> typename std::enable_if_t<std::is_base_of_v<core::DatagramHeader, H>, bool> {
    core::NullBody b;
    return send(header, b);
  }

  /**
   * @brief Send header data to devices
   * \return if this function returns true and check_trials > 0, it guarantees that the devices have processed the data.
   */
  template <typename H>
  auto send(H&& header) -> typename std::enable_if_t<std::is_base_of_v<core::DatagramHeader, H>, bool> {
    return send(header);
  }

  /**
   * @brief Send body data to devices
   * \return if this function returns true and check_trials > 0, it guarantees that the devices have processed the data.
   */
  template <typename B>
  auto send(B& body) -> typename std::enable_if_t<std::is_base_of_v<core::DatagramBody, B>, bool> {
    core::NullHeader h;
    return send(h, body);
  }

  /**
   * @brief Send body data to devices
   * \return if this function returns true and check_trials > 0, it guarantees that the devices have processed the data.
   */
  template <typename B>
  auto send(B&& body) -> typename std::enable_if_t<std::is_base_of_v<core::DatagramBody, B>, bool> {
    return send(body);
  }

  /**
   * @brief Send header and body data to devices
   * \return if this function returns true and check_trials > 0, it guarantees that the devices have processed the data.
   */
  template <typename H, typename B>
  auto send(H& header, B& body) ->
      typename std::enable_if_t<std::is_base_of_v<core::DatagramHeader, H> && std::is_base_of_v<core::DatagramBody, B>, bool> {
    header.init();
    body.init();

    driver::force_fan(_tx_buf, force_fan);
    driver::reads_fpga_info(_tx_buf, reads_fpga_info);

    while (true) {
      const auto msg_id = get_id();
      header.pack(msg_id, _tx_buf);
      body.pack(_geometry, _tx_buf);
      _link->send(_tx_buf);
      const auto trials = wait_msg_processed(check_trials);
      if ((check_trials != 0) && (trials == check_trials)) return false;
      if (header.is_finished() && body.is_finished()) break;
      if (trials == 0) std::this_thread::sleep_for(std::chrono::microseconds(send_interval * driver::EC_SYNC0_CYCLE_TIME_MICRO_SEC));
    }
    return true;
  }

  /**
   * @brief Send header and body data to devices
   * \return if this function returns true and check_trials > 0, it guarantees that the devices have processed the data.
   */
  template <typename H, typename B>
  auto send(H&& header, B&& body) ->
      typename std::enable_if_t<std::is_base_of_v<core::DatagramHeader, H> && std::is_base_of_v<core::DatagramBody, B>, bool> {
    return send(header, body);
  }

  /**
   * @brief If true, the fan will be forced to start.
   */
  bool force_fan;

  /**
   * @brief If true, the devices return FPGA info in all frames. The FPGA info can be read by fpga_info().
   */
  bool reads_fpga_info;

  /**
   * @brief If > 0, this controller check ack from devices. This value represents the maximum number of trials for the check.
   */
  size_t check_trials;

  /**
   * @brief Transmission interval between frames when sending multiple data. The interval will be send_interval *
   * driver::EC_SYNC0_CYCLE_TIME_MICRO_SEC.
   */
  size_t send_interval;

 private:
  static uint8_t get_id() noexcept {
    static std::atomic id_body{driver::MSG_BEGIN};
    if (uint8_t expected = driver::MSG_END; !id_body.compare_exchange_weak(expected, driver::MSG_BEGIN)) id_body.fetch_add(0x01);
    return id_body.load();
  }

  size_t wait_msg_processed(const size_t max_trial) {
    size_t i;
    const auto msg_id = _tx_buf.header().msg_id;
    for (i = 0; i < max_trial; i++) {
      if (_link->receive(_rx_buf) && _rx_buf.is_msg_processed(msg_id)) break;
      std::this_thread::sleep_for(std::chrono::microseconds(send_interval * driver::EC_SYNC0_CYCLE_TIME_MICRO_SEC));
    }
    return i;
  }

  core::Geometry _geometry;
  driver::TxDatagram _tx_buf;
  driver::RxDatagram _rx_buf;
  core::LinkPtr _link;
};

}  // namespace autd3
