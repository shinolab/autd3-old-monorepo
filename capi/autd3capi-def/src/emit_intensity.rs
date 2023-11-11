/*
 * File: emit_intensity.rs
 * Project: src
 * Created Date: 11/11/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 11/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use autd3capi_common::driver::defined::float;

use crate::ResultI32;

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDEmitIntensityNormalizedFrom(pulse_width: u16) -> float {
    autd3capi_common::driver::common::EmitIntensity::new_pulse_width(pulse_width)
        .unwrap()
        .normalized()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDEmitIntensityDutyRatioFrom(pulse_width: u16) -> float {
    autd3capi_common::driver::common::EmitIntensity::new_pulse_width(pulse_width)
        .unwrap()
        .duty_ratio()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDEmitIntensityPulseWidthFrom(pulse_width: u16) -> u16 {
    pulse_width
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDEmitIntensityNormalizedInto(value: float) -> ResultI32 {
    autd3capi_common::driver::common::EmitIntensity::new_normalized(value)
        .map(|v| v.pulse_width() as usize)
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDEmitIntensityNormalizedCorrectedInto(
    value: float,
    alpha: float,
) -> ResultI32 {
    autd3capi_common::driver::common::EmitIntensity::new_normalized_corrected_with_alpha(
        value, alpha,
    )
    .map(|v| v.pulse_width() as usize)
    .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDEmitIntensityDutyRatioInto(value: float) -> ResultI32 {
    autd3capi_common::driver::common::EmitIntensity::new_duty_ratio(value)
        .map(|v| v.pulse_width() as usize)
        .into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDEmitIntensityPulseWidthInto(value: u16) -> ResultI32 {
    autd3capi_common::driver::common::EmitIntensity::new_pulse_width(value)
        .map(|v| v.pulse_width() as usize)
        .into()
}
