// File: remote_twincat_link.h
// Project: link_remote_twincat
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 26/09/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include "../base/header.hpp"

#ifdef __cplusplus
extern "C" {
#endif
EXPORT_AUTD void AUTDLinkRemoteTwinCAT(OUT void** out, IN const char* server_ip_addr, IN const char* server_ams_net_id,
                                       IN const char* client_ams_net_id);
#ifdef __cplusplus
}
#endif
