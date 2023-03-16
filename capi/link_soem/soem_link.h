// File: soem_link.h
// Project: link_soem
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 16/03/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include "../base/header.hpp"

typedef void (*OnLostCallback)(const char*);

#ifdef __cplusplus
extern "C" {
#endif
EXPORT_AUTD int32_t AUTDGetAdapterPointer(OUT void** out);
EXPORT_AUTD void AUTDGetAdapter(IN void* p_adapter, IN int32_t index, OUT char* desc, OUT char* name);
EXPORT_AUTD void AUTDFreeAdapterPointer(IN void* p_adapter);
EXPORT_AUTD void AUTDLinkSOEM(OUT void** out, IN const char* ifname, IN uint16_t sync0_cycle, IN uint16_t send_cycle, IN bool freerun,
                              IN void* on_lost, IN bool high_precision, IN uint64_t state_check_interval, IN int32_t level, IN const void* out_func,
                              IN void* flush_func);
#ifdef __cplusplus
}
#endif
