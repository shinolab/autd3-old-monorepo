// File: exception.hpp
// Project: internal
// Created Date: 29/05/2023
// Author: Shun Suzuki
// -----
// Last Modified: 30/05/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <stdexcept>

namespace autd3::internal {
class AUTDException final : public std::runtime_error {
 public:
  explicit AUTDException(const char *message) : runtime_error(message) {}
};

}  // namespace autd3::internal
