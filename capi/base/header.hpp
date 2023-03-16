// File: header.hpp
// Project: base
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 16/03/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#if WIN32
#define EXPORT_AUTD __declspec(dllexport)
#else
#define EXPORT_AUTD __attribute__((visibility("default")))
#endif

#define IN
#define OUT
#define INOUT

#ifdef __cplusplus
#else
#include <stdbool.h>
#include <stdint.h>
#endif

#ifdef AUTD3_USE_SINGLE_FLOAT
typedef float autd3_float_t;
#else
typedef double autd3_float_t;
#endif

#define AUTD3_CAPI_TRY_RET(action, ret_if_failed) \
  try {                                           \
    action;                                       \
  } catch (std::exception & ex) {                 \
    spdlog::error(ex.what());                     \
    return ret_if_failed;                         \
  }
#define AUTD3_CAPI_TRY_RET_BOOL(action) \
  try {                                 \
    action;                             \
    return true;                        \
  } catch (std::exception & ex) {       \
    spdlog::error(ex.what());           \
    return false;                       \
  }
#define EXPAND(x) x
#define AUTD3_CAPI_TRY_MACRO_CHOOSER(arg0, arg1, arg2, ...) arg2
#define AUTD3_CAPI_TRY(...) EXPAND(AUTD3_CAPI_TRY_MACRO_CHOOSER(__VA_ARGS__, AUTD3_CAPI_TRY_RET, AUTD3_CAPI_TRY_RET_BOOL)(__VA_ARGS__))
