// File: wrapper.hpp
// Project: base
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 16/05/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Hapis Lab. All rights reserved.
//

#pragma once

#include <memory>
#include <vector>

#include "autd3/driver/firmware_version.hpp"
#include "autd3/gain/backend.hpp"

typedef struct {
  std::vector<autd3::driver::FirmwareInfo> list;
} FirmwareInfoListWrapper;

inline FirmwareInfoListWrapper* firmware_info_list_create(const std::vector<autd3::driver::FirmwareInfo>& list) {
  return new FirmwareInfoListWrapper{list};
}
inline void firmware_info_list_delete(const FirmwareInfoListWrapper* ptr) { delete ptr; }

typedef struct {
  autd3::gain::holo::BackendPtr ptr;
} BackendWrapper;

inline BackendWrapper* backend_create(const autd3::gain::holo::BackendPtr& ptr) { return new BackendWrapper{ptr}; }
inline void backend_delete(const BackendWrapper* ptr) { delete ptr; }
