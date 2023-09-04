/*
 * File: prelude.rs
 * Project: src
 * Created Date: 27/04/2022
 * Author: Shun Suzuki
 * -----
 * Last Modified: 04/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2022-2023 Shun Suzuki. All rights reserved.
 *
 */

pub use spdlog::{Level, LevelFilter};

pub use crate::gain::IntoCache as IntoGainCache;
pub use crate::modulation::IntoCache as IntoModulationCache;

pub use crate::{
    autd3_device::AUTD3,
    controller::Controller,
    error::AUTDError,
    gain::{Bessel, Focus, Group, Null, Plane, TransducerTest, Uniform},
    link::NullLink,
    modulation::{IntoRadiationPressure, IntoTransform, Sine, SineLegacy, Square, Static},
};

pub use autd3_driver::{
    datagram::{
        AmpFilter, Clear, DatagramT, ModDelay, ModulationProperty, PhaseFilter, Silencer, Stop,
        Synchronize,
    },
    // amplitude::Amplitudes,
    defined::{float, METER, MILLIMETER, PI},
    fpga::{FPGA_CLK_FREQ, FPGA_SUB_CLK_FREQ},
    geometry::*,
    link::Link,
};
