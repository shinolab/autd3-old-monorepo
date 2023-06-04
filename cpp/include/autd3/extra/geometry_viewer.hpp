// File: geometry_viewer.hpp
// Project: extra
// Created Date: 29/05/2023
// Author: Shun Suzuki
// -----
// Last Modified: 03/06/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include "autd3/internal/geometry.hpp"
#include "autd3/internal/native_methods.hpp"

namespace autd3::extra {

class GeometryViewer {
 public:
  GeometryViewer() : _ptr(internal::native_methods::AUTDGeometryViewer()) {}

  [[nodiscard]] GeometryViewer& window_size(const uint32_t width, const uint32_t height) {
    _ptr = AUTDGeometryViewerSize(_ptr, width, height);
    return *this;
  }

  [[nodiscard]] GeometryViewer& vsync(const uint32_t vsync) {
    _ptr = AUTDGeometryViewerVsync(_ptr, vsync);
    return *this;
  }

  [[nodiscard]] int32_t run(const internal::Geometry& geometry) const { return AUTDGeometryViewerRun(_ptr, geometry.ptr()); }

 private:
  internal::native_methods::GeometryViewerPtr _ptr;
};
}  // namespace autd3::extra
