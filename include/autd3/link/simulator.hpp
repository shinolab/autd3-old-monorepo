// File: simulator.hpp
// Project: link
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 28/04/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <memory>

#include "autd3/core/link.hpp"
#include "autd3/link/builder.hpp"

namespace autd3::link {

/**
 * \brief link for Simulator
 */
class Simulator : public LinkBuilder<Simulator> {
 public:
  /**
   * @brief Constructor
   */
  Simulator() : LinkBuilder(core::Milliseconds(20)) {}
  ~Simulator() override = default;
  Simulator(const Simulator& v) noexcept = delete;
  Simulator& operator=(const Simulator& obj) = delete;
  Simulator(Simulator&& obj) = default;
  Simulator& operator=(Simulator&& obj) = default;

 protected:
  core::LinkPtr build_() override;
};

}  // namespace autd3::link
