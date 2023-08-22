/*
 * File: group.rs
 * Project: gain
 * Created Date: 23/08/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 23/08/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use autd3capi_def::{common::*, GainPtr};
#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainGroupByDevice(
    map_ptr: *const i32,
    map_len: u64,
    keys_ptr: *const i32,
    values_ptr: *const GainPtr,
    kv_len: u64,
) -> GainPtr {
    let mut map = vec![0i32; map_len as usize];
    std::ptr::copy_nonoverlapping(map_ptr, map.as_mut_ptr(), map_len as usize);
    let mut keys = vec![0i32; kv_len as usize];
    std::ptr::copy_nonoverlapping(keys_ptr, keys.as_mut_ptr(), kv_len as usize);
    let mut values = vec![GainPtr(std::ptr::null()); kv_len as usize];
    std::ptr::copy_nonoverlapping(values_ptr, values.as_mut_ptr(), kv_len as usize);
    GainPtr::new(keys.iter().zip(values.iter()).fold(
        Group::by_device(move |dev| {
            let key = map[dev];
            if key < 0 {
                None
            } else {
                Some(key)
            }
        }),
        |acc, (&k, v)| acc.set(k, *Box::from_raw(v.0 as *mut Box<G>)),
    ))
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainGroupByTransducer(
    map_ptr: *const i32,
    map_len: u64,
    keys_ptr: *const i32,
    values_ptr: *const GainPtr,
    kv_len: u64,
) -> GainPtr {
    let mut map = vec![0i32; map_len as usize];
    std::ptr::copy_nonoverlapping(map_ptr, map.as_mut_ptr(), map_len as usize);
    let mut keys = vec![0i32; kv_len as usize];
    std::ptr::copy_nonoverlapping(keys_ptr, keys.as_mut_ptr(), kv_len as usize);
    let mut values = vec![GainPtr(std::ptr::null()); kv_len as usize];
    std::ptr::copy_nonoverlapping(values_ptr, values.as_mut_ptr(), kv_len as usize);
    GainPtr::new(keys.iter().zip(values.iter()).fold(
        Group::by_transducer(move |tr: &DynamicTransducer| {
            let key = map[tr.idx()];
            if key < 0 {
                None
            } else {
                Some(key)
            }
        }),
        |acc, (&k, v)| acc.set(k, *Box::from_raw(v.0 as *mut Box<G>)),
    ))
}

#[cfg(test)]
mod tests {
    use std::ffi::c_char;

    use super::*;

    use crate::{
        gain::{null::AUTDGainNull, *},
        tests::*,
        *,
    };

    use autd3capi_def::{DatagramHeaderPtr, TransMode, AUTD3_TRUE};

    #[test]
    fn test_group_by_device() {
        unsafe {
            let cnt = create_controller();

            let map = vec![0, 1];
            let keys = vec![0, 1];
            let values = vec![AUTDGainNull(), AUTDGainNull()];
            let g = AUTDGainGroupByDevice(
                map.as_ptr(),
                map.len() as _,
                keys.as_ptr(),
                values.as_ptr(),
                values.len() as _,
            );

            let g = AUTDGainIntoDatagram(g);

            let mut err = vec![c_char::default(); 256];
            assert_eq!(
                AUTDSend(
                    cnt,
                    TransMode::Legacy,
                    DatagramHeaderPtr(std::ptr::null()),
                    g,
                    -1,
                    err.as_mut_ptr(),
                ),
                AUTD3_TRUE
            );
        }
    }

    #[test]
    fn test_group_by_transducer() {
        unsafe {
            let cnt = create_controller();
            let geo = AUTDGetGeometry(cnt);
            let num_transducers = AUTDNumTransducers(geo);

            let map = vec![0; num_transducers as _];
            let keys = vec![0];
            let values = vec![AUTDGainNull()];

            let g = AUTDGainGroupByTransducer(
                map.as_ptr(),
                map.len() as _,
                keys.as_ptr(),
                values.as_ptr(),
                values.len() as _,
            );

            let g = AUTDGainIntoDatagram(g);

            let mut err = vec![c_char::default(); 256];
            assert_eq!(
                AUTDSend(
                    cnt,
                    TransMode::Legacy,
                    DatagramHeaderPtr(std::ptr::null()),
                    g,
                    -1,
                    err.as_mut_ptr(),
                ),
                AUTD3_TRUE
            );
        }
    }
}
