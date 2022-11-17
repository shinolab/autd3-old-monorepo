// File: c_api.cpp
// Project: link_debug
// Created Date: 10/10/2022
// Author: Shun Suzuki
// -----
// Last Modified: 17/11/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#include "../base/wrapper_link.hpp"
#include "./debug_link.h"
#include "autd3/link/debug.hpp"

#if _MSC_VER
#pragma warning(push)
#pragma warning(disable : 6285 6385 26437 26800 26498 26451 26495 26450)
#endif
#if defined(__GNUC__) && !defined(__llvm__)
#pragma GCC diagnostic push
#endif
#ifdef __clang__
#pragma clang diagnostic push
#endif
#include <spdlog/spdlog.h>
#if _MSC_VER
#pragma warning(pop)
#endif
#if defined(__GNUC__) && !defined(__llvm__)
#pragma GCC diagnostic pop
#endif
#ifdef __clang__
#pragma clang diagnostic pop
#endif

EXPORT_AUTD void AUTDLinkDebug(void** out) {
  auto* link = link_create(autd3::link::Debug().build());
  *out = link;
}
EXPORT_AUTD void AUTDLinkDebugSetLevel(int32_t level) { spdlog::set_level(static_cast<spdlog::level::level_enum>(level)); }
