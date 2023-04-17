// File: link.hpp
// Project: core
// Created Date: 11/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 17/04/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <chrono>
#include <memory>
#include <optional>
#include <thread>

#include "autd3/core/geometry.hpp"
#include "autd3/driver/cpu/datagram.hpp"

namespace autd3::core {

using Clock = std::chrono::high_resolution_clock;
using Duration = Clock::duration;
using Milliseconds = std::chrono::milliseconds;

/**
 * @brief Link is the interface to the AUTD device
 */
class Link {
 public:
  Link() noexcept {}
  explicit Link(const Duration timeout) noexcept : _timeout(timeout) {}
  virtual ~Link() = default;
  Link(const Link& v) = delete;
  Link& operator=(const Link& obj) = delete;
  Link(Link&& obj) = default;
  Link& operator=(Link&& obj) = default;

  /**
   * @brief Open link
   * @return true if success
   */
  [[nodiscard]] virtual bool open(const Geometry& geometry) = 0;

  /**
   * @brief Close link
   * @return true if success
   */
  [[nodiscard]] virtual bool close() = 0;

  /**
   * @brief  Send data to devices
   * @return true if succeed
   */
  [[nodiscard]] virtual bool send(const driver::TxDatagram& tx) = 0;

  /**
   * @brief  Receive data from devices
   * @return true if succeed
   */
  [[nodiscard]] virtual bool receive(driver::RxDatagram& rx) = 0;

  /**
   * @brief  Send and Read data from devices
   * @return true if succeed
   */
  [[nodiscard]] virtual bool send_receive(const driver::TxDatagram& tx, driver::RxDatagram& rx, const std::optional<Duration> timeout) {
    if (!send(tx)) return false;
    const auto timeout_ = timeout.value_or(_timeout);
    if (timeout_ == Duration::zero()) return receive(rx);
    return wait_msg_processed(tx.header().msg_id, rx, timeout_);
  }

  /**
   * @return true if opened
   */
  [[nodiscard]] virtual bool is_open() = 0;

  [[nodiscard]] Duration timeout() const noexcept { return _timeout; }

 protected:
  bool wait_msg_processed(const uint8_t msg_id, driver::RxDatagram& rx, const Duration timeout) {
    const auto expired = Clock::now() + timeout;
    do {
      std::this_thread::sleep_for(Milliseconds(1));
      if (receive(rx) && rx.is_msg_processed(msg_id)) return true;
    } while (Clock::now() < expired);
    return false;
  }
  Duration _timeout{Duration::zero()};
};

using LinkPtr = std::unique_ptr<Link>;

}  // namespace autd3::core
