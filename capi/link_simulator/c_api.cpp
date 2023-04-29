// File: c_api.cpp
// Project: link_simulator
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 28/04/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#include <autd3/link/simulator.hpp>

#include "../base/wrapper_link.hpp"
#include "./simulator_link.h"

void AUTDLinkSimulator(void** out) { *out = new autd3::link::Simulator; }
void AUTDLinkSimulatorLogLevel(void* simulator, const int32_t level) {
  static_cast<autd3::link::Simulator*>(simulator)->log_level(static_cast<autd3::driver::DebugLevel>(level));
}
void AUTDLinkSimulatorLogFunc(void* simulator, void* out_func, void* flush_func) {
  if (out_func != nullptr && flush_func != nullptr)
    static_cast<autd3::link::Simulator*>(simulator)->log_func(
        [out_func](const std::string& msg) { reinterpret_cast<OutCallback>(out_func)(msg.c_str()); },
        [flush_func] { reinterpret_cast<FlushCallback>(flush_func)(); });
}
void AUTDLinkSimulatorTimeout(void* simulator, const uint64_t timeout_ns) {
  static_cast<autd3::link::Simulator*>(simulator)->timeout(std::chrono::nanoseconds(timeout_ns));
}
void AUTDLinkSimulatorBuild(void** out, void* simulator) {
  auto* builder = static_cast<autd3::link::Simulator*>(simulator);
  auto* link = link_create(builder->build());
  delete builder;
  *out = link;
}
