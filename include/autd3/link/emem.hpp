// File: emem.hpp
// Project: link
// Created Date: 04/02/2023
// Author: Shun Suzuki
// -----
// Last Modified: 04/02/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include "autd3/core/link.hpp"

namespace autd3::link {

/**
 * @brief Link for Experimental Mini EtherCAT Master
 */
class Emem {
 public:
  /**
   * @brief Create Bundle link
   */
  [[nodiscard]] core::LinkPtr build();

  /**
   * @brief Constructor
   */
  Emem() = default;

  ~Emem() = default;
  Emem(const Emem& v) noexcept = delete;
  Emem& operator=(const Emem& obj) = delete;
  Emem(Emem&& obj) = default;
  Emem& operator=(Emem&& obj) = default;
};

}  // namespace autd3::link
