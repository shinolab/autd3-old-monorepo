// File: update_flag.hpp
// Project: simulator
// Created Date: 05/10/2022
// Author: Shun Suzuki
// -----
// Last Modified: 28/04/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

namespace autd3::extra::simulator {

enum class UpdateFlags : uint32_t {
  None = 0,
  UpdateSourceDrive = 1 << 0,
  UpdateColorMap = 1 << 1,
  UpdateCameraPos = 1 << 2,
  UpdateSlicePos = 1 << 3,
  UpdateSliceSize = 1 << 4,
  UpdateSourceAlpha = 1 << 5,
  UpdateSourceFlag = 1 << 6,
  SaveImage = 1 << 7,
  UpdateDeviceInfo = 1 << 8,
};

}  // namespace autd3::extra::simulator
