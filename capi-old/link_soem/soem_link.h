// File: soem_link.h
// Project: link_soem
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

typedef void (*OnLostCallback)(const char*);

#ifdef __cplusplus
extern "C" {
#endif
EXPORT_AUTD int32_t AUTDGetAdapterPointer(OUT void** out);
EXPORT_AUTD void AUTDGetAdapter(IN void* p_adapter, IN int32_t index, OUT char* desc, OUT char* name);
EXPORT_AUTD void AUTDFreeAdapterPointer(IN void* p_adapter);
EXPORT_AUTD void AUTDLinkSOEM(OUT void** out);
EXPORT_AUTD void AUTDLinkSOEMIfname(IN void* soem, IN const char* ifname);
EXPORT_AUTD void AUTDLinkSOEMBufSize(IN void* soem, IN uint64_t buf_size);
EXPORT_AUTD void AUTDLinkSOEMSync0Cycle(IN void* soem, IN uint16_t sync0_cycle);
EXPORT_AUTD void AUTDLinkSOEMSendCycle(IN void* soem, IN uint16_t send_cycle);
EXPORT_AUTD void AUTDLinkSOEMFreerun(IN void* soem, IN bool freerun);
EXPORT_AUTD void AUTDLinkSOEMOnLost(IN void* soem, IN void* on_lost);
EXPORT_AUTD void AUTDLinkSOEMTimerStrategy(IN void* soem, IN uint8_t timer_strategy);
EXPORT_AUTD void AUTDLinkSOEMStateCheckInterval(IN void* soem, IN uint64_t state_check_interval);
EXPORT_AUTD void AUTDLinkSOEMLogLevel(IN void* soem, IN int32_t level);
EXPORT_AUTD void AUTDLinkSOEMLogFunc(IN void* soem, IN void* out_func, IN void* flush_func);
EXPORT_AUTD void AUTDLinkSOEMTimeout(IN void* soem, IN uint64_t timeout_ns);
EXPORT_AUTD void AUTDLinkSOEMBuild(OUT void** out, IN void* soem);
#ifdef __cplusplus
}
#endif
