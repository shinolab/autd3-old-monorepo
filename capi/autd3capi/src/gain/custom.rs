/*
 * File: custom.rs
 * Project: gain
 * Created Date: 23/08/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 23/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use autd3capi_def::{common::*, take_gain, Drive, GainPtr};

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
    let mut drives = Vec::<autd3capi_def::common::driver::common::Drive>::with_capacity(len as _);
    drives.set_len(len as _);
    std::ptr::copy_nonoverlapping(ptr as *const _, drives.as_mut_ptr(), len as _);
    GainPtr::new(take_gain!(custom, CustomGain).set(dev_idx as _, drives))
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{
        gain::*,
        geometry::{device::*, *},
        tests::*,
        *,
    };

    use autd3capi_def::{DatagramPtr, Drive, AUTD3_TRUE};

    #[test]
    fn test_custom_gain() {
        unsafe {
            let cnt = create_controller();
            let geo = AUTDGeometry(cnt);
            let dev0 = AUTDDevice(geo, 0);
            let dev1 = AUTDDevice(geo, 1);

            let g = AUTDGainCustom();

            let num_transducers = AUTDDeviceNumTransducers(dev0);
            let drives = vec![
                Drive {
                    intensity: 0xFF,
                    phase: 0.
                };
                num_transducers as _
            ];
            let g = AUTDGainCustomSet(g, 0, drives.as_ptr(), num_transducers);

            let num_transducers = AUTDDeviceNumTransducers(dev1);
            let drives = vec![
                Drive {
                    intensity: 0xFF,
                    phase: 0.
                };
                num_transducers as _
            ];
            let g = AUTDGainCustomSet(g, 1, drives.as_ptr(), num_transducers);
            let g = AUTDGainIntoDatagram(g);

            let r = AUTDControllerSend(cnt, g, DatagramPtr(std::ptr::null()), -1);
            assert_eq!(r.result, AUTD3_TRUE);

            AUTDControllerDelete(cnt);
        }
    }
}
