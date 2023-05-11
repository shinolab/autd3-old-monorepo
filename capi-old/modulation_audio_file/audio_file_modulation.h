// File: audio_file_modulation.h
// Project: modulation_audio_file
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 22/12/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#ifdef __cplusplus
#include <cstdint>
#endif

#include "../base/header.hpp"

#ifdef __cplusplus
extern "C" {
#endif
EXPORT_AUTD void AUTDModulationRawPCM(OUT void** mod, IN const char* filename, IN autd3_float_t sampling_freq, IN uint32_t mod_sampling_freq_div);
EXPORT_AUTD void AUTDModulationWav(OUT void** mod, IN const char* filename, IN uint32_t mod_sampling_freq_div);
#ifdef __cplusplus
}
#endif
