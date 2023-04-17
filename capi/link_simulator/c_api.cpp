// File: c_api.cpp
// Project: link_simulator
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 17/04/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#include <autd3/link/simulator.hpp>

#include "../base/wrapper_link.hpp"
#include "./simulator_link.h"

void AUTDLinkSimulator(void** out, const uint64_t timeout_ns) {
  *out = link_create(autd3::link::Simulator().timeout(std::chrono::nanoseconds(timeout_ns)).build());
}
