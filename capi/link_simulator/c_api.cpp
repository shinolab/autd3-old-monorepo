// File: c_api.cpp
// Project: link_simulator
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 05/10/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#include <autd3/link/simulator.hpp>

#include "../base/wrapper_link.hpp"
#include "./simulator_link.h"

void AUTDLinkSimulator(void** out) {
  auto* link = link_create(autd3::link::Simulator().build());
  *out = link;
}
