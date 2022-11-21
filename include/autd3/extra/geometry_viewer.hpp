// File: geometry_viewer.hpp
// Project: geometry_viewer
// Created Date: 28/09/2022
// Author: Shun Suzuki
// -----
// Last Modified: 19/11/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <string>
#include <utility>

#include "autd3/core/geometry.hpp"

namespace autd3::extra {

class GeometryViewer {
 public:
  /**
   * @brief Set window size
   */
  GeometryViewer& window_size(const int32_t width, const int32_t height) {
    _width = width;
    _height = height;
    return *this;
  }

  /**
   * @brief Set vsync
   */
  GeometryViewer& vsync(const bool vsync) {
    _vsync = vsync;
    return *this;
  }

  /**
   * @brief Set GPU index
   */
  GeometryViewer& gpu_idx(const size_t idx) {
    _gpu_idx = idx;
    return *this;
  }

  /**
   * @brief View geometry
   */
  [[nodiscard]] bool view(const core::Geometry& geometry) const;

  /**
   * @brief Constructor
   */
  GeometryViewer() noexcept : _width(800), _height(600), _vsync(true), _gpu_idx(0) {}
  ~GeometryViewer() = default;
  GeometryViewer(const GeometryViewer& v) noexcept = delete;
  GeometryViewer& operator=(const GeometryViewer& obj) = delete;
  GeometryViewer(GeometryViewer&& obj) = default;
  GeometryViewer& operator=(GeometryViewer&& obj) = default;

 private:
  int32_t _width;
  int32_t _height;
  bool _vsync;
  size_t _gpu_idx;
};

}  // namespace autd3::extra
