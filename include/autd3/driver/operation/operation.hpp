// File: operation.hpp
// Project: operation
// Created Date: 06/01/2023
// Author: Shun Suzuki
// -----
// Last Modified: 16/01/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <type_traits>

namespace autd3::driver {

struct Mode {};
struct Legacy : Mode {};
struct Normal : Mode {};
struct NormalPhase : Mode {};

template <typename T>
inline constexpr bool is_mode_v = std::is_base_of_v<Mode, T>;

struct Operation {
  virtual ~Operation() = default;
  virtual void init() = 0;
  virtual void pack(TxDatagram& tx) = 0;
  virtual bool is_finished() const = 0;
};

template <typename T>
inline constexpr bool is_operation_v = std::is_base_of_v<Operation, T>;

}  // namespace autd3::driver
