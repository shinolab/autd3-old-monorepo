// File: utils.hpp
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

#ifdef _MSC_VER
#include <intrin.h>
#endif

inline int64_t ec_sync(const int64_t reftime, const int64_t cycletime, int64_t* integral) {
  auto delta = (reftime - 50000) % cycletime;
  if (delta > (cycletime / 2)) delta -= cycletime;
  if (delta > 0) *integral += 1;
  if (delta < 0) *integral -= 1;
  return -(delta / 100) - (*integral / 20);
}

inline void spin_loop_hint() {
#if defined(_MSC_VER)
  _mm_pause();
#elif defined(__x86_64__)
  __asm__ __volatile__("pause;");
#elif defined(i386) || defined(__i386__) || defined(__i386)
  __asm__ __volatile__("pause;");
#endif
}
