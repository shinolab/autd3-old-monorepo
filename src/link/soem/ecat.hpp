// File: ecat_thread.hpp
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

#include <algorithm>
#include <limits>
#include <memory>
#include <queue>
#include <string>
#include <vector>

#include "iomap.hpp"

extern "C" {
#include "./ethercat.h"
}

#if WIN32
#include <timeapi.h>
#endif

#if WIN32
#include "ecat_osal/win32.hpp"
#elif __APPLE__
#include "ecat_osal/macosx.hpp"
#else
#include "ecat_osal/linux.hpp"
#endif

namespace autd3::link {

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

inline int64_t ec_sync(const int64_t reftime, const int64_t cycletime, int64_t* integral) {
  auto delta = (reftime - 50000) % cycletime;
  if (delta > cycletime / 2) delta -= cycletime;
  if (delta > 0) *integral += 1;
  if (delta < 0) *integral -= 1;
  return -(delta / 100) - *integral / 20;
}

}  // namespace autd3::link
