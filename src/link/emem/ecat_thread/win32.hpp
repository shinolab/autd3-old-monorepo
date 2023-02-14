// File: win32.hpp
// Project: ecat_thread
// Created Date: 08/02/2023
// Author: Shun Suzuki
// -----
// Last Modified: 10/02/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include "../ethercat/time_utils.hpp"
#include "autd3/core/utils/hint.hpp"

namespace autd3::link {

static LARGE_INTEGER performance_frequency{};

inline void ecat_init() {
  LARGE_INTEGER f;
  QueryPerformanceFrequency(&f);
  performance_frequency = f;
}

inline void nanosleep(const int64_t t) {
  LARGE_INTEGER start;
  QueryPerformanceCounter(&start);

  const auto sleep_for = t * performance_frequency.QuadPart / (1000LL * 1000LL * 1000LL) + start.QuadPart;

  LARGE_INTEGER now;
  QueryPerformanceCounter(&now);
  while (now.QuadPart <= sleep_for) {
    core::spin_loop_hint();
    QueryPerformanceCounter(&now);
  }
}

inline void add_timespec(timespec& ts, const int64_t addtime) {
  const auto nsec = addtime % 1000000000LL;
  const auto sec = (addtime - nsec) / 1000000000LL;
  ts.tv_sec += sec;
  ts.tv_nsec += static_cast<long>(nsec);  // NOLINT
  if (ts.tv_nsec >= 1000000000L) {
    const auto nsec_ = ts.tv_nsec % 1000000000L;
    ts.tv_sec += (static_cast<int64_t>(ts.tv_nsec) - nsec_) / 1000000000LL;
    ts.tv_nsec = nsec_;
  }
}

inline timespec ecat_setup(const int64_t cycletime_ns) {
  auto tp = timeval{0, 0};
  ethercat::gettimeofday_precise(&tp);

  const auto cycletime_us = cycletime_ns / 1000LL;

  const auto ht = (tp.tv_usec / cycletime_us + 1) * cycletime_us;
  return {tp.tv_sec, static_cast<long>(ht) * 1000L};  // NOLINT
}

inline void timed_wait(const timespec& abs_time) {
  auto tp = timeval{0, 0};
  ethercat::gettimeofday_precise(&tp);

  if (const auto sleep = (static_cast<int64_t>(abs_time.tv_sec) - static_cast<int64_t>(tp.tv_sec)) * 1000000000LL +
                         (static_cast<int64_t>(abs_time.tv_nsec) - static_cast<int64_t>(tp.tv_usec) * 1000LL);
      sleep > 0)
    std::this_thread::sleep_for(std::chrono::nanoseconds(sleep));
}

inline void timed_wait_h(const timespec& abs_time) {
  auto tp = timeval{0, 0};
  ethercat::gettimeofday_precise(&tp);

  const auto sleep = (static_cast<int64_t>(abs_time.tv_sec) - static_cast<int64_t>(tp.tv_sec)) * 1000000000LL +
                     (static_cast<int64_t>(abs_time.tv_nsec) - static_cast<int64_t>(tp.tv_usec) * 1000LL);

  nanosleep(sleep);
}

}  // namespace autd3::link
