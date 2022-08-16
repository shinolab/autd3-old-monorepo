// File: win.hpp
// Project: ecat_thread
// Created Date: 12/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 16/08/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

namespace autd3::link {

static LARGE_INTEGER PERFORMANCE_FREQUENCY{};

inline void ecat_init() {
  LARGE_INTEGER f;
  QueryPerformanceFrequency(&f);
  PERFORMANCE_FREQUENCY = f;
}

inline void gettimeofday_precise(struct timeval* const tv, struct timezone* const tz) {
  FILETIME system_time;
  GetSystemTimePreciseAsFileTime(&system_time);

  int64_t system_time64 = (static_cast<int64_t>(system_time.dwHighDateTime) << 32) + static_cast<int64_t>(system_time.dwLowDateTime);
  system_time64 += -134774LL * 86400LL * 1000000LL * 10LL;
  const auto usecs = system_time64 / 10;

  tv->tv_sec = static_cast<long>(usecs / 1000000);                                        // NOLINT
  tv->tv_usec = static_cast<long>(usecs - (static_cast<int64_t>(tv->tv_sec) * 1000000));  // NOLINT
}

inline void nanosleep(const int64_t t) {
  LARGE_INTEGER start;
  QueryPerformanceCounter(&start);

  const auto sleep_for = t * PERFORMANCE_FREQUENCY.QuadPart / (1000LL * 1000LL * 1000LL) + start.QuadPart;

  LARGE_INTEGER now;
  QueryPerformanceCounter(&now);
  while (now.QuadPart <= sleep_for) {
    spin_loop_hint();
    QueryPerformanceCounter(&now);
  }
}

inline void add_timespec(timespec& ts, const int64_t addtime) {
  const auto nsec = addtime % 1000000000;
  const auto sec = (addtime - nsec) / 1000000000;
  ts.tv_sec += sec;
  ts.tv_nsec += nsec;
  if (ts.tv_nsec >= 1000000000) {
    const auto nsec_ = ts.tv_nsec % 1000000000;
    ts.tv_sec += ((int64_t)ts.tv_nsec - nsec_) / 1000000000LL;
    ts.tv_nsec = nsec_;
  }
}

timespec ecat_setup(const int64_t cycletime_ns) {
  auto ts = timespec{0, 0};

  auto tp = timeval{0, 0};
  gettimeofday_precise(&tp, nullptr);

  const auto cyctime_us = cycletime_ns / 1000;

  ts.tv_sec = tp.tv_sec;
  const auto ht = ((tp.tv_usec / cyctime_us) + 1) * cyctime_us;
  ts.tv_nsec = ht * 1000;
  return ts;
}

void timed_wait(const timespec& abs_time) {
  auto tp = timeval{0, 0};
  gettimeofday_precise(&tp, nullptr);

  const auto sleep = ((int64_t)abs_time.tv_sec - (int64_t)tp.tv_sec) * 1000000000LL + ((int64_t)abs_time.tv_nsec - (int64_t)tp.tv_usec * 1000LL);

  if (sleep > 0) std::this_thread::sleep_for(std::chrono::nanoseconds(sleep));
}

void timed_wait_h(const timespec& abs_time) {
  auto tp = timeval{0, 0};
  gettimeofday_precise(&tp, nullptr);

  const auto sleep = ((int64_t)abs_time.tv_sec - (int64_t)tp.tv_sec) * 1000000000LL + ((int64_t)abs_time.tv_nsec - (int64_t)tp.tv_usec * 1000LL);

  nanosleep(sleep);
}

}  // namespace autd3::link
