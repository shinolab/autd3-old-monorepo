// File: timer_strategy.hpp
// Project: osal_timer
// Created Date: 19/03/2023
// Author: Shun Suzuki
// -----
// Last Modified: 20/03/2023
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
