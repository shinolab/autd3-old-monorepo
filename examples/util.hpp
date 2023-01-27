// File: util.hpp
// Project: examples
// Created Date: 28/01/2023
// Author: Shun Suzuki
// -----
// Last Modified: 28/01/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <exception>

void print_err(std::exception& e) {
  std::cerr << "\033[91m"
            << "ERROR: " << e.what() << "\033[0m" << std::endl;
}
