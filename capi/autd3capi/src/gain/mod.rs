/*
 * File: mod.rs
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

use std::ffi::c_char;

use crate::Drive;
use autd3capi_def::{
    common::{autd3::driver::datagram::GainFilter, *},
    DatagramPtr, DevicePtr, GainPtr, AUTD3_ERR, AUTD3_TRUE,
};

pub mod bessel;
pub mod custom;
pub mod focus;
pub mod group;
pub mod null;
pub mod plane;
pub mod trans_test;
pub mod uniform;

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainIntoDatagram(gain: GainPtr) -> DatagramPtr {
    DatagramPtr::new(*Box::from_raw(gain.0 as *mut Box<G>))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainCalc(
    gain: GainPtr,
    devices: *const DevicePtr,
    drives: *const *mut Drive,
    num_dev: u32,
    err: *mut c_char,
) -> i32 {
    let devices = (0..num_dev as usize)
        .map(|i| cast!(devices.add(i).read().0, Dev))
        .collect::<Vec<_>>();
    let res = try_or_return!(
        Box::from_raw(gain.0 as *mut Box<G>).calc(&devices, GainFilter::All),
        err,
        AUTD3_ERR
    );
    (0..num_dev as usize).for_each(|i| {
        let dev = &devices[i];
        let res = &res[&dev.idx()];
        std::ptr::copy_nonoverlapping(res.as_ptr(), drives.add(i).read() as _, res.len());
    });
    AUTD3_TRUE
}

#[cfg(test)]
mod tests {
    use std::ffi::c_char;

    use super::*;

    use crate::{
        gain::uniform::*,
        geometry::{device::*, *},
        tests::*,
        Drive, *,
    };

    use autd3capi_def::AUTD3_TRUE;

    #[test]
    fn gain() {
        unsafe {
            let cnt = create_controller();
            let geo = AUTDGetGeometry(cnt);

            let dev0 = AUTDGetDevice(geo, 0);
            let dev1 = AUTDGetDevice(geo, 1);

            let g = AUTDGainUniform(0.9);
            let g = AUTDGainUniformWithPhase(g, 0.8);

            let mut drives0 = {
                let num_trans = AUTDDeviceNumTransducers(dev0);
                vec![Drive { amp: 0., phase: 0. }; num_trans as _]
            };
            let mut drives1 = {
                let num_trans = AUTDDeviceNumTransducers(dev1);
                vec![Drive { amp: 0., phase: 0. }; num_trans as _]
            };

            let devices = vec![dev0, dev1];
            let drives = vec![drives0.as_mut_ptr(), drives1.as_mut_ptr()];
            let mut err = vec![c_char::default(); 256];
            assert_eq!(
                AUTDGainCalc(g, devices.as_ptr(), drives.as_ptr(), 2, err.as_mut_ptr()),
                AUTD3_TRUE
            );

            drives0.iter().for_each(|d| {
                assert_eq!(d.amp, 0.9);
                assert_eq!(d.phase, 0.8);
            });
            drives1.iter().for_each(|d| {
                assert_eq!(d.amp, 0.9);
                assert_eq!(d.phase, 0.8);
            });

            AUTDFreeController(cnt);
        }
    }
}
