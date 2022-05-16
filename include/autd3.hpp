// File: autd3.hpp
// Project: include
// Created Date: 10/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 16/05/2022
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

/**
 * @brief core namespace provides an abstraction of the basic functionality for manipulating AUTD3.
 */
namespace core {}

/**
 * @brief driver namespace contains the logic to control the AUTD3 firmware and constans defined by hardware and firmware.
 */
namespace driver {}

constexpr double pi = driver::pi;

using core::Geometry;
using core::LegacyTransducer;
using core::NormalTransducer;
using core::Transducer;

using core::Gain;
using core::Modulation;

using driver::FirmwareInfo;
using driver::FPGAInfo;

using core::Vector3;

using core::GainSTM;
using core::Point;
using core::PointSTM;

using driver::DEVICE_HEIGHT;
using driver::DEVICE_WIDTH;
using driver::NUM_TRANS_X;
using driver::NUM_TRANS_Y;
using driver::TRANS_SPACING_MM;

}  // namespace autd3
