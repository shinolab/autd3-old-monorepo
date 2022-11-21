// File: c_api.cpp
// Project: extra_geometry_viewer
// Created Date: 10/10/2022
// Author: Shun Suzuki
// -----
// Last Modified: 19/11/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#include "./geometry_viewer.h"
#include "autd3/controller.hpp"
#include "autd3/extra/geometry_viewer.hpp"

bool AUTDExtraGeometryViewer(void* cnt, const int32_t width, const int32_t height, const bool vsync, const int32_t gpu_idx) {
  return autd3::extra::GeometryViewer()
      .window_size(width, height)
      .vsync(vsync)
      .gpu_idx(gpu_idx)
      .view(static_cast<autd3::Controller*>(cnt)->geometry());
}
