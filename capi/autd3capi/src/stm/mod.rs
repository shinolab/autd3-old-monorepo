/*
 * File: mod.rs
 * Project: stm
 * Created Date: 24/08/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 24/08/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

#![allow(clippy::missing_safety_doc)]

pub mod focus;
pub mod gain;

use autd3_core::stm::STMProps;
use autd3capi_def::{common::*, STMPropsPtr};

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMProps(freq: float) -> STMPropsPtr {
    STMPropsPtr::new(STMProps::new(freq))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMPropsWithSamplingFreq(freq: float) -> STMPropsPtr {
    STMPropsPtr::new(STMProps::with_sampling_frequency(freq))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMPropsWithSamplingFreqDiv(div: u32) -> STMPropsPtr {
    STMPropsPtr::new(STMProps::with_sampling_frequency_division(div))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMPropsWithStartIdx(props: STMPropsPtr, idx: i32) -> STMPropsPtr {
    let props = Box::from_raw(props.0 as *mut STMProps);
    STMPropsPtr::new(if idx < 0 {
        props.with_start_idx(None)
    } else {
        props.with_start_idx(Some(idx as u16))
    })
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMPropsWithFinishIdx(props: STMPropsPtr, idx: i32) -> STMPropsPtr {
    let props = Box::from_raw(props.0 as *mut STMProps);
    STMPropsPtr::new(if idx < 0 {
        props.with_finish_idx(None)
    } else {
        props.with_finish_idx(Some(idx as u16))
    })
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMPropsFrequency(props: STMPropsPtr, size: u64) -> float {
    Box::from_raw(props.0 as *mut STMProps).freq(size as usize)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMPropsSamplingFrequency(props: STMPropsPtr, size: u64) -> float {
    Box::from_raw(props.0 as *mut STMProps).sampling_frequency(size as usize)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMPropsSamplingFrequencyDivision(
    props: STMPropsPtr,
    size: u64,
) -> u32 {
    Box::from_raw(props.0 as *mut STMProps).sampling_frequency_division(size as usize)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMPropsStartIdx(props: STMPropsPtr) -> i32 {
    if let Some(idx) = cast!(props.0, STMProps).start_idx() {
        idx as i32
    } else {
        -1
    }
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMPropsFinishIdx(props: STMPropsPtr) -> i32 {
    if let Some(idx) = cast!(props.0, STMProps).finish_idx() {
        idx as i32
    } else {
        -1
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn stm_props() {
        unsafe {
            let props = AUTDSTMProps(1.);
            assert_eq!(1., AUTDSTMPropsFrequency(props, 0));

            let props = AUTDSTMPropsWithSamplingFreq(1.);
            assert_eq!(1., AUTDSTMPropsSamplingFrequency(props, 0));

            let props = AUTDSTMPropsWithSamplingFreqDiv(512);
            assert_eq!(512, AUTDSTMPropsSamplingFrequencyDivision(props, 0));

            let props = AUTDSTMPropsWithStartIdx(props, 0);
            assert_eq!(0, AUTDSTMPropsStartIdx(props));

            let props = AUTDSTMPropsWithFinishIdx(props, 1);
            assert_eq!(1, AUTDSTMPropsFinishIdx(props));
        }
    }
}
