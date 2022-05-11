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

#include <utility>
#include <vector>

#include "core/geometry/geometry.hpp"
#include "core/geometry/legacy_transducer.hpp"
#include "core/interface.hpp"
#include "core/link.hpp"
#include "driver/firmware_version.hpp"
#include "silencer_config.hpp"

namespace autd3 {

template <typename T = core::LegacyTransducer>
class Controller {
 public:
  core::Geometry<T>& geometry() noexcept;

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

  bool config_silencer(SilencerConfig config);

  bool synchronize();

  /**
   * @brief Clear all data in hardware
   * \return if this function returns true and check_ack is true, it guarantees that the devices have processed the data.
   */
  bool clear();

  /**
   * @brief Stop outputting
   * \return if this function returns true and check_ack is true, it guarantees that the devices have processed the data.
   */
  bool stop();

  /**
   * @brief Close the controller
   * \return if this function returns true and check_ack is true, it guarantees that the devices have processed the data.
   */
  bool close();

  [[nodiscard]] std::vector<driver::FirmwareInfo> firmware_infos();

  bool send(core::DatagramBody<T>&& body);
  bool send(core::DatagramBody<T>& body);

  bool send(core::DatagramHeader&& header, core::DatagramBody<T>&& body);
  bool send(core::DatagramHeader& header, core::DatagramBody<T>& body);

  bool force_fan;
  bool reads_fpga_info;
  bool check_ack;

 private:
  bool wait_msg_processed(size_t max_trial);

  core::Geometry<T> _geometry;
  driver::TxDatagram _tx_buf;
  driver::RxDatagram _rx_buf;
  core::LinkPtr _link;
};

}  // namespace autd3
