// File: geometry_viewer.h
// Project: extra_geometry_viewer
// Created Date: 10/10/2022
// Author: Shun Suzuki
// -----
// Last Modified: 08/01/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include "../base/header.hpp"

#ifdef __cplusplus
#include <cstdint>
#endif

#ifdef __cplusplus
extern "C" {
#endif
EXPORT_AUTD void AUTDExtraGeometryViewer(IN void* cnt, IN int32_t width, IN int32_t height, IN bool vsync, IN int32_t gpu_idx);
#ifdef __cplusplus
}
#endif
