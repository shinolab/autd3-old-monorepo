// File: link.hpp
// Project: core
// Created Date: 11/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 15/11/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <cstdint>
#include <memory>

#include "autd3/core/geometry.hpp"
#include "autd3/driver/common/cpu/datagram.hpp"

namespace autd3::core {

/**
 * @brief Link is the interface to the AUTD device
 */
class Link {
 public:
  Link() noexcept = default;
  virtual ~Link() = default;
  Link(const Link& v) = delete;
  Link& operator=(const Link& obj) = delete;
  Link(Link&& obj) = default;
  Link& operator=(Link&& obj) = default;

  /**
   * @brief Open link
   */
  virtual void open(const Geometry& geometry) = 0;

  /**
   * @brief Close link
   */
  virtual void close() = 0;

  /**
   * @brief  Send data to devices
   * @return true if succeed
   */
  virtual bool send(const driver::TxDatagram& tx) = 0;

  /**
   * @brief  Read data from devices
   * @return true if succeed
   */
  virtual bool receive(driver::RxDatagram& rx) = 0;

  /**
   * @return true if opened
   */
  [[nodiscard]] virtual bool is_open() = 0;
};

using LinkPtr = std::unique_ptr<Link>;

}  // namespace autd3::core
