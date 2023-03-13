// File: autd3.hpp
// Project: include
// Created Date: 10/05/2022
// Author: Shun Suzuki
// -----
// Last Modified: 13/03/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2022 Shun Suzuki. All rights reserved.
//

#pragma once

#include "autd3/autd3_device.hpp"
#include "autd3/controller.hpp"
#include "autd3/core/amplitudes.hpp"
#include "autd3/core/clear.hpp"
#include "autd3/core/delay.hpp"
#include "autd3/core/geometry.hpp"
#include "autd3/core/mode.hpp"
#include "autd3/core/silencer_config.hpp"
#include "autd3/core/stm/focus.hpp"
#include "autd3/core/stm/gain.hpp"
#include "autd3/core/stop.hpp"
#include "autd3/core/synchronize.hpp"
#include "autd3/core/update_flag.hpp"
#include "autd3/driver/debug_level.hpp"
#include "autd3/gain/primitive.hpp"
#include "autd3/link/log.hpp"
#include "autd3/modulation/primitive.hpp"
#include "autd3/soft_stm.hpp"

namespace autd3 {

/**
 * @brief `core` namespace provides an abstraction of the basic functionality for manipulating AUTD device
 */
namespace core {}

/**
 * @brief `driver` namespace contains the logic to control autd3 firmware and constans defined by hardware and firmware
 */
namespace driver {}

/**
 * @brief `gain` namespace provides pre-defined Gain
 */
namespace gain {}

/**
 * @brief `modulation` namespace provides pre-defined Modulation
 */
namespace modulation {}

/**
 * @brief `link` namespace provides pre-defined Link
 */
namespace link {}

/**
 * @brief `extra` namespace provides features that are not essential to drive the device
 */
namespace extra {}

constexpr driver::autd3_float_t pi = driver::pi;

static inline std::string version = "8.2.1";

using core::Geometry;
using core::Mode;
using core::SilencerConfig;

using core::Amplitudes;

using core::Gain;
using core::Modulation;

using core::Clear;
using core::ModDelayConfig;
using core::Stop;
using core::Synchronize;
using core::UpdateFlag;

using driver::autd3_float_t;
using driver::Drive;

using driver::DebugLevel;
using driver::FirmwareInfo;
using driver::FPGAInfo;
using driver::GainSTMMode;

using core::Quaternion;
using core::Vector3;

using core::FocusSTM;
using core::GainSTM;

using core::LinkPtr;

}  // namespace autd3
