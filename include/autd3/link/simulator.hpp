// File: simulator.hpp
// Project: link
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 05/10/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <memory>
#include <string>
#include <utility>

#include "autd3/core/link.hpp"

namespace autd3::link {

/**
 * \brief link for Simulator
 */
class Simulator {
 public:
  /**
   * @brief Constructor
   */
  Simulator() noexcept = default;
  ~Simulator() = default;
  Simulator(const Simulator& v) noexcept = delete;
  Simulator& operator=(const Simulator& obj) = delete;
  Simulator(Simulator&& obj) = default;
  Simulator& operator=(Simulator&& obj) = default;

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
   * @brief Set callback called when window is closed
   */
  Simulator& exit_callback(std::function<void()> callback) {
    _callback = std::move(callback);
    return *this;
  }

  [[nodiscard]] core::LinkPtr build() const;

 private:
  int32_t _width{800};
  int32_t _height{600};
  bool _vsync{true};
  std::string _shader;
  std::string _texture;
  std::string _font;
  size_t _gpu_idx{0};
  std::function<void()> _callback = []() { std::quick_exit(0); };
};

}  // namespace autd3::link
