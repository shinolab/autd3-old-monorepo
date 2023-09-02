/*
 * File: prelude.rs
 * Project: src
 * Created Date: 27/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 02/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

pub use spdlog::{Level, LevelFilter};

pub use crate::{
    autd3_device::AUTD3,
    controller::Controller,
    error::AUTDError,
    gain::{Bessel, Focus, Group, Null, Plane, TransducerTest, Uniform},
    link::NullLink,
    // modulation::{RadiationPressure, Sine, SineLegacy, Square, Static},
    modulation::{Sine, Static},
};

pub use crate::gain::IntoCache as IntoGainCache;
// pub use crate::modulation::Cache as ModulationCache;

pub use autd3_driver::{
    // amplitude::Amplitudes,
    datagram::DatagramT,
    // modulation::ModulationProperty,
    // stm::{ControlPoint, FocusSTM, GainSTM, GainSTMMode},
    datagram::{Clear, Silencer, Stop},
    // delay::ModDelay,
    defined::{float, METER, MILLIMETER, PI},
    fpga::{FPGA_CLK_FREQ, FPGA_SUB_CLK_FREQ},
    geometry::*,
    link::Link,
    // synchronize::Synchronize,
    // timer_strategy::TimerStrategy,
    // update_flag::UpdateFlags,
    // Mode,
};
