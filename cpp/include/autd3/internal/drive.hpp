// File: drive.hpp
// Project: internal
// Created Date: 24/11/2023
// Author: Shun Suzuki
// -----
// Last Modified: 24/11/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include "autd3/internal/emit_intensity.hpp"

namespace autd3::internal {
struct Drive final {
  double phase;
  EmitIntensity intensity;
};

}  // namespace autd3::internal
