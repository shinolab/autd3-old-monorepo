// File: utils.hpp
// Project: internal
// Created Date: 13/09/2023
// Author: Shun Suzuki
// -----
// Last Modified: 13/11/2023
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

#define AUTD3_DEF_PARAM_AMP(T)                                                                     \
  void with_amp(const double value)& { _amp = internal::EmitIntensity::new_normalized(value); }    \
  [[nodiscard]] T&& with_amp(const double value)&& {                                               \
    _amp = internal::EmitIntensity::new_normalized(value);                                         \
    return std::move(*this);                                                                       \
  }                                                                                                \
  void with_amp(const uint16_t value)& { _amp = internal::EmitIntensity::new_pulse_width(value); } \
  [[nodiscard]] T&& with_amp(const uint16_t value)&& {                                             \
    _amp = internal::EmitIntensity::new_pulse_width(value);                                        \
    return std::move(*this);                                                                       \
  }                                                                                                \
  void with_amp(const internal::EmitIntensity value)& { _amp = value; }                            \
  [[nodiscard]] T&& with_amp(const internal::EmitIntensity value)&& {                              \
    _amp = value;                                                                                  \
    return std::move(*this);                                                                       \
  }

namespace autd3::internal::native_methods {

template <class T>
concept ResultPtr = requires(T& x) { x.result._0; };

template <typename T>
inline constexpr auto validate(T res) {
  const auto [result, err_len, err] = res;
  if (result._0 == nullptr) {
    const std::string err_str(err_len, ' ');
    AUTDGetErr(err, const_cast<char*>(err_str.c_str()));
    throw AUTDException(err_str);
  }
  return result;
}

template <typename T = int32_t>
inline constexpr T validate(ResultI32 res) {
  const auto [result, err_len, err] = res;
  if (result == AUTD3_ERR) {
    const std::string err_str(err_len, ' ');
    AUTDGetErr(err, const_cast<char*>(err_str.c_str()));
    throw AUTDException(err_str);
  }
  return static_cast<T>(result);
}

}  // namespace autd3::internal::native_methods
