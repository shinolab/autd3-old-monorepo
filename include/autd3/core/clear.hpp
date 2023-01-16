// File: clear.hpp
// Project: core
// Created Date: 07/11/2022
// Author: Shun Suzuki
// -----
// Last Modified: 16/01/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include "autd3/core/datagram.hpp"
#include "autd3/driver/operation/clear.hpp"

namespace autd3::core {

/**
 * @brief Clear is a DatagramHeader for clear operation
 */
struct Clear final : DatagramHeader {
  Clear() noexcept = default;

  std::unique_ptr<driver::Operation> operation() override { return std::make_unique<driver::Clear>(); }

}  // namespace autd3::core
