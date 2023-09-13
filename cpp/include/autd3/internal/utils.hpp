// File: utils.hpp
// Project: internal
// Created Date: 13/09/2023
// Author: Shun Suzuki
// -----
// Last Modified: 13/09/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#define AUTD3_DEF_PARAM(T, PARAM_T, PARAM_NAME)                           \
  void with_##PARAM_NAME(const PARAM_T value)& { _##PARAM_NAME = value; } \
  [[nodiscard]] T&& with_##PARAM_NAME(const PARAM_T value)&& {            \
    _##PARAM_NAME = value;                                                \
    return std::move(*this);                                              \
  }
