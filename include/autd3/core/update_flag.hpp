// File: update_flag.hpp
// Project: core
// Created Date: 13/03/2023
// Author: Shun Suzuki
// -----
// Last Modified: 17/04/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <memory>

#include "autd3/core/datagram.hpp"

namespace autd3::core {

/**
 * @brief UpdateFlag is a SpecialData to update flag
 */
struct UpdateFlag final : SpecialData {
  UpdateFlag() noexcept : SpecialData(std::make_unique<NullHeader>(), std::make_unique<NullBody>()) {}
};

}  // namespace autd3::core
