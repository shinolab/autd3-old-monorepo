// File: twincat.hpp
// Project: link
// Created Date: 12/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 28/04/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include "autd3/core/link.hpp"
#include "autd3/link/builder.hpp"

namespace autd3::link {
/**
 * @brief Link using TwinCAT
 */
class TwinCAT : public LinkBuilder<TwinCAT> {
 public:
  /**
   * @brief Constructor
   */
  TwinCAT() : LinkBuilder(core::Milliseconds(0)) {}
  ~TwinCAT() override = default;
  TwinCAT(const TwinCAT& v) noexcept = delete;
  TwinCAT& operator=(const TwinCAT& obj) = delete;
  TwinCAT(TwinCAT&& obj) = delete;
  TwinCAT& operator=(TwinCAT&& obj) = delete;

 protected:
  core::LinkPtr build_() override;
};
}  // namespace autd3::link
