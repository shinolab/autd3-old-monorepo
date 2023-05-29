// File: silencer.hpp
// Project: internal
// Created Date: 29/05/2023
// Author: Shun Suzuki
// -----
// Last Modified: 29/05/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include "autd3/internal/datagram.hpp"
#include "autd3/internal/native_methods.hpp"

namespace autd3::internal {

class SilencerConfig final : public Header {
 public:
  SilencerConfig() noexcept : SilencerConfig(10) {}
  explicit SilencerConfig(const uint16_t step) noexcept : Header(native_methods::AUTDCreateSilencer(step)) {}

  static SilencerConfig none() noexcept { return SilencerConfig(0xFFFF); }
};

}  // namespace autd3::internal
