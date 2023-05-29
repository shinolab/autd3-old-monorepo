// File: remote_soem.cpp
// Project: examples
// Created Date: 02/11/2022
// Author: Shun Suzuki
// -----
// Last Modified: 29/05/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
//

#include "autd3/link/soem.hpp"

#include "autd3.hpp"
#include "runner.hpp"
#include "util.hpp"

int main() try {
  auto geometry = autd3::Geometry::Builder()
                      .add_device(autd3::AUTD3(autd3::Vector3::Zero(), autd3::Vector3::Zero()))
                      .build();

  const std::string ip = "server ip here";
  constexpr uint16_t port = 0;  // SOEMAUTDServer port

  auto link = autd3::link::RemoteSOEM(ip, port).build();
  auto autd = autd3::Controller::open(std::move(geometry), std::move(link));

  return run(autd);
} catch (std::exception& e) {
  print_err(e);
  return -1;
}
