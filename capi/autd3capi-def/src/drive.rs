/*
 * File: drive.rs
 * Project: src
 * Created Date: 22/11/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 23/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use autd3capi_common::driver::defined::float;

pub const DEFAULT_CORRECTED_ALPHA: float = 0.803;

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDEmitIntensityNewWithCorrection(value: u8) -> u8 {
    AUTDEmitIntensityNewWithCorrectionAlpha(value, DEFAULT_CORRECTED_ALPHA)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDEmitIntensityNewWithCorrectionAlpha(value: u8, alpha: float) -> u8 {
    autd3capi_common::driver::common::EmitIntensity::new_with_correction_alpha(value, alpha).value()
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Drive {
    pub phase: float,
    pub intensity: u8,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn drive_test() {
        assert_eq!(
            std::mem::size_of::<autd3capi_common::driver::common::Drive>(),
            std::mem::size_of::<Drive>()
        );
        assert_eq!(
            memoffset::offset_of!(autd3capi_common::driver::common::Drive, phase),
            memoffset::offset_of!(Drive, phase)
        );
        assert_eq!(
            memoffset::offset_of!(autd3capi_common::driver::common::Drive, intensity),
            memoffset::offset_of!(Drive, intensity)
        );
    }
}
