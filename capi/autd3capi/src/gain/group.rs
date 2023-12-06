/*
 * File: group.rs
 * Project: gain
 * Created Date: 23/08/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 06/12/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use std::collections::HashMap;

use autd3capi_def::{autd3::gain::Group, driver::autd3_device::AUTD3, *};

type M = HashMap<usize, Vec<i32>>;

#[no_mangle]
#[must_use]
#[allow(clippy::uninit_vec)]
pub unsafe extern "C" fn AUTDGainGroupCreateMap(
    device_indices_ptr: *const u32,
    num_devices: u32,
) -> GroupGainMapPtr {
    GroupGainMapPtr(Box::into_raw(Box::new(
        (0..num_devices as usize)
            .map(|i| {
                let mut v = Vec::with_capacity(AUTD3::NUM_TRANS_IN_UNIT);
                v.set_len(AUTD3::NUM_TRANS_IN_UNIT);
                (device_indices_ptr.add(i).read() as _, v)
            })
            .collect::<M>(),
    )) as _)
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDGainGroupMapSet(
    map: GroupGainMapPtr,
    dev_idx: u32,
    map_data: *const i32,
) -> GroupGainMapPtr {
    let dev_idx = dev_idx as usize;
    let mut map = Box::from_raw(map.0 as *mut M);
    std::ptr::copy_nonoverlapping(
        map_data,
        map.get_mut(&dev_idx).unwrap().as_mut_ptr(),
        map[&dev_idx].len(),
    );
    GroupGainMapPtr(Box::into_raw(map) as _)
}

#[no_mangle]
#[must_use]
#[allow(clippy::uninit_vec)]
pub unsafe extern "C" fn AUTDGainGroup(
    map: GroupGainMapPtr,
    keys_ptr: *const i32,
    values_ptr: *const GainPtr,
    kv_len: u32,
) -> GainPtr {
    let map = Box::from_raw(map.0 as *mut M);
    let mut keys = Vec::with_capacity(kv_len as usize);
    keys.set_len(kv_len as usize);
    std::ptr::copy_nonoverlapping(keys_ptr, keys.as_mut_ptr(), kv_len as usize);
    let mut values = Vec::with_capacity(kv_len as usize);
    values.set_len(kv_len as usize);
    std::ptr::copy_nonoverlapping(values_ptr, values.as_mut_ptr(), kv_len as usize);
    GainPtr::new(keys.iter().zip(values.iter()).fold(
        Group::new(move |dev, tr| {
            let key = map[&dev.idx()][tr.idx()];
            if key < 0 {
                None
            } else {
                Some(key)
            }
        }),
        |acc, (&k, v)| acc.set(k, *Box::from_raw(v.0 as *mut Box<G>)),
    ))
}
