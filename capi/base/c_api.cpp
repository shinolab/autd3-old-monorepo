// File: c_api.cpp
// Project: base
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 16/05/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Hapis Lab. All rights reserved.
//

#include "autd3.hpp"

using T = autd3::NormalTransducer;
#include "impl.hpp"

void AUTDSetTransFrequency(void* const handle, const int32_t device_idx, const int32_t local_trans_idx, const double frequency) {
  auto* const wrapper = static_cast<Controller*>(handle);
  wrapper->geometry()[device_idx][local_trans_idx].set_frequency(frequency);
}
void AUTDSetTransCycle(void* const handle, const int32_t device_idx, const int32_t local_trans_idx, const uint16_t cycle) {
  auto* const wrapper = static_cast<Controller*>(handle);
  wrapper->geometry()[device_idx][local_trans_idx].set_cycle(cycle);
}
