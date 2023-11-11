/*
 * File: mod.rs
 * Project: gain
 * Created Date: 23/08/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 11/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use std::collections::HashMap;

use crate::Drive;
use autd3capi_def::{
    common::{driver::datagram::GainFilter, *},
    DatagramPtr, GainPtr, GeometryPtr, ResultGainCalcDrivesMap,
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
) -> ResultGainCalcDrivesMap {
    let geo = cast!(geometry.0, Geometry);
    Box::from_raw(gain.0 as *mut Box<G>)
        .calc(geo, GainFilter::All)
        .into()
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGainCalcGetResult(
    src: ResultGainCalcDrivesMap,
    dst: *mut Drive,
    idx: u32,
) {
    let idx = idx as usize;
    let src = cast!(src.result, HashMap<usize, Vec<Drive>>);
    std::ptr::copy_nonoverlapping(src[&idx].as_ptr(), dst, src[&idx].len());
}

#[no_mangle]
pub unsafe extern "C" fn AUTDGainCalcFreeResult(src: ResultGainCalcDrivesMap) {
    let _ = Box::from_raw(src.result as *mut HashMap<usize, Vec<Drive>>);
}

#[cfg(test)]
mod tests {
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
            let geo = AUTDGeometry(cnt);

            let dev0 = AUTDDevice(geo, 0);
            let dev1 = AUTDDevice(geo, 1);

            let g = AUTDGainUniform(0xFE);
            let g = AUTDGainUniformWithPhase(g, 0.8);

            let mut drives0 = {
                let num_trans = AUTDDeviceNumTransducers(dev0);
                vec![Drive { amp: 0, phase: 0. }; num_trans as _]
            };
            let mut drives1 = {
                let num_trans = AUTDDeviceNumTransducers(dev1);
                vec![Drive { amp: 0, phase: 0. }; num_trans as _]
            };

            let res = AUTDGainCalc(g, geo);
            assert!(!res.result.is_null());

            AUTDGainCalcGetResult(res, drives0.as_mut_ptr(), 0);
            AUTDGainCalcGetResult(res, drives1.as_mut_ptr(), 1);

            drives0.iter().for_each(|d| {
                assert_eq!(d.amp, 0xFE);
                assert_eq!(d.phase, 0.8);
            });
            drives1.iter().for_each(|d| {
                assert_eq!(d.amp, 0xFE);
                assert_eq!(d.phase, 0.8);
            });

            AUTDControllerDelete(cnt);
        }
    }
}
