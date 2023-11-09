/*
 * File: cache.rs
 * Project: modulation
 * Created Date: 21/09/2023
 * Author: Shun Suzuki
 * -----
 * Last Modified: 10/11/2023
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2023 Shun Suzuki. All rights reserved.
 *
 */

#![allow(clippy::missing_safety_doc)]

use std::ffi::c_char;

use autd3capi_def::{
    common::{autd3::modulation::ModulationCache, *},
    ModulationPtr,
};

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ResultCachePtr {
    pub result: ConstPtr,
    pub buffer_len: u32,
    pub err_len: u32,
    pub err: *const c_char,
}

impl From<Result<autd3capi_def::common::autd3::modulation::ModulationCache, AUTDInternalError>>
    for ResultCachePtr
{
    fn from(
        r: Result<autd3capi_def::common::autd3::modulation::ModulationCache, AUTDInternalError>,
    ) -> Self {
        match r {
            Ok(v) => Self {
                buffer_len: v.buffer().len() as u32,
                result: Box::into_raw(Box::new(v)) as _,
                err_len: 0,
                err: std::ptr::null(),
            },
            Err(e) => {
                let err = std::ffi::CString::new(e.to_string()).unwrap();
                Self {
                    result: NULL,
                    buffer_len: 0,
                    err_len: err.as_bytes_with_nul().len() as u32,
                    err: err.into_raw(),
                }
            }
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn AUTDResultCachePtrGetErr(r: ResultCachePtr, err: *mut c_char) {
    let err_ = std::ffi::CString::from_raw(r.err as *mut c_char);
    libc::strcpy(err, err_.as_ptr());
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationWithCache(m: ModulationPtr) -> ResultCachePtr {
    Box::from_raw(m.0 as *mut Box<M>).with_cache().into()
}

#[no_mangle]
pub unsafe extern "C" fn AUTDModulationCacheGetBuffer(m: ResultCachePtr, buf: *mut float) {
    let cache = cast!(m.result, ModulationCache);
    std::ptr::copy_nonoverlapping(cache.buffer().as_ptr(), buf, cache.buffer().len());
}

#[no_mangle]
#[must_use]
pub unsafe extern "C" fn AUTDModulationCacheIntoModulation(m: ResultCachePtr) -> ModulationPtr {
    ModulationPtr::new(cast!(m.result, ModulationCache).clone())
}

#[no_mangle]
pub unsafe extern "C" fn AUTDModulationCacheDelete(m: ResultCachePtr) {
    let _ = Box::from_raw(m.result as *mut ModulationCache);
}

#[cfg(test)]
mod tests {
    use super::{super::sine::AUTDModulationSine, *};

    use crate::{modulation::*, tests::*, *};

    use autd3capi_def::{DatagramPtr, AUTD3_TRUE};

    #[test]
    fn test_modulation_cache() {
        unsafe {
            let cnt = create_controller();

            let m = AUTDModulationSine(150);

            let cache = AUTDModulationWithCache(m);
            assert!(!cache.result.is_null());
            let m = AUTDModulationIntoDatagram(AUTDModulationCacheIntoModulation(cache));

            let r = AUTDControllerSend(cnt, m, DatagramPtr(std::ptr::null()), -1);
            assert_eq!(r.result, AUTD3_TRUE);

            AUTDModulationCacheDelete(cache);
            AUTDControllerDelete(cnt);
        }
    }
}
