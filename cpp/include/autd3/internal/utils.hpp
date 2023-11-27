// File: utils.hpp
// Project: internal
// Created Date: 13/09/2023
// Author: Shun Suzuki
// -----
// Last Modified: 24/11/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include "autd3/internal/exception.hpp"

#define AUTD3_DEF_PARAM(T, PARAM_T, PARAM_NAME)                           \
  void with_##PARAM_NAME(const PARAM_T value)& { _##PARAM_NAME = value; } \
  [[nodiscard]] T&& with_##PARAM_NAME(const PARAM_T value)&& {            \
    _##PARAM_NAME = value;                                                \
    return std::move(*this);                                              \
  }

#define AUTD3_DEF_PARAM_INTENSITY(T, PARAM_NAME)                                                   \
  void with_##PARAM_NAME(const uint8_t value)& { _##PARAM_NAME = internal::EmitIntensity(value); } \
  [[nodiscard]] T&& with_##PARAM_NAME(const uint8_t value)&& {                                     \
    _##PARAM_NAME = internal::EmitIntensity(value);                                                \
    return std::move(*this);                                                                       \
  }                                                                                                \
  void with_##PARAM_NAME(const internal::EmitIntensity value)& { _##PARAM_NAME = value; }          \
  [[nodiscard]] T&& with_##PARAM_NAME(const internal::EmitIntensity value)&& {                     \
    _##PARAM_NAME = value;                                                                         \
    return std::move(*this);                                                                       \
  }

namespace autd3::internal::native_methods {

template <class T>
concept result_ptr = requires(T& x) { x.result._0; };

template <result_ptr T>
constexpr auto validate(T res) {
  const auto [result, err_len, err] = res;
  if (result._0 == nullptr) {
    const std::string err_str(err_len, ' ');
    AUTDGetErr(err, const_cast<char*>(err_str.c_str()));
    throw AUTDException(err_str);
  }
  return result;
}

template <typename T = int32_t>
constexpr T validate(ResultI32 res) {
  const auto [result, err_len, err] = res;
  if (result == AUTD3_ERR) {
    const std::string err_str(err_len, ' ');
    AUTDGetErr(err, const_cast<char*>(err_str.c_str()));
    throw AUTDException(err_str);
  }
  return static_cast<T>(result);
}

constexpr SamplingConfiguration validate(ResultSamplingConfig res) {
  const auto [result, err_len, err] = res;
  if (result.div == 0) {
    const std::string err_str(err_len, ' ');
    AUTDGetErr(err, const_cast<char*>(err_str.c_str()));
    throw AUTDException(err_str);
  }
  return result;
}

}  // namespace autd3::internal::native_methods
