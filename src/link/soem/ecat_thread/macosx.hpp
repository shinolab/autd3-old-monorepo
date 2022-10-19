// File: mac.hpp
// Project: ecat_thread
// Created Date: 12/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 19/10/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

namespace autd3::link {

inline void ecat_init() {}

inline void add_timespec(timespec& ts, const int64_t addtime) {
  const auto nsec = addtime % 1000000000;
  const auto sec = (addtime - nsec) / 1000000000;
  ts.tv_sec += sec;
  ts.tv_nsec += nsec;
  if (ts.tv_nsec >= 1000000000) {
    const auto nsec_ = ts.tv_nsec % 1000000000;
    ts.tv_sec += (ts.tv_nsec - nsec_) / 1000000000;
    ts.tv_nsec = nsec_;
  }
}

timespec ecat_setup(const int64_t cycletime_ns) {
  auto ts = timespec{0, 0};

  auto tp = timeval{0, 0};
  gettimeofday(&tp, nullptr);

  const auto cyctime_us = cycletime_ns / 1000;

  ts.tv_sec = tp.tv_sec;
  const auto ht = ((tp.tv_usec / cyctime_us) + 1) * cyctime_us;
  ts.tv_nsec = ht * 1000;
  return ts;
}

void timed_wait(const timespec& abs_time) {
  auto tp = timeval{0, 0};
  gettimeofday(&tp, nullptr);
  const auto sleep = (abs_time.tv_sec - tp.tv_sec) * 1000000000 + (abs_time.tv_nsec - tp.tv_usec * 1000);
  auto tc = timespec{0, sleep};
  if (sleep > 0) nanosleep(&tc, nullptr);
}

void timed_wait_h(const timespec& abs_time) {
  auto tp = timeval{0, 0};
  gettimeofday(&tp, nullptr);
  const auto sleep = (abs_time.tv_sec - tp.tv_sec) * 1000000000 + (abs_time.tv_nsec - tp.tv_usec * 1000);
  auto tc = timespec{0, sleep};
  if (sleep > 0) nanosleep(&tc, nullptr);
}
}  // namespace autd3::link
