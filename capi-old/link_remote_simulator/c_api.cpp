// File: c_api.cpp
// Project: link_remote_simulator
// Created Date: 01/05/2023
// Author: Shun Suzuki
// -----
// Last Modified: 01/05/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#include <autd3/link/remote_simulator.hpp>

#include "../base/wrapper_link.hpp"
#include "./remote_simulator_link.h"

void AUTDLinkRemoteSimulator(void** out, const char* const ip, const uint16_t port) {
  *out = new autd3::link::RemoteSimulator(std::string(ip), port);
}
void AUTDLinkRemoteSimulatorLogLevel(void* remote_simulator, int32_t level) {
  static_cast<autd3::link::RemoteSimulator*>(remote_simulator)->log_level(static_cast<autd3::driver::LogLevel>(level));
}
void AUTDLinkRemoteSimulatorLogFunc(void* remote_simulator, void* out_func, void* flush_func) {
  if (out_func != nullptr && flush_func != nullptr)
    static_cast<autd3::link::RemoteSimulator*>(remote_simulator)
        ->log_func([out_func](const std::string& msg) { reinterpret_cast<OutCallback>(out_func)(msg.c_str()); },
                   [flush_func] { reinterpret_cast<FlushCallback>(flush_func)(); });
}
void AUTDLinkRemoteSimulatorTimeout(void* remote_simulator, const uint64_t timeout_ns) {
  static_cast<autd3::link::RemoteSimulator*>(remote_simulator)->timeout(std::chrono::nanoseconds(timeout_ns));
}
void AUTDLinkRemoteSimulatorBuild(void** out, void* remote_simulator) {
  auto* builder = static_cast<autd3::link::RemoteSimulator*>(remote_simulator);
  auto* link = link_create(builder->build());
  delete builder;
  *out = link;
}
