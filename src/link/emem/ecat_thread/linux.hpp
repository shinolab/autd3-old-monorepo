// File: linux.hpp
// Project: ecat_thread
// Created Date: 08/02/2023
// Author: Shun Suzuki
// -----
// Last Modified: 08/02/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
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

  clock_gettime(CLOCK_MONOTONIC, &ts);

  const auto ht = ((ts.tv_nsec / cycletime_ns) + 1) * cycletime_ns;
  ts.tv_nsec = ht;

  return ts;
}

void timed_wait(const timespec& abs_time) {
  auto tleft = timespec{0, 0};
  clock_nanosleep(CLOCK_MONOTONIC, TIMER_ABSTIME, &abs_time, &tleft);
}

void timed_wait_h(const timespec& abs_time) {
  auto tleft = timespec{0, 0};
  clock_nanosleep(CLOCK_MONOTONIC, TIMER_ABSTIME, &abs_time, &tleft);
}

}  // namespace autd3::link
