// File: autd3.hpp
// Project: autd3
// Created Date: 29/05/2023
// Author: Shun Suzuki
// -----
// Last Modified: 08/12/2023
// Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
// -----
// Copyright (c) 2023 Shun Suzuki. All rights reserved.
//

#pragma once

#include <string>

#include "autd3/datagram/debug.hpp"
#include "autd3/datagram/force_fan.hpp"
#include "autd3/datagram/mod_delay.hpp"
#include "autd3/datagram/reads_fpga_info.hpp"
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
#include "autd3/internal/controller.hpp"
#include "autd3/internal/datagram.hpp"
#include "autd3/internal/def.hpp"
#include "autd3/internal/drive.hpp"
#include "autd3/internal/emit_intensity.hpp"
#include "autd3/internal/geometry/device.hpp"
#include "autd3/internal/geometry/geometry.hpp"
#include "autd3/internal/geometry/rotation.hpp"
#include "autd3/internal/geometry/transducer.hpp"
#include "autd3/internal/phase.hpp"
#include "autd3/internal/sampling_config.hpp"
#include "autd3/internal/special.hpp"
#include "autd3/internal/stm.hpp"
#include "autd3/modulation/cache.hpp"
#include "autd3/modulation/fourier.hpp"
#include "autd3/modulation/modulation.hpp"
#include "autd3/modulation/radiation_pressure.hpp"
#include "autd3/modulation/sine.hpp"
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
static inline std::string version = "19.0.0";

using internal::geometry::AUTD3;
using internal::geometry::deg;
using internal::geometry::Device;
using internal::geometry::EulerAngles;
using internal::geometry::Geometry;
using internal::geometry::rad;
using internal::geometry::Transducer;

using gain::Gain;
using internal::Drive;
using internal::EmitIntensity;
using internal::Phase;
constexpr internal::UnitPhaseRad phase_rad = internal::rad;
using internal::SamplingConfiguration;
using modulation::Modulation;

using datagram::ConfigureDebugOutputIdx;
using datagram::ConfigureForceFan;
using datagram::ConfigureModDelay;
using datagram::ConfigureReadsFPGAInfo;
using internal::Clear;
using internal::Silencer;
using internal::Synchronize;

using internal::Stop;

using internal::ControlPoint;
using internal::FocusSTM;
using internal::GainSTM;

using internal::FirmwareInfo;
using internal::FPGAInfo;
using internal::native_methods::GainSTMMode;

using internal::Quaternion;
using internal::Vector3;

using internal::Controller;
using internal::ControllerBuilder;

using internal::native_methods::TimerStrategy;

namespace modulation {
using internal::native_methods::SamplingMode;
}

}  // namespace autd3
