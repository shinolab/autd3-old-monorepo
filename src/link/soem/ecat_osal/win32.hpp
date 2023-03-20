// File: win.hpp
// Project: ecat_thread
// Created Date: 12/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 21/03/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <timeapi.h>

namespace autd3::link {

inline void gettimeofday(timeval* const tv, struct timezone* const) {
  FILETIME system_time;
  GetSystemTimePreciseAsFileTime(&system_time);

  int64_t system_time64 = (static_cast<int64_t>(system_time.dwHighDateTime) << 32) + static_cast<int64_t>(system_time.dwLowDateTime);
  system_time64 += -134774LL * 86400LL * 1000000LL * 10LL;
  const auto usecs = system_time64 / 10;

  tv->tv_sec = static_cast<long>(usecs / 1000000);                                        // NOLINT
  tv->tv_usec = static_cast<long>(usecs - static_cast<int64_t>(tv->tv_sec) * 1000000LL);  // NOLINT
}

inline timespec ecat_setup(const int64_t cycletime_ns) {
  auto tp = timeval{0, 0};
  gettimeofday(&tp, nullptr);

  const auto cycletime_us = cycletime_ns / 1000LL;

  const auto ht = (tp.tv_usec / cycletime_us + 1) * cycletime_us;
  return {tp.tv_sec, static_cast<long>(ht) * 1000L};  // NOLINT
}

}  // namespace autd3::link
