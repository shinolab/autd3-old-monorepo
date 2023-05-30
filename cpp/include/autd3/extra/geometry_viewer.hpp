// File: geometry_viewer.hpp
// Project: extra
// Created Date: 29/05/2023
// Author: Shun Suzuki
// -----
// Last Modified: 30/05/2023
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
    _ptr = internal::native_methods::AUTDGeometryViewerSize(_ptr, width, height);
    return *this;
  }

  [[nodiscard]] GeometryViewer& vsync(const uint32_t vsync) {
    _ptr = internal::native_methods::AUTDGeometryViewerVsync(_ptr, vsync);
    return *this;
  }

  [[nodiscard]] int32_t run(const internal::Geometry& geometry) const {
    return internal::native_methods::AUTDGeometryViewerRun(_ptr, geometry.ptr());
  }

 private:
  void* _ptr;
};
}  // namespace autd3::extra
