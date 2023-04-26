// File: c_api.cpp
// Project: link_remote_soem
// Created Date: 03/11/2022
// Author: Shun Suzuki
// -----
// Last Modified: 17/04/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#include <autd3/link/remote_soem.hpp>

#include "../base/wrapper_link.hpp"
#include "./remote_soem_link.h"

void AUTDLinkRemoteSOEM(void** out, const char* ip, const uint16_t port, const uint64_t timeout_ns) {
  const std::string ip_ = ip == nullptr ? std::string("") : std::string(ip);
  auto soem_link = autd3::link::RemoteSOEM(ip_, port).timeout(std::chrono::nanoseconds(timeout_ns)).build();
  auto* link = link_create(std::move(soem_link));
  *out = link;
}
