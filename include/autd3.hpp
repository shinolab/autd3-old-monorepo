// File: autd3.hpp
// Project: include
// Created Date: 10/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 12/05/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Hapis Lab. All rights reserved.
//

#pragma once

#include "autd3/controller.hpp"
#include "autd3/core/geometry/geometry.hpp"
#include "autd3/core/stm/gain.hpp"
#include "autd3/core/stm/point.hpp"
#include "autd3/gain/primitive.hpp"
#include "autd3/modulation/primitive.hpp"

namespace autd3 {
constexpr double pi = driver::pi;

using core::Geometry;
using core::LegacyTransducer;
using core::NormalTransducer;

using core::Vector3;

using core::GainSTM;
using core::PointSTM;

using driver::DEVICE_HEIGHT;
using driver::DEVICE_WIDTH;
using driver::NUM_TRANS_X;
using driver::NUM_TRANS_Y;
using driver::TRANS_SPACING_MM;

}  // namespace autd3
