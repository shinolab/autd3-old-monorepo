// File: audio_file_modulation.h
// Project: modulation_audio_file
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 16/05/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Hapis Lab. All rights reserved.
//

#pragma once

#include "../base/header.h"

#ifdef __cplusplus
extern "C" {
#endif
EXPORT_AUTD void AUTDModulationRawPCM(void** mod, const char* filename, double sampling_freq, uint16_t mod_sampling_freq_div);
EXPORT_AUTD void AUTDModulationWav(void** mod, const char* filename, uint16_t mod_sampling_freq_div);
#ifdef __cplusplus
}
#endif
