// File: mode.hpp
// Project: core
// Created Date: 04/01/2023
// Author: Shun Suzuki
// -----
// Last Modified: 04/01/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

namespace autd3::core {

enum class Mode { Legacy, Normal, NormalPhase };

inline Mode legacy_mode() noexcept { return Mode::Legacy; }
inline Mode normal_mode() noexcept { return Mode::Normal; }
inline Mode normal_phase_mode() noexcept { return Mode::NormalPhase; }

}  // namespace autd3::core
