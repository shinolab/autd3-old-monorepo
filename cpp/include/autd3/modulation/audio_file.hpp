// File: audio_file.hpp
// Project: modulation
// Created Date: 29/05/2023
// Author: Shun Suzuki
// -----
// Last Modified: 19/07/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <filesystem>

#include "autd3/internal/exception.hpp"
#include "autd3/internal/modulation.hpp"
#include "autd3/internal/native_methods.hpp"

namespace autd3::modulation {

class Wav final : public internal::Modulation {
 public:
  explicit Wav(std::filesystem::path path) : _path(std::move(path)) {}

  Wav with_sampling_frequency_division(const uint32_t div) {
    _freq_div = div;
    return *this;
  }

  Wav with_sampling_frequency(const double freq) {
    return with_sampling_frequency_division(static_cast<uint32_t>(static_cast<double>(internal::native_methods::FPGA_SUB_CLK_FREQ) / freq));
  }

  [[nodiscard]] internal::native_methods::ModulationPtr modulation_ptr() const override {
    char err[256]{};
    auto ptr = internal::native_methods::AUTDModulationWav(_path.string().c_str(), err);
    if (ptr._0 == nullptr) throw internal::AUTDException(err);
    if (_freq_div.has_value()) ptr = AUTDModulationWavWithSamplingFrequencyDivision(ptr, _freq_div.value());
    return ptr;
  }

 private:
  std::filesystem::path _path;
  std::optional<uint32_t> _freq_div;
};

}  // namespace autd3::modulation
