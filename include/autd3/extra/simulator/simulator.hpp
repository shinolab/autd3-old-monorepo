// File: simulator.hpp
// Project: simulator
// Created Date: 30/09/2022
// Author: Shun Suzuki
// -----
// Last Modified: 05/10/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <autd3/core/geometry.hpp>
#include <memory>
#include <string>

namespace autd3::extra::simulator {

class Simulator {
 public:
  /**
   * @brief Start simulator
   */
  virtual void start(const core::Geometry& geometry) = 0;

  /**
   * @brief Exit simulator
   */
  virtual void exit() = 0;

  virtual bool send(const driver::TxDatagram& tx) = 0;
  virtual bool receive(driver::RxDatagram& rx) = 0;

  /**
   * @brief Create Bundle link
   */
  [[nodiscard]] static std::unique_ptr<Simulator> create(int32_t width, int32_t height, bool vsync, std::string shader, std::string texture,
                                                         std::string font, size_t gpu_idx);

  Simulator() noexcept = default;
  virtual ~Simulator() = default;
  Simulator(const Simulator& v) noexcept = delete;
  Simulator& operator=(const Simulator& obj) = delete;
  Simulator(Simulator&& obj) = default;
  Simulator& operator=(Simulator&& obj) = default;
};

}  // namespace autd3::extra::simulator
