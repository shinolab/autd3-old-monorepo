// File: iter.hpp
// Project: utils
// Created Date: 20/01/2023
// Author: Shun Suzuki
// -----
// Last Modified: 20/01/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <algorithm>

#ifdef AUTD3_PARALLEL_FOR
#include <execution>
#endif

namespace autd3 {
template <class InIt, class OutIt, class Fn>
inline OutIt transform(const InIt first, const InIt last, OutIt dest, Fn func) {
#ifdef AUTD3_PARALLEL_FOR
  return std::transform(std::execution::par_unseq, first, last, dest, func);
#else
  return std::transform(first, last, dest, func);
#endif
}
}  // namespace autd3
