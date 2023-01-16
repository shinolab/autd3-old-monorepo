// File: wrapper.hpp
// Project: base
// Created Date: 16/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 17/01/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include <memory>
#include <utility>
#include <vector>

#include "autd3/driver/firmware_version.hpp"
#include "autd3/gain/backend.hpp"
#include "autd3/gain/holo.hpp"

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

typedef struct {
  std::unique_ptr<autd3::gain::holo::AmplitudeConstraint> ptr;
} ConstraintWrapper;

inline ConstraintWrapper* constraint_create(std::unique_ptr<autd3::gain::holo::AmplitudeConstraint> ptr) {
  return new ConstraintWrapper{std::move(ptr)};
}
inline void constraint_delete(const ConstraintWrapper* ptr) { delete ptr; }
