// File: c_api.cpp
// Project: modulation_audio_file
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 30/05/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#include "./audio_file_modulation.h"
#include "autd3/modulation/audio_file.hpp"

void AUTDModulationRawPCM(void** mod, const char* filename, const double sampling_freq, const uint32_t mod_sampling_freq_div) {
  const auto filename_ = std::string(filename);
  auto* m = new autd3::modulation::RawPCM(filename_, sampling_freq, mod_sampling_freq_div);
  *mod = m;
}
void AUTDModulationWav(void** mod, const char* filename, const uint32_t mod_sampling_freq_div) {
  const auto filename_ = std::string(filename);
  auto* m = new autd3::modulation::Wav(filename_, mod_sampling_freq_div);
  *mod = m;
}
