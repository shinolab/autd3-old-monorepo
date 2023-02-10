// File: time_utils.hpp
// Project: ethercat
// Created Date: 07/02/2023
// Author: Shun Suzuki
// -----
// Last Modified: 10/02/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <cstdint>

#if WIN32
#include <WinSock2.h>
#include <windows.h>
#elif __APPLE__
#else
#endif

namespace autd3::link::ethercat {

#if WIN32
inline void gettimeofday_precise(timeval* const tv) {
  FILETIME system_time;
  GetSystemTimePreciseAsFileTime(&system_time);

  int64_t system_time64 = (static_cast<int64_t>(system_time.dwHighDateTime) << 32) + static_cast<int64_t>(system_time.dwLowDateTime);
  system_time64 += -134774LL * 86400LL * 1000000LL * 10LL;
  system_time64 -= 946684800LL * 1000000LL * 10LL;

  const auto usecs = system_time64 / 10;

  tv->tv_sec = static_cast<long>(usecs / 1000000LL);                                        // NOLINT
  tv->tv_usec = static_cast<long>(usecs - static_cast<long long>(tv->tv_sec) * 1000000LL);  // NOLINT
}
#elif __APPLE__
int osal_gettimeofday(struct timeval *tv) {
  struct timespec ts;
  int return_value;

  return_value = clock_gettime(CLOCK_MONOTONIC, &ts);
  tv->tv_sec = ts.tv_sec;
  tv->tv_usec = ts.tv_nsec / 1000;
  return return_value;
}
#endif

uint64_t get_master_ec_time() {
#if WIN32
  auto tp = timeval{0, 0};
  gettimeofday_precise(&tp);
  return (static_cast<uint64_t>(tp.tv_sec) * 1000ULL * 1000ULL + static_cast<uint64_t>(tp.tv_usec)) * 1000ULL;
#elif __APPLE__
  struct timeval current_time;
  uint64_t return_value;

  osal_gettimeofday(&current_time);
  return_value = current_time.tv_sec * 1000UL * 1000UL + current_time.tv_usec;
  return_value *= 1000ULL;
  return return_value;
#else
  struct timespec current_time;
  ec_timet return_value;

  clock_gettime(CLOCK_REALTIME, &current_time);
  return_value = current_time.tv_sec * 1000UL * 1000UL * 1000UL + current_time.tv_nsec;
  return return_value;
#endif
}

}  // namespace autd3::link::ethercat
