// File: mac.hpp
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

namespace autd3::link {

inline timespec ecat_setup(const int64_t cycletime_ns) {
  auto ts = timespec{0, 0};

  auto tp = timeval{0, 0};
  gettimeofday(&tp, nullptr);

  const auto cyctime_us = cycletime_ns / 1000;

  ts.tv_sec = tp.tv_sec;
  const auto ht = ((tp.tv_usec / cyctime_us) + 1) * cyctime_us;
  ts.tv_nsec = ht * 1000;
  return ts;
}

}  // namespace autd3::link
