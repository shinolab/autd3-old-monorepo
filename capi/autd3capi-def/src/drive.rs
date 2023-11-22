/*
 * File: drive.rs
 * Project: src
 * Created Date: 22/11/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 22/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use autd3capi_common::driver::defined::float;

use crate::EmitIntensity;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct Drive {
    pub phase: float,
    pub intensity: EmitIntensity,
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
