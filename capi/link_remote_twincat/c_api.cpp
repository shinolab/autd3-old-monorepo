// File: c_api.cpp
// Project: link_remote_twincat
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 29/04/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#include "../base/wrapper_link.hpp"
#include "./remote_twincat_link.h"
#include "autd3/link/remote_twincat.hpp"

void AUTDLinkRemoteTwinCAT(void** out, const char* const server_ams_net_id) { *out = new autd3::link::RemoteTwinCAT(std::string(server_ams_net_id)); }
void AUTDLinkRemoteTwinCATServerIpAddr(void* remote_twincat, const char* server_ip_addr) {
  static_cast<autd3::link::RemoteTwinCAT*>(remote_twincat)->server_ip_address(std::string(server_ip_addr));
}
void AUTDLinkRemoteTwinCATClientAmsNetId(void* remote_twincat, const char* client_ams_net_id) {
  static_cast<autd3::link::RemoteTwinCAT*>(remote_twincat)->client_ams_net_id(std::string(client_ams_net_id));
}
void AUTDLinkRemoteTwinCATLogLevel(void* remote_twincat, int32_t level) {
  static_cast<autd3::link::RemoteTwinCAT*>(remote_twincat)->log_level(static_cast<autd3::driver::LogLevel>(level));
}
void AUTDLinkRemoteTwinCATLogFunc(void* remote_twincat, void* out_func, void* flush_func) {
  if (out_func != nullptr && flush_func != nullptr)
    static_cast<autd3::link::RemoteTwinCAT*>(remote_twincat)
        ->log_func([out_func](const std::string& msg) { reinterpret_cast<OutCallback>(out_func)(msg.c_str()); },
                   [flush_func] { reinterpret_cast<FlushCallback>(flush_func)(); });
}
void AUTDLinkRemoteTwinCATTimeout(void* remote_twincat, const uint64_t timeout_ns) {
  static_cast<autd3::link::RemoteTwinCAT*>(remote_twincat)->timeout(std::chrono::nanoseconds(timeout_ns));
}
void AUTDLinkRemoteTwinCATBuild(void** out, void* remote_twincat) {
  auto* builder = static_cast<autd3::link::RemoteTwinCAT*>(remote_twincat);
  auto* link = link_create(builder->build());
  delete builder;
  *out = link;
}
