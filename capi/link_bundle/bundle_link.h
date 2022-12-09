// File: bundle_link.h
// Project: link_bundle
// Created Date: 09/12/2022
// Author: Shun Suzuki
// -----
// Last Modified: 09/12/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include "../base/header.hpp"

#ifdef __cplusplus
extern "C" {
#endif
EXPORT_AUTD void AUTDLinkBundle(OUT void** out, IN void** links, IN int32_t n);
#ifdef __cplusplus
}
#endif
