// File: c_api.cpp
// Project: link_twincat
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 10/06/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#include "../base/wrapper_link.hpp"
#include "./twincat_link.h"
#include "autd3/link/twincat.hpp"

void AUTDLinkTwinCAT(void** out) {
  auto* link = link_create(autd3::link::TwinCAT().build());
  *out = link;
}
