// File: debug_link.h
// Project: link_debug
// Created Date: 10/10/2022
// Author: Shun Suzuki
// -----
// Last Modified: 10/10/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include "../base/header.hpp"

#ifdef __cplusplus
extern "C" {
#endif
EXPORT_AUTD void AUTDLinkDebug(OUT void** out);
EXPORT_AUTD void AUTDLinkDebugSetLevel(IN int32_t level);
#ifdef __cplusplus
}
#endif
