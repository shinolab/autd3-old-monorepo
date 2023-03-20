// File: unix.hpp
// Project: ecat_thread
// Created Date: 12/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 19/03/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <sys/time.h>

namespace autd3::link {

timespec ecat_setup(const int64_t cycletime_ns) {
  auto ts = timespec{0, 0};

  clock_gettime(CLOCK_MONOTONIC, &ts);

  const auto ht = ((ts.tv_nsec / cycletime_ns) + 1) * cycletime_ns;
  ts.tv_nsec = ht;

  return ts;
}

}  // namespace autd3::link
