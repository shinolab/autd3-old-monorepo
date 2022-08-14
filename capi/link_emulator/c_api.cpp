// File: c_api.cpp
// Project: link_emulator
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 12/08/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#include <autd3/controller.hpp>
#include <autd3/link/emulator.hpp>

#include "../base/wrapper_link.hpp"
#include "./emulator_link.h"

void AUTDLinkEmulator(void** out, const uint16_t port) {
  auto* link = link_create(autd3::link::Emulator().port(port).build());
  *out = link;
}
