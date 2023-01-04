// File: clear.hpp
// Project: core
// Created Date: 07/11/2022
// Author: Shun Suzuki
// -----
// Last Modified: 04/01/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <cstdint>

#include "autd3/core/datagram.hpp"

namespace autd3::core {

/**
 * @brief Clear is a DatagramHeader for clear operation
 */
struct Clear final : DatagramHeader {
  Clear() noexcept = default;

  bool init() override { return true; }

  bool pack(const uint8_t, driver::TxDatagram& tx) override { return driver::Clear().pack(tx); }

  [[nodiscard]] bool is_finished() const override { return true; }
};

}  // namespace autd3::core
