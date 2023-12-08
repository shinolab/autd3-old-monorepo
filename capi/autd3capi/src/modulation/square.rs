/*
 * File: square.rs
 * Project: modulation
 * Created Date: 23/08/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 08/12/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

#![allow(clippy::missing_safety_doc)]

use autd3capi_def::{autd3::modulation::Square, *};

use super::SamplingMode;

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationSquare(freq: float) -> ModulationPtr {
    ModulationPtr::new(Square::new(freq))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationSquareWithLow(m: ModulationPtr, low: u8) -> ModulationPtr {
    ModulationPtr::new(take_mod!(m, Square).with_low(low))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationSquareWithHigh(m: ModulationPtr, high: u8) -> ModulationPtr {
    ModulationPtr::new(take_mod!(m, Square).with_high(high))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationSquareWithDuty(
    m: ModulationPtr,
    duty: float,
) -> ModulationPtr {
    ModulationPtr::new(take_mod!(m, Square).with_duty(duty))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationSquareWithSamplingConfig(
    m: ModulationPtr,
    config: SamplingConfiguration,
) -> ModulationPtr {
    ModulationPtr::new(take_mod!(m, Square).with_sampling_config(config.into()))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationSquareWithMode(
    m: ModulationPtr,
    mode: SamplingMode,
) -> ModulationPtr {
    ModulationPtr::new(take_mod!(m, Square).with_mode(mode.into()))
}
