// File: exception.hpp
// Project: internal
// Created Date: 29/05/2023
// Author: Shun Suzuki
// -----
// Last Modified: 29/05/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <stdexcept>

namespace autd3::internal {
class AUTDException : public std::runtime_error {
 public:
  AUTDException(const char *_Message) : runtime_error(_Message) {}
};

}  // namespace autd3::internal
