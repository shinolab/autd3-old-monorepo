// File: simulator.hpp
// Project: link
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 01/11/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <memory>

#include "autd3/core/link.hpp"

namespace autd3::link {

/**
 * \brief link for Simulator
 */
class Simulator {
 public:
  /**
   * @brief Constructor
   */
  Simulator() noexcept = default;
  ~Simulator() = default;
  Simulator(const Simulator& v) noexcept = delete;
  Simulator& operator=(const Simulator& obj) = delete;
  Simulator(Simulator&& obj) = default;
  Simulator& operator=(Simulator&& obj) = default;

  [[nodiscard]] core::LinkPtr build() const;
};

}  // namespace autd3::link
