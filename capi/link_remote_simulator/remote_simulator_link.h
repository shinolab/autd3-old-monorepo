// File: remote_simulator_link.h
// Project: link_remote_simulator
// Created Date: 03/11/2022
// Author: Shun Suzuki
// -----
// Last Modified: 01/05/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include "../base/header.hpp"

#ifdef __cplusplus
extern "C" {
#endif
EXPORT_AUTD void AUTDLinkRemoteSimulator(OUT void** out, IN const char* ip, IN uint16_t port);
EXPORT_AUTD void AUTDLinkRemoteSimulatorLogLevel(IN void* remote_simulator, IN int32_t level);
EXPORT_AUTD void AUTDLinkRemoteSimulatorLogFunc(IN void* remote_simulator, IN void* out_func, IN void* flush_func);
EXPORT_AUTD void AUTDLinkRemoteSimulatorTimeout(IN void* remote_simulator, IN uint64_t timeout_ns);
EXPORT_AUTD void AUTDLinkRemoteSimulatorBuild(OUT void** out, IN void* remote_simulator);
#ifdef __cplusplus
}
#endif
