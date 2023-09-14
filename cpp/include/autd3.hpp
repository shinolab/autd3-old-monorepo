// File: autd3.hpp
// Project: autd3
// Created Date: 29/05/2023
// Author: Shun Suzuki
// -----
// Last Modified: 14/09/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <string>

#include "autd3/gain/bessel.hpp"
#include "autd3/gain/cache.hpp"
#include "autd3/gain/focus.hpp"
#include "autd3/gain/gain.hpp"
#include "autd3/gain/group.hpp"
#include "autd3/gain/null.hpp"
#include "autd3/gain/plane.hpp"
#include "autd3/gain/trans_test.hpp"
#include "autd3/gain/transform.hpp"
#include "autd3/gain/uniform.hpp"
#include "autd3/internal/amplitudes.hpp"
#include "autd3/internal/controller.hpp"
#include "autd3/internal/datagram.hpp"
#include "autd3/internal/def.hpp"
#include "autd3/internal/geometry/device.hpp"
#include "autd3/internal/geometry/geometry.hpp"
#include "autd3/internal/geometry/transducer.hpp"
#include "autd3/internal/special.hpp"
#include "autd3/internal/stm.hpp"
#include "autd3/modulation/cache.hpp"
#include "autd3/modulation/fourier.hpp"
#include "autd3/modulation/modulation.hpp"
#include "autd3/modulation/radiation_pressure.hpp"
#include "autd3/modulation/sine.hpp"
#include "autd3/modulation/sine_legacy.hpp"
#include "autd3/modulation/square.hpp"
#include "autd3/modulation/static.hpp"
#include "autd3/modulation/transform.hpp"

namespace autd3 {

/**
 * @brief Mathematical constant pi
 */
constexpr double pi = internal::pi;

/**
 * @brief AUTD3 software version
 */
static inline std::string version = "15.1.0";

using internal::Device;
using internal::Geometry;
using internal::Transducer;
using Mode = internal::native_methods::TransMode;

using internal::AUTD3;

using gain::Gain;
using internal::native_methods::Drive;
using modulation::Modulation;

using internal::Clear;
using internal::ConfigureAmpFilter;
using internal::ConfigureModDelay;
using internal::ConfigurePhaseFilter;
using internal::Silencer;
using internal::Synchronize;
using internal::UpdateFlags;

using internal::Stop;

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
