// File: clear.hpp
// Project: core
// Created Date: 07/11/2022
// Author: Shun Suzuki
// -----
// Last Modified: 18/11/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <cstdint>

#include "autd3/core/interface.hpp"

namespace autd3::core {

/**
 * @brief Clear
 */
struct Clear final : DatagramHeader {
  Clear() noexcept = default;

  bool init() override { return true; }

  bool pack(const std::unique_ptr<const driver::Driver>& driver, const uint8_t, driver::TxDatagram& tx) override {
    driver->clear(tx);
    return true;
  }

  bool is_finished() const override { return true; }
};

}  // namespace autd3::core
