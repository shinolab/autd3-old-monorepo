// File: remote_twincat_link.h
// Project: link_remote_twincat
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 28/04/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include "../base/header.hpp"

#ifdef __cplusplus
extern "C" {
#endif
EXPORT_AUTD void AUTDLinkRemoteTwinCAT(OUT void** out, IN const char* server_ams_net_id);
EXPORT_AUTD void AUTDLinkRemoteTwinCATServerIpAddr(IN void* remote_twincat, IN const char* server_ip_addr);
EXPORT_AUTD void AUTDLinkRemoteTwinCATClientAmsNetId(IN void* remote_twincat, IN const char* client_ams_net_id);
EXPORT_AUTD void AUTDLinkRemoteTwinCATLogLevel(IN void* remote_twincat, IN int32_t level);
EXPORT_AUTD void AUTDLinkRemoteTwinCATLogFunc(IN void* remote_twincat, IN void* out_func, IN void* flush_func);
EXPORT_AUTD void AUTDLinkRemoteTwinCATTimeout(IN void* remote_twincat, IN uint64_t timeout_ns);
EXPORT_AUTD void AUTDLinkRemoteTwinCATBuild(OUT void** out, IN void* remote_twincat);

#ifdef __cplusplus
}
#endif
