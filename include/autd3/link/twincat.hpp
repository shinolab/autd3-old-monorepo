// File: twincat.hpp
// Project: link
// Created Date: 12/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 30/05/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include "autd3/core/link.hpp"

namespace autd3::link {
/**
 * @brief Link using TwinCAT
 */
class TwinCAT {
 public:
  /**
   * @brief Create TwinCAT link
   */
  core::LinkPtr build();

  /**
   * @brief Constructor
   * @param cycle_ticks This value must be the same as settings of AUTDServer.
   */
  explicit TwinCAT(const uint16_t cycle_ticks = 2) : _cycle_ticks(cycle_ticks) {}
  ~TwinCAT() = default;
  TwinCAT(const TwinCAT& v) noexcept = delete;
  TwinCAT& operator=(const TwinCAT& obj) = delete;
  TwinCAT(TwinCAT&& obj) = delete;
  TwinCAT& operator=(TwinCAT&& obj) = delete;

 private:
  uint16_t _cycle_ticks;
};
}  // namespace autd3::link
