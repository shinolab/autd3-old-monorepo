// File: c_api.cpp
// Project: link_debug
// Created Date: 10/10/2022
// Author: Shun Suzuki
// -----
// Last Modified: 13/01/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#include "../../src/spdlog.hpp"
#include "../base/wrapper_link.hpp"
#include "./debug_link.h"
#include "autd3/link/debug.hpp"

typedef void (*OutCallback)(const char*);
typedef void (*FlushCallback)();

EXPORT_AUTD void AUTDLinkDebug(void** out, const int32_t level, const void* out_func, void* flush_func) {
  std::function<void(std::string)> out_f = nullptr;
  std::function<void()> flush_f = nullptr;
  if (out_func != nullptr) out_f = [out](const std::string& msg) { reinterpret_cast<OutCallback>(out)(msg.c_str()); };
  if (flush_func != nullptr) flush_f = [flush_func] { reinterpret_cast<FlushCallback>(flush_func)(); };
  auto* link =
      link_create(autd3::link::Debug().level(static_cast<autd3::link::DebugLevel>(level)).log_func(std::move(out_f), std::move(flush_f)).build());
  *out = link;
}
