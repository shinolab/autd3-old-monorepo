// File: special.hpp
// Project: autd3
// Created Date: 29/05/2023
// Author: Shun Suzuki
// -----
// Last Modified: 21/09/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include "autd3/internal/datagram.hpp"
#include "autd3/internal/native_methods.hpp"

namespace autd3::internal {

/**
 * @brief SpecialDatagram to stop output
 */
class Stop final : public SpecialDatagram {
 public:
  Stop() = default;

  [[nodiscard]] native_methods::DatagramSpecialPtr ptr() const override { return native_methods::AUTDDatagramStop(); }
};

}  // namespace autd3::internal
