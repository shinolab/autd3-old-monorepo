// File: controller.hpp
// Project: autd3
// Created Date: 10/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 11/05/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Hapis Lab. All rights reserved.
//

#pragma once

#include <algorithm>
#include <atomic>
#include <chrono>
#include <thread>
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

  explicit Controller(core::LinkPtr link, core::Geometry<T> geometry)
      : force_fan(false),
        reads_fpga_info(false),
        check_ack(false),
        _geometry(std::move(geometry)),
        _tx_buf(_geometry.num_devices()),
        _rx_buf(_geometry.num_devices()),
        _link(std::move(link)) {
    _link->open();
  }

  bool config_silencer(SilencerConfig config) {
    _tx_buf.clear();

    driver::force_fan(_tx_buf, force_fan);
    driver::reads_fpga_info(_tx_buf, reads_fpga_info);

    const auto msg_id = get_id();
    driver::config_silencer(msg_id, config.cycle, config.step, _tx_buf);

    _link->send(_tx_buf);
    return wait_msg_processed(50);
  }

  bool synchronize() {
    _tx_buf.clear();

    driver::force_fan(_tx_buf, force_fan);
    driver::reads_fpga_info(_tx_buf, reads_fpga_info);

    const auto msg_id = get_id();
    std::vector<uint16_t> cycles;
    for (const auto& dev : _geometry) std::transform(dev.begin(), dev.end(), std::back_inserter(cycles), [](const T& tr) { return tr.cycle(); });

    sync(msg_id, _link->cycle_ticks(), gsl::span{cycles}, _tx_buf);

    _link->send(_tx_buf);
    return wait_msg_processed(50);
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
    auto g = gain::Null<T>();
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

  bool send(core::DatagramBody<T>&& body) {
    auto& b = body;
    return send(b);
  }
  bool send(core::DatagramBody<T>& body) {
    body.init();

    auto success = true;
    while (true) {
      _tx_buf.clear();

      driver::force_fan(_tx_buf, force_fan);
      driver::reads_fpga_info(_tx_buf, reads_fpga_info);

      const auto msg_id = get_id();
      body.pack(msg_id, _geometry, _tx_buf);
      _link->send(_tx_buf);
      success &= wait_msg_processed(50);
      if (!success || body.is_finished()) {
        break;
      }
    }
    return success;
  }

  bool send(core::DatagramHeader&& header, core::DatagramBody<T>&& body) {
    auto& h = header;
    auto& b = body;
    return send(h, b);
  }
  bool send(core::DatagramHeader& header, core::DatagramBody<T>& body) {
    header.init();
    body.init();

    auto success = true;
    while (true) {
      _tx_buf.clear();

      driver::force_fan(_tx_buf, force_fan);
      driver::reads_fpga_info(_tx_buf, reads_fpga_info);

      const auto msg_id = get_id();
      header.pack(msg_id, _tx_buf);
      body.pack(msg_id, _geometry, _tx_buf);
      _link->send(_tx_buf);
      success &= wait_msg_processed(50);
      if (!success || header.is_finished() && body.is_finished()) {
        break;
      }
    }
    return success;
  }

  bool force_fan;
  bool reads_fpga_info;
  bool check_ack;

 private:
  static uint8_t get_id() noexcept {
    static std::atomic<uint8_t> id{driver::MSG_NORMAL_BEGINNING};
    if (uint8_t expected = driver::MSG_NORMAL_END; !id.compare_exchange_weak(expected, driver::MSG_NORMAL_BEGINNING)) id.fetch_add(0x01);
    return id.load();
  }

  bool wait_msg_processed(const size_t max_trial) {
    if (!check_ack) {
      return true;
    }
    const auto msg_id = _tx_buf.header().msg_id;
    const auto wait = gsl::narrow_cast<uint64_t>(std::ceil(driver::EC_TRAFFIC_DELAY * 1000.0 / static_cast<double>(driver::EC_DEVICE_PER_FRAME) *
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
