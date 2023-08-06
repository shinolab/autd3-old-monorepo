// File: silencer.hpp
// Project: internal
// Created Date: 29/05/2023
// Author: Shun Suzuki
// -----
// Last Modified: 06/08/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include "autd3/internal/datagram.hpp"
#include "autd3/internal/native_methods.hpp"

namespace autd3::internal {

/**
 * @brief Header to configure silencer
 */
class SilencerConfig final : public Header {
 public:
  SilencerConfig() noexcept : SilencerConfig(10) {}
  /**
   * @brief Constructor
   *
   * @param step Update step of silencer. The smaller `step` is, the quieter the output is.
   */
  explicit SilencerConfig(const uint16_t step) noexcept : Header(), _step(step) {}

  /**
   * @brief Disable silencer
   */
  static SilencerConfig none() noexcept { return SilencerConfig(0xFFFF); }

  [[nodiscard]] native_methods::DatagramHeaderPtr ptr() const override { return native_methods::AUTDCreateSilencer(_step); }

 private:
  uint16_t _step;
};

}  // namespace autd3::internal
