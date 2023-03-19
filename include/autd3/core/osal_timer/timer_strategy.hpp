// File: timer_strategy.hpp
// Project: core
// Created Date: 19/03/2023
// Author: Shun Suzuki
// -----
// Last Modified: 19/03/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

namespace autd3::core {

enum class TimerStrategy {
  Sleep,
  BusyWait,
  NativeTimer,
};

}  // namespace autd3::core
