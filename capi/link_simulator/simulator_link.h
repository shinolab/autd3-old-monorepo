// File: simulator_link.h
// Project: link_simulator
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 30/09/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include "../base/header.hpp"

#ifdef __cplusplus
extern "C" {
#endif
EXPORT_AUTD void AUTDLinkSimulator(OUT void** out, IN uint16_t port);
#ifdef __cplusplus
}
#endif
