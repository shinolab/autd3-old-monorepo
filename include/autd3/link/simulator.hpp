// File: simulator.hpp
// Project: link
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 06/10/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <memory>
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
  explicit Simulator(extra::simulator::Settings settings = {}) noexcept : _settings(std::move(settings)) {}
  ~Simulator() = default;
  Simulator(const Simulator& v) noexcept = delete;
  Simulator& operator=(const Simulator& obj) = delete;
  Simulator(Simulator&& obj) = default;
  Simulator& operator=(Simulator&& obj) = default;

  /**
   * @brief Set callback called when window is closed
   */
  Simulator& exit_callback(std::function<void(extra::simulator::Settings)> callback) {
    _callback = std::move(callback);
    return *this;
  }

  [[nodiscard]] core::LinkPtr build() const;

 private:
  extra::simulator::Settings _settings;
  std::function<void(extra::simulator::Settings)> _callback = [](const auto) { std::quick_exit(0); };
};

}  // namespace autd3::link
