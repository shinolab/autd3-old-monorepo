// File: c_api.cpp
// Project: link_remote_soem
// Created Date: 03/11/2022
// Author: Shun Suzuki
// -----
// Last Modified: 28/04/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#include <autd3/link/remote_soem.hpp>

#include "../base/wrapper_link.hpp"
#include "./remote_soem_link.h"

void AUTDLinkRemoteSOEM(void** out, const char* const ip, const uint16_t port) { *out = new autd3::link::RemoteSOEM(std::string(ip), port); }
void AUTDLinkRemoteSOEMLogLevel(void* remote_soem, int32_t level) {
  static_cast<autd3::link::RemoteSOEM*>(remote_soem)->log_level(static_cast<autd3::driver::DebugLevel>(level));
}
void AUTDLinkRemoteSOEMLogFunc(void* remote_soem, void* out_func, void* flush_func) {
  if (out_func != nullptr && flush_func != nullptr)
    static_cast<autd3::link::RemoteSOEM*>(remote_soem)
        ->log_func([out_func](const std::string& msg) { reinterpret_cast<OutCallback>(out_func)(msg.c_str()); },
                   [flush_func] { reinterpret_cast<FlushCallback>(flush_func)(); });
}
void AUTDLinkRemoteSOEMTimeout(void* remote_soem, const uint64_t timeout_ns) {
  static_cast<autd3::link::RemoteSOEM*>(remote_soem)->timeout(std::chrono::nanoseconds(timeout_ns));
}
void AUTDLinkRemoteSOEMBuild(void** out, void* remote_soem) {
  auto* builder = static_cast<autd3::link::RemoteSOEM*>(remote_soem);
  auto* link = link_create(builder->build());
  delete builder;
  *out = link;
}
