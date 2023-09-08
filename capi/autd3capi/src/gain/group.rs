/*
 * File: group.rs
 * Project: gain
 * Created Date: 23/08/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 08/09/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

use std::collections::HashMap;

use autd3capi_def::{common::*, GainPtr};

#[no_mangle]
#[must_use]
#[allow(clippy::uninit_vec)]
pub unsafe extern "C" fn AUTDGainGroup(
    device_indices_ptr: *const u32,
    map_ptr: *const *const i32,
    num_devices: u32,
    keys_ptr: *const i32,
    values_ptr: *const GainPtr,
    kv_len: u32,
) -> GainPtr {
    let map = (0..num_devices as usize)
        .map(|i| {
            let mut v = Vec::with_capacity(AUTD3::NUM_TRANS_IN_UNIT);
            v.set_len(AUTD3::NUM_TRANS_IN_UNIT);
            std::ptr::copy_nonoverlapping(
                map_ptr.add(i).read(),
                v.as_mut_ptr(),
                AUTD3::NUM_TRANS_IN_UNIT,
            );
            (device_indices_ptr.add(i).read() as usize, v)
        })
        .collect::<HashMap<usize, Vec<_>>>();
    let mut keys = Vec::with_capacity(kv_len as usize);
    keys.set_len(kv_len as usize);
    std::ptr::copy_nonoverlapping(keys_ptr, keys.as_mut_ptr(), kv_len as usize);
    let mut values = Vec::with_capacity(kv_len as usize);
    values.set_len(kv_len as usize);
    std::ptr::copy_nonoverlapping(values_ptr, values.as_mut_ptr(), kv_len as usize);
    GainPtr::new(keys.iter().zip(values.iter()).fold(
        Group::new(move |dev, tr: &DynamicTransducer| {
            let key = map[&dev.idx()][tr.local_idx()];
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
        geometry::{
            device::{AUTDDeviceNumTransducers, AUTDGetDevice},
            AUTDGetGeometry,
        },
        tests::*,
        *,
    };

    use autd3capi_def::{DatagramPtr, TransMode, AUTD3_TRUE};

    #[test]
    fn test_group_by_transducer() {
        unsafe {
            let cnt = create_controller();
            let geo = AUTDGetGeometry(cnt);

            let dev0 = AUTDGetDevice(geo, 0);
            let dev1 = AUTDGetDevice(geo, 1);

            let num_transducer = AUTDDeviceNumTransducers(dev0);
            let map0 = vec![0; num_transducer as _];
            let num_transducer = AUTDDeviceNumTransducers(dev1);
            let map1 = vec![1; num_transducer as _];

            let device_indices = [0, 1];
            let map = [map0.as_ptr(), map1.as_ptr()];
            let keys = [0, 1];
            let values = [AUTDGainNull(), AUTDGainNull()];

            let g = AUTDGainGroup(
                device_indices.as_ptr(),
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
