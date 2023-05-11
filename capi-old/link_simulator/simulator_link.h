// File: simulator_link.h
// Project: link_simulator
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
EXPORT_AUTD void AUTDLinkSimulator(OUT void** out);
EXPORT_AUTD void AUTDLinkSimulatorLogLevel(IN void* simulator, IN int32_t level);
EXPORT_AUTD void AUTDLinkSimulatorLogFunc(IN void* simulator, IN void* out_func, IN void* flush_func);
EXPORT_AUTD void AUTDLinkSimulatorTimeout(IN void* simulator, IN uint64_t timeout_ns);
EXPORT_AUTD void AUTDLinkSimulatorBuild(OUT void** out, IN void* simulator);
#ifdef __cplusplus
}
#endif
