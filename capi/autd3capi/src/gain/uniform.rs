/*
 * File: uniform.rs
 * Project: gain
 * Created Date: 24/08/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 06/12/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use autd3capi_def::{autd3::gain::Uniform, *};
use driver::common::Phase;

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainUniform(intensity: u8) -> GainPtr {
    GainPtr::new(Uniform::new(intensity))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainUniformWithPhase(uniform: GainPtr, phase: u8) -> GainPtr {
    GainPtr::new(take_gain!(uniform, Uniform).with_phase(Phase::new(phase)))
}
