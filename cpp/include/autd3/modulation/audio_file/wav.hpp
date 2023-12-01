// File: wav.hpp
// Project: audio_file
// Created Date: 13/09/2023
// Author: Shun Suzuki
// -----
// Last Modified: 01/12/2023
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
class Wav final : public internal::ModulationWithSamplingConfig<Wav>,
                  public IntoCache<Wav>,
                  public IntoRadiationPressure<Wav>,
                  public IntoTransform<Wav> {
 public:
  /**
   * @brief Constructor
   *
   * @param path Path to wav file
   */
  explicit Wav(std::filesystem::path path) : _path(std::move(path)) {}

  [[nodiscard]] internal::native_methods::ModulationPtr modulation_ptr() const override {
    auto ptr = validate(internal::native_methods::AUTDModulationWav(_path.string().c_str()));
    if (_config.has_value())
      ptr = AUTDModulationWavWithSamplingConfig(ptr, static_cast<internal::native_methods::SamplingConfiguration>(_config.value()));
    return ptr;
  }

 private:
  std::filesystem::path _path;
};

}  // namespace autd3::modulation::audio_file
