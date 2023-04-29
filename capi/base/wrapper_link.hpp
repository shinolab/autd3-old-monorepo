// File: wrapper_link.hpp
// Project: base
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 28/04/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <utility>

#include "autd3/core/link.hpp"

typedef void (*OutCallback)(const char*);
typedef void (*FlushCallback)();

typedef struct {
  autd3::core::LinkPtr ptr;
} LinkWrapper;

inline LinkWrapper* link_create(autd3::core::LinkPtr ptr) { return new LinkWrapper{std::move(ptr)}; }
inline void link_delete(const LinkWrapper* ptr) { delete ptr; }
