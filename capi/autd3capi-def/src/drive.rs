/*
 * File: drive.rs
 * Project: src
 * Created Date: 22/11/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 06/12/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use autd3_driver::{common::Phase, defined::float};

pub const DEFAULT_CORRECTED_ALPHA: float = 0.803;

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDEmitIntensityWithCorrectionAlpha(value: u8, alpha: float) -> u8 {
    autd3_driver::common::EmitIntensity::with_correction_alpha(value, alpha).value()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDPhaseFromRad(value: float) -> u8 {
    Phase::from_rad(value).value()
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDPhaseToRad(value: u8) -> float {
    Phase::new(value).radian()
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Drive {
    pub phase: u8,
    pub intensity: u8,
}
