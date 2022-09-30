// File: simulator.hpp
// Project: simulator
// Created Date: 30/09/2022
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

namespace autd3::extra::simulator {

class Simulator {
 public:
  /**
   * @brief Set window size
   */
  Simulator& window_size(const int32_t width, const int32_t height) {
    _width = width;
    _height = height;
    return *this;
  }

  /**
   * @brief Set vsync
   */
  Simulator& vsync(const bool vsync) {
    _vsync = vsync;
    return *this;
  }

  /**
   * @brief Set shader path
   */
  Simulator& shader(std::string shader) {
    _shader = std::move(shader);
    return *this;
  }

  /**
   * @brief Set texture path
   */
  Simulator& texture(std::string texture) {
    _texture = std::move(texture);
    return *this;
  }

  /**
   * @brief Set font path
   */
  Simulator& font(std::string font) {
    _font = std::move(font);
    return *this;
  }

  /**
   * @brief Set GPU index
   */
  Simulator& gpu_idx(const size_t idx) {
    _gpu_idx = idx;
    return *this;
  }

  /**
   * @brief Set Port
   */
  Simulator& port(const uint16_t port) {
    _port = port;
    return *this;
  }

  /**
   * @brief Start simulator
   */
  Simulator start();

  /**
   * @brief Exit simulator
   */
  void exit();

  /**
   * @brief Constructor
   */
  Simulator() noexcept : _width(800), _height(600), _vsync(true), _shader("shaders"), _font(""), _gpu_idx(0), _port(50632) {}
  ~Simulator() = default;
  Simulator(const Simulator& v) noexcept = delete;
  Simulator& operator=(const Simulator& obj) = delete;
  Simulator(Simulator&& obj) = default;
  Simulator& operator=(Simulator&& obj) = default;

 private:
  int32_t _width;
  int32_t _height;
  bool _vsync;
  std::string _shader;
  std::string _texture;
  std::string _font;
  size_t _gpu_idx;
  uint16_t _port;
};

}  // namespace autd3::extra::simulator
