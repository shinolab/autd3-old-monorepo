// File: c_api.cpp
// Project: link_twincat
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 28/04/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#include "../base/wrapper_link.hpp"
#include "./twincat_link.h"
#include "autd3/link/twincat.hpp"

void AUTDLinkTwinCAT(void** out) { *out = new autd3::link::TwinCAT; }
void AUTDLinkTwinCATLogLevel(void* twincat, const int32_t level) {
  static_cast<autd3::link::TwinCAT*>(twincat)->log_level(static_cast<autd3::driver::DebugLevel>(level));
}
void AUTDLinkTwinCATLogFunc(void* twincat, void* out_func, void* flush_func) {
  if (out_func != nullptr && flush_func != nullptr)
    static_cast<autd3::link::TwinCAT*>(twincat)->log_func(
        [out_func](const std::string& msg) { reinterpret_cast<OutCallback>(out_func)(msg.c_str()); },
        [flush_func] { reinterpret_cast<FlushCallback>(flush_func)(); });
}
void AUTDLinkTwinCATTimeout(void* twincat, const uint64_t timeout_ns) {
  static_cast<autd3::link::TwinCAT*>(twincat)->timeout(std::chrono::nanoseconds(timeout_ns));
}
void AUTDLinkTwinCATBuild(void** out, void* twincat) {
  auto* builder = static_cast<autd3::link::TwinCAT*>(twincat);
  auto* link = link_create(builder->build());
  delete builder;
  *out = link;
}
