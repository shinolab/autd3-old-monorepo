// File: log_level.hpp
// Project: driver
// Created Date: 17/01/2023
// Author: Shun Suzuki
// -----
// Last Modified: 29/04/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

namespace autd3::driver {
enum class LogLevel : int { Trace = 0, Debug = 1, Info = 2, Warn = 3, Err = 4, Critical = 5, Off = 6 };
}
