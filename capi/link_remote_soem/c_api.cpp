// File: c_api.cpp
// Project: link_remote_soem
// Created Date: 03/11/2022
// Author: Shun Suzuki
// -----
// Last Modified: 03/11/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#include <autd3/link/remote_soem.hpp>

#include "../base/wrapper_link.hpp"
#include "./remote_soem_link.h"

void AUTDLinkRemoteSOEM(void** out, const char* ip, uint16_t port) {
  std::string ip_ = ip == nullptr ? std::string("") : std::string(ip);
  auto soem_link = autd3::link::RemoteSOEM().ip(ip_).port(port).build();
  auto* link = link_create(std::move(soem_link));
  *out = link;
}
