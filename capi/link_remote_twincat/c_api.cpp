// File: c_api.cpp
// Project: link_remote_twincat
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 10/06/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#include "../base/wrapper_link.hpp"
#include "./remote_twincat_link.h"
#include "autd3/link/remote_twincat.hpp"

void AUTDLinkRemoteTwinCAT(void** out, const char* remote_ip_addr, const char* remote_ams_net_id, const char* local_ams_net_id) {
  const auto remote_ip_addr_ = std::string(remote_ip_addr);
  const auto remote_ams_net_id_ = std::string(remote_ams_net_id);
  const auto local_ams_net_id_ = std::string(local_ams_net_id);

  auto* link = link_create(autd3::link::RemoteTwinCAT(remote_ip_addr_, remote_ams_net_id_).local_ams_net_id(local_ams_net_id_).build());
  *out = link;
}
