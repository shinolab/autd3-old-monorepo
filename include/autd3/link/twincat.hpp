// File: twincat.hpp
// Project: link
// Created Date: 12/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 10/06/2022
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
  core::LinkPtr build() const;

  /**
   * @brief Constructor
   */
  TwinCAT() = default;
  ~TwinCAT() = default;
  TwinCAT(const TwinCAT& v) noexcept = delete;
  TwinCAT& operator=(const TwinCAT& obj) = delete;
  TwinCAT(TwinCAT&& obj) = delete;
  TwinCAT& operator=(TwinCAT&& obj) = delete;
};
}  // namespace autd3::link
