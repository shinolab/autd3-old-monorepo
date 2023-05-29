// File: modulation.hpp
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

class Modulation : public Header {
 public:
  Modulation(void* ptr) : Header(ptr) {}
  ~Modulation() {
    if (_ptr != nullptr) {
      native_methods::AUTDDeleteModulation(_ptr);
    }
  }

  uint32_t sampling_frequency_division() const { return native_methods::AUTDModulationSamplingFrequencyDivision(_ptr); }
  void set_sampling_frequency_division(const uint32_t div) const { native_methods::AUTDModulationSetSamplingFrequencyDivision(_ptr, div); }

  double sampling_frequency() const { return native_methods::AUTDModulationSamplingFrequency(_ptr); }
};

}  // namespace autd3::internal
