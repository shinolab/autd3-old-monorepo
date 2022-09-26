// File: c_api.cpp
// Project: link_remote_twincat
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 26/09/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#include "../base/wrapper_link.hpp"
#include "./remote_twincat_link.h"
#include "autd3/link/remote_twincat.hpp"

void AUTDLinkRemoteTwinCAT(void** out, const char* server_ip_addr, const char* server_ams_net_id, const char* client_ams_net_id) {
  const auto server_ip = std::string(server_ip_addr);
  const auto server_ams_net_id_ = std::string(server_ams_net_id);
  const auto client_ams_net_id_ = std::string(client_ams_net_id);
  auto* link = link_create(autd3::link::RemoteTwinCAT(server_ams_net_id_).server_ip_address(server_ip).client_ams_net_id(client_ams_net_id_).build());
  *out = link;
}
