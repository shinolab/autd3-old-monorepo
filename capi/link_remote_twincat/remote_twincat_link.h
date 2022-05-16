// File: remote_twincat_link.h
// Project: link_remote_twincat
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

#ifdef __cplusplus
extern "C" {
#endif
EXPORT_AUTD void AUTDLinkRemoteTwinCAT(void** out, const char* remote_ip_addr, const char* remote_ams_net_id, const char* local_ams_net_id,
                                       uint16_t cycle_ticks);
#ifdef __cplusplus
}
#endif
