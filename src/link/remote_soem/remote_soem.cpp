// File: remote_soem.cpp
// Project: remote_soem
// Created Date: 26/10/2022
// Author: Shun Suzuki
// -----
// Last Modified: 17/04/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#include "autd3/link/remote_soem.hpp"

#include "remote_soem_tcp.hpp"

//
#include "remote_soem_local.hpp"

namespace autd3::link {
core::LinkPtr RemoteSOEM::build() {
  if (_ip.empty() || _ip == "127.0.0.1" || _ip == "localhost") return std::make_unique<RemoteSOEMLocal>(_timeout);
  return std::make_unique<RemoteSOEMTcp>(_timeout, _ip, _port);
}

}  // namespace autd3::link
