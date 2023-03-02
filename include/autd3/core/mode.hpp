// File: mode.hpp
// Project: core
// Created Date: 04/01/2023
// Author: Shun Suzuki
// -----
// Last Modified: 03/03/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

namespace autd3::core {

enum class Mode { Legacy, Advanced, AdvancedPhase };

inline Mode legacy_mode() noexcept { return Mode::Legacy; }
inline Mode advanced_mode() noexcept { return Mode::Advanced; }
inline Mode advanced_phase_mode() noexcept { return Mode::AdvancedPhase; }

}  // namespace autd3::core
