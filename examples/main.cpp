// File: main.cpp
// Project: examples
// Created Date: 10/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 10/05/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Hapis Lab. All rights reserved.
//

#include <autd3/driver/firmware_version.hpp>
#include <iostream>

int main() {
  std::cout << autd3::driver::FirmwareInfo(0, 0, 0, 0) << std::endl;

  return 0;
}
