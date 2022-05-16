// File: c_api.cpp
// Project: base_legacy
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 16/05/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Hapis Lab. All rights reserved.
//

#include "autd3.hpp"

using T = autd3::LegacyTransducer;
#include "../base/impl.hpp"

void AUTDSetTransFrequency(void*, int32_t, int32_t, double) {}
void AUTDSetTransCycle(void*, int32_t, int32_t, uint16_t) {}
