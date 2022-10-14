// File: c_api.cpp
// Project: extra_geometry_viewer
// Created Date: 10/10/2022
// Author: Shun Suzuki
// -----
// Last Modified: 14/10/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#include "./geometry_viewer.h"
#include "autd3/controller.hpp"
#include "autd3/extra/geometry_viewer.hpp"

void AUTDExtraGeometryViewer(void* cnt, const int32_t width, const int32_t height, const bool vsync, const char* model, const char* font,
                             const int32_t gpu_idx) {
  auto* const wrapper = static_cast<autd3::Controller*>(cnt);
  auto& viewer = autd3::extra::GeometryViewer().window_size(width, height).vsync(vsync).gpu_idx(gpu_idx);
  if (model != nullptr) viewer.model(std::string(model));
  if (font != nullptr) viewer.font(std::string(font));
  viewer.view(wrapper->geometry());
}
