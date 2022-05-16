// File: audio_file.hpp
// Project: modulation
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 16/05/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Hapis Lab. All rights reserved.
//

#pragma once

#include <string>
#include <vector>

#include "autd3/core/modulation.hpp"

namespace autd3::modulation {
/**
 * @brief Modulation created from raw pcm data
 */
class RawPCM final : public core::Modulation {
 public:
  /**
   * @param[in] filename file path to raw pcm data
   * @param[in] sampling_freq sampling frequency of the data
   * @param[in] mod_sampling_freq_div sampling frequency of the Modulation
   * @details The sampling frequency of AUTD is shown in autd::MOD_SAMPLING_FREQ, and it is not possible to modulate beyond the Nyquist frequency.
   * No modulation beyond the Nyquist frequency can be produced.
   * If samplingFreq is less than the Nyquist frequency, the data will be up-sampled.
   * The maximum modulation buffer size is shown in autd::MOD_BUF_SIZE. Only the data up to MOD_BUF_SIZE/MOD_SAMPLING_FREQ seconds can be output.
   */
  explicit RawPCM(const std::string& filename, double sampling_freq, uint16_t mod_sampling_freq_div = 40960);

  void calc() override;

  ~RawPCM() override = default;
  RawPCM(const RawPCM& v) noexcept = delete;
  RawPCM& operator=(const RawPCM& obj) = delete;
  RawPCM(RawPCM&& obj) = default;
  RawPCM& operator=(RawPCM&& obj) = default;

 private:
  double _sampling_freq;
  std::vector<uint8_t> _buf;
};

/**
 * @brief Modulation created from wav file
 */
class Wav final : public core::Modulation {
 public:
  /**
   * @param[in] filename file path to wav data
   * @param[in] mod_sampling_freq_div sampling frequency of the Modulation
   * @details The sampling frequency of AUTD is shown in autd::MOD_SAMPLING_FREQ, and it is not possible to modulate beyond the Nyquist frequency.
   * No modulation beyond the Nyquist frequency can be produced.
   * If samplingFreq is less than the Nyquist frequency, the data will be up-sampled.
   * The maximum modulation buffer size is shown in autd::MOD_BUF_SIZE. Only the data up to MOD_BUF_SIZE/MOD_SAMPLING_FREQ seconds can be output.
   */
  explicit Wav(const std::string& filename, uint16_t mod_sampling_freq_div = 40960);

  void calc() override;

  ~Wav() override = default;
  Wav(const Wav& v) noexcept = delete;
  Wav& operator=(const Wav& obj) = delete;
  Wav(Wav&& obj) = default;
  Wav& operator=(Wav&& obj) = default;

 private:
  uint32_t _sampling_freq;
  std::vector<uint8_t> _buf;
};
}  // namespace autd3::modulation
