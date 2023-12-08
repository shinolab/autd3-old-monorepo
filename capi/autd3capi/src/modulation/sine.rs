/*
 * File: sine.rs
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

use autd3capi_def::{autd3::modulation::Sine, *};

use super::SamplingMode;

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationSine(freq: float) -> ModulationPtr {
    ModulationPtr::new(Sine::new(freq))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationSineWithSamplingConfig(
    m: ModulationPtr,
    config: SamplingConfiguration,
) -> ModulationPtr {
    ModulationPtr::new(take_mod!(m, Sine).with_sampling_config(config.into()))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationSineWithIntensity(
    m: ModulationPtr,
    intensity: u8,
) -> ModulationPtr {
    ModulationPtr::new(take_mod!(m, Sine).with_intensity(intensity))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationSineWithPhase(
    m: ModulationPtr,
    phase: float,
) -> ModulationPtr {
    ModulationPtr::new(take_mod!(m, Sine).with_phase(phase))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationSineWithOffset(
    m: ModulationPtr,
    offset: u8,
) -> ModulationPtr {
    ModulationPtr::new(take_mod!(m, Sine).with_offset(offset))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationSineWithMode(
    m: ModulationPtr,
    mode: SamplingMode,
) -> ModulationPtr {
    ModulationPtr::new(take_mod!(m, Sine).with_mode(mode.into()))
}
