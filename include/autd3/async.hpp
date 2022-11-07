// File: async.hpp
// Project: autd3
// Created Date: 07/11/2022
// Author: Shun Suzuki
// -----
// Last Modified: 07/11/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

namespace autd3 {

template <typename T>
struct Async {
  explicit Async(T t) : raw(std::make_unique<T>(std::move(t))) {}

  std::unique_ptr<T> raw;
};
}  // namespace autd3
