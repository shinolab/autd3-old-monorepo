// File: hint.hpp
// Project: utils
// Created Date: 22/11/2022
// Author: Shun Suzuki
// -----
// Last Modified: 21/01/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

namespace autd3::core {

inline void spin_loop_hint() {
#if defined(_MSC_VER)
#if defined(_M_X64)
  _mm_pause();
#elif defined(_M_ARM64)
  __yield();
#endif
#elif defined(__x86_64__)
  __asm__ __volatile__("pause;");
#elif defined(i386) || defined(__i386__) || defined(__i386)
  __asm__ __volatile__("pause;");
#endif
}

}  // namespace autd3::core
