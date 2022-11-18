// File: c_api.cpp
// Project: link_debug
// Created Date: 10/10/2022
// Author: Shun Suzuki
// -----
// Last Modified: 18/11/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#include "../base/wrapper_link.hpp"
#include "./debug_link.h"
#include "autd3/link/debug.hpp"
#include "autd3/spdlog.hpp"

EXPORT_AUTD void AUTDLinkDebug(void** out) {
  auto* link = link_create(autd3::link::Debug().build());
  *out = link;
}
EXPORT_AUTD void AUTDLinkDebugSetLevel(int32_t level) { spdlog::set_level(static_cast<spdlog::level::level_enum>(level)); }
