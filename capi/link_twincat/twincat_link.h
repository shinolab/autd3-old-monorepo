// File: twincat_link.h
// Project: link_twincat
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
EXPORT_AUTD void AUTDLinkTwinCAT(OUT void** out);
EXPORT_AUTD void AUTDLinkTwinCATLogLevel(IN void* twincat, IN int32_t level);
EXPORT_AUTD void AUTDLinkTwinCATLogFunc(IN void* twincat, IN void* out_func, IN void* flush_func);
EXPORT_AUTD void AUTDLinkTwinCATTimeout(IN void* twincat, IN uint64_t timeout_ns);
EXPORT_AUTD void AUTDLinkTwinCATBuild(OUT void** out, IN void* twincat);
#ifdef __cplusplus
}
#endif
