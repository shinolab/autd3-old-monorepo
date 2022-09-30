// File: geometry_viewer.hpp
// Project: geometry_viewer
// Created Date: 28/09/2022
// Author: Shun Suzuki
// -----
// Last Modified: 30/09/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <string>
#include <utility>

#include "autd3/core/geometry.hpp"

namespace autd3::extra::geometry_viewer {

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
   * @brief Set shader path
   */
  GeometryViewer& shader(std::string shader) {
    _shader = std::move(shader);
    return *this;
  }
  /**
   * @brief Set model path
   */
  GeometryViewer& model(std::string model) {
    _model = std::move(model);
    return *this;
  }

  /**
   * @brief Set font path
   */
  GeometryViewer& font(std::string font) {
    _font = std::move(font);
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
  void view(const core::Geometry& geometry) const;

  /**
   * @brief Constructor
   */
  GeometryViewer() noexcept : _width(800), _height(600), _vsync(true), _shader("shaders"), _model("models/AUTD.glb"), _font(""), _gpu_idx(0) {}
  ~GeometryViewer() = default;
  GeometryViewer(const GeometryViewer& v) noexcept = delete;
  GeometryViewer& operator=(const GeometryViewer& obj) = delete;
  GeometryViewer(GeometryViewer&& obj) = default;
  GeometryViewer& operator=(GeometryViewer&& obj) = default;

 private:
  int32_t _width;
  int32_t _height;
  bool _vsync;
  std::string _shader;
  std::string _model;
  std::string _font;
  size_t _gpu_idx;
};

}  // namespace autd3::extra::geometry_viewer
