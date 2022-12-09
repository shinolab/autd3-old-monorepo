// File: c_api.cpp
// Project: link_debug
// Created Date: 10/10/2022
// Author: Shun Suzuki
// -----
// Last Modified: 09/12/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#include "../base/wrapper_link.hpp"
#include "./bundle_link.h"
#include "autd3/link/bundle.hpp"

EXPORT_AUTD void AUTDLinkBundle(void** out, void** links, const int32_t n) {
  auto* w_link_base = static_cast<LinkWrapper*>(links[0]);
  autd3::link::Bundle bundle(std::move(w_link_base->ptr));
  link_delete(w_link_base);
  for (int32_t i = 1; i < n; i++) {
    auto* w_link = static_cast<LinkWrapper*>(links[i]);
    bundle.link(std::move(w_link->ptr));
    link_delete(w_link);
  }
  auto* link = link_create(bundle.build());
  *out = link;
}
