// File: wav.hpp
// Project: audio_file
// Created Date: 13/09/2023
// Author: Shun Suzuki
// -----
// Last Modified: 10/10/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <chrono>
#include <filesystem>

#include "autd3/internal/native_methods.hpp"
#include "autd3/modulation/cache.hpp"
#include "autd3/modulation/radiation_pressure.hpp"
#include "autd3/modulation/transform.hpp"

namespace autd3::modulation::audio_file {

/**
 * @brief Modulation constructed from wav file
 * @details The wav data is re-sampled to the sampling frequency of Modulation.
 */
class Wav final : public internal::ModulationWithFreqDiv<Wav>, public IntoCache<Wav>, public IntoRadiationPressure<Wav>, public IntoTransform<Wav> {
 public:
  /**
   * @brief Constructor
   *
   * @param path Path to wav file
   */
  explicit Wav(std::filesystem::path path) : _path(std::move(path)) {}

  [[nodiscard]] internal::native_methods::ModulationPtr modulation_ptr() const override {
    char err[256]{};
    auto ptr = internal::native_methods::AUTDModulationWav(_path.string().c_str(), err);
    if (ptr._0 == nullptr) throw internal::AUTDException(err);
    if (_freq_div.has_value()) ptr = AUTDModulationWavWithSamplingFrequencyDivision(ptr, _freq_div.value());
    return ptr;
  }

 private:
  std::filesystem::path _path;
};

}  // namespace autd3::modulation::audio_file
