// File: error.hpp
// Project: emem
// Created Date: 06/02/2023
// Author: Shun Suzuki
// -----
// Last Modified: 07/02/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <variant>

namespace autd3::link {

enum class EmemError {
  SendFrame,
  ReceiveFrame,
  NoFrame,
  UnknownFrame,
  UndefinedBehavior,
  SlaveNotFound,
  TooManySlaves,
};

template <typename T>
struct Result {
  explicit Result(T v) : _v(std::move(v)) {}
  explicit Result(EmemError e) : _v(e) {}

  [[nodiscard]] T value() const { return std::get<T>(_v); }

  [[nodiscard]] EmemError err() const { return std::get<EmemError>(_v); }

  [[nodiscard]] bool is_ok() const noexcept { return std::holds_alternative<T>(_v); }
  [[nodiscard]] bool is_err() const noexcept { return std::holds_alternative<EmemError>(_v); }

 private:
  std::variant<T, EmemError> _v;
};

}  // namespace autd3::link
