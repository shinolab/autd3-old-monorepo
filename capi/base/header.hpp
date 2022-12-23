// File: header.hpp
// Project: base
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 22/12/2022
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

#define AUTD_DEPENDS_EXT_LIB
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
