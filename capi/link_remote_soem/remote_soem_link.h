// File: remote_soem_link.h
// Project: link_remote_soem
// Created Date: 03/11/2022
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
EXPORT_AUTD void AUTDLinkRemoteSOEM(OUT void** out, IN const char* ip, IN uint16_t port);
EXPORT_AUTD void AUTDLinkRemoteSOEMServerIpAddr(IN void* remote_soem, IN const char* server_ip_addr);
EXPORT_AUTD void AUTDLinkRemoteSOEMClientAmsNetId(IN void* remote_soem, IN const char* client_ams_net_id);
EXPORT_AUTD void AUTDLinkRemoteSOEMLogLevel(IN void* remote_soem, IN int32_t level);
EXPORT_AUTD void AUTDLinkRemoteSOEMLogFunc(IN void* remote_soem, IN void* out_func, IN void* flush_func);
EXPORT_AUTD void AUTDLinkRemoteSOEMTimeout(IN void* remote_soem, IN uint64_t timeout_ns);
EXPORT_AUTD void AUTDLinkRemoteSOEMBuild(OUT void** out, IN void* remote_soem);
#ifdef __cplusplus
}
#endif
