// File: remote_twincat.cpp
// Project: examples
// Created Date: 12/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 31/01/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#include "autd3/link/remote_twincat.hpp"

#include "autd3.hpp"
#include "runner.hpp"
#include "util.hpp"

int main() try {
  auto geometry = autd3::Geometry::Builder()
                      .add_device(autd3::AUTD3(autd3::Vector3::Zero(), autd3::Vector3::Zero()))
                      .sound_speed(340.0e3)  // mm/s
                      .build();

  const std::string server_ams_net_id = "your TwinCATAUTDServer AMS net id (e.g. 172.16.99.2.1.1)";
  auto link = autd3::link::RemoteTwinCAT(server_ams_net_id).build();
  auto autd = autd3::Controller::open(std::move(geometry), std::move(link));

  return run(autd);
} catch (std::exception& e) {
  print_err(e);
  return -1;
}
