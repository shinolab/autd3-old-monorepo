// File: callback.hpp
// Project: osal_timer
// Created Date: 18/03/2023
// Author: Shun Suzuki
// -----
// Last Modified: 20/03/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

namespace autd3::core {

class CallbackHandler {
 public:
  virtual void callback() = 0;
  virtual ~CallbackHandler() = default;
};

}  // namespace autd3::core
