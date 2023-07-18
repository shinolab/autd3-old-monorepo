// File: autd3.hpp
// Project: autd3
// Created Date: 29/05/2023
// Author: Shun Suzuki
// -----
// Last Modified: 18/07/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <string>

#include "autd3/gain/primitive.hpp"
#include "autd3/internal/amplitudes.hpp"
#include "autd3/internal/controller.hpp"
#include "autd3/internal/def.hpp"
#include "autd3/internal/geometry.hpp"
#include "autd3/internal/silencer.hpp"
#include "autd3/internal/special.hpp"
#include "autd3/internal/stm.hpp"
#include "autd3/internal/transducer.hpp"
#include "autd3/modulation/primitive.hpp"

namespace autd3 {

constexpr double pi = internal::pi;

static inline std::string version = "14.0.0";

using internal::Geometry;
using internal::Transducer;
using Mode = internal::native_methods::TransMode;
using internal::SilencerConfig;

using internal::AUTD3;

using gain::Gain;
using internal::native_methods::Drive;
using modulation::Modulation;

using internal::Clear;
using internal::ModDelayConfig;
using internal::Stop;
using internal::Synchronize;
using internal::UpdateFlags;

using internal::ControlPoint;
using internal::FocusSTM;
using internal::GainSTM;

using internal::FirmwareInfo;
using internal::FPGAInfo;
using internal::native_methods::GainSTMMode;
using LogLevel = internal::native_methods::Level;

using internal::Quaternion;
using internal::Vector3;

using internal::Controller;

using internal::Amplitudes;

using internal::native_methods::TimerStrategy;

}  // namespace autd3
