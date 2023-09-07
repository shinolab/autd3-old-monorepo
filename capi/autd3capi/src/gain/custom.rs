/*
 * File: custom.rs
 * Project: gain
 * Created Date: 23/08/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 06/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use autd3capi_def::{
    common::{driver::defined::Drive, *},
    take_gain, GainPtr,
};

#[no_mangle]
#[must_use]
#[allow(clippy::uninit_vec)]
pub unsafe extern "C" fn AUTDGainCustom() -> GainPtr {
    GainPtr::new(CustomGain::default())
}

#[no_mangle]
#[must_use]
#[allow(clippy::uninit_vec)]
pub unsafe extern "C" fn AUTDGainCustomSet(
    custom: GainPtr,
    dev_idx: u32,
    ptr: *const Drive,
    len: u32,
) -> GainPtr {
    let mut drives = Vec::<Drive>::with_capacity(len as _);
    drives.set_len(len as _);
    std::ptr::copy_nonoverlapping(ptr as *const _, drives.as_mut_ptr(), len as _);
    GainPtr::new(take_gain!(custom, CustomGain).set(dev_idx as _, drives))
}

#[cfg(test)]
mod tests {
    use std::ffi::c_char;

    use super::*;

    use crate::{
        gain::*,
        geometry::{device::*, *},
        tests::*,
        *,
    };

    use autd3capi_def::{common::driver::defined::Drive, DatagramPtr, TransMode, AUTD3_TRUE};

    #[test]
    fn test_custom_gain() {
        unsafe {
            let cnt = create_controller();
            let geo = AUTDGetGeometry(cnt);
            let dev0 = AUTDGetDevice(geo, 0);
            let dev1 = AUTDGetDevice(geo, 1);

            let g = AUTDGainCustom();

            let num_transducers = AUTDDeviceNumTransducers(dev0);
            let drives = vec![Drive { amp: 1., phase: 0. }; num_transducers as _];
            let g = AUTDGainCustomSet(g, 0, drives.as_ptr(), num_transducers);

            let num_transducers = AUTDDeviceNumTransducers(dev1);
            let drives = vec![Drive { amp: 1., phase: 0. }; num_transducers as _];
            let g = AUTDGainCustomSet(g, 1, drives.as_ptr(), num_transducers);

            let g = AUTDGainIntoDatagram(g);
            let mut err = vec![c_char::default(); 256];
            assert_eq!(
                AUTDSend(
                    cnt,
                    TransMode::Legacy,
                    DatagramPtr(std::ptr::null()),
                    g,
                    -1,
                    err.as_mut_ptr(),
                ),
                AUTD3_TRUE
            );

            AUTDFreeController(cnt);
        }
    }
}
