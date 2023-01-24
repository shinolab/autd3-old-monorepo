// File: audio_file.hpp
// Project: modulation
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 24/01/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <filesystem>
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
   * @param[in] mod_sampling_freq_div sampling frequency division ratio of the Modulation
   * @details The sampling frequency in AUTD device will be autd3::driver::FPGA_CLK_FREQ / mod_sampling_freq_div.
   */
  explicit RawPCM(std::filesystem::path filename, driver::autd3_float_t sampling_freq, uint32_t mod_sampling_freq_div = 40960);

  std::vector<driver::Amp> calc() override;

  ~RawPCM() override = default;
  RawPCM(const RawPCM& v) noexcept = delete;
  RawPCM& operator=(const RawPCM& obj) = delete;
  RawPCM(RawPCM&& obj) = default;
  RawPCM& operator=(RawPCM&& obj) = default;

 private:
  std::filesystem::path _filename;
  driver::autd3_float_t _sampling_freq;
};

/**
 * @brief Modulation created from wav file
 */
class Wav final : public core::Modulation {
 public:
  /**
   * @param[in] filename file path to wav data
   * @param[in] mod_sampling_freq_div sampling frequency division ratio of the Modulation
   * @details The sampling frequency in AUTD device will be autd3::driver::FPGA_CLK_FREQ / mod_sampling_freq_div.
   */
  explicit Wav(std::filesystem::path filename, uint32_t mod_sampling_freq_div = 40960);

  std::vector<driver::Amp> calc() override;

  ~Wav() override = default;
  Wav(const Wav& v) noexcept = delete;
  Wav& operator=(const Wav& obj) = delete;
  Wav(Wav&& obj) = default;
  Wav& operator=(Wav&& obj) = default;

 private:
  std::filesystem::path _filename;
};
}  // namespace autd3::modulation
