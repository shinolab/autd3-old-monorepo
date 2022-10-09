// File: c_api.cpp
// Project: link_simulator
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 09/10/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#include <autd3/link/simulator.hpp>

#include "../base/wrapper_link.hpp"
#include "./simulator_link.h"

void AUTDLinkSimulator(void** out, const uint16_t port, const char* ip_addr) {
  *out = ip_addr == nullptr ? link_create(autd3::link::Simulator().port(port).build())
                            : link_create(autd3::link::Simulator().port(port).ip_addr(std::string(ip_addr)).build());
}
