/*
 * File: emit_intensity.rs
 * Project: src
 * Created Date: 11/11/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 22/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use autd3capi_common::driver::defined::float;

pub const DEFAULT_CORRECTED_ALPHA: float = 0.803;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct EmitIntensity {
    value: u8,
}

impl From<autd3capi_common::driver::common::EmitIntensity> for EmitIntensity {
    fn from(value: autd3capi_common::driver::common::EmitIntensity) -> Self {
        Self {
            value: value.value(),
        }
    }
}

impl From<EmitIntensity> for autd3capi_common::driver::common::EmitIntensity {
    fn from(value: EmitIntensity) -> Self {
        Self::new(value.value)
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDEmitIntensityNew(value: u8) -> EmitIntensity {
    autd3capi_common::driver::common::EmitIntensity::new(value).into()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDEmitIntensityNewWithCorrection(value: u8) -> EmitIntensity {
    AUTDEmitIntensityNewWithCorrectionAlpha(value, DEFAULT_CORRECTED_ALPHA)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDEmitIntensityNewWithCorrectionAlpha(
    value: u8,
    alpha: float,
) -> EmitIntensity {
    autd3capi_common::driver::common::EmitIntensity::new_with_correction_alpha(value, alpha).into()
}
