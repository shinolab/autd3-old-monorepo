/*
 * File: mod.rs
 * Project: stm
 * Created Date: 24/08/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 06/12/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

#![allow(clippy::missing_safety_doc)]

pub mod focus;
pub mod gain;

use autd3_driver::datagram::STMProps;
use autd3capi_def::*;

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMPropsNew(freq: float) -> STMPropsPtr {
    STMPropsPtr::new(STMProps::new(freq))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMPropsFromPeriod(p: u64) -> STMPropsPtr {
    STMPropsPtr::new(STMProps::from_period(std::time::Duration::from_nanos(p)))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMPropsFromSamplingConfig(
    config: SamplingConfiguration,
) -> STMPropsPtr {
    STMPropsPtr::new(STMProps::from_sampling_config(config.into()))
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
pub unsafe extern "C" fn AUTDSTMPropsPeriod(props: STMPropsPtr, size: u64) -> u64 {
    Box::from_raw(props.0 as *mut STMProps)
        .period(size as usize)
        .as_nanos() as _
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDSTMPropsSamplingConfig(
    props: STMPropsPtr,
    size: u64,
) -> ResultSamplingConfig {
    Box::from_raw(props.0 as *mut STMProps)
        .sampling_config(size as usize)
        .into()
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
