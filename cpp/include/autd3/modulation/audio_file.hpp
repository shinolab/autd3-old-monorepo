// File: audio_file.hpp
// Project: modulation
// Created Date: 29/05/2023
// Author: Shun Suzuki
// -----
// Last Modified: 13/09/2023
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

/**
 * @brief Modulation constructed from wav file
 * @details The wav data is resampled to the sampling frequency of Modulation.
 */
class Wav final : public internal::Modulation {
 public:
  /**
   * @brief Constructor
   *
   * @param path Path to wav file
   */
  explicit Wav(std::filesystem::path path) : _path(std::move(path)) {}

  AUTD3_IMPL_WITH_CACHE_MODULATION
  AUTD3_IMPL_WITH_RADIATION_PRESSURE
  AUTD3_IMPL_WITH_TRANSFORM_MODULATION

  AUTD3_IMPL_MOD_PROP(Wav)

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
