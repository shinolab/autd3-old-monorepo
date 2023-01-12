// File: operation.hpp
// Project: operation
// Created Date: 06/01/2023
// Author: Shun Suzuki
// -----
// Last Modified: 11/01/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <type_traits>

namespace autd3::driver {

struct Mode {};
struct Legacy : Mode {};
struct Normal : Mode {};
struct NormalPhase : Mode {};

template <typename T>
inline constexpr bool is_mode_v = std::is_base_of_v<Mode, T>;

}  // namespace autd3::driver
