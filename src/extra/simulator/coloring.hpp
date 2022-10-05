// File: coloring.hpp
// Project: simulator
// Created Date: 05/10/2022
// Author: Shun Suzuki
// -----
// Last Modified: 05/10/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <glm/glm.hpp>

#include "color.hpp"

namespace autd3::extra::simulator {

inline glm::vec4 coloring_hsv(const float h, const float v, const float a) {
  const auto hsv = Hsv{h, 1.0f, v, a};
  return hsv.rgba();
}

}  // namespace autd3::extra::simulator
