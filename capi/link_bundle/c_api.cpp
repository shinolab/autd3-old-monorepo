// File: c_api.cpp
// Project: link_debug
// Created Date: 10/10/2022
// Author: Shun Suzuki
// -----
// Last Modified: 08/01/2023
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
  std::transform(links, links + n, std::back_inserter(bundle), [](void* link) {
    auto* w_link = static_cast<LinkWrapper*>(link);
    autd3::core::LinkPtr l = std::move(w_link->ptr);
    link_delete(w_link);
    return l;
  });
  auto* link = link_create(bundle.build());
  *out = link;
}
