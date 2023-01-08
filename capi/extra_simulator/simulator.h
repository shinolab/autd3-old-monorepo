// File: twincat_link.h
// Project: link_twincat
// Created Date: 16/05/2022
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
extern "C" {
#endif
EXPORT_AUTD void AUTDExtraSimulator(IN const char* settings_path, IN bool vsync, IN int32_t gpu_idx);
#ifdef __cplusplus
}
#endif
