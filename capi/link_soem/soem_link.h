// File: soem_link.h
// Project: link_soem
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 16/05/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Hapis Lab. All rights reserved.
//

#pragma once

#include "../base/header.h"

typedef void (*OnLostCallback)(const char*);

#ifdef __cplusplus
extern "C" {
#endif
EXPORT_AUTD int32_t AUTDGetAdapterPointer(void** out);
EXPORT_AUTD void AUTDGetAdapter(void* p_adapter, int32_t index, char* desc, char* name);
EXPORT_AUTD void AUTDFreeAdapterPointer(void* p_adapter);
EXPORT_AUTD void AUTDLinkSOEM(void** out, const char* ifname, int32_t device_num, uint32_t cycle_ticks, void* on_lost, bool high_precision);
#ifdef __cplusplus
}
#endif
