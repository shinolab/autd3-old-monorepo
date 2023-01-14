// File: debug_level.hpp
// Project: driver
// Created Date: 14/01/2023
// Author: Shun Suzuki
// -----
// Last Modified: 14/01/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

namespace autd3::driver {
enum class DebugLevel : int { Trace = 0, Debug = 1, Info = 2, Warn = 3, Err = 4, Critical = 5, Off = 6 };
}
