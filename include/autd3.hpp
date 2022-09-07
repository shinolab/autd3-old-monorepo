// File: autd3.hpp
// Project: include
// Created Date: 10/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 07/09/2022
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include "autd3/controller.hpp"
#include "autd3/core/amplitudes.hpp"
#include "autd3/core/delay.hpp"
#include "autd3/core/geometry.hpp"
#include "autd3/core/mode.hpp"
#include "autd3/core/stm/gain.hpp"
#include "autd3/core/stm/point.hpp"
#include "autd3/gain/primitive.hpp"
#include "autd3/modulation/primitive.hpp"
#include "autd3/soft_stm.hpp"

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
using core::LegacyMode;
using core::NormalMode;
using core::NormalPhaseMode;

using core::Amplitudes;

using core::Gain;
using core::Modulation;

using core::ModDelayConfig;

using driver::FirmwareInfo;
using driver::FPGAInfo;
using driver::GainSTMMode;

using core::Quaternion;
using core::Vector3;

using core::GainSTM;
using core::Point;
using core::PointSTM;

using core::LinkPtr;

using driver::DEVICE_HEIGHT;
using driver::DEVICE_WIDTH;
using driver::NUM_TRANS_IN_UNIT;
using driver::NUM_TRANS_X;
using driver::NUM_TRANS_Y;
using driver::TRANS_SPACING_MM;

}  // namespace autd3
