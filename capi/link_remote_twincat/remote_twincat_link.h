// File: remote_twincat_link.h
// Project: link_remote_twincat
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 10/06/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include "../base/header.hpp"

#ifdef __cplusplus
extern "C" {
#endif
EXPORT_AUTD void AUTDLinkRemoteTwinCAT(OUT void** out, IN const char* remote_ip_addr, IN const char* remote_ams_net_id,
                                       IN const char* local_ams_net_id);
#ifdef __cplusplus
}
#endif
