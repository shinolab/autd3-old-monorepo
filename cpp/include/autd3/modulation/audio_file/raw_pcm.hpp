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
#include "autd3/internal/utils.hpp"
#include "autd3/modulation/cache.hpp"
#include "autd3/modulation/radiation_pressure.hpp"
#include "autd3/modulation/transform.hpp"

namespace autd3::modulation::audio_file {

/**
 * @brief Modulation constructed from wav file
 * @details The wav data is re-sampled to the sampling frequency of Modulation.
 */
class RawPCM final : public internal::ModulationWithSamplingConfig<RawPCM>,
                     public IntoCache<RawPCM>,
                     public IntoRadiationPressure<RawPCM>,
                     public IntoTransform<RawPCM> {
 public:
  /**
   * @brief Constructor
   *
   * @param path Path to wav file
   * @param sample_rate Sampling frequency of raw pcm file
   */
  explicit RawPCM(std::filesystem::path path, const uint32_t sample_rate) : _sample_rate(sample_rate), _path(std::move(path)) {}

  [[nodiscard]] internal::native_methods::ModulationPtr modulation_ptr() const override {
    auto ptr = validate(internal::native_methods::AUTDModulationRawPCM(_path.string().c_str(), _sample_rate));
    if (_config.has_value())
      ptr = AUTDModulationRawPCMWithSamplingConfig(ptr, static_cast<internal::native_methods::SamplingConfiguration>(_config.value()));
    return ptr;
  }

 private:
  uint32_t _sample_rate;
  std::filesystem::path _path;
};

}  // namespace autd3::modulation::audio_file
