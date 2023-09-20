/*
 * File: mod.rs
 * Project: gain
 * Created Date: 23/08/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 20/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use std::{collections::HashMap, ffi::c_char};

use crate::Drive;
use autd3capi_def::{
    common::{autd3::driver::datagram::GainFilter, *},
    DatagramPtr, GainCalcDrivesMapPtr, GainPtr, GeometryPtr,
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
    geometry: GeometryPtr,
    err: *mut c_char,
) -> GainCalcDrivesMapPtr {
    let geo = cast!(geometry.0, Geo);
    GainCalcDrivesMapPtr(Box::into_raw(Box::new(try_or_return!(
        Box::from_raw(gain.0 as *mut Box<G>).calc(geo, GainFilter::All),
        err,
        GainCalcDrivesMapPtr(std::ptr::null())
    ))) as _)
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGainCalcGetResult(
    src: GainCalcDrivesMapPtr,
    dst: *mut Drive,
    idx: u32,
) {
    let idx = idx as usize;
    let src = cast!(src.0, HashMap<usize, Vec<Drive>>);
    std::ptr::copy_nonoverlapping(src[&idx].as_ptr(), dst, src[&idx].len());
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGainCalcFreeResult(src: GainCalcDrivesMapPtr) {
    let _ = Box::from_raw(src.0 as *mut HashMap<usize, Vec<Drive>>);
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

            let mut err = vec![c_char::default(); 256];
            let res = AUTDGainCalc(g, geo, err.as_mut_ptr());
            assert!(!res.0.is_null());

            AUTDGainCalcGetResult(res, drives0.as_mut_ptr(), 0);
            AUTDGainCalcGetResult(res, drives1.as_mut_ptr(), 1);

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
