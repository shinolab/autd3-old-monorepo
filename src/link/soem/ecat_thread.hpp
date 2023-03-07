// File: ecat_thread.hpp
// Project: ecat_thread
// Created Date: 12/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 07/03/2023
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
#include "ecat_thread/win32.hpp"
#elif __APPLE__
#include "ecat_thread/macosx.hpp"
#else
#include "ecat_thread/linux.hpp"
#endif

#if WIN32
#include <timeapi.h>
#endif

namespace autd3::link {

inline int64_t ec_sync(const int64_t reftime, const int64_t cycletime, int64_t* integral) {
  auto delta = (reftime - 50000) % cycletime;
  if (delta > cycletime / 2) delta -= cycletime;
  if (delta > 0) *integral += 1;
  if (delta < 0) *integral -= 1;
  return -(delta / 100) - *integral / 20;
}

}  // namespace autd3::link
