// File: c_api.cpp
// Project: link_emulator
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 16/05/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Hapis Lab. All rights reserved.
//

#include <autd3/controller.hpp>
#include <autd3/link/emulator.hpp>

#include "../base/wrapper_link.hpp"
#include "./emulator_link.h"
#include "autd3/core/geometry/legacy_transducer.hpp"
#include "autd3/core/geometry/normal_transducer.hpp"

void AUTDLinkEmulator(void** out, const uint16_t port, const void* cnt) {
  const auto* const p_cnt = static_cast<const autd3::Controller<autd3::core::NormalTransducer>*>(cnt);
  auto* link = link_create(autd3::link::Emulator(p_cnt->geometry()).port(port).build());
  *out = link;
}

void AUTDLinkEmulatorLegacy(void** out, const uint16_t port, const void* cnt) {
  const auto* const p_cnt = static_cast<const autd3::Controller<autd3::core::LegacyTransducer>*>(cnt);
  auto* link = link_create(autd3::link::Emulator(p_cnt->geometry()).port(port).build());
  *out = link;
}
